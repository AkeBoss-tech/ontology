# TODO Roadmap & Implementation Guide

This document outlines all TODO items in the codebase, organized by priority and implementation complexity.

## Summary by Category

### 🔴 High Priority - Core Store Implementations (17 TODOs)

These are the foundational implementations needed for the system to function.

#### 1. Elasticsearch Store (`rust-core/indexing/src/store.rs`)
- ✅ Structure exists, needs actual implementation
- **Dependencies needed**: `elasticsearch` crate
- **Methods to implement**:
  - [ ] `index_object` - Index a single object
  - [ ] `search` - Search with filters, sorting, pagination
  - [ ] `get_object` - Retrieve object by ID
  - [ ] `bulk_index` - Batch indexing for performance
  - [ ] `delete_object` - Remove object from index

#### 2. Dgraph Store (`rust-core/indexing/src/store.rs`)
- ✅ Structure exists, needs actual implementation
- **Dependencies needed**: Dgraph client (consider `reqwest` + JSON or `dgraph-rs`)
- **Methods to implement**:
  - [ ] `create_link` - Create graph relationships
  - [ ] `delete_link` - Remove relationships
  - [ ] `get_links` - Query links with filters/direction
  - [ ] `traverse` - Basic graph traversal
  - [ ] `get_connected_objects` - Get objects via link type
  - [ ] `traverse_with_filters` - Filtered traversal (partially implemented)
  - [ ] `traverse_with_aggregation` - Aggregation during traversal (placeholder)

#### 3. Parquet Store (`rust-core/indexing/src/store.rs`)
- ✅ Structure exists, needs actual implementation
- **Dependencies needed**: `arrow` or `polars` crate for Parquet handling
- **Methods to implement**:
  - [ ] `write_batch` - Write objects to Parquet files
  - [ ] `query_analytics` - Query with aggregations and filters

### 🟡 Medium Priority - GraphQL API Integration ✅ MOSTLY COMPLETE

These improve the GraphQL API functionality and type safety.

#### GraphQL Resolvers (`rust-core/graphql-api/src/resolvers.rs`)
- [x] Convert `FilterInput` to `Filter` (multiple locations)
- [ ] Implement caching for queries
- [x] Implement proper `PropertyValue` GraphQL type (currently JSON)
- [x] Implement proper `PropertyMap` GraphQL type (currently JSON)
- [x] Get actual count from search store instead of placeholder

#### Model Resolvers (`rust-core/graphql-api/src/model_resolvers.rs`) ✅ NEW
- [x] Model queries (list, get, compare, bindings)
- [x] Model mutations (register, bind, unbind, update, delete)
- [x] Model comparison endpoints
- [x] Prediction endpoint (placeholder for Python service)

### 🟢 Lower Priority - Advanced Features (5 TODOs)

These are enhancements that can be added incrementally.

#### Validation (`rust-core/ontology-engine/src/validation.rs`)
- [ ] Implement list type checking (2 locations)

#### Schema Migration (`rust-core/ontology-engine/src/dynamic.rs`)
- [ ] Implement blue/green schema migration for search store
- [ ] Implement blue/green schema migration for graph store
- [ ] Implement cleanup and data migration on object removal
- [ ] Implement schema update logic with blue/green indices

#### Pipeline (`python-pipeline/hydration/phonograph.py`)
- [ ] Implement Kafka streaming integration

---

## Recommended Implementation Order

### Phase 1: Elasticsearch Store (Start Here)
**Why**: Most straightforward to implement, well-documented Rust crates available, needed for search functionality.

1. Add `elasticsearch` dependency to `rust-core/indexing/Cargo.toml`
2. Implement `index_object` - simplest method, good starting point
3. Implement `get_object` - straightforward document retrieval
4. Implement `delete_object` - simple deletion
5. Implement `search` - more complex, requires query building
6. Implement `bulk_index` - performance optimization

### Phase 2: GraphQL Filter Conversion
**Why**: Unblocks query functionality, relatively simple parsing logic.

1. Implement `FilterInput` → `Filter` conversion in resolvers
2. Test with GraphQL queries

### Phase 3: Dgraph Store
**Why**: Enables graph traversal features, more complex but well-defined API.

1. Add Dgraph client dependency
2. Implement basic methods (`create_link`, `delete_link`, `get_links`)
3. Implement traversal methods
4. Implement filtered/aggregated traversal

### Phase 4: Parquet Store
**Why**: Needed for analytics, can use existing data for initial testing.

1. Add `arrow` or `polars` dependency
2. Implement `write_batch`
3. Implement `query_analytics`

### Phase 5: GraphQL Type Improvements
**Why**: Better type safety and developer experience.

1. Implement proper GraphQL types for `PropertyValue` and `PropertyMap`
2. Update resolvers to use new types

### Phase 6: Advanced Features
**Why**: Enhancements and optimizations.

1. Implement validation improvements
2. Implement caching
3. Implement schema migration strategies
4. Add Kafka streaming

---

## Getting Started: Step-by-Step Guide for Elasticsearch

### Step 1: Add Dependencies

Add to `rust-core/indexing/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
elasticsearch = "8.5"
url = "2.5"
```

### Step 2: Update ElasticsearchStore Structure

Store the actual client:

```rust
use elasticsearch::{Elasticsearch, http::transport::Transport};

pub struct ElasticsearchStore {
    client: Elasticsearch,
    index_prefix: String,
}

impl ElasticsearchStore {
    pub fn new(endpoint: String) -> Result<Self, StoreError> {
        let transport = Transport::single_node(&endpoint)?;
        let client = Elasticsearch::new(transport);
        Ok(Self {
            client,
            index_prefix: "ontology".to_string(),
        })
    }
    
    fn index_name(&self, object_type: &str) -> String {
        format!("{}_{}", self.index_prefix, object_type)
    }
}
```

### Step 3: Implement Methods

Start with `index_object`:

```rust
async fn index_object(
    &self,
    object_type: &str,
    object_id: &str,
    properties: &PropertyMap,
) -> Result<(), StoreError> {
    let index = self.index_name(object_type);
    let response = self.client
        .index(IndexParts::IndexId(&index, object_id))
        .body(properties)  // Serialize PropertyMap to JSON
        .send()
        .await
        .map_err(|e| StoreError::Connection(e.to_string()))?;
    
    // Handle response, check for errors
    Ok(())
}
```

### Step 4: Testing

1. Start local Elasticsearch: `docker run -p 9200:9200 -e "discovery.type=single-node" docker.elastic.co/elasticsearch/elasticsearch:8.5.0`
2. Write unit tests using a test Elasticsearch instance
3. Test each method incrementally

---

## Resources

- [Elasticsearch Rust Client Docs](https://docs.rs/elasticsearch/)
- [Dgraph Documentation](https://dgraph.io/docs/)
- [Arrow Rust Docs](https://docs.rs/arrow/)
- [Polars Documentation](https://pola-rs.github.io/polars-book/)

---

## Notes

- All store implementations are async - use `async/await` throughout
- PropertyMap needs to be serialized to JSON for Elasticsearch
- Consider indexing strategy: one index per object type vs. single index with type field
- Error handling should map library errors to `StoreError` enum
- Consider connection pooling and retry logic for production use






