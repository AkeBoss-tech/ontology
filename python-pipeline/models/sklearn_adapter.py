"""
Scikit-learn Model Adapter

Adapter for scikit-learn models providing serialization,
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


class SklearnAdapter(ModelAdapter):
    """
    Adapter for scikit-learn models.
    
    Supports all sklearn estimators including:
    - Classifiers (RandomForest, SVC, LogisticRegression, etc.)
    - Regressors (LinearRegression, Ridge, GradientBoosting, etc.)
    - Clustering (KMeans, DBSCAN, etc.)
    """
    
    @property
    def framework(self) -> ModelFramework:
        return ModelFramework.SKLEARN
    
    def serialize(self, model: Any, path: str, metadata: Optional[ModelMetadata] = None) -> str:
        """
        Serialize sklearn model using joblib.
        
        Args:
            model: Trained sklearn estimator
            path: Base path for saving
            metadata: Optional metadata
            
        Returns:
            Full path to saved model
        """
        import joblib
        
        # Ensure directory exists
        os.makedirs(os.path.dirname(path) if os.path.dirname(path) else '.', exist_ok=True)
        
        # Add .joblib extension if not present
        if not path.endswith('.joblib'):
            path = f"{path}.joblib"
        
        # Save model
        joblib.dump(model, path)
        
        # Save metadata if provided
        if metadata:
            metadata_path = path.replace('.joblib', '_metadata.json')
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
                    'updated_at': metadata.updated_at.isoformat(),
                    'tags': metadata.tags,
                    'custom_metadata': metadata.custom_metadata,
                }, f, indent=2)
        
        return path
    
    def deserialize(self, path: str) -> Any:
        """
        Load sklearn model from disk.
        
        Args:
            path: Path to saved model
            
        Returns:
            Loaded sklearn estimator
        """
        import joblib
        
        # Add extension if needed
        if not path.endswith('.joblib'):
            path = f"{path}.joblib"
        
        return joblib.load(path)
    
    def predict(self, model: Any, inputs: Dict[str, Any]) -> PredictionResult:
        """
        Execute prediction with sklearn model.
        
        Args:
            model: Sklearn estimator
            inputs: Dictionary with 'features' key containing input data
            
        Returns:
            PredictionResult with predictions
        """
        start_time = time.time()
        
        # Extract features from inputs
        if 'features' in inputs:
            X = inputs['features']
        else:
            # Assume inputs is already feature array
            X = np.array([list(inputs.values())])
        
        # Ensure 2D array
        X = np.atleast_2d(X)
        
        # Make predictions
        predictions = model.predict(X)
        
        # Get probabilities if available (for classifiers)
        probabilities = None
        confidence = None
        if hasattr(model, 'predict_proba'):
            try:
                probabilities = model.predict_proba(X).tolist()
                confidence = [max(p) for p in probabilities]
            except AttributeError:
                pass
        
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
            model: Sklearn estimator
            inputs: List of input dictionaries
            
        Returns:
            List of PredictionResults
        """
        # Combine all inputs
        if 'features' in inputs[0]:
            X = np.array([inp['features'] for inp in inputs])
        else:
            X = np.array([list(inp.values()) for inp in inputs])
        
        start_time = time.time()
        predictions = model.predict(X)
        
        probabilities = None
        if hasattr(model, 'predict_proba'):
            try:
                probabilities = model.predict_proba(X)
            except AttributeError:
                pass
        
        execution_time = (time.time() - start_time) * 1000
        
        # Create individual results
        results = []
        for i, pred in enumerate(predictions):
            prob = probabilities[i].tolist() if probabilities is not None else None
            conf = max(prob) if prob else None
            results.append(PredictionResult(
                predictions=[pred.tolist() if hasattr(pred, 'tolist') else pred],
                confidence=[conf] if conf else None,
                probabilities=[prob] if prob else None,
                execution_time_ms=execution_time / len(inputs),
            ))
        
        return results
    
    def evaluate(self, model: Any, X: Any, y: Any, model_type: ModelType) -> ModelMetrics:
        """
        Evaluate sklearn model performance.
        
        Args:
            model: Sklearn estimator
            X: Feature data
            y: Target data
            model_type: Type of model
            
        Returns:
            ModelMetrics with computed metrics
        """
        from sklearn import metrics as sklearn_metrics
        
        y_pred = model.predict(X)
        
        result = ModelMetrics(
            evaluated_at=datetime.utcnow(),
            sample_count=len(y),
        )
        
        if model_type == ModelType.CLASSIFICATION:
            # Classification metrics
            result.accuracy = sklearn_metrics.accuracy_score(y, y_pred)
            
            # Handle multiclass
            average = 'weighted' if len(set(y)) > 2 else 'binary'
            result.precision = sklearn_metrics.precision_score(y, y_pred, average=average, zero_division=0)
            result.recall = sklearn_metrics.recall_score(y, y_pred, average=average, zero_division=0)
            result.f1_score = sklearn_metrics.f1_score(y, y_pred, average=average, zero_division=0)
            
            # AUC-ROC for binary classification
            if hasattr(model, 'predict_proba') and len(set(y)) == 2:
                try:
                    y_proba = model.predict_proba(X)[:, 1]
                    result.auc_roc = sklearn_metrics.roc_auc_score(y, y_proba)
                except (AttributeError, IndexError):
                    pass
        
        elif model_type == ModelType.REGRESSION:
            # Regression metrics
            result.rmse = np.sqrt(sklearn_metrics.mean_squared_error(y, y_pred))
            result.mae = sklearn_metrics.mean_absolute_error(y, y_pred)
            result.r2_score = sklearn_metrics.r2_score(y, y_pred)
            
            # MAPE (avoid division by zero)
            non_zero_mask = y != 0
            if np.any(non_zero_mask):
                result.mape = np.mean(np.abs((y[non_zero_mask] - y_pred[non_zero_mask]) / y[non_zero_mask])) * 100
        
        elif model_type == ModelType.CLUSTERING:
            # Clustering metrics
            result.silhouette_score = sklearn_metrics.silhouette_score(X, y_pred)
            result.davies_bouldin_score = sklearn_metrics.davies_bouldin_score(X, y_pred)
        
        return result
    
    def get_feature_importance(self, model: Any) -> Optional[Dict[str, float]]:
        """
        Get feature importance if available.
        
        Args:
            model: Sklearn estimator
            
        Returns:
            Dictionary of feature names to importance scores
        """
        if hasattr(model, 'feature_importances_'):
            importances = model.feature_importances_
            if hasattr(model, 'feature_names_in_'):
                return dict(zip(model.feature_names_in_, importances))
            return {f"feature_{i}": imp for i, imp in enumerate(importances)}
        elif hasattr(model, 'coef_'):
            coef = model.coef_
            if coef.ndim > 1:
                coef = coef[0]
            if hasattr(model, 'feature_names_in_'):
                return dict(zip(model.feature_names_in_, np.abs(coef)))
            return {f"feature_{i}": abs(c) for i, c in enumerate(coef)}
        return None
