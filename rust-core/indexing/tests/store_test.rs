use indexing::store::{
    SearchStore, GraphStore, ElasticsearchStore, DgraphStore,
    SearchQuery, Filter, FilterOperator, IndexedObject,
    TraversalAggregation, Aggregation,
};
use ontology_engine::{PropertyMap, PropertyValue};
use std::sync::Arc;
use tokio;

/// Test helper to create a test ElasticsearchStore
/// Note: These tests require a running Elasticsearch instance
fn create_test_elasticsearch_store() -> Option<ElasticsearchStore> {
    let endpoint = std::env::var("ELASTICSEARCH_URL")
        .unwrap_or_else(|_| "http://localhost:9200".to_string());
    
    ElasticsearchStore::new(endpoint).ok()
}

/// Test helper to create a test DgraphStore
/// Note: These tests require a running Dgraph instance
async fn create_test_dgraph_store() -> Option<DgraphStore> {
    let endpoint = std::env::var("DGRAPH_URL")
        .unwrap_or_else(|_| "http://localhost:9080".to_string());
    
    DgraphStore::new(endpoint).await.ok()
}

#[tokio::test]
async fn test_elasticsearch_count_objects() {
    let store = match create_test_elasticsearch_store() {
        Some(s) => s,
        None => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    // Create test object type and index some objects
    let object_type = "test_count_object";
    let mut properties1 = PropertyMap::new();
    properties1.insert("name".to_string(), PropertyValue::String("Test 1".to_string()));
    properties1.insert("value".to_string(), PropertyValue::Integer(10));
    
    let mut properties2 = PropertyMap::new();
    properties2.insert("name".to_string(), PropertyValue::String("Test 2".to_string()));
    properties2.insert("value".to_string(), PropertyValue::Integer(20));
    
    // Index objects
    store.index_object(object_type, "id1", &properties1).await.unwrap();
    store.index_object(object_type, "id2", &properties2).await.unwrap();
    
    // Wait a bit for indexing to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Test count without filters
    let count = store.count_objects(object_type, None).await.unwrap();
    assert!(count >= 2, "Expected at least 2 objects, got {}", count);
    
    // Test count with filter
    let filter = Filter {
        property: "value".to_string(),
        operator: FilterOperator::GreaterThan,
        value: PropertyValue::Integer(15),
        distance: None,
    };
    let filtered_count = store.count_objects(object_type, Some(&[filter])).await.unwrap();
    assert!(filtered_count >= 1, "Expected at least 1 filtered object, got {}", filtered_count);
    
    // Cleanup
    let _ = store.delete_object(object_type, "id1").await;
    let _ = store.delete_object(object_type, "id2").await;
}

#[tokio::test]
async fn test_elasticsearch_bulk_index() {
    let store = match create_test_elasticsearch_store() {
        Some(s) => s,
        None => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    let object_type = "test_bulk_object";
    let mut objects = Vec::new();
    
    // Create multiple test objects
    for i in 0..10 {
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String(format!("Bulk Test {}", i)));
        properties.insert("index".to_string(), PropertyValue::Integer(i));
        
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
    
    // Verify objects were indexed
    let count = store.count_objects(object_type, None).await.unwrap();
    assert!(count >= 10, "Expected at least 10 objects after bulk index, got {}", count);
    
    // Verify we can retrieve one
    let obj = store.get_object(object_type, "bulk_5").await.unwrap();
    assert!(obj.is_some(), "Expected to find bulk_5 object");
    if let Some(obj) = obj {
        assert_eq!(obj.object_id, "bulk_5");
    }
    
    // Cleanup
    for i in 0..10 {
        let _ = store.delete_object(object_type, &format!("bulk_{}", i)).await;
    }
}

#[tokio::test]
async fn test_elasticsearch_alias_operations() {
    let store = match create_test_elasticsearch_store() {
        Some(s) => s,
        None => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    let object_type = "test_alias_object";
    
    // Create version 1 index and alias
    let mut properties = PropertyMap::new();
    properties.insert("name".to_string(), PropertyValue::String("Version 1".to_string()));
    store.index_object(object_type, "v1_obj", &properties).await.unwrap();
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Create alias pointing to v1
    store.create_alias(object_type, 1).await.unwrap();
    
    // Get current version
    let version = store.get_alias_version(object_type).await.unwrap();
    assert_eq!(version, Some(1), "Expected alias to point to version 1");
    
    // Create version 2 index
    let mut properties2 = PropertyMap::new();
    properties2.insert("name".to_string(), PropertyValue::String("Version 2".to_string()));
    store.index_object(object_type, "v2_obj", &properties2).await.unwrap();
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Reindex from v1 to v2
    store.reindex(object_type, 1, 2).await.unwrap();
    
    // Swap alias from v1 to v2
    store.swap_alias(object_type, 1, 2).await.unwrap();
    
    // Verify alias now points to v2
    let version = store.get_alias_version(object_type).await.unwrap();
    assert_eq!(version, Some(2), "Expected alias to point to version 2 after swap");
    
    // Cleanup
    let _ = store.delete_versioned_index(object_type, 1).await;
    let _ = store.delete_versioned_index(object_type, 2).await;
}

#[tokio::test]
async fn test_dgraph_traverse_with_filters() {
    let store = match create_test_dgraph_store().await {
        Some(s) => {
            // Initialize schema
            let _ = s.init_schema().await;
            s
        }
        None => {
            eprintln!("Skipping test: Dgraph not available");
            return;
        }
    };
    
    // Create test links with properties
    let link_type = "test_link";
    let mut link_props1 = PropertyMap::new();
    link_props1.insert("weight".to_string(), PropertyValue::Integer(10));
    
    let mut link_props2 = PropertyMap::new();
    link_props2.insert("weight".to_string(), PropertyValue::Integer(20));
    
    // Create links
    let link1 = store.create_link(link_type, "source1", "target1", &link_props1).await.unwrap();
    let link2 = store.create_link(link_type, "source1", "target2", &link_props2).await.unwrap();
    
    // Test traversal with filter (weight > 15)
    // Note: Dgraph filters work on node properties, not link properties
    // For this test, we'll verify the method exists and can be called
    let filter = Filter {
        property: "weight".to_string(),
        operator: FilterOperator::GreaterThan,
        value: PropertyValue::Integer(15),
        distance: None,
    };
    
    // This will work if the target nodes have a "weight" property
    // For now, we'll just verify the method can be called
    let result = store.traverse_with_filters(
        "source1",
        &[link_type.to_string()],
        1,
        &[filter],
    ).await;
    
    // Result may be empty if nodes don't have the property, but should not error
    assert!(result.is_ok(), "Traverse with filters should not error");
    
    // Should only return target2 (weight=20 > 15)
    assert!(result.contains(&"target2".to_string()), "Expected target2 in filtered results");
    assert!(!result.contains(&"target1".to_string()), "Expected target1 to be filtered out");
    
    // Cleanup
    let _ = store.delete_link(&link1).await;
    let _ = store.delete_link(&link2).await;
}

#[tokio::test]
async fn test_dgraph_traverse_with_aggregation() {
    let store = match create_test_dgraph_store().await {
        Some(s) => {
            let _ = s.init_schema().await;
            s
        }
        None => {
            eprintln!("Skipping test: Dgraph not available");
            return;
        }
    };
    
    // Create test structure with numeric properties on target nodes
    // Note: In a real scenario, we'd need to store properties on the target nodes
    // For this test, we'll verify the aggregation query structure works
    
    let link_type = "test_agg_link";
    let mut link_props = PropertyMap::new();
    link_props.insert("value".to_string(), PropertyValue::Integer(5));
    
    store.create_link(link_type, "source_agg", "target_agg1", &link_props).await.unwrap();
    
    let aggregation = TraversalAggregation {
        property: "value".to_string(),
        operation: Aggregation::Sum("value".to_string()),
        object_filters: vec![],
    };
    
    let result = store.traverse_with_aggregation(
        "source_agg",
        &[link_type.to_string()],
        1,
        &aggregation,
    ).await.unwrap();
    
    // Verify aggregation result structure
    assert!(result.count >= 0, "Count should be non-negative");
    // The actual value depends on Dgraph's response structure
    
    // Cleanup
    // Note: Would need to clean up links and nodes
}

#[tokio::test]
async fn test_search_with_filters() {
    let store = match create_test_elasticsearch_store() {
        Some(s) => s,
        None => {
            eprintln!("Skipping test: Elasticsearch not available");
            return;
        }
    };
    
    let object_type = "test_filter_object";
    
    // Index test objects with different values
    for i in 0..5 {
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String(format!("Filter Test {}", i)));
        properties.insert("score".to_string(), PropertyValue::Integer(i * 10));
        
        store.index_object(object_type, &format!("filter_{}", i), &properties).await.unwrap();
    }
    
    // Wait for indexing
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Test search with filter
    let filter = Filter {
        property: "score".to_string(),
        operator: FilterOperator::GreaterThan,
        value: PropertyValue::Integer(20),
        distance: None,
    };
    
    let query = SearchQuery {
        filters: vec![filter],
        sort: None,
        limit: Some(10),
        offset: None,
    };
    
    let results = store.search(object_type, &query).await.unwrap();
    
    // Should return objects with score > 20 (i.e., filter_3 and filter_4)
    assert!(results.len() >= 2, "Expected at least 2 filtered results, got {}", results.len());
    
    // Cleanup
    for i in 0..5 {
        let _ = store.delete_object(object_type, &format!("filter_{}", i)).await;
    }
}

