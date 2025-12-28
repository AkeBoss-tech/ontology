use ontology_engine::crosswalk::{CrosswalkTraverser, CrosswalkLink};
use std::collections::HashMap;

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
    let total: f64 = result.iter().map(|(_, v)| v).sum();
    assert!((total - 1000.0).abs() < 0.01);
}




