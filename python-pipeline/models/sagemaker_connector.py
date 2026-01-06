"""
AWS SageMaker Connector

Connector for invoking models deployed on AWS SageMaker endpoints.
"""

import json
import time
from typing import Any, Dict, List, Optional
from datetime import datetime
from dataclasses import dataclass
import numpy as np

from .model_adapter import (
    ModelMetrics,
    PredictionResult,
)


@dataclass
class SageMakerEndpoint:
    """Configuration for a SageMaker endpoint."""
    endpoint_name: str
    region: str = 'us-east-1'
    input_format: str = 'json'  # 'json', 'csv', 'npy'
    output_format: str = 'json'


class SageMakerConnector:
    """
    Connector for AWS SageMaker endpoints.
    
    Provides methods for invoking predictions on SageMaker-hosted models
    and syncing model metadata.
    """
    
    def __init__(
        self,
        aws_access_key_id: Optional[str] = None,
        aws_secret_access_key: Optional[str] = None,
        region: str = 'us-east-1',
    ):
        """
        Initialize SageMaker connector.
        
        Args:
            aws_access_key_id: AWS access key (uses env/profile if not provided)
            aws_secret_access_key: AWS secret key (uses env/profile if not provided)
            region: Default AWS region
        """
        self._region = region
        self._client = None
        self._runtime_client = None
        self._credentials = {
            'aws_access_key_id': aws_access_key_id,
            'aws_secret_access_key': aws_secret_access_key,
        }
    
    def _get_client(self):
        """Get SageMaker client."""
        if self._client is None:
            import boto3
            
            kwargs = {'region_name': self._region}
            if self._credentials['aws_access_key_id']:
                kwargs.update(self._credentials)
            
            self._client = boto3.client('sagemaker', **kwargs)
        return self._client
    
    def _get_runtime_client(self):
        """Get SageMaker Runtime client for predictions."""
        if self._runtime_client is None:
            import boto3
            
            kwargs = {'region_name': self._region}
            if self._credentials['aws_access_key_id']:
                kwargs.update(self._credentials)
            
            self._runtime_client = boto3.client('sagemaker-runtime', **kwargs)
        return self._runtime_client
    
    def list_endpoints(self, status: str = 'InService') -> List[Dict[str, Any]]:
        """
        List available SageMaker endpoints.
        
        Args:
            status: Filter by status ('InService', 'Creating', etc.)
            
        Returns:
            List of endpoint information dictionaries
        """
        client = self._get_client()
        
        paginator = client.get_paginator('list_endpoints')
        endpoints = []
        
        for page in paginator.paginate(StatusEquals=status):
            for endpoint in page['Endpoints']:
                endpoints.append({
                    'name': endpoint['EndpointName'],
                    'status': endpoint['EndpointStatus'],
                    'created_at': endpoint['CreationTime'],
                    'last_modified': endpoint['LastModifiedTime'],
                })
        
        return endpoints
    
    def get_endpoint_info(self, endpoint_name: str) -> Dict[str, Any]:
        """
        Get detailed information about an endpoint.
        
        Args:
            endpoint_name: Name of the endpoint
            
        Returns:
            Endpoint details
        """
        client = self._get_client()
        response = client.describe_endpoint(EndpointName=endpoint_name)
        
        return {
            'name': response['EndpointName'],
            'arn': response['EndpointArn'],
            'status': response['EndpointStatus'],
            'config_name': response['EndpointConfigName'],
            'created_at': response['CreationTime'],
            'last_modified': response['LastModifiedTime'],
        }
    
    def predict(
        self,
        endpoint: SageMakerEndpoint,
        inputs: Dict[str, Any],
    ) -> PredictionResult:
        """
        Invoke prediction on a SageMaker endpoint.
        
        Args:
            endpoint: Endpoint configuration
            inputs: Input data dictionary
            
        Returns:
            PredictionResult with predictions
        """
        runtime = self._get_runtime_client()
        start_time = time.time()
        
        # Format input based on content type
        if endpoint.input_format == 'json':
            body = json.dumps(inputs)
            content_type = 'application/json'
        elif endpoint.input_format == 'csv':
            if 'features' in inputs:
                body = ','.join(map(str, inputs['features']))
            else:
                body = ','.join(map(str, inputs.values()))
            content_type = 'text/csv'
        else:
            raise ValueError(f"Unsupported input format: {endpoint.input_format}")
        
        # Invoke endpoint
        response = runtime.invoke_endpoint(
            EndpointName=endpoint.endpoint_name,
            Body=body,
            ContentType=content_type,
        )
        
        # Parse response
        result_body = response['Body'].read().decode('utf-8')
        
        if endpoint.output_format == 'json':
            result = json.loads(result_body)
            predictions = result.get('predictions', result)
        else:
            predictions = [float(x) for x in result_body.strip().split(',')]
        
        execution_time = (time.time() - start_time) * 1000
        
        return PredictionResult(
            predictions=predictions,
            execution_time_ms=execution_time,
        )
    
    def predict_batch(
        self,
        endpoint: SageMakerEndpoint,
        inputs: List[Dict[str, Any]],
    ) -> List[PredictionResult]:
        """
        Execute batch predictions.
        
        Args:
            endpoint: Endpoint configuration
            inputs: List of input dictionaries
            
        Returns:
            List of PredictionResults
        """
        # SageMaker can handle batch in single request
        combined = {'instances': inputs}
        result = self.predict(endpoint, combined)
        
        # Split predictions if combined
        predictions = result.predictions
        if isinstance(predictions, list) and len(predictions) == len(inputs):
            return [
                PredictionResult(
                    predictions=[pred],
                    execution_time_ms=result.execution_time_ms / len(inputs) if result.execution_time_ms else None,
                )
                for pred in predictions
            ]
        return [result]
    
    def deploy_model(
        self,
        model_name: str,
        endpoint_name: str,
        instance_type: str = 'ml.m4.xlarge',
        initial_instance_count: int = 1,
    ) -> Dict[str, str]:
        """
        Deploy a registered model to an endpoint.
        
        Args:
            model_name: Name of the registered model
            endpoint_name: Name for the new endpoint
            instance_type: EC2 instance type
            initial_instance_count: Number of instances
            
        Returns:
            Endpoint information
        """
        client = self._get_client()
        
        # Create endpoint config
        config_name = f"{endpoint_name}-config"
        client.create_endpoint_config(
            EndpointConfigName=config_name,
            ProductionVariants=[{
                'VariantName': 'primary',
                'ModelName': model_name,
                'InstanceType': instance_type,
                'InitialInstanceCount': initial_instance_count,
            }],
        )
        
        # Create endpoint
        client.create_endpoint(
            EndpointName=endpoint_name,
            EndpointConfigName=config_name,
        )
        
        return {
            'endpoint_name': endpoint_name,
            'config_name': config_name,
            'status': 'Creating',
        }
    
    def delete_endpoint(self, endpoint_name: str, delete_config: bool = True) -> None:
        """
        Delete a SageMaker endpoint.
        
        Args:
            endpoint_name: Name of the endpoint to delete
            delete_config: Also delete the endpoint configuration
        """
        client = self._get_client()
        
        # Get config name before deleting
        endpoint_info = self.get_endpoint_info(endpoint_name)
        
        client.delete_endpoint(EndpointName=endpoint_name)
        
        if delete_config:
            client.delete_endpoint_config(
                EndpointConfigName=endpoint_info['config_name']
            )
