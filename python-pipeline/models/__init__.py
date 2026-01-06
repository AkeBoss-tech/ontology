"""
Models package for ML model integration.

Provides adapters for various ML frameworks and a unified
interface for model registration, execution, and evaluation.
"""

from .model_adapter import (
    ModelAdapter,
    ModelFramework,
    ModelType,
    ModelMetadata,
    ModelMetrics,
    PredictionResult,
    ModelRegistry,
    create_adapter,
)

from .sklearn_adapter import SklearnAdapter
from .pytorch_adapter import PyTorchAdapter
from .tensorflow_adapter import TensorFlowAdapter
from .sagemaker_connector import SageMakerConnector, SageMakerEndpoint
from .datarobot_connector import DataRobotConnector, DataRobotDeployment
from .model_service import ModelService, HTTPModelServer, serve

__all__ = [
    # Base types
    "ModelAdapter",
    "ModelFramework",
    "ModelType",
    "ModelMetadata",
    "ModelMetrics",
    "PredictionResult",
    "ModelRegistry",
    "create_adapter",
    # Adapters
    "SklearnAdapter",
    "PyTorchAdapter",
    "TensorFlowAdapter",
    # Connectors
    "SageMakerConnector",
    "SageMakerEndpoint",
    "DataRobotConnector",
    "DataRobotDeployment",
    # Service
    "ModelService",
    "HTTPModelServer",
    "serve",
]

