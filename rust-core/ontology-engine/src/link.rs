use serde::{Deserialize, Serialize};

/// Link cardinality - defines the relationship between source and target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LinkCardinality {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl Default for LinkCardinality {
    fn default() -> Self {
        LinkCardinality::OneToMany
    }
}

/// Link direction for traversal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkDirection {
    Forward,  // Source -> Target
    Backward, // Target -> Source
    Bidirectional,
}

/// Link instance - represents a runtime connection between two objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub link_type_id: String,
    pub source_id: String,
    pub target_id: String,
    pub properties: crate::property::PropertyMap,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Link {
    /// Create a new link
    pub fn new(
        id: String,
        link_type_id: String,
        source_id: String,
        target_id: String,
    ) -> Self {
        Self {
            id,
            link_type_id,
            source_id,
            target_id,
            properties: crate::property::PropertyMap::new(),
            created_at: chrono::Utc::now(),
        }
    }
    
    /// Get the other end of the link given one object ID
    pub fn get_other_end(&self, object_id: &str) -> Option<&str> {
        if self.source_id == object_id {
            Some(&self.target_id)
        } else if self.target_id == object_id {
            Some(&self.source_id)
        } else {
            None
        }
    }
    
    /// Check if this link connects to the given object ID
    pub fn connects_to(&self, object_id: &str) -> bool {
        self.source_id == object_id || self.target_id == object_id
    }
    
    /// Get direction from source perspective
    pub fn direction_from(&self, object_id: &str) -> Option<LinkDirection> {
        if self.source_id == object_id {
            Some(LinkDirection::Forward)
        } else if self.target_id == object_id {
            Some(LinkDirection::Backward)
        } else {
            None
        }
    }
}

/// Link Type definition (re-exported from meta_model for convenience)
pub use crate::meta_model::LinkTypeDef as LinkType;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_link_creation() {
        let link = Link::new(
            "link1".to_string(),
            "link_type1".to_string(),
            "source1".to_string(),
            "target1".to_string(),
        );
        
        assert_eq!(link.id, "link1");
        assert_eq!(link.source_id, "source1");
        assert_eq!(link.target_id, "target1");
    }
    
    #[test]
    fn test_link_get_other_end() {
        let link = Link::new(
            "link1".to_string(),
            "link_type1".to_string(),
            "source1".to_string(),
            "target1".to_string(),
        );
        
        assert_eq!(link.get_other_end("source1"), Some("target1"));
        assert_eq!(link.get_other_end("target1"), Some("source1"));
        assert_eq!(link.get_other_end("other"), None);
    }
    
    #[test]
    fn test_link_connects_to() {
        let link = Link::new(
            "link1".to_string(),
            "link_type1".to_string(),
            "source1".to_string(),
            "target1".to_string(),
        );
        
        assert!(link.connects_to("source1"));
        assert!(link.connects_to("target1"));
        assert!(!link.connects_to("other"));
    }
    
    #[test]
    fn test_link_direction_from() {
        let link = Link::new(
            "link1".to_string(),
            "link_type1".to_string(),
            "source1".to_string(),
            "target1".to_string(),
        );
        
        assert_eq!(link.direction_from("source1"), Some(LinkDirection::Forward));
        assert_eq!(link.direction_from("target1"), Some(LinkDirection::Backward));
        assert_eq!(link.direction_from("other"), None);
    }
    
    #[test]
    fn test_link_cardinality_default() {
        assert_eq!(LinkCardinality::default(), LinkCardinality::OneToMany);
    }
}
