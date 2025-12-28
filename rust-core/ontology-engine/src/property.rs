use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Struct definition for nested object types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDef {
    pub id: String,
    pub fields: Vec<Property>,
}

/// Property Type enumeration with support for complex types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyType {
    // Simple types (kept for backward compatibility with string deserialization)
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "datetime")]
    DateTime,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "object_reference")]
    ObjectReference,
    #[serde(rename = "objectreference")]
    ObjectReferenceAlt,
    #[serde(rename = "geojson")]
    GeoJSON,
    #[serde(rename = "geo_json")]
    GeoJSONAlt,
    // Complex types
    #[serde(rename = "array")]
    Array {
        #[serde(rename = "elementType")]
        element_type: Box<PropertyType>,
    },
    #[serde(rename = "map")]
    Map {
        #[serde(rename = "keyType")]
        key_type: Box<PropertyType>,
        #[serde(rename = "valueType")]
        value_type: Box<PropertyType>,
    },
    #[serde(rename = "object")]
    Object(StructDef),
    #[serde(rename = "union")]
    Union {
        #[serde(rename = "types")]
        types: Vec<PropertyType>,
    },
}

impl PropertyType {
    /// Parse property type from string (for backward compatibility)
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "string" => Ok(PropertyType::String),
            "integer" | "int" => Ok(PropertyType::Integer),
            "double" | "float" => Ok(PropertyType::Double),
            "boolean" | "bool" => Ok(PropertyType::Boolean),
            "date" => Ok(PropertyType::Date),
            "datetime" | "timestamp" => Ok(PropertyType::DateTime),
            "object_reference" | "objectreference" => Ok(PropertyType::ObjectReference),
            "geojson" | "geo_json" => Ok(PropertyType::GeoJSON),
            _ => Err(format!("Unknown property type: {}", s)),
        }
    }
    
    /// Check if this is a simple type (not complex)
    pub fn is_simple(&self) -> bool {
        !matches!(
            self,
            PropertyType::Array { .. } | PropertyType::Map { .. } | PropertyType::Object(_) | PropertyType::Union { .. }
        )
    }
    
    /// Get the underlying simple type if this is a simple type variant
    pub fn as_simple(&self) -> Option<Self> {
        match self {
            PropertyType::String | PropertyType::Int => Some(PropertyType::String),
            PropertyType::Integer => Some(PropertyType::Integer),
            PropertyType::Double | PropertyType::Float => Some(PropertyType::Double),
            PropertyType::Boolean | PropertyType::Bool => Some(PropertyType::Boolean),
            PropertyType::Date => Some(PropertyType::Date),
            PropertyType::DateTime | PropertyType::Timestamp => Some(PropertyType::DateTime),
            PropertyType::ObjectReference | PropertyType::ObjectReferenceAlt => Some(PropertyType::ObjectReference),
            PropertyType::GeoJSON | PropertyType::GeoJSONAlt => Some(PropertyType::GeoJSON),
            _ => None,
        }
    }
}

/// Property definition for Object Types and Link Types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    
    // Enhanced metadata
    #[serde(default)]
    pub description: Option<String>,
    
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    
    #[serde(default)]
    pub unit: Option<String>,
    
    #[serde(default)]
    pub format: Option<PropertyFormat>,
    
    #[serde(rename = "sensitivityTags")]
    #[serde(default)]
    pub sensitivity_tags: Vec<String>,
    
    #[serde(default)]
    pub pii: bool,
    
    #[serde(default)]
    pub deprecated: Option<DeprecationInfo>,
}

fn deserialize_property_type<'de, D>(deserializer: D) -> Result<PropertyType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    // Support both string (backward compatibility) and object forms
    let value: serde_json::Value = serde_json::Value::deserialize(deserializer)?;
    
    match value {
        serde_json::Value::String(s) => {
            PropertyType::from_str(&s).map_err(D::Error::custom)
        }
        serde_json::Value::Object(mut obj) => {
            // Check for complex types
            if obj.contains_key("elementType") {
                let element_type_val = obj.remove("elementType")
                    .ok_or_else(|| D::Error::custom("array type missing elementType"))?;
                let element_type: PropertyType = serde_json::from_value(element_type_val)
                    .map_err(D::Error::custom)?;
                Ok(PropertyType::Array {
                    element_type: Box::new(element_type),
                })
            } else if obj.contains_key("keyType") && obj.contains_key("valueType") {
                let key_type_val = obj.remove("keyType")
                    .ok_or_else(|| D::Error::custom("map type missing keyType"))?;
                let value_type_val = obj.remove("valueType")
                    .ok_or_else(|| D::Error::custom("map type missing valueType"))?;
                let key_type: PropertyType = serde_json::from_value(key_type_val)
                    .map_err(D::Error::custom)?;
                let value_type: PropertyType = serde_json::from_value(value_type_val)
                    .map_err(D::Error::custom)?;
                Ok(PropertyType::Map {
                    key_type: Box::new(key_type),
                    value_type: Box::new(value_type),
                })
            } else if obj.contains_key("types") {
                let types_val = obj.remove("types")
                    .ok_or_else(|| D::Error::custom("union type missing types"))?;
                let types: Vec<PropertyType> = serde_json::from_value(types_val)
                    .map_err(D::Error::custom)?;
                Ok(PropertyType::Union { types })
            } else {
                // Try to deserialize as Object with struct definition
                // For now, fall back to trying string deserialization
                Err(D::Error::custom("Unknown property type format"))
            }
        }
        _ => Err(D::Error::custom("Property type must be string or object")),
    }
}

/// Property format hints for display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PropertyFormat {
    Currency {
        symbol: Option<String>,
    },
    Percentage {
        decimals: Option<usize>,
    },
    DateFormat {
        format: String,
    },
    NumberFormat {
        decimals: usize,
        separator: Option<char>,
    },
}

/// Deprecation information for properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeprecationInfo {
    #[serde(rename = "deprecatedSince")]
    pub deprecated_since: String,
    pub replacement: Option<String>,
    #[serde(rename = "removalDate")]
    pub removal_date: Option<String>,
}

/// Validation rules for properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        self.validate_value_with_reference_check(value, None)
    }
    
    /// Validate a property value with optional reference existence check
    pub fn validate_value_with_reference_check(
        &self,
        value: &PropertyValue,
        reference_checker: Option<&dyn Fn(&str, &str) -> bool>, // (object_type, object_id) -> exists
    ) -> Result<(), String> {
        // Handle complex types recursively
        match (&self.property_type, value) {
            // Array validation
            (PropertyType::Array { element_type }, PropertyValue::Array(arr)) => {
                for (idx, item) in arr.iter().enumerate() {
                    // Create a temporary property for element validation
                    let element_prop = Property {
                        id: format!("{}[{}]", self.id, idx),
                        display_name: None,
                        property_type: *element_type.clone(),
                        required: false,
                        default: None,
                        validation: None,
                        description: None,
                        annotations: HashMap::new(),
                        unit: None,
                        format: None,
                        sensitivity_tags: Vec::new(),
                        pii: false,
                        deprecated: None,
                    };
                    element_prop.validate_value_with_reference_check(item, reference_checker)
                        .map_err(|e| format!("Array element {}: {}", idx, e))?;
                }
            }
            // Map validation
            (PropertyType::Map { key_type, value_type }, PropertyValue::Map(map)) => {
                for (key, val) in map.iter() {
                    // Validate key type
                    let key_prop = Property {
                        id: format!("{}.key", self.id),
                        display_name: None,
                        property_type: *key_type.clone(),
                        required: false,
                        default: None,
                        validation: None,
                        description: None,
                        annotations: HashMap::new(),
                        unit: None,
                        format: None,
                        sensitivity_tags: Vec::new(),
                        pii: false,
                        deprecated: None,
                    };
                    // Convert key to PropertyValue based on key type
                    let key_value = match key_type.as_ref() {
                        PropertyType::String | PropertyType::Int => PropertyValue::String(key.clone()),
                        PropertyType::Integer => {
                            key.parse::<i64>()
                                .map(PropertyValue::Integer)
                                .map_err(|_| format!("Map key '{}' is not a valid integer", key))?
                        }
                        _ => PropertyValue::String(key.clone()), // Default to string
                    };
                    key_prop.validate_value_with_reference_check(&key_value, reference_checker)
                        .map_err(|e| format!("Map key '{}': {}", key, e))?;
                    
                    // Validate value type
                    let val_prop = Property {
                        id: format!("{}.{}", self.id, key),
                        display_name: None,
                        property_type: *value_type.clone(),
                        required: false,
                        default: None,
                        validation: None,
                        description: None,
                        annotations: HashMap::new(),
                        unit: None,
                        format: None,
                        sensitivity_tags: Vec::new(),
                        pii: false,
                        deprecated: None,
                    };
                    val_prop.validate_value_with_reference_check(val, reference_checker)
                        .map_err(|e| format!("Map value for key '{}': {}", key, e))?;
                }
            }
            // Object validation
            (PropertyType::Object(struct_def), PropertyValue::Object(obj)) => {
                // Check all required fields are present
                for field in &struct_def.fields {
                    if field.required && !obj.contains_key(&field.id) {
                        return Err(format!(
                            "Object '{}' missing required field '{}'",
                            self.id, field.id
                        ));
                    }
                }
                
                // Validate all fields that are present
                for (field_id, field_value) in obj.iter() {
                    let field = struct_def.fields.iter()
                        .find(|f| f.id == *field_id)
                        .ok_or_else(|| format!(
                            "Object '{}' has unknown field '{}'",
                            self.id, field_id
                        ))?;
                    
                    field.validate_value_with_reference_check(field_value, reference_checker)
                        .map_err(|e| format!("Object field '{}': {}", field_id, e))?;
                }
            }
            // Union validation - try each type until one matches
            (PropertyType::Union { types }, value) => {
                let mut last_error = None;
                let mut matched = false;
                for union_type in types {
                    let union_prop = Property {
                        id: self.id.clone(),
                        display_name: None,
                        property_type: union_type.clone(),
                        required: false,
                        default: None,
                        validation: None,
                        description: None,
                        annotations: HashMap::new(),
                        unit: None,
                        format: None,
                        sensitivity_tags: Vec::new(),
                        pii: false,
                        deprecated: None,
                    };
                    match union_prop.validate_value_with_reference_check(value, reference_checker) {
                        Ok(()) => {
                            matched = true;
                            break;
                        }
                        Err(e) => {
                            last_error = Some(e);
                        }
                    }
                }
                if !matched {
                    return Err(format!(
                        "Property '{}' value does not match any union type: {}",
                        self.id,
                        last_error.unwrap_or_else(|| "Unknown error".to_string())
                    ));
                }
            }
            // Simple type checking
            (PropertyType::String | PropertyType::Int, PropertyValue::String(_)) => {}
            (PropertyType::Integer, PropertyValue::Integer(_)) => {}
            (PropertyType::Double | PropertyType::Float, PropertyValue::Double(_)) => {}
            (PropertyType::Boolean | PropertyType::Bool, PropertyValue::Boolean(_)) => {}
            (PropertyType::Date, PropertyValue::Date(_)) => {}
            (PropertyType::DateTime | PropertyType::Timestamp, PropertyValue::DateTime(_)) => {}
            (PropertyType::ObjectReference | PropertyType::ObjectReferenceAlt, PropertyValue::ObjectReference(ref_id)) => {
                // If reference checker is provided, validate that the referenced object exists
                if let Some(checker) = reference_checker {
                    // Extract object type and ID from reference (format: "object_type:object_id" or just "object_id")
                    let parts: Vec<&str> = ref_id.split(':').collect();
                    let (obj_type, obj_id) = if parts.len() == 2 {
                        (parts[0], parts[1])
                    } else {
                        // If no type prefix, we can't validate - this is a limitation
                        // In a real system, we'd need the object type context
                        return Err(format!(
                            "Object reference '{}' must be in format 'object_type:object_id' for validation",
                            ref_id
                        ));
                    };
                    
                    if !checker(obj_type, obj_id) {
                        return Err(format!(
                            "Referenced object '{}' of type '{}' does not exist",
                            obj_id, obj_type
                        ));
                    }
                }
            }
            (PropertyType::GeoJSON | PropertyType::GeoJSONAlt, PropertyValue::GeoJSON(gj)) => {
                // Validate GeoJSON format
                if let Err(e) = geojson::GeoJson::from_str(gj) {
                    return Err(format!(
                        "Property '{}' contains invalid GeoJSON: {}",
                        self.id, e
                    ));
                }
            }
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
                PropertyValue::Array(arr) => {
                    // Validate array length
                    if let Some(min) = validation.min_length {
                        if arr.len() < min {
                            return Err(format!(
                                "Property '{}' array length {} is less than minimum {}",
                                self.id, arr.len(), min
                            ));
                        }
                    }
                    if let Some(max) = validation.max_length {
                        if arr.len() > max {
                            return Err(format!(
                                "Property '{}' array length {} exceeds maximum {}",
                                self.id, arr.len(), max
                            ));
                        }
                    }
                }
                PropertyValue::Map(map) => {
                    // Validate map size (using length validation)
                    if let Some(min) = validation.min_length {
                        if map.len() < min {
                            return Err(format!(
                                "Property '{}' map size {} is less than minimum {}",
                                self.id, map.len(), min
                            ));
                        }
                    }
                    if let Some(max) = validation.max_length {
                        if map.len() > max {
                            return Err(format!(
                                "Property '{}' map size {} exceeds maximum {}",
                                self.id, map.len(), max
                            ));
                        }
                    }
                }
                PropertyValue::GeoJSON(gj) => {
                    // Validate GeoJSON format if not already validated in type check
                    if let Err(e) = geojson::GeoJson::from_str(gj) {
                        return Err(format!(
                            "Property '{}' contains invalid GeoJSON: {}",
                            self.id, e
                        ));
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
    GeoJSON(String), // GeoJSON string (can be parsed to validate)
    Array(Vec<PropertyValue>),
    Map(HashMap<String, PropertyValue>),
    Object(HashMap<String, PropertyValue>), // Field name -> value
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
            PropertyValue::GeoJSON(gj) => gj.clone(),
            PropertyValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            PropertyValue::Map(map) => {
                let items: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            PropertyValue::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
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
            description: None,
            annotations: HashMap::new(),
            unit: None,
            format: None,
            sensitivity_tags: Vec::new(),
            pii: false,
            deprecated: None,
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
            description: None,
            annotations: HashMap::new(),
            unit: None,
            format: None,
            sensitivity_tags: Vec::new(),
            pii: false,
            deprecated: None,
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
            description: None,
            annotations: HashMap::new(),
            unit: None,
            format: None,
            sensitivity_tags: Vec::new(),
            pii: false,
            deprecated: None,
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
