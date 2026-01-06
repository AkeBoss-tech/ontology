"""
Model Adapter Interface

Base interface for ML model adapters that handle serialization,
deserialization, and prediction across different ML frameworks.
"""

from abc import ABC, abstractmethod
from typing import Any, Dict, List, Optional, Union
from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
import json
import hashlib


class ModelFramework(Enum):
    """Supported ML frameworks."""
    SKLEARN = "sklearn"
    PYTORCH = "pytorch"
    TENSORFLOW = "tensorflow"
    XGBOOST = "xgboost"
    LIGHTGBM = "lightgbm"
    CUSTOM = "custom"


class ModelType(Enum):
    """Types of ML models."""
    CLASSIFICATION = "classification"
    REGRESSION = "regression"
    CLUSTERING = "clustering"
    TIME_SERIES = "time_series"
    ANOMALY_DETECTION = "anomaly_detection"
    RECOMMENDATION = "recommendation"
    CUSTOM = "custom"


@dataclass
class ModelMetadata:
    """Metadata about a registered model."""
    model_id: str
    name: str
    version: str
    framework: ModelFramework
    model_type: ModelType
    description: Optional[str] = None
    input_schema: Optional[Dict[str, str]] = None  # input_name -> type
    output_schema: Optional[Dict[str, str]] = None  # output_name -> type
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)
    tags: List[str] = field(default_factory=list)
    custom_metadata: Dict[str, Any] = field(default_factory=dict)


@dataclass
class ModelMetrics:
    """Performance metrics for a model."""
    # Classification metrics
    accuracy: Optional[float] = None
    precision: Optional[float] = None
    recall: Optional[float] = None
    f1_score: Optional[float] = None
    auc_roc: Optional[float] = None
    
    # Regression metrics
    rmse: Optional[float] = None
    mae: Optional[float] = None
    r2_score: Optional[float] = None
    mape: Optional[float] = None
    
    # Clustering metrics
    silhouette_score: Optional[float] = None
    davies_bouldin_score: Optional[float] = None
    
    # Custom metrics
    custom_metrics: Dict[str, float] = field(default_factory=dict)
    
    # Evaluation metadata
    evaluated_at: Optional[datetime] = None
    evaluation_dataset: Optional[str] = None
    sample_count: Optional[int] = None

    def to_dict(self) -> Dict[str, Any]:
        """Convert metrics to dictionary."""
        result = {}
        for field_name in ['accuracy', 'precision', 'recall', 'f1_score', 'auc_roc',
                          'rmse', 'mae', 'r2_score', 'mape',
                          'silhouette_score', 'davies_bouldin_score']:
            value = getattr(self, field_name)
            if value is not None:
                result[field_name] = value
        if self.custom_metrics:
            result['custom_metrics'] = self.custom_metrics
        return result

    def primary_metric(self, model_type: ModelType) -> Optional[float]:
        """Get the primary metric based on model type."""
        if model_type == ModelType.CLASSIFICATION:
            return self.f1_score or self.accuracy
        elif model_type == ModelType.REGRESSION:
            return self.r2_score
        elif model_type == ModelType.CLUSTERING:
            return self.silhouette_score
        return None


@dataclass
class PredictionResult:
    """Result of a model prediction."""
    predictions: Any
    confidence: Optional[List[float]] = None
    probabilities: Optional[List[List[float]]] = None
    execution_time_ms: Optional[float] = None
    model_id: Optional[str] = None
    cached: bool = False


class ModelAdapter(ABC):
    """
    Abstract base class for ML model adapters.
    
    Each adapter handles a specific ML framework and provides
    a unified interface for model operations.
    """
    
    @property
    @abstractmethod
    def framework(self) -> ModelFramework:
        """Return the ML framework this adapter supports."""
        pass
    
    @abstractmethod
    def serialize(self, model: Any, path: str, metadata: Optional[ModelMetadata] = None) -> str:
        """
        Serialize a model to disk.
        
        Args:
            model: The trained model object
            path: Base path for saving the model
            metadata: Optional metadata to save alongside the model
            
        Returns:
            The full path where the model was saved
        """
        pass
    
    @abstractmethod
    def deserialize(self, path: str) -> Any:
        """
        Load a model from disk.
        
        Args:
            path: Path to the saved model
            
        Returns:
            The loaded model object
        """
        pass
    
    @abstractmethod
    def predict(self, model: Any, inputs: Dict[str, Any]) -> PredictionResult:
        """
        Execute prediction with the model.
        
        Args:
            model: The model object
            inputs: Dictionary of input features
            
        Returns:
            PredictionResult with predictions and metadata
        """
        pass
    
    @abstractmethod
    def predict_batch(self, model: Any, inputs: List[Dict[str, Any]]) -> List[PredictionResult]:
        """
        Execute batch predictions.
        
        Args:
            model: The model object
            inputs: List of input dictionaries
            
        Returns:
            List of PredictionResults
        """
        pass
    
    @abstractmethod
    def evaluate(self, model: Any, X: Any, y: Any, model_type: ModelType) -> ModelMetrics:
        """
        Evaluate model performance.
        
        Args:
            model: The model object
            X: Feature data
            y: Target data
            model_type: Type of model for selecting appropriate metrics
            
        Returns:
            ModelMetrics with computed performance metrics
        """
        pass
    
    def get_model_hash(self, model: Any) -> str:
        """
        Generate a hash of the model for versioning.
        
        Args:
            model: The model object
            
        Returns:
            SHA-256 hash of the model
        """
        import pickle
        model_bytes = pickle.dumps(model)
        return hashlib.sha256(model_bytes).hexdigest()[:16]
    
    def validate_inputs(self, inputs: Dict[str, Any], schema: Dict[str, str]) -> bool:
        """
        Validate inputs against expected schema.
        
        Args:
            inputs: Input dictionary
            schema: Expected schema (name -> type)
            
        Returns:
            True if valid, raises ValueError if invalid
        """
        for field_name, field_type in schema.items():
            if field_name not in inputs:
                raise ValueError(f"Missing required input field: {field_name}")
            # Type validation could be extended here
        return True


class ModelRegistry:
    """
    Registry for managing multiple model adapters.
    
    Provides a unified interface for working with models
    across different ML frameworks.
    """
    
    def __init__(self):
        self._adapters: Dict[ModelFramework, ModelAdapter] = {}
        self._models: Dict[str, tuple[Any, ModelMetadata]] = {}
    
    def register_adapter(self, adapter: ModelAdapter) -> None:
        """Register a model adapter for a framework."""
        self._adapters[adapter.framework] = adapter
    
    def get_adapter(self, framework: ModelFramework) -> ModelAdapter:
        """Get the adapter for a specific framework."""
        if framework not in self._adapters:
            raise ValueError(f"No adapter registered for framework: {framework}")
        return self._adapters[framework]
    
    def register_model(
        self,
        model: Any,
        metadata: ModelMetadata,
        save_path: Optional[str] = None
    ) -> str:
        """
        Register a model with the registry.
        
        Args:
            model: The trained model object
            metadata: Model metadata
            save_path: Optional path to save the model
            
        Returns:
            The model ID
        """
        adapter = self.get_adapter(metadata.framework)
        
        if save_path:
            adapter.serialize(model, save_path, metadata)
        
        self._models[metadata.model_id] = (model, metadata)
        return metadata.model_id
    
    def get_model(self, model_id: str) -> tuple[Any, ModelMetadata]:
        """Get a model by ID."""
        if model_id not in self._models:
            raise ValueError(f"Model not found: {model_id}")
        return self._models[model_id]
    
    def predict(self, model_id: str, inputs: Dict[str, Any]) -> PredictionResult:
        """Execute prediction with a registered model."""
        model, metadata = self.get_model(model_id)
        adapter = self.get_adapter(metadata.framework)
        result = adapter.predict(model, inputs)
        result.model_id = model_id
        return result
    
    def list_models(self, framework: Optional[ModelFramework] = None) -> List[ModelMetadata]:
        """List all registered models, optionally filtered by framework."""
        models = [metadata for _, metadata in self._models.values()]
        if framework:
            models = [m for m in models if m.framework == framework]
        return models
    
    def compare_models(
        self,
        model_ids: List[str],
        X: Any,
        y: Any,
        model_type: ModelType
    ) -> Dict[str, ModelMetrics]:
        """
        Compare multiple models on the same dataset.
        
        Args:
            model_ids: List of model IDs to compare
            X: Feature data
            y: Target data
            model_type: Type of models
            
        Returns:
            Dictionary mapping model_id to ModelMetrics
        """
        results = {}
        for model_id in model_ids:
            model, metadata = self.get_model(model_id)
            adapter = self.get_adapter(metadata.framework)
            metrics = adapter.evaluate(model, X, y, model_type)
            results[model_id] = metrics
        return results


# Convenience function for creating adapters
def create_adapter(framework: Union[str, ModelFramework]) -> ModelAdapter:
    """
    Factory function to create the appropriate adapter.
    
    Args:
        framework: Framework name or enum
        
    Returns:
        The appropriate ModelAdapter instance
    """
    if isinstance(framework, str):
        framework = ModelFramework(framework.lower())
    
    if framework == ModelFramework.SKLEARN:
        from .sklearn_adapter import SklearnAdapter
        return SklearnAdapter()
    elif framework == ModelFramework.PYTORCH:
        from .pytorch_adapter import PyTorchAdapter
        return PyTorchAdapter()
    elif framework == ModelFramework.TENSORFLOW:
        from .tensorflow_adapter import TensorFlowAdapter
        return TensorFlowAdapter()
    else:
        raise ValueError(f"No adapter available for framework: {framework}")
