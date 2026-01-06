use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Usage metrics for an object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectUsageMetrics {
    pub object_type: String,
    pub object_id: String,
    
    // Query frequency
    pub query_count: usize,
    pub last_queried: Option<DateTime<Utc>>,
    pub query_frequency: f64, // queries per day
    
    // Access patterns
    pub accessed_by_users: Vec<String>,
    pub access_count_by_user: HashMap<String, usize>,
    
    // Edit frequency
    pub edit_count: usize,
    pub last_edited: Option<DateTime<Utc>>,
    
    // Link traversal
    pub traversed_count: usize,
    pub traversed_as_source: usize,
    pub traversed_as_target: usize,
    
    // Time range
    pub first_accessed: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
}

impl ObjectUsageMetrics {
    pub fn new(object_type: String, object_id: String) -> Self {
        Self {
            object_type,
            object_id,
            query_count: 0,
            last_queried: None,
            query_frequency: 0.0,
            accessed_by_users: Vec::new(),
            access_count_by_user: HashMap::new(),
            edit_count: 0,
            last_edited: None,
            traversed_count: 0,
            traversed_as_source: 0,
            traversed_as_target: 0,
            first_accessed: None,
            last_accessed: None,
        }
    }
    
    /// Record a query access
    pub fn record_query(&mut self, user_id: Option<String>) {
        let now = Utc::now();
        
        self.query_count += 1;
        self.last_queried = Some(now);
        
        if let Some(user) = user_id {
            if !self.accessed_by_users.contains(&user) {
                self.accessed_by_users.push(user.clone());
            }
            *self.access_count_by_user.entry(user).or_insert(0) += 1;
        }
        
        if self.first_accessed.is_none() {
            self.first_accessed = Some(now);
        }
        self.last_accessed = Some(now);
        
        // Update query frequency
        self.update_query_frequency();
    }
    
    /// Record an edit
    pub fn record_edit(&mut self) {
        self.edit_count += 1;
        self.last_edited = Some(Utc::now());
    }
    
    /// Record a traversal
    pub fn record_traversal(&mut self, as_source: bool) {
        self.traversed_count += 1;
        if as_source {
            self.traversed_as_source += 1;
        } else {
            self.traversed_as_target += 1;
        }
    }
    
    /// Update query frequency based on time since first access
    fn update_query_frequency(&mut self) {
        if let (Some(first), Some(last)) = (self.first_accessed, self.last_accessed) {
            let days = (last - first).num_seconds() as f64 / 86400.0;
            if days > 0.0 {
                self.query_frequency = self.query_count as f64 / days;
            }
        }
    }
}

/// Usage metrics aggregator
pub struct UsageTracker {
    metrics: HashMap<String, ObjectUsageMetrics>, // Key: "{object_type}:{object_id}"
}

impl UsageTracker {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }
    
    /// Get or create metrics for an object
    fn get_or_create_metrics(&mut self, object_type: &str, object_id: &str) -> &mut ObjectUsageMetrics {
        let key = format!("{}:{}", object_type, object_id);
        self.metrics.entry(key).or_insert_with(|| {
            ObjectUsageMetrics::new(object_type.to_string(), object_id.to_string())
        })
    }
    
    /// Record a query
    pub fn record_query(&mut self, object_type: &str, object_id: &str, user_id: Option<String>) {
        let metrics = self.get_or_create_metrics(object_type, object_id);
        metrics.record_query(user_id);
    }
    
    /// Record an edit
    pub fn record_edit(&mut self, object_type: &str, object_id: &str) {
        let metrics = self.get_or_create_metrics(object_type, object_id);
        metrics.record_edit();
    }
    
    /// Record a traversal
    pub fn record_traversal(&mut self, object_type: &str, object_id: &str, as_source: bool) {
        let metrics = self.get_or_create_metrics(object_type, object_id);
        metrics.record_traversal(as_source);
    }
    
    /// Get metrics for an object
    pub fn get_metrics(&self, object_type: &str, object_id: &str) -> Option<&ObjectUsageMetrics> {
        let key = format!("{}:{}", object_type, object_id);
        self.metrics.get(&key)
    }
    
    /// Get all metrics for an object type
    pub fn get_metrics_by_type(&self, object_type: &str) -> Vec<&ObjectUsageMetrics> {
        self.metrics.values()
            .filter(|m| m.object_type == object_type)
            .collect()
    }
}

impl Default for UsageTracker {
    fn default() -> Self {
        Self::new()
    }
}


