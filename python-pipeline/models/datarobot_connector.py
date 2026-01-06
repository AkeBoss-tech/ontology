"""
DataRobot Connector

Connector for invoking models deployed on DataRobot platform.
"""

import json
import time
from typing import Any, Dict, List, Optional
from datetime import datetime
from dataclasses import dataclass
import requests


@dataclass
class DataRobotDeployment:
    """Configuration for a DataRobot deployment."""
    deployment_id: str
    api_url: str = 'https://app.datarobot.com/api/v2'


class DataRobotConnector:
    """
    Connector for DataRobot deployments.
    
    Provides methods for invoking predictions on DataRobot-hosted models
    and syncing model information from the leaderboard.
    """
    
    def __init__(
        self,
        api_token: str,
        api_url: str = 'https://app.datarobot.com/api/v2',
    ):
        """
        Initialize DataRobot connector.
        
        Args:
            api_token: DataRobot API token
            api_url: DataRobot API base URL
        """
        self._api_token = api_token
        self._api_url = api_url.rstrip('/')
        self._session = requests.Session()
        self._session.headers.update({
            'Authorization': f'Bearer {api_token}',
            'Content-Type': 'application/json',
        })
    
    def _request(
        self,
        method: str,
        endpoint: str,
        data: Optional[Dict] = None,
        params: Optional[Dict] = None,
    ) -> Dict[str, Any]:
        """Make an API request."""
        url = f"{self._api_url}/{endpoint.lstrip('/')}"
        
        response = self._session.request(
            method=method,
            url=url,
            json=data,
            params=params,
        )
        response.raise_for_status()
        return response.json()
    
    def list_projects(self) -> List[Dict[str, Any]]:
        """
        List all DataRobot projects.
        
        Returns:
            List of project information
        """
        response = self._request('GET', '/projects/')
        return [
            {
                'id': proj['id'],
                'name': proj['projectName'],
                'created_at': proj['created'],
                'target': proj.get('target'),
                'metric': proj.get('metric'),
            }
            for proj in response
        ]
    
    def list_deployments(self) -> List[Dict[str, Any]]:
        """
        List all deployments.
        
        Returns:
            List of deployment information
        """
        response = self._request('GET', '/deployments/')
        return [
            {
                'id': deploy['id'],
                'label': deploy['label'],
                'model_id': deploy.get('model', {}).get('id'),
                'status': deploy['status'],
                'importance': deploy.get('importance'),
            }
            for deploy in response.get('data', [])
        ]
    
    def get_deployment(self, deployment_id: str) -> Dict[str, Any]:
        """
        Get detailed deployment information.
        
        Args:
            deployment_id: ID of the deployment
            
        Returns:
            Deployment details
        """
        response = self._request('GET', f'/deployments/{deployment_id}/')
        return {
            'id': response['id'],
            'label': response['label'],
            'model': response.get('model', {}),
            'status': response['status'],
            'prediction_url': response.get('defaultPredictionServer', {}).get('url'),
            'service_health': response.get('serviceHealth', {}),
            'accuracy': response.get('accuracy', {}),
        }
    
    def get_leaderboard(self, project_id: str) -> List[Dict[str, Any]]:
        """
        Get model leaderboard for a project.
        
        Args:
            project_id: ID of the project
            
        Returns:
            List of models with metrics
        """
        response = self._request('GET', f'/projects/{project_id}/models/')
        
        models = []
        for model in response:
            models.append({
                'id': model['id'],
                'model_type': model['modelType'],
                'blueprint_id': model.get('blueprintId'),
                'sample_pct': model.get('samplePct'),
                'metrics': model.get('metrics', {}),
                'training_duration': model.get('trainingDuration'),
                'is_frozen': model.get('isFrozen', False),
            })
        
        return models
    
    def predict(
        self,
        deployment: DataRobotDeployment,
        inputs: Dict[str, Any],
    ) -> Dict[str, Any]:
        """
        Execute prediction on a deployment.
        
        Args:
            deployment: Deployment configuration
            inputs: Input data dictionary
            
        Returns:
            Prediction result
        """
        start_time = time.time()
        
        # Get prediction server URL
        deploy_info = self.get_deployment(deployment.deployment_id)
        prediction_url = deploy_info.get('prediction_url')
        
        if not prediction_url:
            # Use default predictions endpoint
            prediction_url = f"{self._api_url}/deployments/{deployment.deployment_id}/predictions"
        else:
            prediction_url = f"{prediction_url}/predictions"
        
        # Format data as array of records
        data = [inputs] if isinstance(inputs, dict) else inputs
        
        response = self._session.post(
            prediction_url,
            json=data,
            headers={'datarobot-key': self._api_token},
        )
        response.raise_for_status()
        
        result = response.json()
        execution_time = (time.time() - start_time) * 1000
        
        predictions = []
        probabilities = []
        
        for row in result.get('data', []):
            prediction = row.get('prediction')
            predictions.append(prediction)
            
            # Extract class probabilities if available
            pred_values = row.get('predictionValues', [])
            if pred_values:
                probs = {pv['label']: pv['value'] for pv in pred_values}
                probabilities.append(probs)
        
        return {
            'predictions': predictions,
            'probabilities': probabilities,
            'execution_time_ms': execution_time,
        }
    
    def predict_batch(
        self,
        deployment: DataRobotDeployment,
        inputs: List[Dict[str, Any]],
    ) -> List[Dict[str, Any]]:
        """
        Execute batch predictions.
        
        Args:
            deployment: Deployment configuration
            inputs: List of input dictionaries
            
        Returns:
            List of prediction results
        """
        # DataRobot handles batch natively
        result = self.predict(deployment, inputs)
        
        results = []
        for i, pred in enumerate(result['predictions']):
            probs = result['probabilities'][i] if result['probabilities'] else None
            results.append({
                'predictions': [pred],
                'probabilities': probs,
                'execution_time_ms': result['execution_time_ms'] / len(inputs),
            })
        
        return results
    
    def get_feature_impact(self, project_id: str, model_id: str) -> List[Dict[str, float]]:
        """
        Get feature impact for a model.
        
        Args:
            project_id: Project ID
            model_id: Model ID
            
        Returns:
            List of features with impact scores
        """
        response = self._request(
            'GET',
            f'/projects/{project_id}/models/{model_id}/featureImpact/'
        )
        
        return [
            {
                'feature': fi['featureName'],
                'impact': fi['impactNormalized'],
                'impact_unnormalized': fi['impactUnnormalized'],
            }
            for fi in response.get('featureImpacts', [])
        ]
    
    def create_deployment(
        self,
        model_id: str,
        label: str,
        default_prediction_server_id: Optional[str] = None,
    ) -> Dict[str, str]:
        """
        Deploy a model.
        
        Args:
            model_id: ID of the model to deploy
            label: Name for the deployment
            default_prediction_server_id: Optional prediction server
            
        Returns:
            Deployment information
        """
        data = {
            'modelId': model_id,
            'label': label,
        }
        if default_prediction_server_id:
            data['defaultPredictionServerId'] = default_prediction_server_id
        
        response = self._request('POST', '/deployments/fromModel/', data=data)
        
        return {
            'id': response['id'],
            'label': response['label'],
            'status': response['status'],
        }
    
    def delete_deployment(self, deployment_id: str) -> None:
        """
        Delete a deployment.
        
        Args:
            deployment_id: ID of the deployment to delete
        """
        self._request('DELETE', f'/deployments/{deployment_id}/')
