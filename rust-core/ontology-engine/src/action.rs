use serde::{Deserialize, Serialize};
use crate::property::{PropertyValue, PropertyMap};

/// Action Type definition (re-exported from meta_model for convenience)
pub use crate::meta_model::ActionTypeDef as ActionType;

/// Action validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionValidation {
    #[serde(default)]
    pub required_roles: Vec<String>,
    
    #[serde(default)]
    pub required_badges: Vec<String>,
    
    #[serde(default)]
    pub conditions: Vec<ActionCondition>,
}

/// Action condition for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionCondition {
    pub property: String,
    pub operator: ConditionOperator,
    pub value: PropertyValue,
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    In,
    NotIn,
}

/// Action operation - what the action does
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOperation {
    pub operation: OperationType,
    
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    
    #[serde(rename = "linkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    
    #[serde(default)]
    pub properties: PropertyMap,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// Operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    CreateObject,
    UpdateObject,
    DeleteObject,
    CreateLink,
    DeleteLink,
    UpdateProperty,
}

/// Action side effect - external actions triggered by the action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSideEffect {
    #[serde(rename = "type")]
    pub effect_type: SideEffectType,
    
    #[serde(default)]
    pub config: PropertyMap,
}

/// Side effect types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SideEffectType {
    Email,
    Webhook,
    Notification,
    Log,
}

/// Runtime action execution context
#[derive(Debug, Clone)]
pub struct Action {
    pub action_type_id: String,
    pub parameters: PropertyMap,
    pub executed_by: String, // User ID
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Action {
    pub fn new(
        action_type_id: String,
        parameters: PropertyMap,
        executed_by: String,
    ) -> Self {
        Self {
            action_type_id,
            parameters,
            executed_by,
            timestamp: chrono::Utc::now(),
        }
    }
}

