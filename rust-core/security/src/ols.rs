use ontology_engine::{PropertyMap, PropertyValue};
use std::collections::HashSet;

/// Object Level Security - controls access to individual objects based on user attributes
pub struct ObjectLevelSecurity;

/// Security context for a user request
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: String,
    pub roles: HashSet<String>,
    pub badges: HashSet<String>,
    pub clearances: HashSet<String>,
}

impl SecurityContext {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            roles: HashSet::new(),
            badges: HashSet::new(),
            clearances: HashSet::new(),
        }
    }
    
    pub fn with_role(mut self, role: String) -> Self {
        self.roles.insert(role);
        self
    }
    
    pub fn with_badge(mut self, badge: String) -> Self {
        self.badges.insert(badge);
        self
    }
    
    pub fn with_clearance(mut self, clearance: String) -> Self {
        self.clearances.insert(clearance);
        self
    }
    
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
    
    pub fn has_badge(&self, badge: &str) -> bool {
        self.badges.contains(badge)
    }
    
    pub fn has_clearance(&self, clearance: &str) -> bool {
        self.clearances.contains(clearance)
    }
}

/// Security requirements for an object
#[derive(Debug, Clone)]
pub struct ObjectSecurityPolicy {
    pub required_roles: HashSet<String>,
    pub required_badges: HashSet<String>,
    pub required_clearances: HashSet<String>,
    pub property_level_access: Option<PropertyAccessControl>,
}

/// Property-level access control
#[derive(Debug, Clone)]
pub struct PropertyAccessControl {
    pub restricted_properties: HashSet<String>,
    pub required_clearance_for_properties: std::collections::HashMap<String, String>,
}

impl ObjectSecurityPolicy {
    pub fn new() -> Self {
        Self {
            required_roles: HashSet::new(),
            required_badges: HashSet::new(),
            required_clearances: HashSet::new(),
            property_level_access: None,
        }
    }
    
    pub fn with_required_role(mut self, role: String) -> Self {
        self.required_roles.insert(role);
        self
    }
    
    pub fn with_required_badge(mut self, badge: String) -> Self {
        self.required_badges.insert(badge);
        self
    }
    
    pub fn with_required_clearance(mut self, clearance: String) -> Self {
        self.required_clearances.insert(clearance);
        self
    }
    
    pub fn with_property_access_control(mut self, pac: PropertyAccessControl) -> Self {
        self.property_level_access = Some(pac);
        self
    }
}

/// Check if a user has access to an object
pub fn check_access(
    context: &SecurityContext,
    policy: &ObjectSecurityPolicy,
) -> Result<(), SecurityError> {
    // Check roles
    if !policy.required_roles.is_empty() {
        let has_role = policy.required_roles.iter()
            .any(|role| context.has_role(role));
        if !has_role {
            return Err(SecurityError::AccessDenied(format!(
                "Missing required role. Required: {:?}, User has: {:?}",
                policy.required_roles, context.roles
            )));
        }
    }
    
    // Check badges
    if !policy.required_badges.is_empty() {
        let has_badge = policy.required_badges.iter()
            .any(|badge| context.has_badge(badge));
        if !has_badge {
            return Err(SecurityError::AccessDenied(format!(
                "Missing required badge. Required: {:?}, User has: {:?}",
                policy.required_badges, context.badges
            )));
        }
    }
    
    // Check clearances
    if !policy.required_clearances.is_empty() {
        let has_clearance = policy.required_clearances.iter()
            .any(|clearance| context.has_clearance(clearance));
        if !has_clearance {
            return Err(SecurityError::AccessDenied(format!(
                "Missing required clearance. Required: {:?}, User has: {:?}",
                policy.required_clearances, context.clearances
            )));
        }
    }
    
    Ok(())
}

/// Filter object properties based on property-level access control
pub fn filter_properties(
    context: &SecurityContext,
    properties: &PropertyMap,
    policy: &ObjectSecurityPolicy,
) -> PropertyMap {
    let mut filtered = PropertyMap::new();
    
    if let Some(ref pac) = policy.property_level_access {
        for (key, value) in properties.iter() {
            // Check if property is restricted
            if pac.restricted_properties.contains(key.as_str()) {
                continue; // Skip restricted property
            }
            
            // Check clearance requirements for specific properties
            if let Some(required_clearance) = pac.required_clearance_for_properties.get(key) {
                if !context.has_clearance(required_clearance) {
                    continue; // Skip property requiring clearance
                }
            }
            
            // Property is accessible
            filtered.insert(key.clone(), value.clone());
        }
    } else {
        // No property-level restrictions, return all properties
        for (key, value) in properties.iter() {
            filtered.insert(key.clone(), value.clone());
        }
    }
    
    filtered
}

impl ObjectLevelSecurity {
    /// Create a security policy from object properties
    /// In a real system, this would read from object metadata or a security store
    pub fn get_policy_for_object(
        object_type: &str,
        object_properties: &PropertyMap,
    ) -> ObjectSecurityPolicy {
        // Example: Objects with "classification" property might require clearance
        // This is a simplified implementation - production would use a policy store
        let mut policy = ObjectSecurityPolicy::new();
        
        if let Some(classification) = object_properties.get("classification") {
            if let PropertyValue::String(cls) = classification {
                match cls.as_str() {
                    "Top Secret" => {
                        policy = policy.with_required_clearance("Top Secret".to_string());
                    }
                    "Secret" => {
                        policy = policy.with_required_clearance("Secret".to_string());
                    }
                    "Confidential" => {
                        policy = policy.with_required_clearance("Confidential".to_string());
                    }
                    _ => {}
                }
            }
        }
        
        policy
    }
}

impl Default for ObjectSecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// Security errors
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    #[error("Invalid security context: {0}")]
    InvalidContext(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_context() {
        let context = SecurityContext::new("user1".to_string())
            .with_role("admin".to_string())
            .with_badge("premium".to_string())
            .with_clearance("Secret".to_string());
        
        assert!(context.has_role("admin"));
        assert!(context.has_badge("premium"));
        assert!(context.has_clearance("Secret"));
        assert!(!context.has_role("user"));
    }
    
    #[test]
    fn test_check_access_with_role() {
        let policy = ObjectSecurityPolicy::new()
            .with_required_role("admin".to_string());
        
        // Should fail without role
        let context = SecurityContext::new("user1".to_string());
        assert!(check_access(&context, &policy).is_err());
        
        // Should pass with role
        let context = SecurityContext::new("user1".to_string())
            .with_role("admin".to_string());
        assert!(check_access(&context, &policy).is_ok());
    }
    
    #[test]
    fn test_check_access_with_clearance() {
        let policy = ObjectSecurityPolicy::new()
            .with_required_clearance("Top Secret".to_string());
        
        // Should fail without clearance
        let context = SecurityContext::new("user1".to_string());
        assert!(check_access(&context, &policy).is_err());
        
        // Should pass with clearance
        let context = SecurityContext::new("user1".to_string())
            .with_clearance("Top Secret".to_string());
        assert!(check_access(&context, &policy).is_ok());
    }
    
    #[test]
    fn test_filter_properties() {
        let mut properties = PropertyMap::new();
        properties.insert("public".to_string(), PropertyValue::String("data".to_string()));
        properties.insert("secret".to_string(), PropertyValue::String("classified".to_string()));
        
        let mut pac = PropertyAccessControl {
            restricted_properties: HashSet::new(),
            required_clearance_for_properties: std::collections::HashMap::new(),
        };
        pac.restricted_properties.insert("secret".to_string());
        
        let policy = ObjectSecurityPolicy::new()
            .with_property_access_control(pac);
        
        let context = SecurityContext::new("user1".to_string());
        let filtered = filter_properties(&context, &properties, &policy);
        
        assert!(filtered.contains_key("public"));
        assert!(!filtered.contains_key("secret"));
    }
    
    #[test]
    fn test_get_policy_for_object() {
        let mut properties = PropertyMap::new();
        properties.insert("classification".to_string(), PropertyValue::String("Secret".to_string()));
        
        let policy = ObjectLevelSecurity::get_policy_for_object("document", &properties);
        assert!(!policy.required_clearances.is_empty());
    }
}
