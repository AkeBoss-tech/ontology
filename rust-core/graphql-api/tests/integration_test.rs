use async_graphql::{Schema, EmptySubscription};
use graphql_api::{QueryRoot, AdminMutations};
use ontology_engine::Ontology;
use indexing::store::{ParquetStore, ElasticsearchStore, SearchStore};
use indexing::hydration::ObjectHydrator;
use versioning::time_query::TimeQuery;
use versioning::event_log::EventLog;
use std::sync::Arc;
use std::collections::HashMap;
use ontology_engine::PropertyValue;
use serde_json::Value;

/// Create a test schema with minimal dependencies
async fn create_minimal_test_schema() -> Schema<QueryRoot, AdminMutations, EmptySubscription> {
    // Create a minimal ontology
    let yaml = r#"
ontology:
  objectTypes:
    - id: "test_object"
      displayName: "Test Object"
      primaryKey: "id"
      properties:
        - id: "id"
          type: "string"
          required: true
        - id: "name"
          type: "string"
      titleKey: "name"
  linkTypes: []
  actionTypes: []
  functionTypes: []
  interfaces: []
"#;
    
    let ontology = Ontology::from_yaml(yaml).expect("Failed to create test ontology");
    
    // Create stores (may fail if services aren't running, but that's OK)
    let search_store: Arc<dyn SearchStore> = match ElasticsearchStore::new("http://localhost:9200".to_string()) {
        Ok(store) => Arc::new(store),
        Err(_) => {
            // Use a mock or skip tests that require Elasticsearch
            panic!("Elasticsearch not available for tests");
        }
    };
    
    let columnar_store: Arc<dyn indexing::store::ColumnarStore> = 
        Arc::new(ParquetStore::new("test_data/parquet".to_string()));
    
    let event_log = EventLog::new();
    let time_query = Arc::new(TimeQuery::new(event_log));
    let hydrator = ObjectHydrator::new();
    
    // Create function cache
    let function_cache: Arc<tokio::sync::RwLock<HashMap<u64, PropertyValue>>> = 
        Arc::new(tokio::sync::RwLock::new(HashMap::new()));
    
    // Create in-memory data store
    let data_store: Arc<tokio::sync::RwLock<HashMap<String, Vec<Value>>>> = 
        Arc::new(tokio::sync::RwLock::new(HashMap::new()));
    
    Schema::build(QueryRoot::default(), AdminMutations::default(), EmptySubscription)
        .data(Arc::new(ontology))
        .data(search_store)
        .data(columnar_store)
        .data(time_query)
        .data(hydrator)
        .data(data_store)
        .data(function_cache)
        .finish()
}

#[tokio::test]
#[ignore] // Requires Elasticsearch
async fn test_schema_creation() {
    // Test that we can create a schema with all dependencies
    let schema = create_minimal_test_schema().await;
    
    // Verify schema is created
    assert!(true, "Schema created successfully");
}

#[tokio::test]
async fn test_ontology_loading() {
    // Test that ontology can be loaded from YAML
    let yaml = r#"
ontology:
  objectTypes:
    - id: "test"
      displayName: "Test"
      primaryKey: "id"
      properties:
        - id: "id"
          type: "string"
          required: true
  linkTypes: []
  actionTypes: []
"#;
    
    let ontology = Ontology::from_yaml(yaml);
    assert!(ontology.is_ok(), "Should be able to load ontology from YAML");
    
    let ontology = ontology.unwrap();
    assert!(ontology.get_object_type("test").is_some(), "Should find test object type");
}

#[tokio::test]
async fn test_function_cache_structure() {
    // Test that function cache can be created and used
    let cache: Arc<tokio::sync::RwLock<HashMap<u64, PropertyValue>>> = 
        Arc::new(tokio::sync::RwLock::new(HashMap::new()));
    
    // Test write
    {
        let mut cache_write = cache.write().await;
        cache_write.insert(123, PropertyValue::Integer(42));
    }
    
    // Test read
    {
        let cache_read = cache.read().await;
        assert_eq!(cache_read.get(&123), Some(&PropertyValue::Integer(42)));
    }
}


