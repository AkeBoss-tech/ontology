use crate::store::{SearchStore, GraphStore, IndexedObject, StoreError};
use ontology_engine::{ObjectType, PropertyMap};

/// Object hydrator - converts indexed data back into full object representations
pub struct ObjectHydrator {
    // In a real implementation, might need access to ontology for validation
}

impl ObjectHydrator {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Hydrate an object from search index results
    pub fn hydrate_from_indexed(
        &self,
        indexed: &IndexedObject,
        object_type: &ObjectType,
    ) -> Result<HydratedObject, StoreError> {
        // Validate that all required properties are present
        for prop_def in &object_type.properties {
            if prop_def.required {
                if !indexed.properties.contains_key(&prop_def.id) {
                    return Err(StoreError::Query(format!(
                        "Missing required property '{}' for object type '{}'",
                        prop_def.id, object_type.id
                    )));
                }
            }
        }
        
        // Build title from title_key if specified
        let title = object_type.title_key.as_ref()
            .and_then(|key| indexed.properties.get(key))
            .map(|v| v.to_string())
            .unwrap_or_else(|| indexed.object_id.clone());
        
        Ok(HydratedObject {
            object_type: indexed.object_type.clone(),
            object_id: indexed.object_id.clone(),
            title,
            properties: indexed.properties.clone(),
        })
    }
    
    /// Bulk hydrate multiple objects
    pub fn hydrate_batch(
        &self,
        indexed_objects: &[IndexedObject],
        object_type: &ObjectType,
    ) -> Result<Vec<HydratedObject>, StoreError> {
        indexed_objects.iter()
            .map(|idx| self.hydrate_from_indexed(idx, object_type))
            .collect()
    }
    
    /// Get linked objects from graph store and hydrate them
    pub async fn get_and_hydrate_linked(
        &self,
        graph_store: &dyn GraphStore,
        search_store: &dyn SearchStore,
        object_id: &str,
        link_type_id: &str,
        object_type: &ObjectType,
    ) -> Result<Vec<HydratedObject>, StoreError> {
        // Get connected object IDs from graph store
        let connected_ids = graph_store
            .get_connected_objects(object_id, link_type_id)
            .await?;
        
        // Bulk fetch objects from search store
        let mut hydrated = Vec::new();
        for id in connected_ids {
            if let Some(indexed) = search_store
                .get_object(&object_type.id, &id)
                .await?
            {
                match self.hydrate_from_indexed(&indexed, object_type) {
                    Ok(obj) => hydrated.push(obj),
                    Err(e) => {
                        eprintln!("Error hydrating object {}: {}", id, e);
                        // Continue with other objects
                    }
                }
            }
        }
        
        Ok(hydrated)
    }
}

impl Default for ObjectHydrator {
    fn default() -> Self {
        Self::new()
    }
}

/// A fully hydrated object ready for API responses
#[derive(Debug, Clone)]
pub struct HydratedObject {
    pub object_type: String,
    pub object_id: String,
    pub title: String,
    pub properties: PropertyMap,
}

impl HydratedObject {
    /// Convert to JSON-friendly representation
    pub fn to_json_value(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("object_type".to_string(), self.object_type.clone().into());
        map.insert("object_id".to_string(), self.object_id.clone().into());
        map.insert("title".to_string(), self.title.clone().into());
        
        let mut props = serde_json::Map::new();
        for (key, value) in self.properties.iter() {
            props.insert(key.clone(), property_value_to_json(value));
        }
        map.insert("properties".to_string(), props.into());
        
        map.into()
    }
}

/// Convert PropertyValue to JSON
fn property_value_to_json(value: &ontology_engine::PropertyValue) -> serde_json::Value {
    match value {
        ontology_engine::PropertyValue::String(s) => s.clone().into(),
        ontology_engine::PropertyValue::Integer(i) => (*i).into(),
        ontology_engine::PropertyValue::Double(d) => serde_json::Number::from_f64(*d)
            .map(|n| n.into())
            .unwrap_or(serde_json::Value::Null),
        ontology_engine::PropertyValue::Boolean(b) => (*b).into(),
        ontology_engine::PropertyValue::Date(d) => d.clone().into(),
        ontology_engine::PropertyValue::DateTime(dt) => dt.clone().into(),
        ontology_engine::PropertyValue::ObjectReference(id) => id.clone().into(),
        ontology_engine::PropertyValue::GeoJSON(gj) => {
            // Parse GeoJSON string to validate, then return as JSON value
            serde_json::from_str(gj).unwrap_or_else(|_| serde_json::Value::String(gj.clone()))
        },
        ontology_engine::PropertyValue::Array(arr) => {
            let items: Vec<serde_json::Value> = arr.iter()
                .map(property_value_to_json)
                .collect();
            serde_json::Value::Array(items)
        },
        ontology_engine::PropertyValue::Map(map) => {
            let mut json_map = serde_json::Map::new();
            for (key, value) in map {
                json_map.insert(key.clone(), property_value_to_json(value));
            }
            serde_json::Value::Object(json_map)
        },
        ontology_engine::PropertyValue::Object(obj) => {
            let mut json_obj = serde_json::Map::new();
            for (key, value) in obj {
                json_obj.insert(key.clone(), property_value_to_json(value));
            }
            serde_json::Value::Object(json_obj)
        },
        ontology_engine::PropertyValue::Null => serde_json::Value::Null,
    }
}

