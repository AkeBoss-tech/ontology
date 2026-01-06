"""
TensorFlow/Keras Model Adapter

Adapter for TensorFlow and Keras models providing serialization,
prediction, and evaluation capabilities.
"""

import os
import time
import json
from typing import Any, Dict, List, Optional
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


class TensorFlowAdapter(ModelAdapter):
    """
    Adapter for TensorFlow/Keras models.
    
    Supports:
    - Keras Sequential and Functional models
    - SavedModel format
    - H5 format
    """
    
    @property
    def framework(self) -> ModelFramework:
        return ModelFramework.TENSORFLOW
    
    def serialize(self, model: Any, path: str, metadata: Optional[ModelMetadata] = None) -> str:
        """
        Serialize TensorFlow model to disk.
        
        Args:
            model: TensorFlow/Keras model
            path: Base path for saving
            metadata: Optional metadata
            
        Returns:
            Full path to saved model
        """
        # Ensure directory exists
        os.makedirs(path, exist_ok=True)
        
        # Save in SavedModel format (recommended)
        model.save(path)
        
        # Also save as H5 for compatibility
        h5_path = os.path.join(path, 'model.h5')
        try:
            model.save(h5_path, save_format='h5')
        except Exception:
            pass  # H5 format may not support all models
        
        # Save metadata
        if metadata:
            metadata_path = os.path.join(path, 'metadata.json')
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
    
    def deserialize(self, path: str) -> Any:
        """
        Load TensorFlow model from disk.
        
        Args:
            path: Path to saved model
            
        Returns:
            Loaded TensorFlow model
        """
        import tensorflow as tf
        
        # Try SavedModel format first
        if os.path.isdir(path):
            return tf.keras.models.load_model(path)
        
        # Try H5 format
        if path.endswith('.h5'):
            return tf.keras.models.load_model(path)
        
        # Try with .h5 extension
        h5_path = f"{path}.h5"
        if os.path.exists(h5_path):
            return tf.keras.models.load_model(h5_path)
        
        raise ValueError(f"Could not load model from {path}")
    
    def predict(self, model: Any, inputs: Dict[str, Any]) -> PredictionResult:
        """
        Execute prediction with TensorFlow model.
        
        Args:
            model: TensorFlow/Keras model
            inputs: Dictionary with 'features' key or direct feature dict
            
        Returns:
            PredictionResult with predictions
        """
        start_time = time.time()
        
        # Extract features
        if 'features' in inputs:
            X = np.array(inputs['features'])
        else:
            X = np.array([list(inputs.values())])
        
        # Ensure batch dimension
        if X.ndim == 1:
            X = X.reshape(1, -1)
        
        # Make predictions
        outputs = model.predict(X, verbose=0)
        
        # Handle different output formats
        predictions = outputs
        confidence = None
        probabilities = None
        
        # Classification with softmax output
        if outputs.ndim > 1 and outputs.shape[1] > 1:
            probabilities = outputs.tolist()
            confidence = [float(max(p)) for p in outputs]
            predictions = np.argmax(outputs, axis=1)
        # Binary classification with sigmoid
        elif outputs.ndim > 1 and outputs.shape[1] == 1:
            probabilities = [[1 - p[0], p[0]] for p in outputs]
            confidence = [max(p) for p in probabilities]
            predictions = (outputs > 0.5).astype(int).flatten()
        
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
            model: TensorFlow/Keras model
            inputs: List of input dictionaries
            
        Returns:
            List of PredictionResults
        """
        # Combine inputs
        if 'features' in inputs[0]:
            X = np.array([inp['features'] for inp in inputs])
        else:
            X = np.array([list(inp.values()) for inp in inputs])
        
        # Single batch prediction
        result = self.predict(model, {'features': X})
        
        # Split results
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
        Evaluate TensorFlow model performance.
        
        Args:
            model: TensorFlow/Keras model
            X: Feature data
            y: Target data
            model_type: Type of model
            
        Returns:
            ModelMetrics with computed metrics
        """
        from sklearn import metrics as sklearn_metrics
        
        X = np.array(X)
        y = np.array(y)
        
        # Get predictions
        outputs = model.predict(X, verbose=0)
        
        result = ModelMetrics(
            evaluated_at=datetime.utcnow(),
            sample_count=len(y),
        )
        
        if model_type == ModelType.CLASSIFICATION:
            # Get class predictions
            if outputs.ndim > 1 and outputs.shape[1] > 1:
                y_pred = np.argmax(outputs, axis=1)
            else:
                y_pred = (outputs > 0.5).astype(int).flatten()
            
            result.accuracy = sklearn_metrics.accuracy_score(y, y_pred)
            average = 'weighted' if len(set(y)) > 2 else 'binary'
            result.precision = sklearn_metrics.precision_score(y, y_pred, average=average, zero_division=0)
            result.recall = sklearn_metrics.recall_score(y, y_pred, average=average, zero_division=0)
            result.f1_score = sklearn_metrics.f1_score(y, y_pred, average=average, zero_division=0)
            
            # AUC-ROC for binary
            if len(set(y)) == 2:
                if outputs.ndim > 1 and outputs.shape[1] > 1:
                    y_proba = outputs[:, 1]
                else:
                    y_proba = outputs.flatten()
                result.auc_roc = sklearn_metrics.roc_auc_score(y, y_proba)
        
        elif model_type == ModelType.REGRESSION:
            y_pred = outputs.flatten()
            result.rmse = np.sqrt(sklearn_metrics.mean_squared_error(y, y_pred))
            result.mae = sklearn_metrics.mean_absolute_error(y, y_pred)
            result.r2_score = sklearn_metrics.r2_score(y, y_pred)
        
        return result
    
    def get_model_summary(self, model: Any) -> str:
        """Get model architecture summary."""
        import io
        stream = io.StringIO()
        model.summary(print_fn=lambda x: stream.write(x + '\n'))
        return stream.getvalue()
