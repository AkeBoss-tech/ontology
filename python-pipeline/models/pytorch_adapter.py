"""
PyTorch Model Adapter

Adapter for PyTorch models providing serialization,
prediction, and evaluation capabilities.
"""

import os
import time
import json
from typing import Any, Dict, List, Optional, Callable
from datetime import datetime
import numpy as np

from .model_adapter import (
    ModelAdapter,
    ModelFramework,
    ModelType,
    ModelMetadata,
    ModelMetrics,
    PredictionResult,
)


class PyTorchAdapter(ModelAdapter):
    """
    Adapter for PyTorch models.
    
    Supports PyTorch nn.Module models for inference.
    Handles CPU/GPU placement and input preprocessing.
    """
    
    def __init__(self, device: Optional[str] = None):
        """
        Initialize PyTorch adapter.
        
        Args:
            device: Target device ('cpu', 'cuda', 'mps', or None for auto-detect)
        """
        self._device = device
        self._preprocessor: Optional[Callable] = None
        self._postprocessor: Optional[Callable] = None
    
    @property
    def framework(self) -> ModelFramework:
        return ModelFramework.PYTORCH
    
    @property
    def device(self) -> str:
        """Get the target device, auto-detecting if needed."""
        if self._device:
            return self._device
        
        try:
            import torch
            if torch.cuda.is_available():
                return 'cuda'
            elif hasattr(torch.backends, 'mps') and torch.backends.mps.is_available():
                return 'mps'
        except ImportError:
            pass
        return 'cpu'
    
    def set_preprocessor(self, fn: Callable) -> None:
        """Set input preprocessing function."""
        self._preprocessor = fn
    
    def set_postprocessor(self, fn: Callable) -> None:
        """Set output postprocessing function."""
        self._postprocessor = fn
    
    def serialize(self, model: Any, path: str, metadata: Optional[ModelMetadata] = None) -> str:
        """
        Serialize PyTorch model to disk.
        
        Args:
            model: PyTorch nn.Module
            path: Base path for saving
            metadata: Optional metadata
            
        Returns:
            Full path to saved model
        """
        import torch
        
        # Ensure directory exists
        os.makedirs(os.path.dirname(path) if os.path.dirname(path) else '.', exist_ok=True)
        
        # Add extension if not present
        if not path.endswith('.pt') and not path.endswith('.pth'):
            path = f"{path}.pt"
        
        # Save model state dict (recommended approach)
        torch.save({
            'model_state_dict': model.state_dict(),
            'model_class': model.__class__.__name__,
        }, path)
        
        # Optionally save full model
        full_model_path = path.replace('.pt', '_full.pt')
        torch.save(model, full_model_path)
        
        # Save metadata
        if metadata:
            metadata_path = path.replace('.pt', '_metadata.json').replace('.pth', '_metadata.json')
            with open(metadata_path, 'w') as f:
                json.dump({
                    'model_id': metadata.model_id,
                    'name': metadata.name,
                    'version': metadata.version,
                    'framework': metadata.framework.value,
                    'model_type': metadata.model_type.value,
                    'description': metadata.description,
                    'input_schema': metadata.input_schema,
                    'output_schema': metadata.output_schema,
                    'created_at': metadata.created_at.isoformat(),
                    'tags': metadata.tags,
                }, f, indent=2)
        
        return path
    
    def deserialize(self, path: str, model_class: Optional[type] = None) -> Any:
        """
        Load PyTorch model from disk.
        
        Args:
            path: Path to saved model
            model_class: Optional model class for state dict loading
            
        Returns:
            Loaded PyTorch model
        """
        import torch
        
        # Add extension if needed
        if not path.endswith('.pt') and not path.endswith('.pth'):
            path = f"{path}.pt"
        
        # Try loading full model first
        full_model_path = path.replace('.pt', '_full.pt')
        if os.path.exists(full_model_path):
            model = torch.load(full_model_path, map_location=self.device)
            model.eval()
            return model
        
        # Load checkpoint
        checkpoint = torch.load(path, map_location=self.device)
        
        if model_class is None:
            raise ValueError("model_class required when loading from state_dict")
        
        model = model_class()
        model.load_state_dict(checkpoint['model_state_dict'])
        model.to(self.device)
        model.eval()
        
        return model
    
    def predict(self, model: Any, inputs: Dict[str, Any]) -> PredictionResult:
        """
        Execute prediction with PyTorch model.
        
        Args:
            model: PyTorch nn.Module
            inputs: Dictionary with 'features' or 'tensor' key
            
        Returns:
            PredictionResult with predictions
        """
        import torch
        
        start_time = time.time()
        
        # Extract and prepare inputs
        if 'tensor' in inputs:
            X = inputs['tensor']
        elif 'features' in inputs:
            X = torch.tensor(inputs['features'], dtype=torch.float32)
        else:
            X = torch.tensor([list(inputs.values())], dtype=torch.float32)
        
        # Ensure batch dimension
        if X.dim() == 1:
            X = X.unsqueeze(0)
        
        # Apply preprocessor if set
        if self._preprocessor:
            X = self._preprocessor(X)
        
        # Move to device
        X = X.to(self.device)
        model = model.to(self.device)
        model.eval()
        
        # Inference
        with torch.no_grad():
            outputs = model(X)
        
        # Apply postprocessor if set
        if self._postprocessor:
            outputs = self._postprocessor(outputs)
        
        # Convert to numpy
        predictions = outputs.cpu().numpy()
        
        # Handle classification outputs
        confidence = None
        probabilities = None
        if predictions.ndim > 1 and predictions.shape[1] > 1:
            # Softmax for classification
            from torch.nn.functional import softmax
            probs = softmax(outputs, dim=1).cpu().numpy()
            probabilities = probs.tolist()
            confidence = [float(max(p)) for p in probs]
            predictions = np.argmax(predictions, axis=1)
        
        execution_time = (time.time() - start_time) * 1000
        
        return PredictionResult(
            predictions=predictions.tolist() if hasattr(predictions, 'tolist') else predictions,
            confidence=confidence,
            probabilities=probabilities,
            execution_time_ms=execution_time,
        )
    
    def predict_batch(self, model: Any, inputs: List[Dict[str, Any]]) -> List[PredictionResult]:
        """
        Execute batch predictions.
        
        Args:
            model: PyTorch nn.Module
            inputs: List of input dictionaries
            
        Returns:
            List of PredictionResults
        """
        import torch
        
        # Combine inputs into batch
        if 'features' in inputs[0]:
            X = torch.tensor([inp['features'] for inp in inputs], dtype=torch.float32)
        elif 'tensor' in inputs[0]:
            X = torch.stack([inp['tensor'] for inp in inputs])
        else:
            X = torch.tensor([list(inp.values()) for inp in inputs], dtype=torch.float32)
        
        # Single batch prediction
        result = self.predict(model, {'tensor': X})
        
        # Split into individual results
        predictions = result.predictions
        results = []
        for i in range(len(inputs)):
            prob = result.probabilities[i] if result.probabilities else None
            conf = result.confidence[i] if result.confidence else None
            results.append(PredictionResult(
                predictions=[predictions[i]] if isinstance(predictions, list) else predictions[i],
                confidence=[conf] if conf else None,
                probabilities=[prob] if prob else None,
                execution_time_ms=result.execution_time_ms / len(inputs) if result.execution_time_ms else None,
            ))
        
        return results
    
    def evaluate(self, model: Any, X: Any, y: Any, model_type: ModelType) -> ModelMetrics:
        """
        Evaluate PyTorch model performance.
        
        Args:
            model: PyTorch nn.Module
            X: Feature data (numpy array or tensor)
            y: Target data
            model_type: Type of model
            
        Returns:
            ModelMetrics with computed metrics
        """
        import torch
        from sklearn import metrics as sklearn_metrics
        
        # Convert to tensors if needed
        if not isinstance(X, torch.Tensor):
            X = torch.tensor(X, dtype=torch.float32)
        if not isinstance(y, np.ndarray):
            y = np.array(y)
        
        # Get predictions
        X = X.to(self.device)
        model = model.to(self.device)
        model.eval()
        
        with torch.no_grad():
            outputs = model(X)
        
        outputs_np = outputs.cpu().numpy()
        
        result = ModelMetrics(
            evaluated_at=datetime.utcnow(),
            sample_count=len(y),
        )
        
        if model_type == ModelType.CLASSIFICATION:
            # Get class predictions
            if outputs_np.ndim > 1 and outputs_np.shape[1] > 1:
                y_pred = np.argmax(outputs_np, axis=1)
            else:
                y_pred = (outputs_np > 0.5).astype(int).flatten()
            
            result.accuracy = sklearn_metrics.accuracy_score(y, y_pred)
            average = 'weighted' if len(set(y)) > 2 else 'binary'
            result.precision = sklearn_metrics.precision_score(y, y_pred, average=average, zero_division=0)
            result.recall = sklearn_metrics.recall_score(y, y_pred, average=average, zero_division=0)
            result.f1_score = sklearn_metrics.f1_score(y, y_pred, average=average, zero_division=0)
        
        elif model_type == ModelType.REGRESSION:
            y_pred = outputs_np.flatten()
            result.rmse = np.sqrt(sklearn_metrics.mean_squared_error(y, y_pred))
            result.mae = sklearn_metrics.mean_absolute_error(y, y_pred)
            result.r2_score = sklearn_metrics.r2_score(y, y_pred)
        
        return result
