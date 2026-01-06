use ontology_engine::PropertyValue;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Reference to another object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectReference {
    pub object_type: String,
    pub object_id: String,
}

/// Transformation applied to data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub name: String,
    pub description: String,
    pub applied_at: DateTime<Utc>,
    pub parameters: HashMap<String, String>,
}

/// Data lineage and provenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    pub object_id: String,
    pub object_type: String,
    
    // Source information
    pub source_system: String,
    pub source_table: Option<String>,
    pub source_file: Option<String>,
    pub source_query: Option<String>,
    
    // Transformation history
    pub transformations: Vec<Transformation>,
    
    // Ingestion metadata
    pub ingested_at: DateTime<Utc>,
    pub ingested_by: String, // Pipeline/process name
    pub ingestion_version: String,
    
    // Dependencies
    pub depends_on: Vec<ObjectReference>, // Other objects this depends on
    pub derived_from: Vec<ObjectReference>, // Objects this was derived from
}

impl DataLineage {
    pub fn new(
        object_type: String,
        object_id: String,
        source_system: String,
        ingested_by: String,
    ) -> Self {
        Self {
            object_id,
            object_type,
            source_system,
            source_table: None,
            source_file: None,
            source_query: None,
            transformations: Vec::new(),
            ingested_at: Utc::now(),
            ingested_by,
            ingestion_version: "1.0.0".to_string(),
            depends_on: Vec::new(),
            derived_from: Vec::new(),
        }
    }
    
    /// Add a transformation to the lineage
    pub fn add_transformation(&mut self, name: String, description: String, parameters: HashMap<String, String>) {
        self.transformations.push(Transformation {
            name,
            description,
            applied_at: Utc::now(),
            parameters,
        });
    }
    
    /// Add a dependency
    pub fn add_dependency(&mut self, object_type: String, object_id: String) {
        self.depends_on.push(ObjectReference {
            object_type,
            object_id,
        });
    }
    
    /// Add a source object this was derived from
    pub fn add_derived_from(&mut self, object_type: String, object_id: String) {
        self.derived_from.push(ObjectReference {
            object_type,
            object_id,
        });
    }
}


