use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Property Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    String,
    Integer,
    Double,
    Boolean,
    Date,
    DateTime,
    ObjectReference,
    // Add more types as needed
}

impl PropertyType {
    /// Parse property type from string
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "string" => Ok(PropertyType::String),
            "integer" | "int" => Ok(PropertyType::Integer),
            "double" | "float" => Ok(PropertyType::Double),
            "boolean" | "bool" => Ok(PropertyType::Boolean),
            "date" => Ok(PropertyType::Date),
            "datetime" | "timestamp" => Ok(PropertyType::DateTime),
            "object_reference" | "objectreference" => Ok(PropertyType::ObjectReference),
            _ => Err(format!("Unknown property type: {}", s)),
        }
    }
}

/// Property definition for Object Types and Link Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id: String,
    
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: Option<String>,
    
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_property_type")]
    pub property_type: PropertyType,
    
    #[serde(default)]
    pub required: bool,
    
    #[serde(default)]
    pub default: Option<PropertyValue>,
    
    #[serde(default)]
    pub validation: Option<PropertyValidation>,
}

fn deserialize_property_type<'de, D>(deserializer: D) -> Result<PropertyType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    PropertyType::from_str(&s).map_err(serde::de::Error::custom)
}

/// Validation rules for properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValidation {
    #[serde(default)]
    pub min_length: Option<usize>,
    
    #[serde(default)]
    pub max_length: Option<usize>,
    
    #[serde(default)]
    pub min: Option<f64>,
    
    #[serde(default)]
    pub max: Option<f64>,
    
    #[serde(default)]
    pub pattern: Option<String>, // Regex pattern
    
    #[serde(default)]
    pub enum_values: Option<Vec<String>>,
}

impl Property {
    /// Validate a property value against this property's rules
    pub fn validate_value(&self, value: &PropertyValue) -> Result<(), String> {
        // Type checking
        match (&self.property_type, value) {
            (PropertyType::String, PropertyValue::String(_)) => {}
            (PropertyType::Integer, PropertyValue::Integer(_)) => {}
            (PropertyType::Double, PropertyValue::Double(_)) => {}
            (PropertyType::Boolean, PropertyValue::Boolean(_)) => {}
            (PropertyType::Date, PropertyValue::Date(_)) => {}
            (PropertyType::DateTime, PropertyValue::DateTime(_)) => {}
            (PropertyType::ObjectReference, PropertyValue::ObjectReference(_)) => {}
            _ => {
                return Err(format!(
                    "Property '{}' expects type {:?}, got incompatible value",
                    self.id, self.property_type
                ));
            }
        }
        
        // Additional validation rules
        if let Some(validation) = &self.validation {
            match value {
                PropertyValue::String(s) => {
                    if let Some(min) = validation.min_length {
                        if s.len() < min {
                            return Err(format!(
                                "Property '{}' string length {} is less than minimum {}",
                                self.id, s.len(), min
                            ));
                        }
                    }
                    if let Some(max) = validation.max_length {
                        if s.len() > max {
                            return Err(format!(
                                "Property '{}' string length {} exceeds maximum {}",
                                self.id, s.len(), max
                            ));
                        }
                    }
                    if let Some(pattern) = &validation.pattern {
                        // Simple substring matching for now - can be enhanced with regex crate later
                        // For production, consider using regex crate: regex::Regex::new(pattern)
                        if !s.contains(pattern) {
                            return Err(format!(
                                "Property '{}' value does not match pattern '{}'",
                                self.id, pattern
                            ));
                        }
                    }
                    if let Some(enum_values) = &validation.enum_values {
                        if !enum_values.contains(s) {
                            return Err(format!(
                                "Property '{}' value '{}' is not in allowed enum values",
                                self.id, s
                            ));
                        }
                    }
                }
                PropertyValue::Integer(i) => {
                    let num = *i as f64;
                    if let Some(min) = validation.min {
                        if num < min {
                            return Err(format!(
                                "Property '{}' value {} is less than minimum {}",
                                self.id, num, min
                            ));
                        }
                    }
                    if let Some(max) = validation.max {
                        if num > max {
                            return Err(format!(
                                "Property '{}' value {} exceeds maximum {}",
                                self.id, num, max
                            ));
                        }
                    }
                }
                PropertyValue::Double(d) => {
                    let num = *d;
                    if let Some(min) = validation.min {
                        if num < min {
                            return Err(format!(
                                "Property '{}' value {} is less than minimum {}",
                                self.id, num, min
                            ));
                        }
                    }
                    if let Some(max) = validation.max {
                        if num > max {
                            return Err(format!(
                                "Property '{}' value {} exceeds maximum {}",
                                self.id, num, max
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
}

/// Property value - runtime representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
    Date(String), // ISO 8601 date string
    DateTime(String), // ISO 8601 datetime string
    ObjectReference(String), // Object ID
    Null,
}

impl PropertyValue {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Integer(i) => i.to_string(),
            PropertyValue::Double(d) => d.to_string(),
            PropertyValue::Boolean(b) => b.to_string(),
            PropertyValue::Date(d) => d.clone(),
            PropertyValue::DateTime(dt) => dt.clone(),
            PropertyValue::ObjectReference(id) => id.clone(),
            PropertyValue::Null => "null".to_string(),
        }
    }
    
    /// Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, PropertyValue::Null)
    }
}

/// A collection of property values (object properties at runtime)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PropertyMap {
    properties: HashMap<String, PropertyValue>,
}

impl PropertyMap {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: String, value: PropertyValue) {
        self.properties.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.get(key)
    }
    
    pub fn contains_key(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&String, &PropertyValue)> {
        self.properties.iter()
    }
    
    pub fn len(&self) -> usize {
        self.properties.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_property_type_from_str() {
        assert_eq!(PropertyType::from_str("string").unwrap(), PropertyType::String);
        assert_eq!(PropertyType::from_str("integer").unwrap(), PropertyType::Integer);
        assert_eq!(PropertyType::from_str("double").unwrap(), PropertyType::Double);
        assert_eq!(PropertyType::from_str("boolean").unwrap(), PropertyType::Boolean);
        assert!(PropertyType::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_property_validation_string_length() {
        let prop = Property {
            id: "test".to_string(),
            display_name: None,
            property_type: PropertyType::String,
            required: false,
            default: None,
            validation: Some(PropertyValidation {
                min_length: Some(3),
                max_length: Some(10),
                min: None,
                max: None,
                pattern: None,
                enum_values: None,
            }),
        };
        
        assert!(prop.validate_value(&PropertyValue::String("test".to_string())).is_ok());
        assert!(prop.validate_value(&PropertyValue::String("ab".to_string())).is_err()); // Too short
        assert!(prop.validate_value(&PropertyValue::String("this is too long".to_string())).is_err()); // Too long
    }
    
    #[test]
    fn test_property_validation_numeric() {
        let prop = Property {
            id: "test".to_string(),
            display_name: None,
            property_type: PropertyType::Integer,
            required: false,
            default: None,
            validation: Some(PropertyValidation {
                min_length: None,
                max_length: None,
                min: Some(10.0),
                max: Some(100.0),
                pattern: None,
                enum_values: None,
            }),
        };
        
        assert!(prop.validate_value(&PropertyValue::Integer(50)).is_ok());
        assert!(prop.validate_value(&PropertyValue::Integer(5)).is_err()); // Too small
        assert!(prop.validate_value(&PropertyValue::Integer(200)).is_err()); // Too large
    }
    
    #[test]
    fn test_property_validation_enum() {
        let prop = Property {
            id: "test".to_string(),
            display_name: None,
            property_type: PropertyType::String,
            required: false,
            default: None,
            validation: Some(PropertyValidation {
                min_length: None,
                max_length: None,
                min: None,
                max: None,
                pattern: None,
                enum_values: Some(vec!["option1".to_string(), "option2".to_string()]),
            }),
        };
        
        assert!(prop.validate_value(&PropertyValue::String("option1".to_string())).is_ok());
        assert!(prop.validate_value(&PropertyValue::String("invalid".to_string())).is_err());
    }
    
    #[test]
    fn test_property_map() {
        let mut map = PropertyMap::new();
        assert!(map.is_empty());
        
        map.insert("key1".to_string(), PropertyValue::String("value1".to_string()));
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("key1"));
        assert_eq!(map.get("key1"), Some(&PropertyValue::String("value1".to_string())));
    }
    
    #[test]
    fn test_property_value_to_string() {
        assert_eq!(PropertyValue::String("test".to_string()).to_string(), "test");
        assert_eq!(PropertyValue::Integer(42).to_string(), "42");
        assert_eq!(PropertyValue::Double(3.14).to_string(), "3.14");
        assert_eq!(PropertyValue::Boolean(true).to_string(), "true");
        assert_eq!(PropertyValue::Null.to_string(), "null");
    }
    
    #[test]
    fn test_property_value_is_null() {
        assert!(PropertyValue::Null.is_null());
        assert!(!PropertyValue::String("test".to_string()).is_null());
    }
}
