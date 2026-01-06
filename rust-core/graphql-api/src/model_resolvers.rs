use async_graphql::{Context, Object, FieldResult, InputObject, SimpleObject, Json};
use ontology_engine::{
    ModelObjective, ModelType, ModelStatus, ModelMetrics as EngineModelMetrics,
    ModelBinding, ModelBindingConfig, ModelPlatform, ModelRegistry,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use chrono::{DateTime, Utc};

// ============================================================================
// GraphQL Types for Model Objectives
// ============================================================================

/// GraphQL type for model platforms
#[derive(SimpleObject)]
pub struct ModelPlatformOutput {
    pub platform_type: String,
    pub framework: Option<String>,
    pub endpoint: Option<String>,
    pub region: Option<String>,
}

/// GraphQL metrics output
#[derive(SimpleObject)]
pub struct ModelMetricsOutput {
    // Classification metrics
    pub accuracy: Option<f64>,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1_score: Option<f64>,
    pub auc_roc: Option<f64>,
    
    // Regression metrics
    pub rmse: Option<f64>,
    pub mae: Option<f64>,
    pub r2_score: Option<f64>,
    pub mape: Option<f64>,
    
    // Clustering metrics
    pub silhouette_score: Option<f64>,
    pub davies_bouldin_score: Option<f64>,
    
    // Custom metrics
    pub custom_metrics: Option<Json<Value>>,
    
    // Primary metric
    pub primary_metric: Option<f64>,
}

/// GraphQL type for model objectives
#[derive(SimpleObject)]
pub struct ModelObjectiveOutput {
    pub id: String,
    pub name: String,
    pub model_type: String,
    pub version: String,
    pub description: Option<String>,
    pub platform: ModelPlatformOutput,
    pub metrics: ModelMetricsOutput,
    pub status: String,
    pub artifact_path: String,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: Option<String>,
    pub tags: Vec<String>,
}

/// GraphQL type for model bindings
#[derive(SimpleObject)]
pub struct ModelBindingOutput {
    pub model_id: String,
    pub object_type: String,
    pub property_id: String,
    pub bound_at: String,
    pub bound_by: Option<String>,
    pub config: ModelBindingConfigOutput,
}

/// GraphQL type for binding configuration
#[derive(SimpleObject)]
pub struct ModelBindingConfigOutput {
    pub input_properties: Vec<String>,
    pub cache_predictions: bool,
    pub cache_ttl_seconds: i64,
    pub async_execution: bool,
    pub fallback_value: Option<Json<Value>>,
}

/// GraphQL type for model comparison
#[derive(SimpleObject)]
pub struct ModelComparisonOutput {
    pub models: Vec<ModelObjectiveOutput>,
    pub comparison_table: Json<Value>,
    pub best_model_id: Option<String>,
}

// ============================================================================
// GraphQL Input Types
// ============================================================================

/// Input for registering a new model
#[derive(InputObject)]
pub struct RegisterModelInput {
    pub id: String,
    pub name: String,
    pub model_type: String,
    pub version: String,
    pub artifact_path: String,
    pub description: Option<String>,
    pub platform: ModelPlatformInput,
    pub tags: Option<Vec<String>>,
}

/// Input for model platform
#[derive(InputObject)]
pub struct ModelPlatformInput {
    pub platform_type: String, // "local", "sagemaker", "datarobot", "custom"
    pub framework: Option<String>, // "sklearn", "pytorch", "tensorflow"
    pub endpoint: Option<String>,
    pub region: Option<String>,
}

/// Input for updating model metrics
#[derive(InputObject)]
pub struct ModelMetricsInput {
    pub accuracy: Option<f64>,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub f1_score: Option<f64>,
    pub auc_roc: Option<f64>,
    pub rmse: Option<f64>,
    pub mae: Option<f64>,
    pub r2_score: Option<f64>,
    pub mape: Option<f64>,
    pub silhouette_score: Option<f64>,
    pub davies_bouldin_score: Option<f64>,
    pub custom_metrics: Option<String>, // JSON string
}

/// Input for binding a model to a property
#[derive(InputObject)]
pub struct BindModelInput {
    pub model_id: String,
    pub object_type: String,
    pub property_id: String,
    pub input_properties: Option<Vec<String>>,
    pub cache_predictions: Option<bool>,
    pub cache_ttl_seconds: Option<i64>,
    pub async_execution: Option<bool>,
}

/// Input for model prediction
#[derive(InputObject)]
pub struct PredictInput {
    pub model_id: String,
    pub inputs: String, // JSON string of input features
    pub use_cache: Option<bool>,
}

// ============================================================================
// Query Extensions for Models
// ============================================================================

/// Model-related queries
#[derive(Default)]
pub struct ModelQueries;

#[Object]
impl ModelQueries {
    /// List all registered models
    async fn models(
        &self,
        ctx: &Context<'_>,
        model_type: Option<String>,
        status: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> FieldResult<Vec<ModelObjectiveOutput>> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        let mut models: Vec<&ModelObjective> = registry_read.list();
        
        // Filter by type
        if let Some(type_filter) = model_type {
            let target_type = parse_model_type(&type_filter)?;
            models.retain(|m| m.model_type == target_type);
        }
        
        // Filter by status
        if let Some(status_filter) = status {
            let target_status = parse_model_status(&status_filter)?;
            models.retain(|m| m.status == target_status);
        }
        
        // Apply pagination
        let start = offset.unwrap_or(0);
        let end = limit.map(|l| start + l).unwrap_or(models.len());
        let paginated: Vec<_> = models.into_iter().skip(start).take(end - start).collect();
        
        Ok(paginated.into_iter().map(convert_model_to_output).collect())
    }
    
    /// Get a specific model by ID
    async fn model(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<Option<ModelObjectiveOutput>> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        Ok(registry_read.get(&id).map(convert_model_to_output))
    }
    
    /// Compare multiple models by ID
    async fn compare_models(
        &self,
        ctx: &Context<'_>,
        model_ids: Vec<String>,
    ) -> FieldResult<ModelComparisonOutput> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        let comparison = registry_read.compare_models(&model_ids)
            .map_err(|e| async_graphql::Error::new(format!("Comparison error: {}", e)))?;
        
        // comparison is Vec<ModelComparison>
        let models: Vec<ModelObjectiveOutput> = model_ids.iter()
            .filter_map(|id| registry_read.get(id))
            .map(convert_model_to_output)
            .collect();
        
        // Build comparison table from ModelComparison results
        let comparison_table = serde_json::json!({
            "metrics": comparison.iter().map(|c| {
                serde_json::json!({
                    "model_id": c.model_id,
                    "name": c.model_name,
                    "primary_metric": c.primary_metric,
                    "accuracy": c.metrics.accuracy,
                    "f1_score": c.metrics.f1_score,
                    "rmse": c.metrics.rmse,
                    "r2_score": c.metrics.r2,
                })
            }).collect::<Vec<_>>(),
        });
        
        // Find best model by primary metric
        let best_model_id = comparison.iter()
            .filter_map(|c| c.primary_metric.map(|m| (c.model_id.clone(), m)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id);
        
        Ok(ModelComparisonOutput {
            models,
            comparison_table: Json(comparison_table),
            best_model_id,
        })
    }
    
    /// Get all model bindings
    async fn model_bindings(
        &self,
        ctx: &Context<'_>,
        object_type: Option<String>,
    ) -> FieldResult<Vec<ModelBindingOutput>> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        let bindings: Vec<ModelBindingOutput> = registry_read.list_bindings()
            .into_iter()
            .filter(|b| object_type.as_ref().map_or(true, |ot| &b.object_type == ot))
            .map(convert_binding_to_output)
            .collect();
        
        Ok(bindings)
    }
    
    /// Get binding for a specific property
    async fn get_property_binding(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        property_id: String,
    ) -> FieldResult<Option<ModelBindingOutput>> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        Ok(registry_read.get_binding(&object_type, &property_id)
            .map(convert_binding_to_output))
    }
    
    /// Get models by type
    async fn models_by_type(
        &self,
        ctx: &Context<'_>,
        model_type: String,
    ) -> FieldResult<Vec<ModelObjectiveOutput>> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        let target_type = parse_model_type(&model_type)?;
        let models: Vec<ModelObjectiveOutput> = registry_read.list_by_type(&target_type)
            .into_iter()
            .map(convert_model_to_output)
            .collect();
        
        Ok(models)
    }
    
    /// Get models by status
    async fn models_by_status(
        &self,
        ctx: &Context<'_>,
        status: String,
    ) -> FieldResult<Vec<ModelObjectiveOutput>> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        let target_status = parse_model_status(&status)?;
        let models: Vec<ModelObjectiveOutput> = registry_read.list_by_status(&target_status)
            .into_iter()
            .map(convert_model_to_output)
            .collect();
        
        Ok(models)
    }
}

// ============================================================================
// Mutations for Models
// ============================================================================

/// Model-related mutations
#[derive(Default)]
pub struct ModelMutations;

#[Object]
impl ModelMutations {
    /// Register a new model
    async fn register_model(
        &self,
        ctx: &Context<'_>,
        input: RegisterModelInput,
    ) -> FieldResult<ModelObjectiveOutput> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let mut registry_write = registry.write().await;
        
        let model_type = parse_model_type(&input.model_type)?;
        let platform = parse_platform_input(input.platform)?;
        
        let model = ModelObjective::new(
            input.id,
            input.name,
            model_type,
            input.version,
            input.artifact_path,
            platform,
        );
        
        registry_write.register(model.clone())
            .map_err(|e| async_graphql::Error::new(format!("Registration failed: {}", e)))?;
        
        Ok(convert_model_to_output(&model))
    }
    
    /// Update model metrics
    async fn update_model_metrics(
        &self,
        ctx: &Context<'_>,
        model_id: String,
        metrics: ModelMetricsInput,
    ) -> FieldResult<ModelObjectiveOutput> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let mut registry_write = registry.write().await;
        
        let engine_metrics = convert_metrics_input(metrics)?;
        
        registry_write.update_metrics(&model_id, engine_metrics)
            .map_err(|e| async_graphql::Error::new(format!("Update failed: {}", e)))?;
        
        let model = registry_write.get(&model_id)
            .ok_or_else(|| async_graphql::Error::new("Model not found after update"))?;
        
        Ok(convert_model_to_output(model))
    }
    
    /// Bind a model to a property
    async fn bind_model(
        &self,
        ctx: &Context<'_>,
        input: BindModelInput,
    ) -> FieldResult<ModelBindingOutput> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let mut registry_write = registry.write().await;
        
        let config = ModelBindingConfig {
            input_properties: input.input_properties.unwrap_or_default(),
            cache_enabled: input.cache_predictions.unwrap_or(true),
            cache_ttl: input.cache_ttl_seconds.unwrap_or(3600) as u64,
            async_execution: input.async_execution.unwrap_or(false),
        };
        
        let binding = registry_write.bind_model(
            &input.model_id,
            input.object_type,
            input.property_id,
            None, // bound_by - would come from auth context
            config,
        ).map_err(|e| async_graphql::Error::new(format!("Binding failed: {}", e)))?;
        
        Ok(convert_binding_to_output(&binding))
    }
    
    /// Unbind a model from a property
    async fn unbind_model(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        property_id: String,
    ) -> FieldResult<bool> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let mut registry_write = registry.write().await;
        
        registry_write.unbind_model(&object_type, &property_id)
            .map_err(|e| async_graphql::Error::new(format!("Unbind failed: {}", e)))?;
        
        Ok(true)
    }
    
    /// Update model status
    async fn update_model_status(
        &self,
        ctx: &Context<'_>,
        model_id: String,
        status: String,
    ) -> FieldResult<ModelObjectiveOutput> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let mut registry_write = registry.write().await;
        
        let new_status = parse_model_status(&status)?;
        
        registry_write.update_status(&model_id, new_status)
            .map_err(|e| async_graphql::Error::new(format!("Status update failed: {}", e)))?;
        
        let model = registry_write.get(&model_id)
            .ok_or_else(|| async_graphql::Error::new("Model not found after update"))?;
        
        Ok(convert_model_to_output(model))
    }
    
    /// Delete a model
    async fn delete_model(
        &self,
        ctx: &Context<'_>,
        model_id: String,
    ) -> FieldResult<bool> {
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let mut registry_write = registry.write().await;
        
        registry_write.delete(&model_id)
            .map_err(|e| async_graphql::Error::new(format!("Delete failed: {}", e)))?;
        
        Ok(true)
    }
    
    /// Execute model prediction
    async fn predict(
        &self,
        ctx: &Context<'_>,
        input: PredictInput,
    ) -> FieldResult<Json<Value>> {
        // Get model from registry
        let registry = ctx.data::<Arc<RwLock<ModelRegistry>>>()?;
        let registry_read = registry.read().await;
        
        let model = registry_read.get(&input.model_id)
            .ok_or_else(|| async_graphql::Error::new("Model not found"))?;
        
        // Parse inputs
        let inputs: serde_json::Value = serde_json::from_str(&input.inputs)
            .map_err(|e| async_graphql::Error::new(format!("Invalid input JSON: {}", e)))?;
        
        // For now, return placeholder - actual execution would go through Python service
        let result = serde_json::json!({
            "model_id": model.id,
            "status": "prediction_pending",
            "message": "Model execution requires Python service. See model_service.py",
            "inputs": inputs,
        });
        
        Ok(Json(result))
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn parse_model_type(s: &str) -> FieldResult<ModelType> {
    match s.to_lowercase().as_str() {
        "classification" => Ok(ModelType::Classification),
        "regression" => Ok(ModelType::Regression),
        "clustering" => Ok(ModelType::Clustering),
        "time_series" | "timeseries" => Ok(ModelType::TimeSeries),
        "custom" => Ok(ModelType::Custom("custom".to_string())),
        _ => Err(async_graphql::Error::new(format!(
            "Invalid model type: {}. Valid: classification, regression, clustering, time_series, custom",
            s
        ))),
    }
}

fn parse_model_status(s: &str) -> FieldResult<ModelStatus> {
    match s.to_lowercase().as_str() {
        "training" => Ok(ModelStatus::Training),
        "registered" => Ok(ModelStatus::Registered),
        "bound" => Ok(ModelStatus::Bound),
        "deprecated" => Ok(ModelStatus::Deprecated),
        "archived" => Ok(ModelStatus::Archived),
        _ => Err(async_graphql::Error::new(format!(
            "Invalid model status: {}. Valid: training, registered, bound, deprecated, archived",
            s
        ))),
    }
}

fn parse_platform_input(input: ModelPlatformInput) -> FieldResult<ModelPlatform> {
    match input.platform_type.to_lowercase().as_str() {
        "local" => Ok(ModelPlatform::Local {
            framework: input.framework.unwrap_or_else(|| "sklearn".to_string()),
        }),
        "sagemaker" => Ok(ModelPlatform::SageMaker {
            endpoint_name: input.endpoint.unwrap_or_default(),
            region: input.region.unwrap_or_else(|| "us-east-1".to_string()),
        }),
        "datarobot" => Ok(ModelPlatform::DataRobot {
            deployment_id: input.endpoint.unwrap_or_default(),
        }),
        "custom" => Ok(ModelPlatform::Custom {
            platform_name: "custom".to_string(),
            endpoint_url: input.endpoint.unwrap_or_default(),
        }),
        _ => Err(async_graphql::Error::new(format!(
            "Invalid platform type: {}. Valid: local, sagemaker, datarobot, custom",
            input.platform_type
        ))),
    }
}

fn convert_model_to_output(model: &ModelObjective) -> ModelObjectiveOutput {
    let platform = match &model.platform {
        ModelPlatform::Local { framework } => ModelPlatformOutput {
            platform_type: "local".to_string(),
            framework: Some(framework.clone()),
            endpoint: None,
            region: None,
        },
        ModelPlatform::SageMaker { endpoint_name, region } => ModelPlatformOutput {
            platform_type: "sagemaker".to_string(),
            framework: None,
            endpoint: Some(endpoint_name.clone()),
            region: Some(region.clone()),
        },
        ModelPlatform::DataRobot { deployment_id } => ModelPlatformOutput {
            platform_type: "datarobot".to_string(),
            framework: None,
            endpoint: Some(deployment_id.clone()),
            region: None,
        },
        ModelPlatform::Custom { platform_name: _platform_name, endpoint_url } => ModelPlatformOutput {
            platform_type: "custom".to_string(),
            framework: None,
            endpoint: Some(endpoint_url.clone()),
            region: None,
        },
    };
    
    let metrics = ModelMetricsOutput {
        accuracy: model.metrics.accuracy,
        precision: model.metrics.precision,
        recall: model.metrics.recall,
        f1_score: model.metrics.f1_score,
        auc_roc: model.metrics.auc_roc,
        rmse: model.metrics.rmse,
        mae: model.metrics.mae,
        r2_score: model.metrics.r2,
        mape: model.metrics.mape,
        silhouette_score: model.metrics.silhouette_score,
        davies_bouldin_score: model.metrics.davies_bouldin_score,
        custom_metrics: if model.metrics.custom.is_empty() {
            None
        } else {
            Some(Json(serde_json::to_value(&model.metrics.custom).unwrap_or_default()))
        },
        primary_metric: model.metrics.primary_metric(&model.model_type),
    };
    
    let status = match model.status {
        ModelStatus::Training => "training",
        ModelStatus::Registered => "registered",
        ModelStatus::Bound => "bound",
        ModelStatus::Deprecated => "deprecated",
        ModelStatus::Archived => "archived",
    };
    
    let model_type = match model.model_type {
        ModelType::Classification => "classification",
        ModelType::Regression => "regression",
        ModelType::Clustering => "clustering",
        ModelType::TimeSeries => "time_series",
        ModelType::Custom(_) => "custom",
    };
    
    ModelObjectiveOutput {
        id: model.id.clone(),
        name: model.name.clone(),
        model_type: model_type.to_string(),
        version: model.version.clone(),
        description: model.description.clone(),
        platform,
        metrics,
        status: status.to_string(),
        artifact_path: model.artifact_path.clone(),
        created_at: model.created_at.to_rfc3339(),
        updated_at: model.updated_at.to_rfc3339(),
        created_by: model.created_by.clone(),
        tags: model.tags.clone(),
    }
}

fn convert_binding_to_output(binding: &ModelBinding) -> ModelBindingOutput {
    let config = ModelBindingConfigOutput {
        input_properties: binding.config.input_properties.clone(),
        cache_predictions: binding.config.cache_enabled,
        cache_ttl_seconds: binding.config.cache_ttl as i64,
        async_execution: binding.config.async_execution,
        fallback_value: None,
    };
    
    ModelBindingOutput {
        model_id: binding.model_id.clone(),
        object_type: binding.object_type.clone(),
        property_id: binding.property_id.clone(),
        bound_at: binding.bound_at.to_rfc3339(),
        bound_by: binding.bound_by.clone(),
        config,
    }
}

fn convert_metrics_input(input: ModelMetricsInput) -> FieldResult<EngineModelMetrics> {
    let custom_metrics = if let Some(json_str) = input.custom_metrics {
        serde_json::from_str(&json_str)
            .map_err(|e| async_graphql::Error::new(format!("Invalid custom metrics JSON: {}", e)))?
    } else {
        std::collections::HashMap::new()
    };
    
    Ok(EngineModelMetrics {
        accuracy: input.accuracy,
        precision: input.precision,
        recall: input.recall,
        f1_score: input.f1_score,
        auc_roc: input.auc_roc,
        rmse: input.rmse,
        mae: input.mae,
        r2: input.r2_score,
        mape: input.mape,
        silhouette_score: input.silhouette_score,
        davies_bouldin_score: input.davies_bouldin_score,
        custom: custom_metrics,
    })
}
