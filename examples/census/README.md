# Census Data Example

This example demonstrates the ontology framework with census data, including geospatial visualization, temporal queries, graph traversal, and crosswalk operations.

## Setup

### 1. Generate Sample Data

```bash
python3 scripts/load_sample_data.py
```

This creates sample data files in the `data/` directory:
- `tracts.json` - Census tract data with GeoJSON shapes
- `counties.json` - County data
- `pumas.json` - PUMA data
- `crosswalks.json` - Boundary crosswalk data
- `households.json` - PUMS household data
- `persons.json` - PUMS person data

### 2. Start the GraphQL Server

```bash
cd ../../rust-core/graphql-api
cargo run --bin server
```

The server will start on `http://localhost:8080` with:
- GraphQL endpoint: `http://localhost:8080/graphql`
- WebSocket endpoint: `ws://localhost:8080/graphql/ws`

You can set environment variables:
- `ONTOLOGY_PATH` - Path to ontology YAML file (default: `examples/census/config/census_ontology.yaml`)
- `PORT` - Server port (default: `8080`)

### 3. Start the Frontend

```bash
cd ../../ui-framework/apps/census-example
npm install
npm run dev
```

The frontend will start on `http://localhost:3000` and connect to the GraphQL server.

## Features Demonstrated

### 1. Tract Map
- Visualize census tracts on a map
- Use time slider to view different census vintages (1990, 2000, 2010, 2020)
- Click on tracts to view details

### 2. PUMS Analysis
- Browse census tracts
- Traverse graph: Tract → PUMA → Household → Person
- Adjust max hops for traversal depth
- View aggregated statistics

### 3. Crosswalk View
- Select source tract and year
- Normalize boundaries to target year
- View crosswalk allocations based on overlap percentage

### 4. Cohort Builder
- Build filters for PUMS Person objects
- Search by age, sex, race, occupation, wages, etc.
- View filtered results

### 5. Data Source Management
- View configured data sources
- Add new data sources
- Test connections

## GraphQL Queries

### Search Objects
```graphql
query {
  searchObjects(
    objectType: "census_tract_vintage"
    filters: [
      { property: "year", operator: "equals", value: "2010" }
    ]
    limit: 10
  ) {
    objectId
    title
    properties
  }
}
```

### Temporal Query
```graphql
query {
  temporalQuery(
    objectType: "census_tract_vintage"
    year: 2010
  ) {
    objectId
    title
    properties
  }
}
```

### Get Available Years
```graphql
query {
  getAvailableYears(objectType: "census_tract_vintage")
}
```

### Spatial Query
```graphql
query {
  spatialQuery(
    objectType: "census_tract_vintage"
    property: "geoshape"
    operator: "intersects"
    geometry: "{\"type\":\"Polygon\",\"coordinates\":[[[-98.5,39.8],[-98.4,39.8],[-98.4,39.9],[-98.5,39.9],[-98.5,39.8]]]}"
  ) {
    objectId
    title
  }
}
```

### Graph Traversal
```graphql
query {
  traverseGraph(
    objectType: "census_tract_vintage"
    objectId: "14000US2010001_2010"
    linkTypes: ["tract_to_puma", "puma_to_household", "household_to_person"]
    maxHops: 3
  ) {
    objectIds
    count
  }
}
```

### Aggregation
```graphql
query {
  aggregate(
    objectType: "pums_person"
    aggregations: [
      { property: "wages", operation: "avg" }
      { property: "age", operation: "max" }
    ]
    filters: [
      { property: "year", operator: "equals", value: "2010" }
    ]
  ) {
    rows
    total
  }
}
```

## Data Structure

The ontology defines:

- **Geospatial Layer**: `census_tract_vintage`, `county_vintage`, `puma_vintage`
- **Crosswalks**: `boundary_crosswalk` for boundary normalization
- **PUMS Microdata**: `pums_household`, `pums_person`
- **Harmonization**: `standardized_occupation`, `occ_code_mapping`, `variable_concept`, `variable_mapping`

Links connect:
- Tracts to Counties and PUMAs
- PUMAs to Households
- Households to Persons
- Persons to Occupations (via mappings)

## Testing

### Test Ontology Loading
```bash
python3 scripts/test_ontology_loading.py
```

### Test Rust Compilation
```bash
cd ../../rust-core
cargo build --workspace
```

### Test Frontend Build
```bash
cd ../../ui-framework/apps/census-example
npm run build
```

## Next Steps

1. Connect to real Census API to load actual data
2. Implement full Elasticsearch/Dgraph backends
3. Add authentication and authorization
4. Enhance map visualization with Leaflet/Mapbox
5. Add more sophisticated graph visualization
6. Implement real-time updates via WebSocket
