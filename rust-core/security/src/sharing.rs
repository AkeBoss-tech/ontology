use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Sharing rule that defines who can access an object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingRule {
    pub id: String,
    pub object_type: String,
    pub object_id: String,
    
    /// Users who have explicit access
    pub shared_with_users: HashSet<String>,
    
    /// Groups/roles that have access
    pub shared_with_groups: HashSet<String>,
    
    /// Permission level: "read", "write", "admin"
    pub permission: SharingPermission,
    
    /// Whether this rule is inherited from a parent object
    pub inherited: bool,
    
    /// Parent object reference if inherited
    pub inherited_from: Option<(String, String)>, // (object_type, object_id)
}

/// Permission levels for sharing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SharingPermission {
    Read,
    Write,
    Admin,
}

impl SharingPermission {
    pub fn can_read(&self) -> bool {
        true // All permissions include read
    }
    
    pub fn can_write(&self) -> bool {
        matches!(self, SharingPermission::Write | SharingPermission::Admin)
    }
    
    pub fn can_admin(&self) -> bool {
        matches!(self, SharingPermission::Admin)
    }
}

/// Store for managing sharing rules
pub trait SharingRuleStore: Send + Sync {
    fn get_rules_for_object(&self, object_type: &str, object_id: &str) -> Vec<SharingRule>;
    fn add_rule(&mut self, rule: SharingRule) -> Result<(), SharingError>;
    fn remove_rule(&mut self, rule_id: &str) -> Result<(), SharingError>;
    fn get_rules_for_user(&self, user_id: &str) -> Vec<SharingRule>;
    fn get_rules_for_group(&self, group_id: &str) -> Vec<SharingRule>;
}

/// In-memory implementation of SharingRuleStore
pub struct InMemorySharingStore {
    rules: HashMap<String, SharingRule>, // rule_id -> rule
    object_index: HashMap<(String, String), Vec<String>>, // (object_type, object_id) -> [rule_ids]
    user_index: HashMap<String, Vec<String>>, // user_id -> [rule_ids]
    group_index: HashMap<String, Vec<String>>, // group_id -> [rule_ids]
}

impl InMemorySharingStore {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            object_index: HashMap::new(),
            user_index: HashMap::new(),
            group_index: HashMap::new(),
        }
    }
}

impl SharingRuleStore for InMemorySharingStore {
    fn get_rules_for_object(&self, object_type: &str, object_id: &str) -> Vec<SharingRule> {
        let key = (object_type.to_string(), object_id.to_string());
        if let Some(rule_ids) = self.object_index.get(&key) {
            rule_ids.iter()
                .filter_map(|id| self.rules.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn add_rule(&mut self, rule: SharingRule) -> Result<(), SharingError> {
        let rule_id = rule.id.clone();
        let object_key = (rule.object_type.clone(), rule.object_id.clone());
        
        // Add to main store
        self.rules.insert(rule_id.clone(), rule.clone());
        
        // Index by object
        self.object_index.entry(object_key)
            .or_insert_with(Vec::new)
            .push(rule_id.clone());
        
        // Index by users
        for user_id in &rule.shared_with_users {
            self.user_index.entry(user_id.clone())
                .or_insert_with(Vec::new)
                .push(rule_id.clone());
        }
        
        // Index by groups
        for group_id in &rule.shared_with_groups {
            self.group_index.entry(group_id.clone())
                .or_insert_with(Vec::new)
                .push(rule_id.clone());
        }
        
        Ok(())
    }
    
    fn remove_rule(&mut self, rule_id: &str) -> Result<(), SharingError> {
        if let Some(rule) = self.rules.remove(rule_id) {
            // Remove from object index
            let object_key = (rule.object_type, rule.object_id);
            if let Some(rule_ids) = self.object_index.get_mut(&object_key) {
                rule_ids.retain(|id| id != rule_id);
            }
            
            // Remove from user index
            for user_id in &rule.shared_with_users {
                if let Some(rule_ids) = self.user_index.get_mut(user_id) {
                    rule_ids.retain(|id| id != rule_id);
                }
            }
            
            // Remove from group index
            for group_id in &rule.shared_with_groups {
                if let Some(rule_ids) = self.group_index.get_mut(group_id) {
                    rule_ids.retain(|id| id != rule_id);
                }
            }
            
            Ok(())
        } else {
            Err(SharingError::RuleNotFound(rule_id.to_string()))
        }
    }
    
    fn get_rules_for_user(&self, user_id: &str) -> Vec<SharingRule> {
        if let Some(rule_ids) = self.user_index.get(user_id) {
            rule_ids.iter()
                .filter_map(|id| self.rules.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    fn get_rules_for_group(&self, group_id: &str) -> Vec<SharingRule> {
        if let Some(rule_ids) = self.group_index.get(group_id) {
            rule_ids.iter()
                .filter_map(|id| self.rules.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for InMemorySharingStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a user has access to an object via sharing rules
pub fn check_sharing_access(
    user_id: &str,
    user_groups: &HashSet<String>,
    object_type: &str,
    object_id: &str,
    store: &dyn SharingRuleStore,
    required_permission: &SharingPermission,
) -> bool {
    let rules = store.get_rules_for_object(object_type, object_id);
    
    for rule in rules {
        // Check if user has direct access
        if rule.shared_with_users.contains(user_id) {
            return has_permission(&rule.permission, required_permission);
        }
        
        // Check if user is in any of the shared groups
        for group in &rule.shared_with_groups {
            if user_groups.contains(group) {
                return has_permission(&rule.permission, required_permission);
            }
        }
    }
    
    false
}

fn has_permission(rule_permission: &SharingPermission, required: &SharingPermission) -> bool {
    match required {
        SharingPermission::Read => rule_permission.can_read(),
        SharingPermission::Write => rule_permission.can_write(),
        SharingPermission::Admin => rule_permission.can_admin(),
    }
}

/// Errors for sharing operations
#[derive(Debug, thiserror::Error)]
pub enum SharingError {
    #[error("Sharing rule not found: {0}")]
    RuleNotFound(String),
    
    #[error("Invalid sharing rule: {0}")]
    InvalidRule(String),
    
    #[error("Access denied: {0}")]
    AccessDenied(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sharing_store() {
        let mut store = InMemorySharingStore::new();
        
        let rule = SharingRule {
            id: "rule1".to_string(),
            object_type: "Person".to_string(),
            object_id: "person1".to_string(),
            shared_with_users: ["user1".to_string()].iter().cloned().collect(),
            shared_with_groups: ["group1".to_string()].iter().cloned().collect(),
            permission: SharingPermission::Read,
            inherited: false,
            inherited_from: None,
        };
        
        store.add_rule(rule).unwrap();
        
        let rules = store.get_rules_for_object("Person", "person1");
        assert_eq!(rules.len(), 1);
        
        let user_rules = store.get_rules_for_user("user1");
        assert_eq!(user_rules.len(), 1);
    }
}

