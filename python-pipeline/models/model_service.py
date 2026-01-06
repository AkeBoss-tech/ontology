"""
gRPC Model Service

gRPC service for executing model predictions from Rust.
Provides a lightweight RPC interface for model execution.
"""

import json
import time
import asyncio
from typing import Any, Dict, Optional
from concurrent import futures
import logging

# Try to import grpcio - will be optional dependency
try:
    import grpc
    from grpc import aio
    GRPC_AVAILABLE = True
except ImportError:
    GRPC_AVAILABLE = False

from .model_adapter import (
    ModelRegistry,
    ModelFramework,
    ModelType,
    ModelMetadata,
    PredictionResult,
    create_adapter,
)

logger = logging.getLogger(__name__)


# Protocol buffer message equivalents (for when protos aren't generated)
class PredictRequest:
    """Request for model prediction."""
    def __init__(
        self,
        model_id: str,
        inputs: Dict[str, Any],
        use_cache: bool = True,
    ):
        self.model_id = model_id
        self.inputs = inputs
        self.use_cache = use_cache


class PredictResponse:
    """Response from model prediction."""
    def __init__(
        self,
        predictions: Any,
        confidence: Optional[list] = None,
        probabilities: Optional[list] = None,
        execution_time_ms: float = 0.0,
        cached: bool = False,
        error: Optional[str] = None,
    ):
        self.predictions = predictions
        self.confidence = confidence
        self.probabilities = probabilities
        self.execution_time_ms = execution_time_ms
        self.cached = cached
        self.error = error
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'predictions': self.predictions,
            'confidence': self.confidence,
            'probabilities': self.probabilities,
            'execution_time_ms': self.execution_time_ms,
            'cached': self.cached,
            'error': self.error,
        }


class ModelService:
    """
    Model service that can run as gRPC or HTTP server.
    
    Handles model loading, caching, and prediction requests.
    """
    
    def __init__(self, model_registry: Optional[ModelRegistry] = None):
        """
        Initialize model service.
        
        Args:
            model_registry: Optional pre-configured registry
        """
        self.registry = model_registry or ModelRegistry()
        self._cache: Dict[str, tuple[Any, float]] = {}  # key -> (result, timestamp)
        self._cache_ttl = 3600  # 1 hour default
        
        # Register default adapters
        try:
            from .sklearn_adapter import SklearnAdapter
            self.registry.register_adapter(SklearnAdapter())
        except ImportError:
            pass
        
        try:
            from .pytorch_adapter import PyTorchAdapter
            self.registry.register_adapter(PyTorchAdapter())
        except ImportError:
            pass
        
        try:
            from .tensorflow_adapter import TensorFlowAdapter
            self.registry.register_adapter(TensorFlowAdapter())
        except ImportError:
            pass
    
    def _cache_key(self, model_id: str, inputs: Dict[str, Any]) -> str:
        """Generate cache key from model ID and inputs."""
        import hashlib
        input_str = json.dumps(inputs, sort_keys=True, default=str)
        return f"{model_id}:{hashlib.md5(input_str.encode()).hexdigest()}"
    
    def _get_cached(self, key: str) -> Optional[PredictionResult]:
        """Get cached prediction if available and not expired."""
        if key in self._cache:
            result, timestamp = self._cache[key]
            if time.time() - timestamp < self._cache_ttl:
                return result
            else:
                del self._cache[key]
        return None
    
    def _set_cached(self, key: str, result: PredictionResult) -> None:
        """Cache a prediction result."""
        self._cache[key] = (result, time.time())
    
    def load_model(
        self,
        model_id: str,
        path: str,
        framework: str,
        model_type: str,
        name: str = "",
        version: str = "1.0.0",
    ) -> Dict[str, str]:
        """
        Load a model from disk into the registry.
        
        Args:
            model_id: Unique identifier for the model
            path: Path to the saved model
            framework: ML framework ('sklearn', 'pytorch', 'tensorflow')
            model_type: Type of model ('classification', 'regression', etc.)
            name: Human-readable name
            version: Model version
            
        Returns:
            Status dict
        """
        framework_enum = ModelFramework(framework.lower())
        type_enum = ModelType(model_type.lower())
        
        adapter = self.registry.get_adapter(framework_enum)
        model = adapter.deserialize(path)
        
        metadata = ModelMetadata(
            model_id=model_id,
            name=name or model_id,
            version=version,
            framework=framework_enum,
            model_type=type_enum,
        )
        
        self.registry.register_model(model, metadata)
        
        return {'status': 'loaded', 'model_id': model_id}
    
    def predict(self, request: PredictRequest) -> PredictResponse:
        """
        Execute a prediction.
        
        Args:
            request: Prediction request
            
        Returns:
            Prediction response
        """
        try:
            # Check cache
            if request.use_cache:
                cache_key = self._cache_key(request.model_id, request.inputs)
                cached = self._get_cached(cache_key)
                if cached:
                    return PredictResponse(
                        predictions=cached.predictions,
                        confidence=cached.confidence,
                        probabilities=cached.probabilities,
                        execution_time_ms=0,
                        cached=True,
                    )
            
            # Execute prediction
            result = self.registry.predict(request.model_id, request.inputs)
            
            # Cache result
            if request.use_cache:
                self._set_cached(cache_key, result)
            
            return PredictResponse(
                predictions=result.predictions,
                confidence=result.confidence,
                probabilities=result.probabilities,
                execution_time_ms=result.execution_time_ms or 0,
                cached=False,
            )
        
        except Exception as e:
            logger.exception(f"Prediction error for model {request.model_id}")
            return PredictResponse(
                predictions=None,
                error=str(e),
            )
    
    def list_models(self) -> list:
        """List all loaded models."""
        models = self.registry.list_models()
        return [
            {
                'model_id': m.model_id,
                'name': m.name,
                'version': m.version,
                'framework': m.framework.value,
                'model_type': m.model_type.value,
            }
            for m in models
        ]


class HTTPModelServer:
    """
    Simple HTTP server for model predictions.
    
    Alternative to gRPC for simpler deployments.
    """
    
    def __init__(self, service: ModelService, port: int = 50051):
        self.service = service
        self.port = port
        self._app = None
    
    def _create_app(self):
        """Create Flask/FastAPI app."""
        try:
            from fastapi import FastAPI, HTTPException
            from pydantic import BaseModel
            from typing import List, Optional, Any
            
            app = FastAPI(title="Model Service", version="1.0.0")
            
            class PredictRequestModel(BaseModel):
                model_id: str
                inputs: Dict[str, Any]
                use_cache: bool = True
            
            @app.get("/models")
            def list_models():
                return self.service.list_models()
            
            @app.post("/predict")
            def predict(request: PredictRequestModel):
                req = PredictRequest(
                    model_id=request.model_id,
                    inputs=request.inputs,
                    use_cache=request.use_cache,
                )
                response = self.service.predict(req)
                if response.error:
                    raise HTTPException(status_code=500, detail=response.error)
                return response.to_dict()
            
            @app.post("/load")
            def load_model(
                model_id: str,
                path: str,
                framework: str,
                model_type: str,
                name: str = "",
                version: str = "1.0.0",
            ):
                return self.service.load_model(
                    model_id=model_id,
                    path=path,
                    framework=framework,
                    model_type=model_type,
                    name=name,
                    version=version,
                )
            
            return app
        except ImportError:
            raise ImportError("FastAPI required for HTTP server. Install with: pip install fastapi uvicorn")
    
    def run(self):
        """Run the HTTP server."""
        import uvicorn
        app = self._create_app()
        uvicorn.run(app, host="0.0.0.0", port=self.port)


def serve(port: int = 50051, use_http: bool = True):
    """
    Start the model service.
    
    Args:
        port: Port to listen on
        use_http: Use HTTP instead of gRPC
    """
    service = ModelService()
    
    if use_http:
        server = HTTPModelServer(service, port=port)
        logger.info(f"Starting HTTP model server on port {port}")
        server.run()
    else:
        if not GRPC_AVAILABLE:
            raise ImportError("gRPC not available. Install with: pip install grpcio")
        
        # gRPC server implementation would go here
        logger.info(f"Starting gRPC model server on port {port}")
        raise NotImplementedError("gRPC server not yet implemented. Use HTTP mode.")


if __name__ == '__main__':
    import argparse
    
    parser = argparse.ArgumentParser(description='Model Service')
    parser.add_argument('--port', type=int, default=50051, help='Port to listen on')
    parser.add_argument('--http', action='store_true', default=True, help='Use HTTP instead of gRPC')
    
    args = parser.parse_args()
    
    logging.basicConfig(level=logging.INFO)
    serve(port=args.port, use_http=args.http)
