use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::property::PropertyValue;
use crate::model_objectives::{ModelObjective, ModelPlatform};

/// Result of model execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelExecutionResult {
    pub prediction: PropertyValue,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probabilities: Option<HashMap<String, f64>>,
    
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Trait for executing models
#[async_trait::async_trait]
pub trait ModelExecutor: Send + Sync {
    /// Execute a model with given inputs
    async fn execute(
        &self,
        model: &ModelObjective,
        inputs: HashMap<String, PropertyValue>,
    ) -> Result<ModelExecutionResult, ModelExecutionError>;
    
    /// Check if this executor can handle the given model platform
    fn can_handle(&self, platform: &ModelPlatform) -> bool;
}

/// Python model executor - executes local Python models via gRPC
pub struct PythonModelExecutor {
    grpc_endpoint: String,
}

impl PythonModelExecutor {
    pub fn new(grpc_endpoint: String) -> Self {
        Self { grpc_endpoint }
    }
}

#[async_trait::async_trait]
impl ModelExecutor for PythonModelExecutor {
    async fn execute(
        &self,
        model: &ModelObjective,
        inputs: HashMap<String, PropertyValue>,
    ) -> Result<ModelExecutionResult, ModelExecutionError> {
        // TODO: Implement gRPC call to Python model service
        // For now, return a placeholder
        Err(ModelExecutionError::NotImplemented(
            "Python model execution via gRPC not yet implemented".to_string()
        ))
    }
    
    fn can_handle(&self, platform: &ModelPlatform) -> bool {
        matches!(platform, ModelPlatform::Local { .. })
    }
}

/// Remote model executor - executes models on external platforms
pub struct RemoteModelExecutor {
    http_client: reqwest::Client,
}

impl RemoteModelExecutor {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }
}

impl Default for RemoteModelExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ModelExecutor for RemoteModelExecutor {
    async fn execute(
        &self,
        model: &ModelObjective,
        inputs: HashMap<String, PropertyValue>,
    ) -> Result<ModelExecutionResult, ModelExecutionError> {
        match &model.platform {
            ModelPlatform::SageMaker { endpoint_name, region } => {
                self.execute_sagemaker(endpoint_name, region, inputs).await
            }
            ModelPlatform::DataRobot { deployment_id } => {
                self.execute_datarobot(deployment_id, inputs).await
            }
            ModelPlatform::Custom { endpoint_url, .. } => {
                self.execute_custom(endpoint_url, inputs).await
            }
            _ => Err(ModelExecutionError::UnsupportedPlatform(
                format!("Platform {:?} not supported by RemoteModelExecutor", model.platform)
            )),
        }
    }
    
    fn can_handle(&self, platform: &ModelPlatform) -> bool {
        matches!(
            platform,
            ModelPlatform::SageMaker { .. } | 
            ModelPlatform::DataRobot { .. } | 
            ModelPlatform::Custom { .. }
        )
    }
}

impl RemoteModelExecutor {
    async fn execute_sagemaker(
        &self,
        _endpoint_name: &str,
        _region: &str,
        _inputs: HashMap<String, PropertyValue>,
    ) -> Result<ModelExecutionResult, ModelExecutionError> {
        // TODO: Implement SageMaker invocation via AWS SDK
        Err(ModelExecutionError::NotImplemented(
            "SageMaker execution not yet implemented".to_string()
        ))
    }
    
    async fn execute_datarobot(
        &self,
        _deployment_id: &str,
        _inputs: HashMap<String, PropertyValue>,
    ) -> Result<ModelExecutionResult, ModelExecutionError> {
        // TODO: Implement DataRobot API call
        Err(ModelExecutionError::NotImplemented(
            "DataRobot execution not yet implemented".to_string()
        ))
    }
    
    async fn execute_custom(
        &self,
        _endpoint_url: &str,
        _inputs: HashMap<String, PropertyValue>,
    ) -> Result<ModelExecutionResult, ModelExecutionError> {
        // TODO: Implement custom endpoint invocation
        Err(ModelExecutionError::NotImplemented(
            "Custom endpoint execution not yet implemented".to_string()
        ))
    }
}

/// Model prediction cache
pub struct ModelCache {
    cache: HashMap<String, CachedPrediction>,
}

#[derive(Debug, Clone)]
struct CachedPrediction {
    result: ModelExecutionResult,
    cached_at: std::time::Instant,
    ttl: std::time::Duration,
}

impl ModelCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    /// Get a cached prediction if it exists and is not expired
    pub fn get(
        &self,
        cache_key: &str,
    ) -> Option<ModelExecutionResult> {
        if let Some(cached) = self.cache.get(cache_key) {
            if cached.cached_at.elapsed() < cached.ttl {
                return Some(cached.result.clone());
            }
        }
        None
    }
    
    /// Store a prediction in the cache
    pub fn put(
        &mut self,
        cache_key: String,
        result: ModelExecutionResult,
        ttl_seconds: u64,
    ) {
        self.cache.insert(
            cache_key,
            CachedPrediction {
                result,
                cached_at: std::time::Instant::now(),
                ttl: std::time::Duration::from_secs(ttl_seconds),
            },
        );
    }
    
    /// Clear expired entries
    pub fn clear_expired(&mut self) {
        self.cache.retain(|_, cached| {
            cached.cached_at.elapsed() < cached.ttl
        });
    }
    
    /// Clear all cache entries
    pub fn clear_all(&mut self) {
        self.cache.clear();
    }
    
    /// Generate cache key from model ID and inputs
    pub fn generate_key(
        model_id: &str,
        inputs: &HashMap<String, PropertyValue>,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        model_id.hash(&mut hasher);
        
        // Sort keys for consistent hashing
        let mut sorted_inputs: Vec<_> = inputs.iter().collect();
        sorted_inputs.sort_by_key(|(k, _)| *k);
        
        for (key, value) in sorted_inputs {
            key.hash(&mut hasher);
            // Hash the string representation of the value
            value.to_string().hash(&mut hasher);
        }
        
        format!("{}:{}", model_id, hasher.finish())
    }
}

impl Default for ModelCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Orchestrator for model execution with caching and multiple executors
pub struct ModelExecutionOrchestrator {
    executors: Vec<Box<dyn ModelExecutor>>,
    cache: ModelCache,
}

impl ModelExecutionOrchestrator {
    pub fn new() -> Self {
        Self {
            executors: Vec::new(),
            cache: ModelCache::new(),
        }
    }
    
    /// Add an executor to the orchestrator
    pub fn add_executor(&mut self, executor: Box<dyn ModelExecutor>) {
        self.executors.push(executor);
    }
    
    /// Execute a model with caching support
    pub async fn execute(
        &mut self,
        model: &ModelObjective,
        inputs: HashMap<String, PropertyValue>,
        use_cache: bool,
        cache_ttl: u64,
    ) -> Result<ModelExecutionResult, ModelExecutionError> {
        // Check cache first if enabled
        if use_cache {
            let cache_key = ModelCache::generate_key(&model.id, &inputs);
            if let Some(cached_result) = self.cache.get(&cache_key) {
                return Ok(cached_result);
            }
        }
        
        // Find an executor that can handle this model
        let executor = self.executors.iter()
            .find(|e| e.can_handle(&model.platform))
            .ok_or_else(|| ModelExecutionError::NoExecutorFound(
                format!("No executor found for platform {:?}", model.platform)
            ))?;
        
        // Execute the model
        let result = executor.execute(model, inputs.clone()).await?;
        
        // Cache the result if enabled
        if use_cache {
            let cache_key = ModelCache::generate_key(&model.id, &inputs);
            self.cache.put(cache_key, result.clone(), cache_ttl);
        }
        
        Ok(result)
    }
    
    /// Clear expired cache entries
    pub fn clear_expired_cache(&mut self) {
        self.cache.clear_expired();
    }
}

impl Default for ModelExecutionOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors for model execution
#[derive(Debug, thiserror::Error)]
pub enum ModelExecutionError {
    #[error("Model execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
    
    #[error("No executor found: {0}")]
    NoExecutorFound(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Timeout")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model_objectives::{ModelObjective, ModelType};

    #[test]
    fn test_cache_key_generation() {
        let mut inputs1 = HashMap::new();
        inputs1.insert("feature1".to_string(), PropertyValue::Integer(42));
        inputs1.insert("feature2".to_string(), PropertyValue::Double(3.14));
        
        let mut inputs2 = HashMap::new();
        inputs2.insert("feature2".to_string(), PropertyValue::Double(3.14));
        inputs2.insert("feature1".to_string(), PropertyValue::Integer(42));
        
        // Same inputs in different order should produce same key
        let key1 = ModelCache::generate_key("model_1", &inputs1);
        let key2 = ModelCache::generate_key("model_1", &inputs2);
        
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_cache_expiration() {
        let mut cache = ModelCache::new();
        
        let result = ModelExecutionResult {
            prediction: PropertyValue::Double(0.85),
            confidence: Some(0.92),
            probabilities: None,
            metadata: HashMap::new(),
        };
        
        // Cache with 0 second TTL (should expire immediately)
        cache.put("key1".to_string(), result.clone(), 0);
        
        // Sleep briefly to ensure expiration
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Should not find the cached value
        assert!(cache.get("key1").is_none());
    }

    #[test]
    fn test_python_executor_can_handle() {
        let executor = PythonModelExecutor::new("localhost:50051".to_string());
        
        let local_platform = ModelPlatform::Local {
            framework: "sklearn".to_string(),
        };
        
        let sagemaker_platform = ModelPlatform::SageMaker {
            endpoint_name: "test".to_string(),
            region: "us-east-1".to_string(),
        };
        
        assert!(executor.can_handle(&local_platform));
        assert!(!executor.can_handle(&sagemaker_platform));
    }

    #[test]
    fn test_remote_executor_can_handle() {
        let executor = RemoteModelExecutor::new();
        
        let local_platform = ModelPlatform::Local {
            framework: "sklearn".to_string(),
        };
        
        let sagemaker_platform = ModelPlatform::SageMaker {
            endpoint_name: "test".to_string(),
            region: "us-east-1".to_string(),
        };
        
        assert!(!executor.can_handle(&local_platform));
        assert!(executor.can_handle(&sagemaker_platform));
    }
}
