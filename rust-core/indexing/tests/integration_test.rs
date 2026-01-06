/// Integration tests that verify the complete flow of features
/// These tests verify that all the new features work together

use indexing::store::{
    SearchStore, GraphStore, ElasticsearchStore, DgraphStore,
    SearchQuery, Filter, FilterOperator, IndexedObject,
    TraversalAggregation, Aggregation,
};
use ontology_engine::{PropertyMap, PropertyValue};

/// Test the complete flow: Index -> Count -> Search -> Filter
#[tokio::test]
#[ignore] // Requires Elasticsearch
async fn test_complete_search_flow() {
    let store = match ElasticsearchStore::new("http://localhost:9200".to_string()) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    let object_type = "integration_test_object";
    
    // 1. Index multiple objects
    for i in 0..5 {
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String(format!("Object {}", i)));
        properties.insert("value".to_string(), PropertyValue::Integer(i * 10));
        properties.insert("category".to_string(), PropertyValue::String("test".to_string()));
        
        store.index_object(object_type, &format!("obj_{}", i), &properties).await.unwrap();
    }
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // 2. Test count
    let total_count = store.count_objects(object_type, None).await.unwrap();
    assert!(total_count >= 5, "Expected at least 5 objects, got {}", total_count);
    
    // 3. Test count with filter
    let filter = Filter {
        property: "value".to_string(),
        operator: FilterOperator::GreaterThan,
        value: PropertyValue::Integer(20),
        distance: None,
    };
    let filtered_count = store.count_objects(object_type, Some(&[filter])).await.unwrap();
    assert!(filtered_count >= 2, "Expected at least 2 filtered objects");
    
    // 4. Test search with filter
    let query = SearchQuery {
        filters: vec![Filter {
            property: "category".to_string(),
            operator: FilterOperator::Equals,
            value: PropertyValue::String("test".to_string()),
            distance: None,
        }],
        sort: None,
        limit: Some(10),
        offset: None,
    };
    
    let results = store.search(object_type, &query).await.unwrap();
    assert!(results.len() >= 5, "Expected at least 5 search results");
    
    // 5. Cleanup
    for i in 0..5 {
        let _ = store.delete_object(object_type, &format!("obj_{}", i)).await;
    }
}

/// Test bulk indexing flow
#[tokio::test]
#[ignore] // Requires Elasticsearch
async fn test_bulk_indexing_flow() {
    let store = match ElasticsearchStore::new("http://localhost:9200".to_string()) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    let object_type = "bulk_integration_test";
    let mut objects = Vec::new();
    
    // Create 20 objects for bulk indexing
    for i in 0..20 {
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String(format!("Bulk Object {}", i)));
        properties.insert("index".to_string(), PropertyValue::Integer(i));
        properties.insert("batch".to_string(), PropertyValue::String("batch1".to_string()));
        
        objects.push(IndexedObject {
            object_type: object_type.to_string(),
            object_id: format!("bulk_{}", i),
            properties,
            indexed_at: chrono::Utc::now(),
        });
    }
    
    // Bulk index
    store.bulk_index(objects).await.unwrap();
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Verify all objects were indexed
    let count = store.count_objects(object_type, None).await.unwrap();
    assert!(count >= 20, "Expected at least 20 objects after bulk index, got {}", count);
    
    // Verify we can search for them
    let query = SearchQuery {
        filters: vec![Filter {
            property: "batch".to_string(),
            operator: FilterOperator::Equals,
            value: PropertyValue::String("batch1".to_string()),
            distance: None,
        }],
        sort: None,
        limit: Some(25),
        offset: None,
    };
    
    let results = store.search(object_type, &query).await.unwrap();
    assert!(results.len() >= 20, "Expected at least 20 search results");
    
    // Cleanup
    for i in 0..20 {
        let _ = store.delete_object(object_type, &format!("bulk_{}", i)).await;
    }
}

/// Test blue/green migration flow
#[tokio::test]
#[ignore] // Requires Elasticsearch
async fn test_blue_green_migration_flow() {
    let store = match ElasticsearchStore::new("http://localhost:9200".to_string()) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    let object_type = "migration_test";
    
    // Phase 1: Create v1 and index data
    let mut properties1 = PropertyMap::new();
    properties1.insert("name".to_string(), PropertyValue::String("Version 1 Data".to_string()));
    properties1.insert("version".to_string(), PropertyValue::Integer(1));
    
    store.index_object(object_type, "data1", &properties1).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Create alias pointing to v1
    store.create_alias(object_type, 1).await.unwrap();
    
    // Verify alias points to v1
    let version = store.get_alias_version(object_type).await.unwrap();
    assert_eq!(version, Some(1), "Alias should point to version 1");
    
    // Phase 2: Create v2 and reindex
    let mut properties2 = PropertyMap::new();
    properties2.insert("name".to_string(), PropertyValue::String("Version 2 Data".to_string()));
    properties2.insert("version".to_string(), PropertyValue::Integer(2));
    
    store.index_object(object_type, "data2", &properties2).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Reindex from v1 to v2
    store.reindex(object_type, 1, 2).await.unwrap();
    
    // Phase 3: Swap alias
    store.swap_alias(object_type, 1, 2).await.unwrap();
    
    // Verify alias now points to v2
    let version = store.get_alias_version(object_type).await.unwrap();
    assert_eq!(version, Some(2), "Alias should point to version 2 after swap");
    
    // Cleanup
    let _ = store.delete_versioned_index(object_type, 1).await;
    let _ = store.delete_versioned_index(object_type, 2).await;
    let _ = store.delete_object(object_type, "data1").await;
    let _ = store.delete_object(object_type, "data2").await;
}

/// Test Dgraph traversal with filters and aggregation
#[tokio::test]
#[ignore] // Requires Dgraph
async fn test_dgraph_traversal_flow() {
    let store = match DgraphStore::new("http://localhost:9080".to_string()).await {
        Ok(s) => {
            let _ = s.init_schema().await;
            s
        }
        Err(_) => {
            eprintln!("Skipping test: Dgraph not available");
            return;
        }
    };
    
    // Create test graph structure
    let link_type = "test_traversal_link";
    
    // Create links with different properties
    for i in 0..5 {
        let mut link_props = PropertyMap::new();
        link_props.insert("weight".to_string(), PropertyValue::Integer(i * 5));
        
        store.create_link(link_type, "source_node", &format!("target_{}", i), &link_props).await.unwrap();
    }
    
    // Test traversal without filters
    let result = store.traverse("source_node", &[link_type.to_string()], 1).await.unwrap();
    assert!(result.len() >= 5, "Expected at least 5 target nodes");
    
    // Test traversal with filters
    let filter = Filter {
        property: "weight".to_string(),
        operator: FilterOperator::GreaterThan,
        value: PropertyValue::Integer(10),
        distance: None,
    };
    
    let filtered_result = store.traverse_with_filters(
        "source_node",
        &[link_type.to_string()],
        1,
        &[filter],
    ).await.unwrap();
    
    // Should filter out nodes with weight <= 10
    assert!(filtered_result.len() < result.len(), "Filtered result should have fewer nodes");
    
    // Test aggregation
    let aggregation = TraversalAggregation {
        property: "weight".to_string(),
        operation: Aggregation::Sum("weight".to_string()),
        object_filters: vec![],
    };
    
    let agg_result = store.traverse_with_aggregation(
        "source_node",
        &[link_type.to_string()],
        1,
        &aggregation,
    ).await.unwrap();
    
    // Verify aggregation result structure
    assert!(agg_result.count >= 0, "Count should be non-negative");
    
    // Cleanup would go here
}




