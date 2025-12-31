use indexing::store::{Filter, FilterOperator, SearchQuery, Aggregation, TraversalAggregation};
use ontology_engine::{PropertyValue, PropertyMap};

#[test]
fn test_filter_creation() {
    // Test filter creation and serialization
    let filter = Filter {
        property: "age".to_string(),
        operator: FilterOperator::GreaterThan,
        value: PropertyValue::Integer(18),
        distance: None,
    };
    
    assert_eq!(filter.property, "age");
    assert!(matches!(filter.operator, FilterOperator::GreaterThan));
    assert!(matches!(filter.value, PropertyValue::Integer(18)));
}

#[test]
fn test_search_query_creation() {
    let filter = Filter {
        property: "status".to_string(),
        operator: FilterOperator::Equals,
        value: PropertyValue::String("active".to_string()),
        distance: None,
    };
    
    let query = SearchQuery {
        filters: vec![filter],
        sort: None,
        limit: Some(10),
        offset: Some(0),
    };
    
    assert_eq!(query.filters.len(), 1);
    assert_eq!(query.limit, Some(10));
    assert_eq!(query.offset, Some(0));
}

#[test]
fn test_aggregation_enum() {
    // Test all aggregation types
    let count = Aggregation::Count;
    let sum = Aggregation::Sum("value".to_string());
    let avg = Aggregation::Avg("value".to_string());
    let min = Aggregation::Min("value".to_string());
    let max = Aggregation::Max("value".to_string());
    
    // Verify they can be created
    assert!(matches!(count, Aggregation::Count));
    assert!(matches!(sum, Aggregation::Sum(_)));
    assert!(matches!(avg, Aggregation::Avg(_)));
    assert!(matches!(min, Aggregation::Min(_)));
    assert!(matches!(max, Aggregation::Max(_)));
}

#[test]
fn test_traversal_aggregation() {
    let aggregation = TraversalAggregation {
        property: "score".to_string(),
        operation: Aggregation::Sum("score".to_string()),
        object_filters: vec![],
    };
    
    assert_eq!(aggregation.property, "score");
    assert!(matches!(aggregation.operation, Aggregation::Sum(_)));
    assert!(aggregation.object_filters.is_empty());
}

#[test]
fn test_property_map_operations() {
    let mut map = PropertyMap::new();
    
    map.insert("name".to_string(), PropertyValue::String("Test".to_string()));
    map.insert("age".to_string(), PropertyValue::Integer(25));
    map.insert("active".to_string(), PropertyValue::Boolean(true));
    
    assert_eq!(map.len(), 3);
    assert!(map.contains_key("name"));
    assert!(map.contains_key("age"));
    assert!(map.contains_key("active"));
    
    if let Some(PropertyValue::String(name)) = map.get("name") {
        assert_eq!(name, "Test");
    } else {
        panic!("Expected string value");
    }
}

#[test]
fn test_filter_operators() {
    // Test all filter operators
    let operators = vec![
        FilterOperator::Equals,
        FilterOperator::NotEquals,
        FilterOperator::GreaterThan,
        FilterOperator::LessThan,
        FilterOperator::GreaterThanOrEqual,
        FilterOperator::LessThanOrEqual,
        FilterOperator::Contains,
        FilterOperator::StartsWith,
        FilterOperator::EndsWith,
        FilterOperator::In,
        FilterOperator::WithinDistance,
    ];
    
    // Verify all operators can be created
    assert_eq!(operators.len(), 11);
}


