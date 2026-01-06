use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Model type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    Classification,
    Regression,
    Clustering,
    TimeSeries,
    Custom(String),
}

/// Model lifecycle status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelStatus {
    Training,
    Registered,
    Bound,
    Deprecated,
    Archived,
}

/// Model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    // Classification metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auc_roc: Option<f64>,
    
    // Regression metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmse: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mae: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mape: Option<f64>,
    
    // Clustering metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silhouette_score: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub davies_bouldin_score: Option<f64>,
    
    // Custom metrics
    #[serde(default)]
    pub custom: HashMap<String, f64>,
}

impl ModelMetrics {
    pub fn new() -> Self {
        Self {
            accuracy: None,
            precision: None,
            recall: None,
            f1_score: None,
            auc_roc: None,
            rmse: None,
            mae: None,
            r2: None,
            mape: None,
            silhouette_score: None,
            davies_bouldin_score: None,
            custom: HashMap::new(),
        }
    }
    
    /// Get the primary metric for this model based on type
    pub fn primary_metric(&self, model_type: &ModelType) -> Option<f64> {
        match model_type {
            ModelType::Classification => self.accuracy.or(self.f1_score),
            ModelType::Regression => self.r2.or(self.rmse.map(|v| -v)), // Negative RMSE for comparison
            ModelType::Clustering => self.silhouette_score,
            ModelType::TimeSeries => self.rmse.map(|v| -v),
            ModelType::Custom(_) => None,
        }
    }
}

impl Default for ModelMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Model binding - links a model to an object property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBinding {
    pub model_id: String,
    
    #[serde(rename = "objectType")]
    pub object_type: String,
    
    #[serde(rename = "propertyId")]
    pub property_id: String,
    
    #[serde(rename = "boundAt")]
    pub bound_at: DateTime<Utc>,
    
    #[serde(rename = "boundBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_by: Option<String>,
    
    /// Configuration for how the model should be invoked
    #[serde(default)]
    pub config: ModelBindingConfig,
}

/// Configuration for model binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBindingConfig {
    /// Input properties to pass to the model
    #[serde(default)]
    pub input_properties: Vec<String>,
    
    /// Whether to cache predictions
    #[serde(default = "default_cache_enabled")]
    pub cache_enabled: bool,
    
    /// Cache TTL in seconds
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl: u64,
    
    /// Whether to execute asynchronously
    #[serde(default)]
    pub async_execution: bool,
}

fn default_cache_enabled() -> bool {
    true
}

fn default_cache_ttl() -> u64 {
    3600 // 1 hour
}

impl Default for ModelBindingConfig {
    fn default() -> Self {
        Self {
            input_properties: Vec::new(),
            cache_enabled: default_cache_enabled(),
            cache_ttl: default_cache_ttl(),
            async_execution: false,
        }
    }
}

/// Model platform - where the model is hosted/executed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelPlatform {
    Local {
        framework: String, // "sklearn", "pytorch", "tensorflow"
    },
    SageMaker {
        endpoint_name: String,
        region: String,
    },
    DataRobot {
        deployment_id: String,
    },
    Custom {
        platform_name: String,
        endpoint_url: String,
    },
}

/// Model objective - represents a registered ML model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelObjective {
    pub id: String,
    pub name: String,
    
    #[serde(rename = "modelType")]
    pub model_type: ModelType,
    
    pub version: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Path to the serialized model artifact
    #[serde(rename = "artifactPath")]
    pub artifact_path: String,
    
    /// Platform where the model is hosted
    pub platform: ModelPlatform,
    
    /// Performance metrics
    pub metrics: ModelMetrics,
    
    /// Current status
    pub status: ModelStatus,
    
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl ModelObjective {
    /// Create a new model objective
    pub fn new(
        id: String,
        name: String,
        model_type: ModelType,
        version: String,
        artifact_path: String,
        platform: ModelPlatform,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            model_type,
            version,
            description: None,
            artifact_path,
            platform,
            metrics: ModelMetrics::new(),
            status: ModelStatus::Registered,
            created_at: now,
            updated_at: now,
            created_by: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Update model metrics
    pub fn update_metrics(&mut self, metrics: ModelMetrics) {
        self.metrics = metrics;
        self.updated_at = Utc::now();
    }
    
    /// Update model status
    pub fn update_status(&mut self, status: ModelStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Validate the model objective
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() {
            return Err("Model ID cannot be empty".to_string());
        }
        
        if self.name.is_empty() {
            return Err("Model name cannot be empty".to_string());
        }
        
        if self.version.is_empty() {
            return Err("Model version cannot be empty".to_string());
        }
        
        if self.artifact_path.is_empty() {
            return Err("Model artifact path cannot be empty".to_string());
        }
        
        Ok(())
    }
}

/// Model registry - manages registered models
pub struct ModelRegistry {
    models: HashMap<String, ModelObjective>,
    bindings: HashMap<(String, String), ModelBinding>, // (object_type, property_id) -> binding
}

impl ModelRegistry {
    /// Create a new model registry
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            bindings: HashMap::new(),
        }
    }
    
    /// Register a new model
    pub fn register(&mut self, model: ModelObjective) -> Result<(), String> {
        model.validate()?;
        
        if self.models.contains_key(&model.id) {
            return Err(format!("Model with ID '{}' already exists", model.id));
        }
        
        self.models.insert(model.id.clone(), model);
        Ok(())
    }
    
    /// Get a model by ID
    pub fn get(&self, id: &str) -> Option<&ModelObjective> {
        self.models.get(id)
    }
    
    /// Get a mutable reference to a model
    pub fn get_mut(&mut self, id: &str) -> Option<&mut ModelObjective> {
        self.models.get_mut(id)
    }
    
    /// List all models
    pub fn list(&self) -> Vec<&ModelObjective> {
        self.models.values().collect()
    }
    
    /// List models by type
    pub fn list_by_type(&self, model_type: &ModelType) -> Vec<&ModelObjective> {
        self.models
            .values()
            .filter(|m| &m.model_type == model_type)
            .collect()
    }
    
    /// List models by status
    pub fn list_by_status(&self, status: &ModelStatus) -> Vec<&ModelObjective> {
        self.models
            .values()
            .filter(|m| &m.status == status)
            .collect()
    }
    
    /// Update model metrics
    pub fn update_metrics(&mut self, id: &str, metrics: ModelMetrics) -> Result<(), String> {
        let model = self.models.get_mut(id)
            .ok_or_else(|| format!("Model '{}' not found", id))?;
        
        model.update_metrics(metrics);
        Ok(())
    }
    
    /// Update model status
    pub fn update_status(&mut self, id: &str, status: ModelStatus) -> Result<(), String> {
        let model = self.models.get_mut(id)
            .ok_or_else(|| format!("Model '{}' not found", id))?;
        
        model.update_status(status);
        Ok(())
    }
    
    /// Bind a model to a property
    pub fn bind_model(
        &mut self,
        model_id: &str,
        object_type: String,
        property_id: String,
        bound_by: Option<String>,
        config: ModelBindingConfig,
    ) -> Result<ModelBinding, String> {
        // Check if model exists
        if !self.models.contains_key(model_id) {
            return Err(format!("Model '{}' not found", model_id));
        }
        
        // Check if property is already bound
        let key = (object_type.clone(), property_id.clone());
        if self.bindings.contains_key(&key) {
            return Err(format!(
                "Property '{}.{}' is already bound to a model",
                object_type, property_id
            ));
        }
        
        // Create binding
        let binding = ModelBinding {
            model_id: model_id.to_string(),
            object_type: object_type.clone(),
            property_id: property_id.clone(),
            bound_at: Utc::now(),
            bound_by,
            config,
        };
        
        // Update model status to Bound
        self.update_status(model_id, ModelStatus::Bound)?;
        
        // Store binding
        self.bindings.insert(key, binding.clone());
        
        Ok(binding)
    }
    
    /// Unbind a model from a property
    pub fn unbind_model(
        &mut self,
        object_type: &str,
        property_id: &str,
    ) -> Result<(), String> {
        let key = (object_type.to_string(), property_id.to_string());
        
        let binding = self.bindings.remove(&key)
            .ok_or_else(|| format!(
                "No binding found for property '{}.{}'",
                object_type, property_id
            ))?;
        
        // Check if this was the last binding for the model
        let model_id = &binding.model_id;
        let has_other_bindings = self.bindings.values()
            .any(|b| &b.model_id == model_id);
        
        // If no other bindings, update status back to Registered
        if !has_other_bindings {
            self.update_status(model_id, ModelStatus::Registered)?;
        }
        
        Ok(())
    }
    
    /// Get binding for a property
    pub fn get_binding(&self, object_type: &str, property_id: &str) -> Option<&ModelBinding> {
        let key = (object_type.to_string(), property_id.to_string());
        self.bindings.get(&key)
    }
    
    /// List all bindings
    pub fn list_bindings(&self) -> Vec<&ModelBinding> {
        self.bindings.values().collect()
    }
    
    /// List bindings for a specific model
    pub fn list_bindings_for_model(&self, model_id: &str) -> Vec<&ModelBinding> {
        self.bindings
            .values()
            .filter(|b| b.model_id == model_id)
            .collect()
    }
    
    /// Compare multiple models by their primary metrics
    pub fn compare_models(&self, model_ids: &[String]) -> Result<Vec<ModelComparison>, String> {
        let mut comparisons = Vec::new();
        
        for id in model_ids {
            let model = self.get(id)
                .ok_or_else(|| format!("Model '{}' not found", id))?;
            
            comparisons.push(ModelComparison {
                model_id: model.id.clone(),
                model_name: model.name.clone(),
                model_type: model.model_type.clone(),
                version: model.version.clone(),
                metrics: model.metrics.clone(),
                primary_metric: model.metrics.primary_metric(&model.model_type),
                status: model.status.clone(),
            });
        }
        
        Ok(comparisons)
    }
    
    /// Delete a model (only if not bound)
    pub fn delete(&mut self, id: &str) -> Result<(), String> {
        // Check if model has any bindings
        let has_bindings = self.bindings.values()
            .any(|b| b.model_id == id);
        
        if has_bindings {
            return Err(format!(
                "Cannot delete model '{}' because it has active bindings. Unbind first.",
                id
            ));
        }
        
        self.models.remove(id)
            .ok_or_else(|| format!("Model '{}' not found", id))?;
        
        Ok(())
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Model comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelComparison {
    pub model_id: String,
    pub model_name: String,
    pub model_type: ModelType,
    pub version: String,
    pub metrics: ModelMetrics,
    pub primary_metric: Option<f64>,
    pub status: ModelStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_model_objective() {
        let model = ModelObjective::new(
            "model_1".to_string(),
            "Test Model".to_string(),
            ModelType::Classification,
            "1.0.0".to_string(),
            "/models/model_1.pkl".to_string(),
            ModelPlatform::Local {
                framework: "sklearn".to_string(),
            },
        );
        
        assert_eq!(model.id, "model_1");
        assert_eq!(model.name, "Test Model");
        assert_eq!(model.status, ModelStatus::Registered);
    }

    #[test]
    fn test_model_registry_register() {
        let mut registry = ModelRegistry::new();
        
        let model = ModelObjective::new(
            "model_1".to_string(),
            "Test Model".to_string(),
            ModelType::Regression,
            "1.0.0".to_string(),
            "/models/model_1.pkl".to_string(),
            ModelPlatform::Local {
                framework: "sklearn".to_string(),
            },
        );
        
        assert!(registry.register(model).is_ok());
        assert!(registry.get("model_1").is_some());
    }

    #[test]
    fn test_model_binding() {
        let mut registry = ModelRegistry::new();
        
        let model = ModelObjective::new(
            "model_1".to_string(),
            "Demand Forecast".to_string(),
            ModelType::Regression,
            "1.0.0".to_string(),
            "/models/model_1.pkl".to_string(),
            ModelPlatform::Local {
                framework: "sklearn".to_string(),
            },
        );
        
        registry.register(model).unwrap();
        
        let binding = registry.bind_model(
            "model_1",
            "Plant".to_string(),
            "demand_forecast".to_string(),
            Some("user_123".to_string()),
            ModelBindingConfig::default(),
        );
        
        assert!(binding.is_ok());
        
        let retrieved = registry.get_binding("Plant", "demand_forecast");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().model_id, "model_1");
        
        // Check model status updated to Bound
        let model = registry.get("model_1").unwrap();
        assert_eq!(model.status, ModelStatus::Bound);
    }

    #[test]
    fn test_unbind_model() {
        let mut registry = ModelRegistry::new();
        
        let model = ModelObjective::new(
            "model_1".to_string(),
            "Test Model".to_string(),
            ModelType::Classification,
            "1.0.0".to_string(),
            "/models/model_1.pkl".to_string(),
            ModelPlatform::Local {
                framework: "sklearn".to_string(),
            },
        );
        
        registry.register(model).unwrap();
        registry.bind_model(
            "model_1",
            "Customer".to_string(),
            "churn_risk".to_string(),
            None,
            ModelBindingConfig::default(),
        ).unwrap();
        
        assert!(registry.unbind_model("Customer", "churn_risk").is_ok());
        assert!(registry.get_binding("Customer", "churn_risk").is_none());
        
        // Check model status reverted to Registered
        let model = registry.get("model_1").unwrap();
        assert_eq!(model.status, ModelStatus::Registered);
    }

    #[test]
    fn test_compare_models() {
        let mut registry = ModelRegistry::new();
        
        let mut model1 = ModelObjective::new(
            "model_1".to_string(),
            "Model 1".to_string(),
            ModelType::Classification,
            "1.0.0".to_string(),
            "/models/model_1.pkl".to_string(),
            ModelPlatform::Local {
                framework: "sklearn".to_string(),
            },
        );
        model1.metrics.accuracy = Some(0.85);
        
        let mut model2 = ModelObjective::new(
            "model_2".to_string(),
            "Model 2".to_string(),
            ModelType::Classification,
            "1.0.0".to_string(),
            "/models/model_2.pkl".to_string(),
            ModelPlatform::Local {
                framework: "sklearn".to_string(),
            },
        );
        model2.metrics.accuracy = Some(0.92);
        
        registry.register(model1).unwrap();
        registry.register(model2).unwrap();
        
        let comparison = registry.compare_models(&[
            "model_1".to_string(),
            "model_2".to_string(),
        ]).unwrap();
        
        assert_eq!(comparison.len(), 2);
        assert_eq!(comparison[0].primary_metric, Some(0.85));
        assert_eq!(comparison[1].primary_metric, Some(0.92));
    }
}
