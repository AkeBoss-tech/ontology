use crate::queue::UserEdit;
use ontology_engine::PropertyMap;

/// Merge source data with user edits (overlay architecture)
pub fn merge_source_and_edits(
    source_properties: &PropertyMap,
    edits: &[UserEdit],
) -> MergeResult {
    let mut merged = source_properties.clone();
    let mut overridden_properties = std::collections::HashSet::new();
    let mut conflicts = Vec::new();
    
    // Apply edits in chronological order (oldest first)
    // In practice, we'd want the most recent edit for each property
    let mut edits_by_property: std::collections::HashMap<String, &UserEdit> = std::collections::HashMap::new();
    for edit in edits {
        if edit.deleted {
            // Mark property as deleted
            merged = merged; // Would need a way to track deleted properties
            overridden_properties.insert(edit.property_name.clone());
            edits_by_property.insert(edit.property_name.clone(), edit);
        } else {
            // Use the most recent edit for each property
            let existing = edits_by_property.get(&edit.property_name);
            if existing.map_or(true, |e| edit.timestamp > e.timestamp) {
                edits_by_property.insert(edit.property_name.clone(), edit);
            }
        }
    }
    
    // Apply the most recent edit for each property
    for (property_name, edit) in edits_by_property {
        if edit.deleted {
            // Remove the property (in a real implementation, we'd track this differently)
            // For now, we'll leave it but mark as overridden
            overridden_properties.insert(property_name.clone());
        } else {
            // Check if source had this property
            let had_source_value = source_properties.contains_key(&property_name);
            
            // Apply the edit
            merged.insert(property_name.clone(), edit.property_value.clone());
            overridden_properties.insert(property_name.clone());
            
            // If source had a different value, record as conflict
            if had_source_value {
                if let Some(source_value) = source_properties.get(&property_name) {
                    if source_value != &edit.property_value {
                        conflicts.push(PropertyConflict {
                            property_name: property_name.clone(),
                            source_value: source_value.clone(),
                            edited_value: edit.property_value.clone(),
                            edit_timestamp: edit.timestamp,
                        });
                    }
                }
            }
        }
    }
    
    MergeResult {
        merged_properties: merged,
        overridden_properties,
        conflicts,
    }
}

/// Result of merging source data with edits
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub merged_properties: PropertyMap,
    pub overridden_properties: std::collections::HashSet<String>,
    pub conflicts: Vec<PropertyConflict>,
}

/// A conflict between source value and edited value
#[derive(Debug, Clone)]
pub struct PropertyConflict {
    pub property_name: String,
    pub source_value: ontology_engine::PropertyValue,
    pub edited_value: ontology_engine::PropertyValue,
    pub edit_timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontology_engine::PropertyValue;
    use chrono::Utc;
    
    #[test]
    fn test_merge_source_and_edits() {
        let mut source = PropertyMap::new();
        source.insert("prop1".to_string(), PropertyValue::String("source_value".to_string()));
        source.insert("prop2".to_string(), PropertyValue::String("source_value2".to_string()));
        
        let edit = UserEdit {
            edit_id: "edit1".to_string(),
            object_type: "test".to_string(),
            object_id: "test_id".to_string(),
            property_name: "prop1".to_string(),
            property_value: PropertyValue::String("edited_value".to_string()),
            user_id: "user1".to_string(),
            timestamp: Utc::now(),
            deleted: false,
        };
        
        let result = merge_source_and_edits(&source, &[edit]);
        
        // Check that prop1 was overridden
        assert!(result.overridden_properties.contains("prop1"));
        
        // Check that the merged value is the edited value
        assert_eq!(
            result.merged_properties.get("prop1"),
            Some(&PropertyValue::String("edited_value".to_string()))
        );
        
        // Check that prop2 remains unchanged
        assert_eq!(
            result.merged_properties.get("prop2"),
            Some(&PropertyValue::String("source_value2".to_string()))
        );
    }
    
    #[test]
    fn test_merge_with_conflict() {
        let mut source = PropertyMap::new();
        source.insert("prop1".to_string(), PropertyValue::String("source_value".to_string()));
        
        let edit = UserEdit {
            edit_id: "edit1".to_string(),
            object_type: "test".to_string(),
            object_id: "test_id".to_string(),
            property_name: "prop1".to_string(),
            property_value: PropertyValue::String("edited_value".to_string()),
            user_id: "user1".to_string(),
            timestamp: Utc::now(),
            deleted: false,
        };
        
        let result = merge_source_and_edits(&source, &[edit]);
        
        // Should have a conflict
        assert!(!result.conflicts.is_empty());
        assert_eq!(result.conflicts[0].property_name, "prop1");
    }
}
