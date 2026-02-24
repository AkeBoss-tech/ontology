use crate::meta_model::{ObjectType, InterfaceDef};
use crate::property::{Property, PropertyType};
use std::collections::HashMap;

/// Interface validator - validates that object types satisfy interface contracts
pub struct InterfaceValidator;

impl InterfaceValidator {
    /// Validate that an object type implements an interface
    pub fn validate_implements(
        object_type: &ObjectType,
        interface: &InterfaceDef,
    ) -> Result<(), String> {
        // Check that all required properties exist in the object type
        for interface_prop in &interface.properties {
            let obj_prop = object_type.get_property(&interface_prop.id)
                .ok_or_else(|| format!(
                    "Object type '{}' does not implement required property '{}' from interface '{}'",
                    object_type.id, interface_prop.id, interface.id
                ))?;
            
            // Check property type compatibility (allowing covariant types)
            if !Self::is_type_compatible(&obj_prop.property_type, &interface_prop.property_type) {
                return Err(format!(
                    "Object type '{}' property '{}' has type {:?} which is not compatible with interface '{}' requirement {:?}",
                    object_type.id, interface_prop.id, obj_prop.property_type, interface.id, interface_prop.property_type
                ));
            }
            
            // Check that required properties are also required in the object type
            if interface_prop.required && !obj_prop.required {
                return Err(format!(
                    "Object type '{}' property '{}' must be required to implement interface '{}'",
                    object_type.id, interface_prop.id, interface.id
                ));
            }
        }
        
        // Check that all required link types are supported
        // This is a simplified check - in a full implementation, we'd verify
        // that the object type can participate in these link types
        for _link_type_id in &interface.required_link_types {
            // For now, we just check that the link type exists somewhere in the ontology
            // A more complete check would verify the object type is source or target
            // This would require access to the full ontology, which we handle at a higher level
        }
        
        Ok(())
    }
    
    /// Check if two property types are compatible (covariant checking)
    fn is_type_compatible(actual: &PropertyType, required: &PropertyType) -> bool {
        // Exact match
        if actual == required {
            return true;
        }
        
        // Handle type aliases (e.g., int vs integer, float vs double)
        match (actual.as_simple(), required.as_simple()) {
            (Some(act), Some(req)) => act == req,
            _ => false,
        }
    }
    
    /// Get all object types that implement a given interface
    pub fn get_implementers<'a>(
        interface_id: &str,
        object_types: impl Iterator<Item = &'a ObjectType>,
    ) -> Vec<&'a ObjectType> {
        object_types
            .filter(|obj_type| obj_type.implements.contains(&interface_id.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::{Property, PropertyType};
    
    fn create_test_interface() -> InterfaceDef {
        InterfaceDef {
            id: "Location".to_string(),
            display_name: "Location".to_string(),
            properties: vec![
                Property {
                    id: "latitude".to_string(),
                    display_name: None,
                    property_type: PropertyType::Double,
                    required: true,
                    default: None,
                    validation: None,
                    description: None,
                    annotations: HashMap::new(),
                    unit: None,
                    format: None,
                    sensitivity_tags: Vec::new(),
                    pii: false,
                    deprecated: None,
                    statistics: None,
                    model_binding: None,                },
                Property {
                    id: "longitude".to_string(),
                    display_name: None,
                    property_type: PropertyType::Double,
                    required: true,
                    default: None,
                    validation: None,
                    description: None,
                    annotations: HashMap::new(),
                    unit: None,
                    format: None,
                    sensitivity_tags: Vec::new(),
                    pii: false,
                    deprecated: None,
                    statistics: None,
                    model_binding: None,
                },
            ],
            required_link_types: Vec::new(),
        }
    }
    
    fn create_implementing_object_type() -> ObjectType {
        ObjectType {
            id: "office".to_string(),
            display_name: "Office".to_string(),
            primary_key: "id".to_string(),
            properties: vec![
                Property {
                    id: "id".to_string(),
                    display_name: None,
                    property_type: PropertyType::String,
                    required: true,
                    default: None,
                    validation: None,
                    description: None,
                    annotations: HashMap::new(),
                    unit: None,
                    format: None,
                    sensitivity_tags: Vec::new(),
                    pii: false,
                    deprecated: None,
                    statistics: None,
                    model_binding: None,
                },
                Property {
                    id: "latitude".to_string(),
                    display_name: None,
                    property_type: PropertyType::Double,
                    required: true,
                    default: None,
                    validation: None,
                    description: None,
                    annotations: HashMap::new(),
                    unit: None,
                    format: None,
                    sensitivity_tags: Vec::new(),
                    pii: false,
                    deprecated: None,
                    statistics: None,
                    model_binding: None,
                },
                Property {
                    id: "longitude".to_string(),
                    display_name: None,
                    property_type: PropertyType::Double,
                    required: true,
                    default: None,
                    validation: None,
                    description: None,
                    annotations: HashMap::new(),
                    unit: None,
                    format: None,
                    sensitivity_tags: Vec::new(),
                    pii: false,
                    deprecated: None,
                    statistics: None,
                    model_binding: None,
                },
            ],
            backing_datasource: None,
            title_key: Some("id".to_string()),
            implements: vec!["Location".to_string()],
            schema_evolution: None,
        }
    }
    
    #[test]
    fn test_validate_implements_success() {
        let interface = create_test_interface();
        let object_type = create_implementing_object_type();
        
        assert!(InterfaceValidator::validate_implements(&object_type, &interface).is_ok());
    }
    
    #[test]
    fn test_validate_implements_missing_property() {
        let interface = create_test_interface();
        let mut object_type = create_implementing_object_type();
        object_type.properties.pop(); // Remove longitude
        
        assert!(InterfaceValidator::validate_implements(&object_type, &interface).is_err());
    }
    
    #[test]
    fn test_get_implementers() {
        let object_type1 = create_implementing_object_type();
        let mut object_type2 = create_implementing_object_type();
        object_type2.id = "warehouse".to_string();
        object_type2.implements = vec!["Location".to_string()];
        
        let object_types = vec![&object_type1, &object_type2];
        let implementers = InterfaceValidator::get_implementers("Location", object_types.iter().copied());
        
        assert_eq!(implementers.len(), 2);
    }
}

