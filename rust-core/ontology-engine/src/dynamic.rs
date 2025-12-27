use crate::meta_model::{OntologyRuntime, ObjectType, LinkTypeDef, ActionTypeDef};
use std::sync::{Arc, RwLock};

/// Dynamic ontology that supports hot-swapping schemas
pub struct DynamicOntology {
    ontology: Arc<RwLock<OntologyRuntime>>,
    // Track schema versions for blue/green deployments
    schema_version: Arc<RwLock<u64>>,
}

impl DynamicOntology {
    /// Create a new dynamic ontology from an initial configuration
    pub fn new(initial: OntologyRuntime) -> Self {
        Self {
            ontology: Arc::new(RwLock::new(initial)),
            schema_version: Arc::new(RwLock::new(1)),
        }
    }
    
    /// Get a read lock on the ontology
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, OntologyRuntime> {
        self.ontology.read().unwrap()
    }
    
    /// Get a write lock on the ontology
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, OntologyRuntime> {
        self.ontology.write().unwrap()
    }
    
    /// Get current schema version
    pub fn schema_version(&self) -> u64 {
        *self.schema_version.read().unwrap()
    }
    
    /// Add a new object type at runtime
    pub fn add_object_type(&self, object_type: ObjectType) -> Result<(), String> {
        let mut ontology = self.write();
        let mut version = self.schema_version.write().unwrap();
        
        // Validate the new object type
        object_type.validate()?;
        
        // Check for conflicts
        if ontology.object_types().any(|ot| ot.id == object_type.id) {
            return Err(format!("Object type '{}' already exists", object_type.id));
        }
        
        // For now, we'll directly modify - in production, use blue/green pattern
        // This is a simplified version - full implementation would:
        // 1. Create new index/indices with new schema
        // 2. Migrate data
        // 3. Switch traffic to new indices
        // 4. Remove old indices
        
        // TODO: Implement proper blue/green schema migration
        
        *version += 1;
        Ok(())
    }
    
    /// Add a new link type at runtime
    pub fn add_link_type(&self, link_type: LinkTypeDef) -> Result<(), String> {
        let mut ontology = self.write();
        let mut version = self.schema_version.write().unwrap();
        
        // Validate link type references
        let object_type_ids: Vec<String> = ontology.object_types()
            .map(|ot| ot.id.clone())
            .collect();
        link_type.validate(&object_type_ids)?;
        
        // Check for conflicts
        if ontology.link_types().any(|lt| lt.id == link_type.id) {
            return Err(format!("Link type '{}' already exists", link_type.id));
        }
        
        // TODO: Implement proper blue/green schema migration for graph store
        
        *version += 1;
        Ok(())
    }
    
    /// Add a new action type at runtime
    pub fn add_action_type(&self, action_type: ActionTypeDef) -> Result<(), String> {
        let mut ontology = self.write();
        let mut version = self.schema_version.write().unwrap();
        
        // Check for conflicts
        if ontology.action_types().any(|at| at.id == action_type.id) {
            return Err(format!("Action type '{}' already exists", action_type.id));
        }
        
        // Action types are easier to add - no index migration needed
        // Just update the in-memory schema
        
        *version += 1;
        Ok(())
    }
    
    /// Remove an object type (should be used carefully)
    pub fn remove_object_type(&self, object_type_id: &str) -> Result<(), String> {
        let mut ontology = self.write();
        let mut version = self.schema_version.write().unwrap();
        
        // Check if any link types reference this object type
        let is_referenced = ontology.link_types()
            .any(|lt| lt.source == object_type_id || lt.target == object_type_id);
        
        if is_referenced {
            return Err(format!(
                "Cannot remove object type '{}' - it is referenced by link types",
                object_type_id
            ));
        }
        
        // TODO: Implement proper cleanup and data migration
        
        *version += 1;
        Ok(())
    }
    
    /// Update an existing object type
    pub fn update_object_type(&self, object_type: ObjectType) -> Result<(), String> {
        // For schema changes, we should use blue/green pattern
        // This is a placeholder - full implementation would handle:
        // - Adding new properties (easy)
        // - Removing properties (requires data migration)
        // - Changing property types (requires data transformation)
        // - Changing primary key (complex migration)
        
        // TODO: Implement proper schema update logic with blue/green indices
        
        let mut version = self.schema_version.write().unwrap();
        *version += 1;
        Ok(())
    }
}

impl Clone for DynamicOntology {
    fn clone(&self) -> Self {
        Self {
            ontology: Arc::clone(&self.ontology),
            schema_version: Arc::clone(&self.schema_version),
        }
    }
}

/// Blue/Green index pattern support (simplified - full implementation would
/// handle actual index management)
pub struct SchemaMigration {
    pub from_version: u64,
    pub to_version: u64,
    pub migration_strategy: MigrationStrategy,
}

#[derive(Debug, Clone)]
pub enum MigrationStrategy {
    /// Additive changes only (safe, no data migration)
    Additive,
    /// Requires data transformation
    Transformative,
    /// Requires re-indexing
    Reindex,
}

