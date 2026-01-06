use ontology_engine::PropertyValue;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Data quality metrics for a property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityMetrics {
    pub object_type: String,
    pub property_id: String,
    
    // Completeness
    pub null_count: usize,
    pub null_percentage: f64,
    pub empty_count: usize,
    
    // Validity
    pub invalid_count: usize,
    pub validation_errors: Vec<String>,
    
    // Consistency
    pub duplicate_count: usize,
    pub uniqueness_ratio: f64,
    
    // Accuracy (if ground truth available)
    pub accuracy_score: Option<f64>,
    
    // Freshness
    pub last_updated: Option<DateTime<Utc>>,
    pub staleness_days: Option<i64>,
    
    // Distribution
    pub value_distribution: HashMap<String, usize>, // For categorical
    pub min_value: Option<PropertyValue>,
    pub max_value: Option<PropertyValue>,
    
    // Computed at
    pub computed_at: DateTime<Utc>,
}

impl DataQualityMetrics {
    pub fn new(object_type: String, property_id: String) -> Self {
        Self {
            object_type,
            property_id,
            null_count: 0,
            null_percentage: 0.0,
            empty_count: 0,
            invalid_count: 0,
            validation_errors: Vec::new(),
            duplicate_count: 0,
            uniqueness_ratio: 1.0,
            accuracy_score: None,
            last_updated: None,
            staleness_days: None,
            value_distribution: HashMap::new(),
            min_value: None,
            max_value: None,
            computed_at: Utc::now(),
        }
    }
    
    /// Compute overall quality score (0.0 to 1.0)
    pub fn quality_score(&self) -> f64 {
        let mut score = 1.0;
        
        // Penalize for nulls
        score -= self.null_percentage * 0.3;
        
        // Penalize for invalids
        if self.invalid_count > 0 {
            score -= 0.2;
        }
        
        // Penalize for duplicates (if uniqueness is important)
        if self.uniqueness_ratio < 0.9 {
            score -= (1.0 - self.uniqueness_ratio) * 0.2;
        }
        
        // Penalize for staleness
        if let Some(days) = self.staleness_days {
            if days > 30 {
                score -= 0.1;
            } else if days > 7 {
                score -= 0.05;
            }
        }
        
        score.max(0.0).min(1.0)
    }
}

/// Data quality metrics for an entire object type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTypeQualityMetrics {
    pub object_type: String,
    pub property_metrics: HashMap<String, DataQualityMetrics>,
    pub overall_score: f64,
    pub computed_at: DateTime<Utc>,
}

impl ObjectTypeQualityMetrics {
    pub fn new(object_type: String) -> Self {
        Self {
            object_type,
            property_metrics: HashMap::new(),
            overall_score: 1.0,
            computed_at: Utc::now(),
        }
    }
    
    /// Compute overall quality score as average of property scores
    pub fn compute_overall_score(&mut self) {
        if self.property_metrics.is_empty() {
            self.overall_score = 1.0;
            return;
        }
        
        let sum: f64 = self.property_metrics.values()
            .map(|m| m.quality_score())
            .sum();
        
        self.overall_score = sum / self.property_metrics.len() as f64;
    }
}


