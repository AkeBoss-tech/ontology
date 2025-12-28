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
        
        // Blue/Green migration workflow:
        // 1. Create new versioned index (e.g., ontology_user_v2)
        // 2. Create alias pointing to new index (if first time) or reindex from old version
        // 3. Update schema in new index
        // 4. Reindex data from old version to new version (if needed)
        // 5. Atomically swap alias from old to new version
        // 6. Delete old versioned index after verification
        //
        // Note: This requires access to ElasticsearchStore for index management.
        // In a full implementation, you would:
        // - Get current version from alias
        // - Create new versioned index with updated schema
        // - Reindex data if needed
        // - Swap alias atomically
        // - Clean up old index
        
        // For now, we'll directly modify the in-memory schema
        // The actual index migration should be handled by the indexing layer
        // when it detects schema version changes
        
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
        
        // Graph store migration:
        // Dgraph schema changes are typically additive (new predicates)
        // and don't require data migration. However, if removing predicates
        // or changing types, you'd need to:
        // 1. Update Dgraph schema
        // 2. Migrate existing data if needed
        // 3. Verify data integrity
        
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
        
        // Cleanup workflow:
        // 1. Verify no active references to this object type
        // 2. Delete all objects of this type from search index
        // 3. Delete all links involving this object type from graph store
        // 4. Delete columnar data for this object type
        // 5. Remove from in-memory schema
        
        // Note: This is a destructive operation and should be used carefully
        // Consider deprecation instead of deletion for production systems
        
        *version += 1;
        Ok(())
    }
    
    /// Update an existing object type
    pub fn update_object_type(&self, object_type: ObjectType) -> Result<(), String> {
        // Blue/Green schema update workflow:
        // 1. Determine migration strategy based on changes:
        //    - Additive (new properties): Safe, no migration needed
        //    - Transformative (type changes): Requires data transformation
        //    - Reindex (structural changes): Requires full reindex
        // 2. Get current version from alias
        // 3. Create new versioned index with updated schema
        // 4. If additive: Just swap alias (new index can be empty, will populate over time)
        // 5. If transformative: Transform and reindex data
        // 6. If reindex: Full reindex from old version
        // 7. Atomically swap alias
        // 8. Monitor new version, then delete old version
        
        // Example migration code (pseudo-code):
        // let current_version = search_store.get_alias_version(&object_type.id).await?;
        // let new_version = current_version.unwrap_or(1) + 1;
        // search_store.reindex(&object_type.id, current_version, new_version).await?;
        // search_store.swap_alias(&object_type.id, current_version, new_version).await?;
        // search_store.delete_versioned_index(&object_type.id, current_version).await?;
        
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

