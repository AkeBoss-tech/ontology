use ontology_engine::PropertyMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Event log for tracking all changes to objects (event sourcing)
pub struct EventLog {
    events: Vec<ObjectEvent>,
    // In production, this would be a persistent store (database, event stream, etc.)
}

/// Event types that can occur on objects
#[derive(Debug, Clone)]
pub enum EventType {
    ObjectCreated {
        object_type: String,
        object_id: String,
        properties: PropertyMap,
    },
    ObjectUpdated {
        object_type: String,
        object_id: String,
        changed_properties: PropertyMap,
    },
    ObjectDeleted {
        object_type: String,
        object_id: String,
    },
    PropertyChanged {
        object_type: String,
        object_id: String,
        property_name: String,
        old_value: Option<ontology_engine::PropertyValue>,
        new_value: ontology_engine::PropertyValue,
    },
}

/// An event in the log
#[derive(Debug, Clone)]
pub struct ObjectEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>, // None means still valid
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    /// Record an event
    pub fn record(&mut self, event: ObjectEvent) {
        self.events.push(event);
    }
    
    /// Record an object creation event
    pub fn record_created(
        &mut self,
        object_type: String,
        object_id: String,
        properties: PropertyMap,
        user_id: Option<String>,
    ) {
        let event = ObjectEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: EventType::ObjectCreated {
                object_type,
                object_id,
                properties,
            },
            timestamp: Utc::now(),
            user_id,
            valid_from: Utc::now(),
            valid_to: None,
        };
        self.record(event);
    }
    
    /// Record an object update event
    pub fn record_updated(
        &mut self,
        object_type: String,
        object_id: String,
        changed_properties: PropertyMap,
        user_id: Option<String>,
    ) {
        // Invalidate previous events for these properties
        self.invalidate_properties(&object_type, &object_id, &changed_properties);
        
        let event = ObjectEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: EventType::ObjectUpdated {
                object_type,
                object_id,
                changed_properties,
            },
            timestamp: Utc::now(),
            user_id,
            valid_from: Utc::now(),
            valid_to: None,
        };
        self.record(event);
    }
    
    /// Invalidate previous events for properties that are being updated
    fn invalidate_properties(
        &mut self,
        object_type: &str,
        object_id: &str,
        changed_properties: &PropertyMap,
    ) {
        let now = Utc::now();
        for event in &mut self.events {
            if let Some(valid_to) = &mut event.valid_to {
                if *valid_to == DateTime::<Utc>::MAX_UTC {
                    // Check if this event affects any of the changed properties
                    let should_invalidate = match &event.event_type {
                        EventType::ObjectCreated { .. } => true,
                        EventType::ObjectUpdated { changed_properties: old_changes, .. } => {
                            changed_properties.iter().any(|(key, _)| old_changes.contains_key(key))
                        }
                        EventType::PropertyChanged { property_name, .. } => {
                            changed_properties.contains_key(property_name)
                        }
                        _ => false,
                    };
                    
                    if should_invalidate {
                        *valid_to = now;
                    }
                }
            }
        }
    }
    
    /// Get all events for an object
    pub fn get_events_for_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Vec<&ObjectEvent> {
        self.events.iter()
            .filter(|e| match &e.event_type {
                EventType::ObjectCreated { object_type: ot, object_id: oid, .. } |
                EventType::ObjectUpdated { object_type: ot, object_id: oid, .. } |
                EventType::ObjectDeleted { object_type: ot, object_id: oid } |
                EventType::PropertyChanged { object_type: ot, object_id: oid, .. } => {
                    ot == object_type && oid == object_id
                }
            })
            .collect()
    }
    
    /// Get events valid at a specific time
    pub fn get_events_at_time(
        &self,
        timestamp: DateTime<Utc>,
    ) -> Vec<&ObjectEvent> {
        self.events.iter()
            .filter(|e| {
                e.valid_from <= timestamp &&
                e.valid_to.map_or(true, |to| to > timestamp)
            })
            .collect()
    }
    
    /// Get events for an object at a specific time
    pub fn get_object_events_at_time(
        &self,
        object_type: &str,
        object_id: &str,
        timestamp: DateTime<Utc>,
    ) -> Vec<&ObjectEvent> {
        self.get_events_for_object(object_type, object_id)
            .into_iter()
            .filter(|e| {
                e.valid_from <= timestamp &&
                e.valid_to.map_or(true, |to| to > timestamp)
            })
            .collect()
    }
}

impl Default for EventLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontology_engine::PropertyValue;
    
    #[test]
    fn test_event_log_creation() {
        let log = EventLog::new();
        assert_eq!(log.events.len(), 0);
    }
    
    #[test]
    fn test_record_created() {
        let mut log = EventLog::new();
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String("test".to_string()));
        
        log.record_created("test_type".to_string(), "test_id".to_string(), properties, Some("user1".to_string()));
        
        assert_eq!(log.events.len(), 1);
        let events = log.get_events_for_object("test_type", "test_id");
        assert_eq!(events.len(), 1);
    }
    
    #[test]
    fn test_record_updated() {
        let mut log = EventLog::new();
        let mut props = PropertyMap::new();
        props.insert("name".to_string(), PropertyValue::String("test".to_string()));
        log.record_created("test_type".to_string(), "test_id".to_string(), props.clone(), None);
        
        let mut updated_props = PropertyMap::new();
        updated_props.insert("name".to_string(), PropertyValue::String("updated".to_string()));
        log.record_updated("test_type".to_string(), "test_id".to_string(), updated_props, None);
        
        assert_eq!(log.events.len(), 2);
    }
    
    #[test]
    fn test_get_events_at_time() {
        let mut log = EventLog::new();
        let props = PropertyMap::new();
        let before = Utc::now();
        
        log.record_created("test_type".to_string(), "test_id".to_string(), props, None);
        
        let after = Utc::now();
        let events = log.get_events_at_time(after);
        assert!(!events.is_empty());
        
        let events_before = log.get_events_at_time(before - chrono::Duration::seconds(1));
        assert!(events_before.is_empty());
    }
}
