# Missing Features for Census Ontology Implementation

This document tracks features needed to fully support the Census Ontology use case.

## Critical Missing Features

### 1. Geospatial Data Support
**Status:** ❌ Not Implemented  
**Priority:** 🔴 Critical  
**Description:** The ontology needs to store and query GeoJSON polygons for map visualization.

**What's Needed:**
- Property type for GeoJSON (currently using `string`)
- Spatial indexing in the search store (e.g., Elasticsearch geo_shape)
- Spatial queries (contains, intersects, within, etc.)
- Coordinate system handling (WGS84, Web Mercator)

**Implementation Notes:**
- Consider adding `GeoJSON` property type or extending `string` with validation
- Update indexing layer to support spatial indices
- Add spatial query operators to SearchQuery

---

### 2. Temporal Query Support
**Status:** ⚠️ Partially Implemented  
**Priority:** 🔴 Critical  
**Description:** Need to query objects by year/vintage and perform time-series analysis.

**What's Needed:**
- Time-based filtering in GraphQL API
- Year/vintage property indexing
- Time-series aggregation queries
- Support for "as of date" queries

**Current State:**
- Versioning system exists but is for object change history, not temporal data
- Need temporal indexing strategy for vintage-specific objects

---

### 3. Crosswalk/Aggregation Operations
**Status:** ❌ Not Implemented  
**Priority:** 🔴 Critical  
**Description:** Need to aggregate data from one vintage to another using crosswalk objects.

**What's Needed:**
- Crosswalk traversal logic (source_tract -> target_tract via crosswalk)
- Weighted aggregation based on overlap_percentage
- Action type implementation for boundary normalization
- Data interpolation algorithms

**Example Use Case:**
- User wants 1990 tract data normalized to 2010 boundaries
- System uses crosswalk links to distribute values proportionally

---

### 4. Graph Traversal Queries
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟡 High  
**Description:** Need complex multi-hop graph queries for PUMS analysis.

**What's Needed:**
- Multi-hop graph traversal (e.g., Tract -> PUMA -> Household -> Person)
- Filtered traversal (only follow links with specific properties)
- Aggregation during traversal (count persons in tract)
- Path queries (find all paths between two objects)

**Current State:**
- Basic get_connected_objects exists
- Need more sophisticated traversal API

---

### 5. Property Type: Object Reference
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟡 High  
**Description:** Object references exist but need validation and traversal support.

**What's Needed:**
- Validation that referenced object exists
- Automatic link creation when setting object reference
- Reverse reference lookups
- Cascade delete behavior

---

### 6. Aggregation Queries
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟡 High  
**Description:** Need to aggregate properties across linked objects.

**What's Needed:**
- SUM, AVG, COUNT, MIN, MAX aggregations
- Group by properties
- Aggregation with filters
- GraphQL aggregation resolvers

**Current State:**
- Columnar store has aggregation support (placeholder)
- Need GraphQL API for aggregations

---

### 7. Map Visualization Support
**Status:** ❌ Not Implemented  
**Priority:** 🟡 High  
**Description:** Need API endpoints and data format for map rendering.

**What's Needed:**
- GeoJSON endpoint for objects with geoshape properties
- Choropleth data endpoint (value per feature)
- Time slider data endpoint
- Map bounds queries

**Implementation Options:**
- Add GraphQL query for geo features
- Create REST endpoint for GeoJSON
- Support map tile generation

---

### 8. Census API Integration
**Status:** ❌ Not Implemented  
**Priority:** 🟡 High  
**Description:** Need Python scripts to ingest data from Census APIs.

**What's Needed:**
- Census Data API client (ACS, Decennial)
- PUMS Microdata API client
- TIGERweb REST API client for shapes
- Crosswalk file ingestion (MCDC)
- Batch ingestion pipeline
- Error handling and retry logic

**Files to Create:**
- `scripts/ingest_census_data.py`
- `scripts/ingest_pums_data.py`
- `scripts/ingest_geography.py`
- `scripts/ingest_crosswalks.py`

---

### 9. Time Slider / Temporal Playback
**Status:** ❌ Not Implemented  
**Priority:** 🟢 Medium  
**Description:** Need API to support time slider in UI.

**What's Needed:**
- Query objects by year range
- Available years endpoint (for slider bounds)
- Temporal playback data format
- Change detection between years

---

### 10. Cohort Builder / Faceted Search
**Status:** ❌ Not Implemented  
**Priority:** 🟢 Medium  
**Description:** Need complex filtering for PUMS Person objects.

**What's Needed:**
- Multi-filter queries (AND/OR logic)
- Faceted search results (counts per filter option)
- Histogram/statistics generation
- Export cohort data

**Current State:**
- Basic filtering exists but not faceted

---

### 11. Harmonization Visualization
**Status:** ❌ Not Implemented  
**Priority:** 🟢 Medium  
**Description:** Need API for Sankey diagram data (concept mappings).

**What's Needed:**
- Query variable concept mappings
- Format for Sankey diagram (nodes and links)
- Historical code lookup
- Confidence score display

---

### 12. Action Type Execution Engine
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟡 High  
**Description:** Action types are defined but not fully executable.

**What's Needed:**
- Template variable substitution ({{variable_name}})
- Action execution runtime
- Side effect handlers (webhooks, notifications)
- Rollback support

**Current State:**
- Action types defined in ontology
- Validation exists
- Execution logic is placeholder

---

### 13. Bulk Data Ingestion Pipeline
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟡 High  
**Description:** Python phonograph exists but needs Census-specific adapters.

**What's Needed:**
- Census API adapter (extends SourceAdapter)
- TIGERweb adapter
- PUMS file adapter
- Batch processing with progress tracking
- Data validation and quality checks

---

### 14. Property Type: Integer/Double Validation
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟢 Medium  
**Description:** Basic validation exists but could be enhanced.

**What's Needed:**
- Range validation (already exists)
- Precision handling for doubles
- Integer overflow detection
- Currency formatting support

---

### 15. GraphQL API Enhancements
**Status:** ⚠️ Partially Implemented  
**Priority:** 🟡 High  
**Description:** Current GraphQL API is basic and needs enhancements.

**What's Needed:**
- Proper PropertyValue/PropertyMap GraphQL types (currently JSON strings)
- Filter operators in GraphQL schema
- Aggregation queries
- Geospatial queries
- Pagination (cursor-based)
- Field selection optimization

---

### 16. Storage Backend Implementations
**Status:** ⚠️ Placeholder Only  
**Priority:** 🔴 Critical  
**Description:** Store traits are defined but not connected to real backends.

**What's Needed:**
- Elasticsearch implementation (with geo_shape support)
- Dgraph/Neo4j implementation
- Parquet writer implementation
- Connection pooling
- Transaction support

---

### 17. Write-Back Queue Enhancements
**Status:** ⚠️ Basic Implementation  
**Priority:** 🟢 Medium  
**Description:** Write-back queue exists but needs enhancements for census data.

**What's Needed:**
- Conflict resolution UI
- Edit history tracking
- Approval workflow
- Bulk edit support

---

### 18. Performance Optimizations
**Status:** ❌ Not Implemented  
**Priority:** 🟡 High  
**Description:** Need optimizations for large-scale census data.

**What's Needed:**
- Caching layer (Redis)
- Query result caching
- Batch graph queries
- Lazy loading for large properties (geoshape)
- Index optimization

---

### 19. Error Handling & Monitoring
**Status:** ❌ Not Implemented  
**Priority:** 🟢 Medium  
**Description:** Need robust error handling and observability.

**What's Needed:**
- Structured logging
- Metrics collection (Prometheus)
- Distributed tracing
- Error reporting (Sentry)
- Health check endpoints

---

### 20. Documentation & Examples
**Status:** ⚠️ Partial  
**Priority:** 🟢 Medium  
**Description:** Need comprehensive docs for census use case.

**What's Needed:**
- Census ontology usage guide
- API documentation
- Example queries
- Data ingestion tutorials
- Map visualization examples

---

## Implementation Priority Summary

### Phase 1 (Critical - Must Have)
1. Geospatial Data Support
2. Temporal Query Support
3. Crosswalk/Aggregation Operations
4. Storage Backend Implementations (at least one full implementation)

### Phase 2 (High Priority)
5. Graph Traversal Queries
6. Aggregation Queries
7. Census API Integration
8. Action Type Execution Engine
9. GraphQL API Enhancements

### Phase 3 (Medium Priority)
10. Map Visualization Support
11. Cohort Builder / Faceted Search
12. Bulk Data Ingestion Pipeline
13. Performance Optimizations

### Phase 4 (Nice to Have)
14. Time Slider / Temporal Playback
15. Harmonization Visualization
16. Write-Back Queue Enhancements
17. Error Handling & Monitoring
18. Documentation & Examples







