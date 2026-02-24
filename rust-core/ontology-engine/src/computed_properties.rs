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
        let parts: Vec<&str> = expression.split_whitespace().collect();

        let mut accumulator: Option<f64> = None;
        let mut pending_op: char = '+';

        for part in &parts {
            match *part {
                "+" | "-" | "*" | "/" => {
                    pending_op = part.chars().next().unwrap();
                }
                _ => {
                    let value = if let Some(prop_val) = properties.get(*part) {
                        match prop_val {
                            PropertyValue::Integer(i) => *i as f64,
                            PropertyValue::Double(d) => *d,
                            _ => return Err(ComputedPropertyError::InvalidType(
                                format!("Property '{}' is not numeric", part)
                            )),
                        }
                    } else if let Ok(num) = part.parse::<f64>() {
                        num
                    } else {
                        return Err(ComputedPropertyError::EvaluationError(
                            format!("Unknown token '{}': not a property name or number", part)
                        ));
                    };

                    accumulator = Some(match accumulator {
                        None => value,
                        Some(acc) => match pending_op {
                            '+' => acc + value,
                            '-' => acc - value,
                            '*' => acc * value,
                            '/' => {
                                if value == 0.0 {
                                    return Err(ComputedPropertyError::EvaluationError(
                                        "Division by zero".to_string()
                                    ));
                                }
                                acc / value
                            }
                            _ => value,
                        },
                    });
                    pending_op = '+';
                }
            }
        }

        match accumulator {
            Some(r) => Ok(PropertyValue::Double(r)),
            None => Err(ComputedPropertyError::EvaluationError(
                "Empty arithmetic expression".to_string()
            )),
        }
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
        condition: &str,
        then_expr: &str,
        else_expr: &str,
        properties: &PropertyMap,
    ) -> Result<PropertyValue, ComputedPropertyError> {
        // Parse condition: "operand1 op operand2"
        let parts: Vec<&str> = condition.splitn(3, ' ').collect();
        if parts.len() != 3 {
            return Err(ComputedPropertyError::EvaluationError(
                format!("Invalid condition '{}'. Expected format: 'property op value'", condition)
            ));
        }
        let (operand1, op, operand2) = (parts[0], parts[1], parts[2]);

        let val1 = Self::resolve_token(operand1, properties);
        let val2 = Self::resolve_token(operand2, properties);

        let condition_met = match op {
            "==" | "=" => val1 == val2,
            "!=" => val1 != val2,
            ">" => Self::compare_values(&val1, &val2).map_or(false, |o| o == std::cmp::Ordering::Greater),
            ">=" => Self::compare_values(&val1, &val2).map_or(false, |o| o != std::cmp::Ordering::Less),
            "<" => Self::compare_values(&val1, &val2).map_or(false, |o| o == std::cmp::Ordering::Less),
            "<=" => Self::compare_values(&val1, &val2).map_or(false, |o| o != std::cmp::Ordering::Greater),
            _ => return Err(ComputedPropertyError::EvaluationError(
                format!("Unknown operator '{}'. Valid: ==, !=, >, >=, <, <=", op)
            )),
        };

        let branch = if condition_met { then_expr } else { else_expr };
        Ok(Self::resolve_token(branch, properties))
    }

    fn resolve_token(token: &str, properties: &PropertyMap) -> PropertyValue {
        if let Some(v) = properties.get(token) {
            return v.clone();
        }
        if let Ok(i) = token.parse::<i64>() {
            return PropertyValue::Integer(i);
        }
        if let Ok(f) = token.parse::<f64>() {
            return PropertyValue::Double(f);
        }
        if token == "true" { return PropertyValue::Boolean(true); }
        if token == "false" { return PropertyValue::Boolean(false); }
        if token == "null" { return PropertyValue::Null; }
        // Strip surrounding quotes if present
        if (token.starts_with('"') && token.ends_with('"')) || (token.starts_with('\'') && token.ends_with('\'')) {
            return PropertyValue::String(token[1..token.len()-1].to_string());
        }
        PropertyValue::String(token.to_string())
    }

    fn compare_values(a: &PropertyValue, b: &PropertyValue) -> Option<std::cmp::Ordering> {
        match (a, b) {
            (PropertyValue::Integer(x), PropertyValue::Integer(y)) => Some(x.cmp(y)),
            (PropertyValue::Double(x), PropertyValue::Double(y)) => x.partial_cmp(y),
            (PropertyValue::Integer(x), PropertyValue::Double(y)) => (*x as f64).partial_cmp(y),
            (PropertyValue::Double(x), PropertyValue::Integer(y)) => x.partial_cmp(&(*y as f64)),
            (PropertyValue::String(x), PropertyValue::String(y)) => Some(x.cmp(y)),
            _ => None,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn props(pairs: &[(&str, PropertyValue)]) -> PropertyMap {
        let mut map = PropertyMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.clone());
        }
        map
    }

    #[test]
    fn test_arithmetic_addition() {
        let p = props(&[("a", PropertyValue::Integer(3)), ("b", PropertyValue::Integer(7))]);
        let result = ComputedPropertyEvaluator::evaluate_arithmetic("a + b", &p).unwrap();
        assert_eq!(result, PropertyValue::Double(10.0));
    }

    #[test]
    fn test_arithmetic_subtraction() {
        let p = props(&[("x", PropertyValue::Double(10.0)), ("y", PropertyValue::Double(3.5))]);
        let result = ComputedPropertyEvaluator::evaluate_arithmetic("x - y", &p).unwrap();
        assert_eq!(result, PropertyValue::Double(6.5));
    }

    #[test]
    fn test_arithmetic_multiply_literal() {
        let p = props(&[("population", PropertyValue::Integer(1000)), ("area", PropertyValue::Double(50.0))]);
        let result = ComputedPropertyEvaluator::evaluate_arithmetic("population / area", &p).unwrap();
        assert_eq!(result, PropertyValue::Double(20.0));
    }

    #[test]
    fn test_arithmetic_mixed() {
        // 2 * 3 + 4 = 10
        let p = props(&[]);
        let result = ComputedPropertyEvaluator::evaluate_arithmetic("2 * 3 + 4", &p).unwrap();
        assert_eq!(result, PropertyValue::Double(10.0));
    }

    #[test]
    fn test_arithmetic_division_by_zero() {
        let p = props(&[]);
        let result = ComputedPropertyEvaluator::evaluate_arithmetic("10 / 0", &p);
        assert!(result.is_err());
    }

    #[test]
    fn test_conditional_gt_true_branch() {
        let p = props(&[("score", PropertyValue::Integer(75)), ("grade", PropertyValue::String("A".to_string()))]);
        let result = ComputedPropertyEvaluator::evaluate_conditional("score > 50", "grade", "F", &p).unwrap();
        assert_eq!(result, PropertyValue::String("A".to_string()));
    }

    #[test]
    fn test_conditional_gt_false_branch() {
        let p = props(&[("score", PropertyValue::Integer(30))]);
        let result = ComputedPropertyEvaluator::evaluate_conditional("score > 50", "pass", "0", &p).unwrap();
        assert_eq!(result, PropertyValue::Integer(0));
    }

    #[test]
    fn test_conditional_eq_strings() {
        let p = props(&[("status", PropertyValue::String("active".to_string()))]);
        let result = ComputedPropertyEvaluator::evaluate_conditional("status == active", "1", "0", &p).unwrap();
        assert_eq!(result, PropertyValue::Integer(1));
    }

    #[test]
    fn test_conditional_neq() {
        let p = props(&[("x", PropertyValue::Integer(5))]);
        let result = ComputedPropertyEvaluator::evaluate_conditional("x != 5", "yes", "no", &p).unwrap();
        assert_eq!(result, PropertyValue::String("no".to_string()));
    }

    #[test]
    fn test_string_format() {
        let p = props(&[("name", PropertyValue::String("Alice".to_string())), ("year", PropertyValue::Integer(2020))]);
        let result = ComputedPropertyEvaluator::evaluate_string_format("{name} ({year})", &p).unwrap();
        assert_eq!(result, PropertyValue::String("Alice (2020)".to_string()));
    }
}
