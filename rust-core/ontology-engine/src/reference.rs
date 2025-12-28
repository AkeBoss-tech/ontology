use crate::property::{PropertyValue, PropertyMap};
use crate::link::Link;

/// Helper for managing object references and auto-creating links
pub struct ReferenceManager;

impl ReferenceManager {
    /// Validate that a referenced object exists
    pub fn validate_reference_exists(
        object_type: &str,
        object_id: &str,
        checker: &dyn Fn(&str, &str) -> bool,
    ) -> Result<(), String> {
        if !checker(object_type, object_id) {
            return Err(format!(
                "Referenced object '{}' of type '{}' does not exist",
                object_id, object_type
            ));
        }
        Ok(())
    }
    
    /// Parse object reference string (format: "object_type:object_id" or just "object_id")
    pub fn parse_reference(ref_str: &str) -> Result<(String, String), String> {
        let parts: Vec<&str> = ref_str.split(':').collect();
        if parts.len() == 2 {
            Ok((parts[0].to_string(), parts[1].to_string()))
        } else if parts.len() == 1 {
            // If no type prefix, we need the object type from context
            // For now, return error - caller should provide full reference
            Err(format!(
                "Object reference '{}' must be in format 'object_type:object_id'",
                ref_str
            ))
        } else {
            Err(format!("Invalid object reference format: {}", ref_str))
        }
    }
    
    /// Create a link automatically when an object reference is set
    /// Returns the link that should be created
    pub fn create_link_for_reference(
        source_object_type: &str,
        source_object_id: &str,
        reference_property: &str,
        target_reference: &str,
        link_type_id: Option<&str>,
    ) -> Result<Link, String> {
        let (target_type, target_id) = Self::parse_reference(target_reference)?;
        
        // Generate link type ID if not provided (format: "{source_type}_to_{target_type}")
        let link_type = link_type_id.map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}_to_{}", source_object_type, target_type));
        
        Ok(Link::new(
            uuid::Uuid::new_v4().to_string(),
            link_type,
            format!("{}:{}", source_object_type, source_object_id),
            format!("{}:{}", target_type, target_id),
        ))
    }
    
    /// Get reverse references - find all objects that reference a given object
    pub fn find_reverse_references(
        target_object_type: &str,
        target_object_id: &str,
        reference_checker: &dyn Fn(&str, &str, &str) -> Vec<String>, // (object_type, property_name, target_id) -> source_ids
    ) -> Vec<(String, String, String)> { // (source_object_type, source_object_id, property_name)
        let mut results = Vec::new();
        
        // This would typically query the search store for objects with matching reference values
        // For now, this is a placeholder that uses the checker function
        // In a real implementation, this would query: property_name = "target_type:target_id"
        let ref_value = format!("{}:{}", target_object_type, target_object_id);
        
        // The checker function should return source object IDs that have this reference
        // This is a simplified interface - real implementation would be more sophisticated
        results
    }
}

/// Configuration for cascade delete behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CascadeDeleteBehavior {
    /// Delete referenced objects when source is deleted
    Cascade,
    /// Set reference to null when source is deleted
    SetNull,
    /// Prevent deletion if references exist
    Restrict,
    /// No special behavior
    NoAction,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_reference() {
        let (obj_type, obj_id) = ReferenceManager::parse_reference("person:123").unwrap();
        assert_eq!(obj_type, "person");
        assert_eq!(obj_id, "123");
    }
    
    #[test]
    fn test_parse_reference_invalid() {
        assert!(ReferenceManager::parse_reference("invalid:format:here").is_err());
    }
    
    #[test]
    fn test_create_link_for_reference() {
        let link = ReferenceManager::create_link_for_reference(
            "person",
            "p1",
            "occupation",
            "occupation:occ1",
            None,
        ).unwrap();
        
        assert_eq!(link.link_type_id, "person_to_occupation");
        assert!(link.source_id.contains("person:p1"));
        assert!(link.target_id.contains("occupation:occ1"));
    }
}




