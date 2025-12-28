use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::property::Property;

/// Property group - organizes related properties together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyGroup {
    pub id: String,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    #[serde(default)]
    pub description: Option<String>,
    
    /// Property IDs that belong to this group
    pub properties: Vec<String>,
    
    /// Display order (lower numbers appear first)
    #[serde(default)]
    pub order: i32,
    
    /// Whether this group is collapsible in the UI
    #[serde(default)]
    pub collapsible: bool,
    
    /// Whether this group is collapsed by default
    #[serde(default)]
    pub collapsed_by_default: bool,
}

impl PropertyGroup {
    pub fn new(id: String, display_name: String) -> Self {
        Self {
            id,
            display_name,
            description: None,
            properties: Vec::new(),
            order: 0,
            collapsible: false,
            collapsed_by_default: false,
        }
    }
    
    pub fn with_property(mut self, property_id: String) -> Self {
        self.properties.push(property_id);
        self
    }
    
    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }
}

/// Manager for property groups on an object type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyGroupManager {
    pub groups: Vec<PropertyGroup>,
}

impl PropertyGroupManager {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
        }
    }
    
    pub fn add_group(&mut self, group: PropertyGroup) {
        self.groups.push(group);
    }
    
    pub fn get_group(&self, group_id: &str) -> Option<&PropertyGroup> {
        self.groups.iter().find(|g| g.id == group_id)
    }
    
    pub fn get_groups_for_property(&self, property_id: &str) -> Vec<&PropertyGroup> {
        self.groups.iter()
            .filter(|g| g.properties.contains(&property_id.to_string()))
            .collect()
    }
    
    pub fn get_ungrouped_properties<'a>(&self, all_properties: &'a [Property]) -> Vec<&'a Property> {
        let grouped_ids: std::collections::HashSet<&String> = self.groups.iter()
            .flat_map(|g| &g.properties)
            .collect();
        
        all_properties.iter()
            .filter(|p| !grouped_ids.contains(&p.id))
            .collect()
    }
    
    pub fn get_sorted_groups(&self) -> Vec<&PropertyGroup> {
        let mut groups: Vec<&PropertyGroup> = self.groups.iter().collect();
        groups.sort_by_key(|g| g.order);
        groups
    }
}

impl Default for PropertyGroupManager {
    fn default() -> Self {
        Self::new()
    }
}
