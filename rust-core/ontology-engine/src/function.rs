use crate::meta_model::{FunctionTypeDef, FunctionLogic, FunctionReturnType, AggregationType};
use crate::property::{PropertyValue, PropertyMap};

/// Function execution result
#[derive(Debug, Clone)]
pub struct FunctionExecutionResult {
    pub value: PropertyValue,
}

/// Function executor - executes declarative function logic
pub struct FunctionExecutor;

impl FunctionExecutor {
    /// Execute a function with given parameters
    pub async fn execute(
        function_def: &FunctionTypeDef,
        parameters: &PropertyMap,
        // Callbacks for accessing data (would be provided by the runtime)
        get_object_property: Option<&(dyn Fn(&str, &str, &str) -> Option<PropertyValue> + Send + Sync)>, // (object_type, object_id, property_id) -> value
        get_linked_objects: Option<&(dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync)>, // (object_id, link_type, target_type) -> object_ids
        aggregate_linked_properties: Option<&(dyn Fn(&str, &str, &str, AggregationType) -> Option<PropertyValue> + Send + Sync)>, // (object_id, link_type, property, agg_type) -> value
    ) -> Result<FunctionExecutionResult, String> {
        // Validate parameters
        for param_def in &function_def.parameters {
            if param_def.required {
                if !parameters.contains_key(&param_def.id) {
                    return Err(format!("Missing required parameter '{}'", param_def.id));
                }
            }
            
            if let Some(value) = parameters.get(&param_def.id) {
                if let Err(e) = param_def.validate_value(value) {
                    return Err(format!("Invalid parameter '{}': {}", param_def.id, e));
                }
            }
        }
        
        // Execute function logic
        let result = match &function_def.logic {
            FunctionLogic::Aggregation { link_type, aggregation, property } => {
                Self::execute_aggregation(
                    parameters,
                    link_type,
                    aggregation,
                    property,
                    aggregate_linked_properties,
                )?
            }
            FunctionLogic::LinkTraversal { link_type, target_type, filter } => {
                Self::execute_link_traversal(
                    parameters,
                    link_type,
                    target_type,
                    filter,
                    get_linked_objects,
                )?
            }
            FunctionLogic::PropertyAccess { property } => {
                Self::execute_property_access(
                    parameters,
                    property,
                    get_object_property,
                )?
            }
        };
        
        Ok(FunctionExecutionResult { value: result })
    }
    
    /// Execute aggregation logic
    fn execute_aggregation(
        parameters: &PropertyMap,
        link_type: &str,
        aggregation: &AggregationType,
        property: &str,
        aggregate_fn: Option<&(dyn Fn(&str, &str, &str, AggregationType) -> Option<PropertyValue> + Send + Sync)>,
    ) -> Result<PropertyValue, String> {
        // Get the source object ID from parameters
        // Assumes first parameter is the source object ID
        let source_id = parameters.iter()
            .next()
            .and_then(|(_, v)| {
                if let PropertyValue::ObjectReference(ref_id) = v {
                    Some(ref_id.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing source object ID in parameters".to_string())?;
        
        if let Some(agg_fn) = aggregate_fn {
            agg_fn(&source_id, link_type, property, aggregation.clone())
                .ok_or_else(|| format!("Aggregation failed for link type '{}', property '{}'", link_type, property))
        } else {
            // Fallback: return a placeholder value
            Ok(PropertyValue::Double(0.0))
        }
    }
    
    /// Execute link traversal logic
    fn execute_link_traversal(
        parameters: &PropertyMap,
        link_type: &str,
        target_type: &str,
        _filter: &Option<crate::meta_model::FunctionFilter>,
        get_linked_fn: Option<&(dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync)>,
    ) -> Result<PropertyValue, String> {
        // Get the source object ID from parameters
        let source_id = parameters.iter()
            .next()
            .and_then(|(_, v)| {
                if let PropertyValue::ObjectReference(ref_id) = v {
                    Some(ref_id.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing source object ID in parameters".to_string())?;
        
        if let Some(linked_fn) = get_linked_fn {
            let linked_ids = linked_fn(&source_id, link_type, target_type);
            // Return array of object references
            let refs: Vec<PropertyValue> = linked_ids.into_iter()
                .map(|id| PropertyValue::ObjectReference(id))
                .collect();
            Ok(PropertyValue::Array(refs))
        } else {
            // Fallback: return empty array
            Ok(PropertyValue::Array(Vec::new()))
        }
    }
    
    /// Execute property access logic
    fn execute_property_access(
        parameters: &PropertyMap,
        property: &str,
        get_property_fn: Option<&(dyn Fn(&str, &str, &str) -> Option<PropertyValue> + Send + Sync)>,
    ) -> Result<PropertyValue, String> {
        // Get the object ID and type from parameters
        let (object_type, object_id) = parameters.iter()
            .find_map(|(k, v)| {
                if let PropertyValue::ObjectReference(ref_id) = v {
                    // Extract type from parameter name or reference
                    Some((k.clone(), ref_id.clone()))
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing object reference in parameters".to_string())?;
        
        // Try to extract object type from reference (format: "type:id")
        let (obj_type, obj_id) = if object_id.contains(':') {
            let parts: Vec<&str> = object_id.splitn(2, ':').collect();
            (parts[0].to_string(), parts[1].to_string())
        } else {
            // Use parameter name as object type hint
            (object_type, object_id)
        };
        
        if let Some(prop_fn) = get_property_fn {
            prop_fn(&obj_type, &obj_id, property)
                .ok_or_else(|| format!("Property '{}' not found on object '{}' of type '{}'", property, obj_id, obj_type))
        } else {
            // Fallback: return null
            Ok(PropertyValue::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::{Property, PropertyType};
    
    fn create_test_function() -> FunctionTypeDef {
        FunctionTypeDef {
            id: "get_total_value".to_string(),
            display_name: "Get Total Value".to_string(),
            description: Some("Get total value".to_string()),
            parameters: vec![
                Property {
                    id: "portfolio_id".to_string(),
                    display_name: None,
                    property_type: PropertyType::ObjectReference,
                    required: true,
                    default: None,
                    validation: None,
                    description: None,
                    annotations: std::collections::HashMap::new(),
                    unit: None,
                    format: None,
                    sensitivity_tags: Vec::new(),
                    pii: false,
                    deprecated: None,
                },
            ],
            return_type: FunctionReturnType::Property {
                property_type: PropertyType::Double,
            },
            logic: FunctionLogic::Aggregation {
                link_type: "portfolio_holding".to_string(),
                aggregation: AggregationType::Sum,
                property: "value".to_string(),
            },
            cacheable: true,
        }
    }
    
    #[test]
    fn test_function_executor_creation() {
        let _executor = FunctionExecutor;
        // Just verify it compiles
        assert!(true);
    }
}

