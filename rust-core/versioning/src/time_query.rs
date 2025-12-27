use crate::event_log::{EventLog, ObjectEvent};
use ontology_engine::PropertyMap;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Time query - query objects at a specific point in time
pub struct TimeQuery {
    event_log: EventLog,
}

/// Historical representation of an object
#[derive(Debug, Clone)]
pub struct HistoricalObject {
    pub object_type: String,
    pub object_id: String,
    pub properties: PropertyMap,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
    pub reconstructed_at: DateTime<Utc>,
}

/// Snapshot of the world at a specific time
pub struct Snapshot {
    pub timestamp: DateTime<Utc>,
    pub objects: HashMap<String, HistoricalObject>, // Key: "{object_type}:{object_id}"
}

impl TimeQuery {
    pub fn new(event_log: EventLog) -> Self {
        Self { event_log }
    }
    
    /// Reconstruct an object's state at a specific time
    pub fn reconstruct_object(
        &self,
        object_type: &str,
        object_id: &str,
        timestamp: DateTime<Utc>,
    ) -> Option<HistoricalObject> {
        // Get all events for this object up to the timestamp
        let events = self.event_log.get_object_events_at_time(
            object_type,
            object_id,
            timestamp,
        );
        
        if events.is_empty() {
            return None;
        }
        
        // Reconstruct properties by applying events in order
        let mut properties = PropertyMap::new();
        
        for event in &events {
            match &event.event_type {
                crate::event_log::EventType::ObjectCreated { properties: props, .. } => {
                    // Start with creation properties
                    for (key, value) in props.iter() {
                        properties.insert(key.clone(), value.clone());
                    }
                }
                crate::event_log::EventType::ObjectUpdated { changed_properties, .. } => {
                    // Apply updates
                    for (key, value) in changed_properties.iter() {
                        properties.insert(key.clone(), value.clone());
                    }
                }
                crate::event_log::EventType::PropertyChanged { property_name, new_value, .. } => {
                    // Apply property change
                    properties.insert(property_name.clone(), new_value.clone());
                }
                crate::event_log::EventType::ObjectDeleted { .. } => {
                    // Object was deleted, return None
                    return None;
                }
            }
        }
        
        // Find the valid_from and valid_to times
        let valid_from = events.first()
            .map(|e| e.valid_from)
            .unwrap_or(timestamp);
        let valid_to = events.iter()
            .find_map(|e| e.valid_to);
        
        Some(HistoricalObject {
            object_type: object_type.to_string(),
            object_id: object_id.to_string(),
            properties,
            valid_from,
            valid_to,
            reconstructed_at: Utc::now(),
        })
    }
    
    /// Create a snapshot of all objects at a specific time
    pub fn create_snapshot(
        &self,
        timestamp: DateTime<Utc>,
        object_types: &[String],
    ) -> Snapshot {
        let mut objects = HashMap::new();
        
        // Get all events at this time
        let events = self.event_log.get_events_at_time(timestamp);
        
        // Group events by object
        let mut object_events: HashMap<(String, String), Vec<&ObjectEvent>> = HashMap::new();
        for event in &events {
            let key = match &event.event_type {
                crate::event_log::EventType::ObjectCreated { object_type, object_id, .. } |
                crate::event_log::EventType::ObjectUpdated { object_type, object_id, .. } |
                crate::event_log::EventType::ObjectDeleted { object_type, object_id } |
                crate::event_log::EventType::PropertyChanged { object_type, object_id, .. } => {
                    (object_type.clone(), object_id.clone())
                }
            };
            
            object_events.entry(key).or_insert_with(Vec::new).push(event);
        }
        
        // Reconstruct each object
        // Note: We don't use _events here but need it for the iteration
        for ((object_type, object_id), _events) in object_events {
            if object_types.is_empty() || object_types.contains(&object_type) {
                if let Some(historical) = self.reconstruct_object(&object_type, &object_id, timestamp) {
                    let key = format!("{}:{}", object_type, object_id);
                    objects.insert(key, historical);
                }
            }
        }
        
        Snapshot {
            timestamp,
            objects,
        }
    }
    
    /// Reconstruct a graph of linked objects at a specific time
    pub fn reconstruct_graph(
        &self,
        start_object_type: &str,
        start_object_id: &str,
        _link_type_ids: &[String],
        _max_hops: usize,
        timestamp: DateTime<Utc>,
    ) -> Vec<HistoricalObject> {
        // This would require integration with the graph store's time-travel capabilities
        // For now, this is a placeholder that reconstructs just the start object
        if let Some(obj) = self.reconstruct_object(start_object_type, start_object_id, timestamp) {
            vec![obj]
        } else {
            vec![]
        }
    }
}

impl Snapshot {
    /// Get an object from the snapshot
    pub fn get_object(&self, object_type: &str, object_id: &str) -> Option<&HistoricalObject> {
        let key = format!("{}:{}", object_type, object_id);
        self.objects.get(&key)
    }
    
    /// Get all objects of a specific type
    pub fn get_objects_by_type(&self, object_type: &str) -> Vec<&HistoricalObject> {
        self.objects.values()
            .filter(|obj| obj.object_type == object_type)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontology_engine::PropertyValue;
    
    #[test]
    fn test_reconstruct_object() {
        let mut event_log = EventLog::new();
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String("test".to_string()));
        
        event_log.record_created("test_type".to_string(), "test_id".to_string(), properties, None);
        
        let time_query = TimeQuery::new(event_log);
        let timestamp = Utc::now();
        
        let reconstructed = time_query.reconstruct_object("test_type", "test_id", timestamp);
        assert!(reconstructed.is_some());
        let obj = reconstructed.unwrap();
        assert_eq!(obj.object_type, "test_type");
        assert_eq!(obj.object_id, "test_id");
    }
    
    #[test]
    fn test_create_snapshot() {
        let mut event_log = EventLog::new();
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String("test".to_string()));
        
        event_log.record_created("test_type".to_string(), "test_id".to_string(), properties, None);
        
        let time_query = TimeQuery::new(event_log);
        let timestamp = Utc::now();
        
        let snapshot = time_query.create_snapshot(timestamp, &[]);
        let obj = snapshot.get_object("test_type", "test_id");
        assert!(obj.is_some());
    }
}
