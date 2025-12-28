use crate::action::{Action, ActionType, ActionOperation, OperationType, ActionSideEffect, SideEffectType};
use crate::property::{PropertyValue, PropertyMap};
use crate::validation::{validate_action, ActionContext, ValidationError};
use std::collections::HashMap;

/// Action execution result
#[derive(Debug, Clone)]
pub struct ActionExecutionResult {
    pub success: bool,
    pub operations_executed: Vec<String>,
    pub errors: Vec<String>,
    pub side_effects_triggered: Vec<String>,
}

/// Action executor - executes actions with template substitution
pub struct ActionExecutor {
    /// Function to execute object operations (create, update, delete)
    pub object_operation_handler: Option<Box<dyn Fn(&OperationType, &str, Option<&PropertyMap>) -> Result<String, String> + Send + Sync>>,
    /// Function to execute link operations
    pub link_operation_handler: Option<Box<dyn Fn(&str, &str, &str, &PropertyMap) -> Result<String, String> + Send + Sync>>,
    /// Function to handle side effects
    pub side_effect_handler: Option<Box<dyn Fn(&SideEffectType, &PropertyMap) -> Result<(), String> + Send + Sync>>,
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            object_operation_handler: None,
            link_operation_handler: None,
            side_effect_handler: None,
        }
    }
    
    /// Execute an action
    pub fn execute(
        &self,
        action: &Action,
        action_type: &ActionType,
        context: &ActionContext,
    ) -> Result<ActionExecutionResult, ValidationError> {
        // Validate action first
        validate_action(action, action_type, context)?;
        
        let mut result = ActionExecutionResult {
            success: true,
            operations_executed: Vec::new(),
            errors: Vec::new(),
            side_effects_triggered: Vec::new(),
        };
        
        // Execute each operation in the action's logic
        for operation in &action_type.logic {
            match self.execute_operation(operation, &action.parameters, context) {
                Ok(op_id) => {
                    result.operations_executed.push(op_id);
                }
                Err(e) => {
                    result.errors.push(e);
                    result.success = false;
                    // Continue executing other operations (could be made configurable)
                }
            }
        }
        
        // Execute side effects
        for side_effect in &action_type.side_effects {
            match self.execute_side_effect(side_effect, &action.parameters, context) {
                Ok(()) => {
                    result.side_effects_triggered.push(format!("{:?}", side_effect.effect_type));
                }
                Err(e) => {
                    result.errors.push(format!("Side effect error: {}", e));
                    // Side effect failures don't fail the action
                }
            }
        }
        
        if result.success {
            Ok(result)
        } else {
            Err(ValidationError::InvalidParameter(format!(
                "Action execution failed: {:?}",
                result.errors
            )))
        }
    }
    
    /// Execute a single operation with template substitution
    fn execute_operation(
        &self,
        operation: &ActionOperation,
        parameters: &PropertyMap,
        _context: &ActionContext,
    ) -> Result<String, String> {
        // Substitute template variables in properties
        let substituted_properties = self.substitute_templates(&operation.properties, parameters)?;
        
        match &operation.operation {
            OperationType::CreateObject => {
                let object_type = operation.object_type.as_ref()
                    .ok_or_else(|| "CreateObject requires object_type".to_string())?;
                
                if let Some(handler) = &self.object_operation_handler {
                    handler(&operation.operation, object_type, Some(&substituted_properties))
                } else {
                    Ok(format!("create_object_{}", uuid::Uuid::new_v4()))
                }
            }
            OperationType::UpdateObject => {
                let object_type = operation.object_type.as_ref()
                    .ok_or_else(|| "UpdateObject requires object_type".to_string())?;
                
                if let Some(handler) = &self.object_operation_handler {
                    handler(&operation.operation, object_type, Some(&substituted_properties))
                } else {
                    Ok(format!("update_object_{}", uuid::Uuid::new_v4()))
                }
            }
            OperationType::DeleteObject => {
                let object_type = operation.object_type.as_ref()
                    .ok_or_else(|| "DeleteObject requires object_type".to_string())?;
                
                if let Some(handler) = &self.object_operation_handler {
                    handler(&operation.operation, object_type, None)
                } else {
                    Ok(format!("delete_object_{}", uuid::Uuid::new_v4()))
                }
            }
            OperationType::CreateLink => {
                let link_type = operation.link_type.as_ref()
                    .ok_or_else(|| "CreateLink requires link_type".to_string())?;
                let from = operation.from.as_ref()
                    .ok_or_else(|| "CreateLink requires from".to_string())?;
                let to = operation.to.as_ref()
                    .ok_or_else(|| "CreateLink requires to".to_string())?;
                
                // Substitute templates in from/to
                let from_sub = self.substitute_string_template(from, parameters)?;
                let to_sub = self.substitute_string_template(to, parameters)?;
                
                if let Some(handler) = &self.link_operation_handler {
                    handler(link_type, &from_sub, &to_sub, &substituted_properties)
                } else {
                    Ok(format!("create_link_{}", uuid::Uuid::new_v4()))
                }
            }
            OperationType::DeleteLink => {
                let link_type = operation.link_type.as_ref()
                    .ok_or_else(|| "DeleteLink requires link_type".to_string())?;
                let from = operation.from.as_ref()
                    .ok_or_else(|| "DeleteLink requires from".to_string())?;
                let to = operation.to.as_ref()
                    .ok_or_else(|| "DeleteLink requires to".to_string())?;
                
                let from_sub = self.substitute_string_template(from, parameters)?;
                let to_sub = self.substitute_string_template(to, parameters)?;
                
                if let Some(handler) = &self.link_operation_handler {
                    handler(link_type, &from_sub, &to_sub, &PropertyMap::new())
                } else {
                    Ok(format!("delete_link_{}", uuid::Uuid::new_v4()))
                }
            }
            OperationType::UpdateProperty => {
                // UpdateProperty would update a specific property
                Ok(format!("update_property_{}", uuid::Uuid::new_v4()))
            }
        }
    }
    
    /// Substitute template variables in a PropertyMap
    fn substitute_templates(
        &self,
        properties: &PropertyMap,
        parameters: &PropertyMap,
    ) -> Result<PropertyMap, String> {
        let mut result = PropertyMap::new();
        
        for (key, value) in properties.iter() {
            let substituted_value = match value {
                PropertyValue::String(s) => {
                    PropertyValue::String(self.substitute_string_template(s, parameters)?)
                }
                PropertyValue::Integer(i) => {
                    // Check if integer is actually a template reference
                    let s = i.to_string();
                    if s.starts_with("{{") && s.ends_with("}}") {
                        let param_name = s.trim_start_matches("{{").trim_end_matches("}}").trim();
                        if let Some(PropertyValue::Integer(val)) = parameters.get(param_name) {
                            PropertyValue::Integer(*val)
                        } else {
                            return Err(format!("Template parameter '{}' not found or not an integer", param_name));
                        }
                    } else {
                        PropertyValue::Integer(*i)
                    }
                }
                PropertyValue::Double(d) => {
                    let s = d.to_string();
                    if s.starts_with("{{") && s.ends_with("}}") {
                        let param_name = s.trim_start_matches("{{").trim_end_matches("}}").trim();
                        if let Some(PropertyValue::Double(val)) = parameters.get(param_name) {
                            PropertyValue::Double(*val)
                        } else {
                            return Err(format!("Template parameter '{}' not found or not a double", param_name));
                        }
                    } else {
                        PropertyValue::Double(*d)
                    }
                }
                v => v.clone(),
            };
            result.insert(key.clone(), substituted_value);
        }
        
        Ok(result)
    }
    
    /// Substitute template variables in a string (format: {{variable_name}})
    fn substitute_string_template(
        &self,
        template: &str,
        parameters: &PropertyMap,
    ) -> Result<String, String> {
        let mut result = template.to_string();
        
        // Find all template variables ({{variable_name}})
        let re = regex::Regex::new(r"\{\{([^}]+)\}\}").map_err(|e| format!("Regex error: {}", e))?;
        
        for cap in re.captures_iter(template) {
            let full_match = cap.get(0).unwrap().as_str();
            let var_name = cap.get(1).unwrap().as_str().trim();
            
            // Get value from parameters
            let value = parameters.get(var_name)
                .ok_or_else(|| format!("Template parameter '{}' not found", var_name))?;
            
            // Convert to string and substitute
            let value_str = match value {
                PropertyValue::String(s) => s.clone(),
                PropertyValue::Integer(i) => i.to_string(),
                PropertyValue::Double(d) => d.to_string(),
                PropertyValue::Boolean(b) => b.to_string(),
                PropertyValue::Date(d) => d.clone(),
                PropertyValue::DateTime(dt) => dt.clone(),
                PropertyValue::ObjectReference(id) => id.clone(),
                PropertyValue::GeoJSON(gj) => gj.clone(),
                PropertyValue::Array(_) => {
                    // Serialize array to JSON string
                    serde_json::to_string(value).unwrap_or_else(|_| "[]".to_string())
                }
                PropertyValue::Map(_) => {
                    // Serialize map to JSON string
                    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
                }
                PropertyValue::Object(_) => {
                    // Serialize object to JSON string
                    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
                }
                PropertyValue::Null => "null".to_string(),
            };
            
            result = result.replace(full_match, &value_str);
        }
        
        Ok(result)
    }
    
    /// Execute a side effect
    fn execute_side_effect(
        &self,
        side_effect: &ActionSideEffect,
        parameters: &PropertyMap,
        _context: &ActionContext,
    ) -> Result<(), String> {
        // Substitute templates in side effect config
        let substituted_config = self.substitute_templates(&side_effect.config, parameters)?;
        
        if let Some(handler) = &self.side_effect_handler {
            handler(&side_effect.effect_type, &substituted_config)
        } else {
            // Default handlers (stubs)
            match &side_effect.effect_type {
                SideEffectType::Email => {
                    // Stub email handler
                    Ok(())
                }
                SideEffectType::Webhook => {
                    // Stub webhook handler
                    Ok(())
                }
                SideEffectType::Notification => {
                    // Stub notification handler
                    Ok(())
                }
                SideEffectType::Log => {
                    // Stub log handler
                    eprintln!("Action side effect: {:?} with config: {:?}", side_effect.effect_type, substituted_config);
                    Ok(())
                }
            }
        }
    }
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::ActionType;
    use crate::property::Property;
    
    #[test]
    fn test_template_substitution() {
        let executor = ActionExecutor::new();
        let mut params = PropertyMap::new();
        params.insert("source_year".to_string(), PropertyValue::Integer(1990));
        params.insert("target_year".to_string(), PropertyValue::Integer(2010));
        params.insert("variable_name".to_string(), PropertyValue::String("population".to_string()));
        
        let result = executor.substitute_string_template(
            "Normalize from {{source_year}} to {{target_year}} for {{variable_name}}",
            &params,
        ).unwrap();
        
        assert_eq!(result, "Normalize from 1990 to 2010 for population");
    }
    
    #[test]
    fn test_template_substitution_missing_param() {
        let executor = ActionExecutor::new();
        let params = PropertyMap::new();
        
        let result = executor.substitute_string_template(
            "Value: {{missing_param}}",
            &params,
        );
        
        assert!(result.is_err());
    }
}



