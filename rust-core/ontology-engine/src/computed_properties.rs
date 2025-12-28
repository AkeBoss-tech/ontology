use serde::{Deserialize, Serialize};
use crate::property::{PropertyValue, PropertyMap, PropertyType};

/// Computed property definition - a property whose value is calculated from other properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedProperty {
    pub id: String,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    #[serde(rename = "type")]
    pub property_type: PropertyType,
    
    #[serde(default)]
    pub description: Option<String>,
    
    /// Expression or function that computes the value
    pub expression: ComputedExpression,
    
    /// Properties this computed property depends on
    #[serde(default)]
    pub dependencies: Vec<String>,
    
    /// Whether this property is cached
    #[serde(default)]
    pub cached: bool,
    
    /// Cache TTL in seconds (if cached)
    #[serde(rename = "cacheTtl")]
    #[serde(default)]
    pub cache_ttl: Option<u64>,
}

/// Expression types for computed properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComputedExpression {
    /// Simple arithmetic expression (e.g., "property1 + property2")
    Arithmetic {
        expression: String,
    },
    /// Function call (e.g., "sum(property1, property2)")
    Function {
        function_id: String,
        parameters: Vec<String>, // Property IDs
    },
    /// Conditional expression (e.g., "if property1 > 0 then property2 else 0")
    Conditional {
        condition: String,
        then_expression: String,
        else_expression: String,
    },
    /// String concatenation or formatting
    StringFormat {
        template: String, // e.g., "{property1} - {property2}"
    },
    /// Aggregation over linked objects
    LinkAggregation {
        link_type: String,
        property: String,
        aggregation: AggregationType,
    },
}

/// Aggregation types for link aggregations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationType {
    Sum,
    Avg,
    Count,
    Min,
    Max,
}

/// Evaluator for computed properties
pub struct ComputedPropertyEvaluator;

impl ComputedPropertyEvaluator {
    /// Evaluate a computed property value
    pub fn evaluate<F>(
        computed: &ComputedProperty,
        properties: &PropertyMap,
        get_linked_property: Option<F>,
    ) -> Result<PropertyValue, ComputedPropertyError>
    where
        F: Fn(&str, &str) -> Option<PropertyValue>,
    {
        match &computed.expression {
            ComputedExpression::Arithmetic { expression } => {
                Self::evaluate_arithmetic(expression, properties)
            }
            ComputedExpression::Function { function_id, parameters } => {
                Self::evaluate_function(function_id, parameters, properties)
            }
            ComputedExpression::Conditional { condition, then_expression, else_expression } => {
                Self::evaluate_conditional(condition, then_expression, else_expression, properties)
            }
            ComputedExpression::StringFormat { template } => {
                Self::evaluate_string_format(template, properties)
            }
            ComputedExpression::LinkAggregation { link_type, property, aggregation } => {
                if let Some(getter) = get_linked_property {
                    Self::evaluate_link_aggregation(link_type, property, aggregation, getter)
                } else {
                    Err(ComputedPropertyError::MissingDependency(
                        "Link property getter required for link aggregation".to_string()
                    ))
                }
            }
        }
    }
    
    fn evaluate_arithmetic(expression: &str, properties: &PropertyMap) -> Result<PropertyValue, ComputedPropertyError> {
        // Simple arithmetic evaluation (would need a proper expression parser in production)
        // For now, support simple operations like "property1 + property2"
        let mut result = 0.0;
        let parts: Vec<&str> = expression.split_whitespace().collect();
        
        for part in parts {
            if let Some(prop_value) = properties.get(part) {
                match prop_value {
                    PropertyValue::Integer(i) => result += *i as f64,
                    PropertyValue::Double(d) => result += d,
                    _ => return Err(ComputedPropertyError::InvalidType(
                        format!("Property {} is not numeric", part)
                    )),
                }
            } else if let Ok(num) = part.parse::<f64>() {
                result += num;
            } else if part == "+" || part == "-" || part == "*" || part == "/" {
                // Operator handling would go here
            }
        }
        
        Ok(PropertyValue::Double(result))
    }
    
    fn evaluate_function(
        _function_id: &str,
        _parameters: &[String],
        _properties: &PropertyMap,
    ) -> Result<PropertyValue, ComputedPropertyError> {
        // Function evaluation would be implemented here
        Err(ComputedPropertyError::NotImplemented("Function evaluation not yet implemented".to_string()))
    }
    
    fn evaluate_conditional(
        _condition: &str,
        _then_expr: &str,
        _else_expr: &str,
        _properties: &PropertyMap,
    ) -> Result<PropertyValue, ComputedPropertyError> {
        // Conditional evaluation would be implemented here
        Err(ComputedPropertyError::NotImplemented("Conditional evaluation not yet implemented".to_string()))
    }
    
    fn evaluate_string_format(template: &str, properties: &PropertyMap) -> Result<PropertyValue, ComputedPropertyError> {
        let mut result = template.to_string();
        
        // Simple template replacement: {property_id}
        for (key, value) in properties.iter() {
            let placeholder = format!("{{{}}}", key);
            if result.contains(&placeholder) {
                result = result.replace(&placeholder, &value.to_string());
            }
        }
        
        Ok(PropertyValue::String(result))
    }
    
    fn evaluate_link_aggregation<F>(
        _link_type: &str,
        _property: &str,
        _aggregation: &AggregationType,
        _getter: F,
    ) -> Result<PropertyValue, ComputedPropertyError>
    where
        F: Fn(&str, &str) -> Option<PropertyValue>,
    {
        // Link aggregation would be implemented here
        Err(ComputedPropertyError::NotImplemented("Link aggregation not yet implemented".to_string()))
    }
}

/// Errors for computed property evaluation
#[derive(Debug, thiserror::Error)]
pub enum ComputedPropertyError {
    #[error("Missing dependency: {0}")]
    MissingDependency(String),
    
    #[error("Invalid type: {0}")]
    InvalidType(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
}
