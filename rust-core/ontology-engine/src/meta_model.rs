use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::property::{Property, PropertyType};
use crate::link::LinkCardinality;

/// Core meta-model representing the ontology configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyConfig {
    pub ontology: OntologyDef,
}

/// The complete ontology definition (for serialization)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyDef {
    #[serde(rename = "objectTypes")]
    pub object_types: Vec<ObjectType>,
    
    #[serde(rename = "linkTypes")]
    pub link_types: Vec<LinkTypeDef>,
    
    #[serde(rename = "actionTypes")]
    #[serde(default)]
    pub action_types: Vec<ActionTypeDef>,
    
    #[serde(rename = "interfaces")]
    #[serde(default)]
    pub interfaces: Vec<InterfaceDef>,
    
    #[serde(rename = "functionTypes")]
    #[serde(default)]
    pub function_types: Vec<FunctionTypeDef>,
}

/// Interface definition - represents a contract that object types can implement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDef {
    pub id: String,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    #[serde(default)]
    pub properties: Vec<Property>,
    
    #[serde(rename = "requiredLinkTypes")]
    #[serde(default)]
    pub required_link_types: Vec<String>,
}

impl InterfaceDef {
    /// Validate that the interface definition is valid
    pub fn validate(&self) -> Result<(), String> {
        // Check for duplicate property IDs
        let mut seen = std::collections::HashSet::new();
        for prop in &self.properties {
            if !seen.insert(&prop.id) {
                return Err(format!(
                    "Duplicate property ID '{}' in interface '{}'",
                    prop.id, self.id
                ));
            }
        }
        Ok(())
    }
}

/// Object Type - represents a real-world concept in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectType {
    pub id: String,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    #[serde(rename = "primaryKey")]
    pub primary_key: String,
    
    pub properties: Vec<Property>,
    
    #[serde(rename = "backingDatasource")]
    #[serde(default)]
    pub backing_datasource: Option<String>,
    
    #[serde(rename = "titleKey")]
    #[serde(default)]
    pub title_key: Option<String>,
    
    #[serde(rename = "implements")]
    #[serde(default)]
    pub implements: Vec<String>, // List of interface IDs this object type implements
}

impl ObjectType {
    /// Get a property by its ID
    pub fn get_property(&self, property_id: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.id == property_id)
    }
    
    /// Validate that all required properties are present
    pub fn validate(&self) -> Result<(), String> {
        // Check that primary_key property exists
        if !self.properties.iter().any(|p| p.id == self.primary_key) {
            return Err(format!(
                "Primary key '{}' not found in properties for object type '{}'",
                self.primary_key, self.id
            ));
        }
        
        // Check for duplicate property IDs
        let mut seen = std::collections::HashSet::new();
        for prop in &self.properties {
            if !seen.insert(&prop.id) {
                return Err(format!(
                    "Duplicate property ID '{}' in object type '{}'",
                    prop.id, self.id
                ));
            }
        }
        
        // Note: Interface implementation validation happens at ontology level
        // where we have access to interface definitions
        
        Ok(())
    }
    
    /// Validate that this object type implements all declared interfaces
    pub fn validate_interface_implementations(
        &self,
        interfaces: &std::collections::HashMap<String, InterfaceDef>,
    ) -> Result<(), String> {
        use crate::interface::InterfaceValidator;
        for interface_id in &self.implements {
            let interface = interfaces.get(interface_id)
                .ok_or_else(|| format!(
                    "Object type '{}' declares implementation of interface '{}' which does not exist",
                    self.id, interface_id
                ))?;
            
            InterfaceValidator::validate_implements(self, interface)?;
        }
        
        Ok(())
    }
}

/// Link Type definition - represents a semantic connection between two Object Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTypeDef {
    pub id: String,
    
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: Option<String>,
    
    pub source: String,
    pub target: String,
    
    #[serde(default)]
    pub cardinality: LinkCardinality,
    
    #[serde(default)]
    pub properties: Vec<Property>,
    
    #[serde(default)]
    pub bidirectional: bool,
}

impl LinkTypeDef {
    /// Validate that source and target object types exist
    pub fn validate(&self, object_type_ids: &[String]) -> Result<(), String> {
        if !object_type_ids.contains(&self.source) {
            return Err(format!(
                "Link type '{}' references unknown source object type '{}'",
                self.id, self.source
            ));
        }
        
        if !object_type_ids.contains(&self.target) {
            return Err(format!(
                "Link type '{}' references unknown target object type '{}'",
                self.id, self.target
            ));
        }
        
        Ok(())
    }
}

/// Action Type definition - represents a transaction that modifies the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTypeDef {
    pub id: String,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    #[serde(default)]
    pub parameters: Vec<Property>,
    
    #[serde(default)]
    pub logic: Vec<crate::action::ActionOperation>,
    
    #[serde(default)]
    pub validation: Option<crate::action::ActionValidation>,
    
    #[serde(default)]
    pub side_effects: Vec<crate::action::ActionSideEffect>,
}

/// Function return type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FunctionReturnType {
    Property {
        property_type: PropertyType,
    },
    ObjectType {
        object_type: String,
    },
    Array {
        element_type: Box<FunctionReturnType>,
    },
}

/// Aggregation type for function logic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationType {
    Sum,
    Avg,
    Count,
    Min,
    Max,
}

/// Function filter for link traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionFilter {
    pub property: String,
    pub operator: String,
    pub value: crate::property::PropertyValue,
}

/// Function logic definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FunctionLogic {
    Aggregation {
        #[serde(rename = "linkType")]
        link_type: String,
        aggregation: AggregationType,
        property: String,
    },
    LinkTraversal {
        #[serde(rename = "linkType")]
        link_type: String,
        #[serde(rename = "targetType")]
        target_type: String,
        filter: Option<FunctionFilter>,
    },
    PropertyAccess {
        property: String,
    },
}

/// Function Type definition - represents a function that returns typed data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTypeDef {
    pub id: String,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    #[serde(default)]
    pub description: Option<String>,
    
    #[serde(default)]
    pub parameters: Vec<Property>,
    
    #[serde(rename = "returnType")]
    pub return_type: FunctionReturnType,
    
    pub logic: FunctionLogic,
    
    #[serde(default)]
    pub cacheable: bool,
}

impl FunctionTypeDef {
    /// Validate that the function definition is valid
    pub fn validate(&self, object_type_ids: &[String], link_type_ids: &[String]) -> Result<(), String> {
        // Validate return type references exist
        match &self.return_type {
            FunctionReturnType::ObjectType { object_type } => {
                if !object_type_ids.contains(object_type) {
                    return Err(format!(
                        "Function '{}' return type references unknown object type '{}'",
                        self.id, object_type
                    ));
                }
            }
            FunctionReturnType::Array { element_type } => {
                // Recursively validate array element type
                // For now, we'll just check that it's valid (simplified)
            }
            _ => {}
        }
        
        // Validate logic references
        match &self.logic {
            FunctionLogic::LinkTraversal { link_type, target_type, .. } => {
                if !link_type_ids.contains(link_type) {
                    return Err(format!(
                        "Function '{}' logic references unknown link type '{}'",
                        self.id, link_type
                    ));
                }
                if !object_type_ids.contains(target_type) {
                    return Err(format!(
                        "Function '{}' logic references unknown target type '{}'",
                        self.id, target_type
                    ));
                }
            }
            FunctionLogic::Aggregation { link_type, .. } => {
                if !link_type_ids.contains(link_type) {
                    return Err(format!(
                        "Function '{}' logic references unknown link type '{}'",
                        self.id, link_type
                    ));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

/// The runtime ontology state
pub struct OntologyRuntime {
    config: OntologyConfig,
    object_types: HashMap<String, ObjectType>,
    link_types: HashMap<String, LinkTypeDef>,
    action_types: HashMap<String, ActionTypeDef>,
    interfaces: HashMap<String, InterfaceDef>,
    function_types: HashMap<String, FunctionTypeDef>,
}

impl OntologyRuntime {
    /// Load ontology from configuration
    pub fn from_config(config: OntologyConfig) -> Result<Self, String> {
        let ontology_def = config.ontology.clone();
        
        // Validate all object types
        let object_type_ids: Vec<String> = ontology_def.object_types.iter()
            .map(|ot| ot.id.clone())
            .collect();
        
        for object_type in &ontology_def.object_types {
            object_type.validate()?;
        }
        
        // Build interfaces map first (needed for interface validation)
        let interfaces: HashMap<String, InterfaceDef> = ontology_def.interfaces
            .iter()
            .cloned()
            .map(|i| (i.id.clone(), i))
            .collect();
        
        // Validate interface implementations for all object types
        for object_type in &ontology_def.object_types {
            object_type.validate_interface_implementations(&interfaces)?;
        }
        
        // Validate all link types
        for link_type in &ontology_def.link_types {
            link_type.validate(&object_type_ids)?;
        }
        
        // Validate all interfaces
        let interface_ids: Vec<String> = ontology_def.interfaces.iter()
            .map(|i| i.id.clone())
            .collect();
        for interface in &ontology_def.interfaces {
            interface.validate()?;
        }
        
        // Validate all function types
        let link_type_ids: Vec<String> = ontology_def.link_types.iter()
            .map(|lt| lt.id.clone())
            .collect();
        for function_type in &ontology_def.function_types {
            function_type.validate(&object_type_ids, &link_type_ids)?;
        }
        
        // Build hash maps for efficient lookup
        let object_types: HashMap<String, ObjectType> = ontology_def.object_types
            .iter()
            .cloned()
            .map(|ot| (ot.id.clone(), ot))
            .collect();
        
        let link_types: HashMap<String, LinkTypeDef> = ontology_def.link_types
            .iter()
            .cloned()
            .map(|lt| (lt.id.clone(), lt))
            .collect();
        
        let action_types: HashMap<String, ActionTypeDef> = ontology_def.action_types
            .iter()
            .cloned()
            .map(|at| (at.id.clone(), at))
            .collect();
        
        let function_types: HashMap<String, FunctionTypeDef> = ontology_def.function_types
            .iter()
            .cloned()
            .map(|ft| (ft.id.clone(), ft))
            .collect();
        
        Ok(Self {
            config: OntologyConfig { ontology: ontology_def },
            object_types,
            link_types,
            action_types,
            interfaces,
            function_types,
        })
    }
    
    /// Load ontology from YAML file
    pub fn from_yaml(content: &str) -> Result<Self, String> {
        let config: OntologyConfig = serde_yaml::from_str(content)
            .map_err(|e| format!("Failed to parse YAML: {}", e))?;
        Self::from_config(config)
    }
    
    /// Load ontology from JSON file
    pub fn from_json(content: &str) -> Result<Self, String> {
        let config: OntologyConfig = serde_json::from_str(content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        Self::from_config(config)
    }
    
    /// Get an object type by ID
    pub fn get_object_type(&self, id: &str) -> Option<&ObjectType> {
        self.object_types.get(id)
    }
    
    /// Get a link type by ID
    pub fn get_link_type(&self, id: &str) -> Option<&LinkTypeDef> {
        self.link_types.get(id)
    }
    
    /// Get an action type by ID
    pub fn get_action_type(&self, id: &str) -> Option<&ActionTypeDef> {
        self.action_types.get(id)
    }
    
    /// Get all object types
    pub fn object_types(&self) -> impl Iterator<Item = &ObjectType> {
        self.object_types.values()
    }
    
    /// Get all link types
    pub fn link_types(&self) -> impl Iterator<Item = &LinkTypeDef> {
        self.link_types.values()
    }
    
    /// Get all action types
    pub fn action_types(&self) -> impl Iterator<Item = &ActionTypeDef> {
        self.action_types.values()
    }
    
    /// Get an interface by ID
    pub fn get_interface(&self, id: &str) -> Option<&InterfaceDef> {
        self.interfaces.get(id)
    }
    
    /// Get all interfaces
    pub fn interfaces(&self) -> impl Iterator<Item = &InterfaceDef> {
        self.interfaces.values()
    }
    
    /// Get a function type by ID
    pub fn get_function_type(&self, id: &str) -> Option<&FunctionTypeDef> {
        self.function_types.get(id)
    }
    
    
    /// Get all function types
    pub fn function_types(&self) -> impl Iterator<Item = &FunctionTypeDef> {
        self.function_types.values()
    }
}

// Re-export for convenience (runtime ontology)
pub use OntologyRuntime as Ontology;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::{Property, PropertyType};
    
    fn create_test_object_type() -> ObjectType {
        ObjectType {
            id: "test_object".to_string(),
            display_name: "Test Object".to_string(),
            primary_key: "id".to_string(),
            properties: vec![
                Property {
                    id: "id".to_string(),
                    display_name: None,
                    property_type: PropertyType::String,
                    required: true,
                    default: None,
                    validation: None,
                },
                Property {
                    id: "name".to_string(),
                    display_name: None,
                    property_type: PropertyType::String,
                    required: false,
                    default: None,
                    validation: None,
                },
            ],
            backing_datasource: None,
            title_key: Some("name".to_string()),
        }
    }
    
    #[test]
    fn test_object_type_validation_success() {
        let obj_type = create_test_object_type();
        assert!(obj_type.validate().is_ok());
    }
    
    #[test]
    fn test_object_type_validation_missing_primary_key() {
        let mut obj_type = create_test_object_type();
        obj_type.primary_key = "nonexistent".to_string();
        assert!(obj_type.validate().is_err());
    }
    
    #[test]
    fn test_object_type_get_property() {
        let obj_type = create_test_object_type();
        assert!(obj_type.get_property("id").is_some());
        assert!(obj_type.get_property("nonexistent").is_none());
    }
    
    #[test]
    fn test_ontology_from_yaml() {
        let yaml = r#"
ontology:
  objectTypes:
    - id: "test"
      displayName: "Test"
      primaryKey: "id"
      properties:
        - id: "id"
          type: "string"
          required: true
  linkTypes: []
  actionTypes: []
"#;
        let result = OntologyRuntime::from_yaml(yaml);
        assert!(result.is_ok());
        let ontology = result.unwrap();
        assert!(ontology.get_object_type("test").is_some());
    }
    
    #[test]
    fn test_ontology_from_json() {
        let json = r#"{
  "ontology": {
    "objectTypes": [
      {
        "id": "test",
        "displayName": "Test",
        "primaryKey": "id",
        "properties": [
          {
            "id": "id",
            "type": "string",
            "required": true
          }
        ]
      }
    ],
    "linkTypes": [],
    "actionTypes": []
  }
}"#;
        let result = OntologyRuntime::from_json(json);
        assert!(result.is_ok());
        let ontology = result.unwrap();
        assert!(ontology.get_object_type("test").is_some());
    }
    
    #[test]
    fn test_link_type_validation() {
        let link_type = LinkTypeDef {
            id: "test_link".to_string(),
            display_name: None,
            source: "source_type".to_string(),
            target: "target_type".to_string(),
            cardinality: LinkCardinality::OneToMany,
            properties: vec![],
            bidirectional: false,
        };
        
        // Should fail validation - source type doesn't exist
        assert!(link_type.validate(&[]).is_err());
        
        // Should pass validation
        assert!(link_type.validate(&["source_type".to_string(), "target_type".to_string()]).is_ok());
    }
}
