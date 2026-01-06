use crate::action::{Action, ActionType, ActionCondition, ConditionOperator};
use crate::property::{PropertyValue, PropertyMap};

/// Validate an action before execution
pub fn validate_action(
    action: &Action,
    action_type: &ActionType,
    context: &ActionContext,
) -> Result<(), ValidationError> {
    // Check required roles
    if let Some(validation) = &action_type.validation {
        if !validation.required_roles.is_empty() {
            let user_roles = context.user_roles.as_deref().unwrap_or(&[]);
            let has_required_role = validation.required_roles.iter()
                .any(|role| user_roles.contains(role));
            if !has_required_role {
                return Err(ValidationError::MissingRole(
                    format!("Missing required role. Required: {:?}, User has: {:?}",
                        validation.required_roles, user_roles)
                ));
            }
        }
        
        // Check required badges
        if !validation.required_badges.is_empty() {
            let user_badges = context.user_badges.as_deref().unwrap_or(&[]);
            let has_required_badge = validation.required_badges.iter()
                .any(|badge| user_badges.contains(badge));
            if !has_required_badge {
                return Err(ValidationError::MissingBadge(
                    format!("Missing required badge. Required: {:?}, User has: {:?}",
                        validation.required_badges, user_badges)
                ));
            }
        }
        
        // Check conditions
        for condition in &validation.conditions {
            if let Err(e) = check_condition(condition, &action.parameters, context) {
                return Err(e);
            }
        }
    }
    
    // Validate parameters against action type definition
    for param_def in &action_type.parameters {
        if param_def.required {
            if !action.parameters.contains_key(&param_def.id) {
                return Err(ValidationError::MissingParameter(param_def.id.clone()));
            }
        }
        
        if let Some(value) = action.parameters.get(&param_def.id) {
            if let Err(e) = param_def.validate_value(value) {
                return Err(ValidationError::InvalidParameter(format!(
                    "Parameter '{}': {}",
                    param_def.id, e
                )));
            }
        }
    }
    
    Ok(())
}

/// Action execution context
#[derive(Debug, Clone)]
pub struct ActionContext {
    pub user_id: String,
    pub user_roles: Option<Vec<String>>,
    pub user_badges: Option<Vec<String>>,
    pub object_properties: Option<PropertyMap>, // Properties of the object being acted upon
}

impl ActionContext {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            user_roles: None,
            user_badges: None,
            object_properties: None,
        }
    }
    
    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.user_roles = Some(roles);
        self
    }
    
    pub fn with_badges(mut self, badges: Vec<String>) -> Self {
        self.user_badges = Some(badges);
        self
    }
    
    pub fn with_object_properties(mut self, properties: PropertyMap) -> Self {
        self.object_properties = Some(properties);
        self
    }
}

/// Check a condition against the action parameters or object properties
fn check_condition(
    condition: &ActionCondition,
    parameters: &PropertyMap,
    context: &ActionContext,
) -> Result<(), ValidationError> {
    // Try to get the property value from parameters first, then object properties
    let value = parameters.get(&condition.property)
        .or_else(|| {
            context.object_properties.as_ref()
                .and_then(|props| props.get(&condition.property))
        })
        .ok_or_else(|| ValidationError::InvalidCondition(format!(
            "Property '{}' not found in parameters or object properties",
            condition.property
        )))?;
    
    let matches = match &condition.operator {
        ConditionOperator::Equals => value == &condition.value,
        ConditionOperator::NotEquals => value != &condition.value,
        ConditionOperator::GreaterThan => compare_values(value, &condition.value, |a, b| a > b),
        ConditionOperator::LessThan => compare_values(value, &condition.value, |a, b| a < b),
        ConditionOperator::GreaterThanOrEqual => compare_values(value, &condition.value, |a, b| a >= b),
        ConditionOperator::LessThanOrEqual => compare_values(value, &condition.value, |a, b| a <= b),
        ConditionOperator::In => {
            // For "In", condition.value should be an Array containing values to check against
            match &condition.value {
                PropertyValue::Array(arr) => {
                    // Check if value is contained in the array
                    arr.iter().any(|item| item == value)
                }
                _ => {
                    // If condition.value is not an array, this is an invalid condition
                    return Err(ValidationError::InvalidCondition(format!(
                        "In operator requires an array value, got: {:?}",
                        condition.value
                    )));
                }
            }
        }
        ConditionOperator::NotIn => {
            // For "NotIn", condition.value should be an Array containing values to check against
            match &condition.value {
                PropertyValue::Array(arr) => {
                    // Check if value is NOT contained in the array
                    !arr.iter().any(|item| item == value)
                }
                _ => {
                    // If condition.value is not an array, this is an invalid condition
                    return Err(ValidationError::InvalidCondition(format!(
                        "NotIn operator requires an array value, got: {:?}",
                        condition.value
                    )));
                }
            }
        }
    };
    
    if !matches {
        return Err(ValidationError::InvalidCondition(format!(
            "Condition failed: {} {:?} {:?}",
            condition.property,
            condition.operator,
            condition.value
        )));
    }
    
    Ok(())
}

/// Compare two property values numerically
fn compare_values<F>(a: &PropertyValue, b: &PropertyValue, op: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    let a_num = match a {
        PropertyValue::Integer(i) => *i as f64,
        PropertyValue::Double(d) => *d,
        _ => return false,
    };
    
    let b_num = match b {
        PropertyValue::Integer(i) => *i as f64,
        PropertyValue::Double(d) => *d,
        _ => return false,
    };
    
    op(a_num, b_num)
}

/// Validation errors
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Missing required role: {0}")]
    MissingRole(String),
    
    #[error("Missing required badge: {0}")]
    MissingBadge(String),
    
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    
    #[error("Invalid condition: {0}")]
    InvalidCondition(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::{ActionType, ActionValidation};
    use crate::property::{Property, PropertyType, PropertyMap};
    
    fn create_test_action_type() -> ActionType {
        ActionType {
            id: "test_action".to_string(),
            display_name: "Test Action".to_string(),
            parameters: vec![
            Property {
                id: "required_param".to_string(),
                display_name: None,
                property_type: PropertyType::String,
                required: true,
                default: None,
                validation: None,
                description: None,
                annotations: std::collections::HashMap::new(),
                unit: None,
                format: None,
                sensitivity_tags: vec![],
                pii: false,
                deprecated: None,
                    statistics: None,
                    model_binding: None,            },
            ],
            logic: vec![],
            validation: None,
            side_effects: vec![],
        }
    }
    
    #[test]
    fn test_action_validation_missing_required_parameter() {
        let action_type = create_test_action_type();
        let action = Action {
            action_type_id: "test_action".to_string(),
            parameters: PropertyMap::new(),
            executed_by: "user1".to_string(),
            timestamp: chrono::Utc::now(),
        };
        let context = ActionContext::new("user1".to_string());
        
        let result = validate_action(&action, &action_type, &context);
        assert!(result.is_err());
        match result.unwrap_err() {
            ValidationError::MissingParameter(_) => {}
            _ => panic!("Expected MissingParameter error"),
        }
    }
    
    #[test]
    fn test_action_validation_with_role_requirement() {
        let mut action_type = create_test_action_type();
        action_type.validation = Some(ActionValidation {
            required_roles: vec!["admin".to_string()],
            required_badges: vec![],
            conditions: vec![],
        });
        
        let mut params = PropertyMap::new();
        params.insert("required_param".to_string(), PropertyValue::String("value".to_string()));
        let action = Action {
            action_type_id: "test_action".to_string(),
            parameters: params,
            executed_by: "user1".to_string(),
            timestamp: chrono::Utc::now(),
        };
        
        // Should fail without admin role
        let context = ActionContext::new("user1".to_string());
        assert!(validate_action(&action, &action_type, &context).is_err());
        
        // Should pass with admin role
        let context = ActionContext::new("user1".to_string()).with_roles(vec!["admin".to_string()]);
        assert!(validate_action(&action, &action_type, &context).is_ok());
    }
    
    #[test]
    fn test_action_context_builder() {
        let context = ActionContext::new("user1".to_string())
            .with_roles(vec!["admin".to_string()])
            .with_badges(vec!["premium".to_string()]);
        
        assert_eq!(context.user_id, "user1");
        assert_eq!(context.user_roles, Some(vec!["admin".to_string()]));
        assert_eq!(context.user_badges, Some(vec!["premium".to_string()]));
    }
}
