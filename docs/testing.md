# Testing Guide

This document describes the test suite for the ontology framework.

## Test Structure

### Unit Tests
- **Location**: `rust-core/*/tests/unit_test.rs` or inline `#[cfg(test)]` modules
- **Purpose**: Test individual functions and data structures
- **Requirements**: No external services required
- **Run**: `cargo test --package <package> --test unit_test`

### Integration Tests
- **Location**: `rust-core/*/tests/store_test.rs`, `rust-core/*/tests/integration_test.rs`
- **Purpose**: Test complete workflows and service integrations
- **Requirements**: May require Elasticsearch, Dgraph, or other services
- **Run**: `cargo test --package <package> --test <test_name>`
- **Note**: Tests marked with `#[ignore]` require external services

## Test Coverage

### Track 1: GraphQL Type Safety
- ✅ Function result caching
- ✅ Json types in GraphQL responses
- ✅ Count() method integration

### Track 2: Indexing Performance
- ✅ ElasticsearchStore::count_objects()
- ✅ ElasticsearchStore::bulk_index()
- ✅ DgraphStore::traverse_with_filters()
- ✅ DgraphStore::traverse_with_aggregation()

### Track 3: Blue/Green Migration
- ✅ ElasticsearchStore::create_alias()
- ✅ ElasticsearchStore::swap_alias()
- ✅ ElasticsearchStore::reindex()
- ✅ ElasticsearchStore::get_alias_version()

## Running Tests

### All Tests (Unit Only)
```bash
cargo test --lib
```

### Specific Package
```bash
cargo test --package indexing
cargo test --package graphql-api
```

### With External Services
```bash
# Start Elasticsearch and Dgraph first
docker-compose up -d

# Run integration tests
cargo test --package indexing --test store_test -- --ignored
cargo test --package indexing --test integration_test -- --ignored
```

### Unit Tests Only (No External Services)
```bash
cargo test --package indexing --test unit_test
```

## Test Environment Setup

### Elasticsearch
```bash
docker run -d -p 9200:9200 -p 9300:9300 \
  -e "discovery.type=single-node" \
  elasticsearch:8.11.0
```

### Dgraph
```bash
docker run -d -p 8080:8080 -p 9080:9080 \
  dgraph/standalone:v23.1.0
```

## Test Features

### Function Caching Tests
- Verify cache key generation
- Test cache hit/miss behavior
- Verify cacheable flag handling

### GraphQL Json Types Tests
- Verify properties are returned as Json objects (not strings)
- Test nested property structures
- Verify type safety

### Count Tests
- Test count without filters
- Test count with various filters
- Verify count accuracy

### Bulk Indexing Tests
- Test bulk indexing of multiple objects
- Verify all objects are indexed
- Test bulk indexing performance

### Filter Tests
- Test all filter operators
- Test filter combinations
- Test Dgraph filter translation

### Aggregation Tests
- Test all aggregation types (count, sum, avg, min, max)
- Test aggregation with filters
- Verify aggregation result structure

### Blue/Green Migration Tests
- Test alias creation
- Test alias swapping
- Test reindexing
- Verify zero-downtime migration

## Continuous Integration

Tests are designed to:
1. Run quickly in CI (unit tests)
2. Be skippable if services aren't available
3. Provide clear error messages when services are missing
4. Clean up test data after execution

## Writing New Tests

### Unit Test Template
```rust
#[test]
fn test_feature_name() {
    // Arrange
    let input = create_test_data();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

### Integration Test Template
```rust
#[tokio::test]
#[ignore] // Requires external service
async fn test_integration_feature() {
    let store = match create_test_store() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Skipping test: Service not available");
            return;
        }
    };
    
    // Test implementation
    // Cleanup
}
```

## Test Data Management

- Tests create their own test data
- Tests clean up after themselves
- Test object types use prefixes like `test_*` or `integration_test_*`
- Test indices are versioned for blue/green tests





