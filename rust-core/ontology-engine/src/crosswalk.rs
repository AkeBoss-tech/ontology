use crate::property::{PropertyValue, PropertyMap};
use std::collections::HashMap;

/// Crosswalk traverser - handles boundary normalization using crosswalk objects
pub struct CrosswalkTraverser;

/// Crosswalk link information
#[derive(Debug, Clone)]
pub struct CrosswalkLink {
    pub source_tract_id: String,
    pub target_tract_id: String,
    pub source_year: i64,
    pub target_year: i64,
    pub overlap_percentage: f64,
    pub allocation_factor: Option<f64>,
}

impl CrosswalkTraverser {
    /// Normalize data from source vintage to target vintage using crosswalk links
    pub fn normalize_boundaries(
        source_tract_id: &str,
        source_year: i64,
        target_year: i64,
        source_value: f64,
        crosswalk_links: &[CrosswalkLink],
    ) -> Result<Vec<(String, f64)>, String> {
        // Find all crosswalk links for this source tract
        let relevant_links: Vec<&CrosswalkLink> = crosswalk_links
            .iter()
            .filter(|link| {
                link.source_tract_id == source_tract_id
                    && link.source_year == source_year
                    && link.target_year == target_year
            })
            .collect();
        
        if relevant_links.is_empty() {
            return Err(format!(
                "No crosswalk links found from {} (year {}) to year {}",
                source_tract_id, source_year, target_year
            ));
        }
        
        // Distribute value proportionally based on overlap_percentage
        let mut results = Vec::new();
        let total_overlap: f64 = relevant_links.iter().map(|l| l.overlap_percentage).sum();
        
        if total_overlap == 0.0 {
            return Err("Total overlap percentage is zero".to_string());
        }
        
        for link in relevant_links {
            // Calculate allocation factor (use provided or calculate from overlap)
            let factor = link.allocation_factor.unwrap_or_else(|| {
                link.overlap_percentage / total_overlap
            });
            
            // Allocate value proportionally
            let allocated_value = source_value * factor;
            
            results.push((link.target_tract_id.clone(), allocated_value));
        }
        
        Ok(results)
    }
    
    /// Aggregate data from multiple source tracts to target tracts
    pub fn aggregate_to_target(
        source_data: &HashMap<String, f64>, // source_tract_id -> value
        source_year: i64,
        target_year: i64,
        crosswalk_links: &[CrosswalkLink],
    ) -> Result<HashMap<String, f64>, String> {
        let mut target_data: HashMap<String, f64> = HashMap::new();
        
        // For each source tract, distribute its value to target tracts
        for (source_id, value) in source_data {
            let allocations = Self::normalize_boundaries(
                source_id,
                source_year,
                target_year,
                *value,
                crosswalk_links,
            )?;
            
            // Accumulate values for each target tract
            for (target_id, allocated_value) in allocations {
                *target_data.entry(target_id).or_insert(0.0) += allocated_value;
            }
        }
        
        Ok(target_data)
    }
    
    /// Create crosswalk link from properties (helper for parsing from object properties)
    pub fn link_from_properties(properties: &PropertyMap) -> Result<CrosswalkLink, String> {
        let source_tract_id = properties.get("source_tract_id")
            .and_then(|v| {
                if let PropertyValue::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing source_tract_id".to_string())?;
        
        let target_tract_id = properties.get("target_tract_id")
            .and_then(|v| {
                if let PropertyValue::String(s) = v {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing target_tract_id".to_string())?;
        
        let source_year = properties.get("source_year")
            .and_then(|v| {
                if let PropertyValue::Integer(i) = v {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing source_year".to_string())?;
        
        let target_year = properties.get("target_year")
            .and_then(|v| {
                if let PropertyValue::Integer(i) = v {
                    Some(*i)
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing target_year".to_string())?;
        
        let overlap_percentage = properties.get("overlap_percentage")
            .and_then(|v| {
                if let PropertyValue::Double(d) = v {
                    Some(*d)
                } else if let PropertyValue::Integer(i) = v {
                    Some(*i as f64)
                } else {
                    None
                }
            })
            .ok_or_else(|| "Missing overlap_percentage".to_string())?;
        
        let allocation_factor = properties.get("allocation_factor")
            .and_then(|v| {
                if let PropertyValue::Double(d) = v {
                    Some(*d)
                } else if let PropertyValue::Integer(i) = v {
                    Some(*i as f64)
                } else {
                    None
                }
            });
        
        Ok(CrosswalkLink {
            source_tract_id,
            target_tract_id,
            source_year,
            target_year,
            overlap_percentage,
            allocation_factor,
        })
    }
    
    /// Interpolate data between years using crosswalks
    pub fn interpolate_between_years(
        source_data: &HashMap<String, f64>,
        source_year: i64,
        target_year: i64,
        crosswalk_links: &[CrosswalkLink],
    ) -> Result<HashMap<String, f64>, String> {
        // Use aggregation to normalize boundaries
        Self::aggregate_to_target(source_data, source_year, target_year, crosswalk_links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_normalize_boundaries() {
        let links = vec![
            CrosswalkLink {
                source_tract_id: "tract1".to_string(),
                target_tract_id: "tract2".to_string(),
                source_year: 1990,
                target_year: 2010,
                overlap_percentage: 0.6,
                allocation_factor: None,
            },
            CrosswalkLink {
                source_tract_id: "tract1".to_string(),
                target_tract_id: "tract3".to_string(),
                source_year: 1990,
                target_year: 2010,
                overlap_percentage: 0.4,
                allocation_factor: None,
            },
        ];
        
        let result = CrosswalkTraverser::normalize_boundaries(
            "tract1",
            1990,
            2010,
            1000.0,
            &links,
        ).unwrap();
        
        assert_eq!(result.len(), 2);
        // Check that values sum to approximately 1000 (allowing for floating point)
        let total: f64 = result.iter().map(|(_, v)| v).sum();
        assert!((total - 1000.0).abs() < 0.01);
    }
    
    #[test]
    fn test_aggregate_to_target() {
        let mut source_data = HashMap::new();
        source_data.insert("tract1".to_string(), 1000.0);
        source_data.insert("tract2".to_string(), 500.0);
        
        let links = vec![
            CrosswalkLink {
                source_tract_id: "tract1".to_string(),
                target_tract_id: "tract4".to_string(),
                source_year: 1990,
                target_year: 2010,
                overlap_percentage: 1.0,
                allocation_factor: None,
            },
            CrosswalkLink {
                source_tract_id: "tract2".to_string(),
                target_tract_id: "tract4".to_string(),
                source_year: 1990,
                target_year: 2010,
                overlap_percentage: 1.0,
                allocation_factor: None,
            },
        ];
        
        let result = CrosswalkTraverser::aggregate_to_target(
            &source_data,
            1990,
            2010,
            &links,
        ).unwrap();
        
        assert_eq!(result.get("tract4"), Some(&1500.0));
    }
}




