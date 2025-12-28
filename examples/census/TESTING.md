# Testing the Census Example

## Quick Start

### 1. Generate Sample Data
```bash
cd examples/census
python3 scripts/load_sample_data.py
```

This creates sample data in `data/` directory with:
- 20 census tracts (across 4 years: 1990, 2000, 2010, 2020)
- 4 counties
- 6 PUMAs
- 3 crosswalks
- 20 households
- 40 persons

### 2. Verify Setup
```bash
python3 scripts/verify_setup.py
```

### 3. Start GraphQL Server

In one terminal:
```bash
cd rust-core/graphql-api
cargo run --bin server
```

The server will:
- Load the census ontology from `examples/census/config/census_ontology.yaml`
- Start on `http://localhost:8080`
- Provide GraphQL endpoint at `/graphql`

### 4. Start Frontend

In another terminal:
```bash
cd ui-framework/apps/census-example
npm install  # First time only
npm run dev
```

The frontend will:
- Start on `http://localhost:3000`
- Connect to GraphQL server at `http://localhost:8080/graphql`

### 5. Test the Application

Open `http://localhost:3000` in your browser and test:

1. **Tract Map**: View census tracts, use time slider to change years
2. **PUMS Analysis**: Browse tracts, traverse graph to households/persons
3. **Crosswalk View**: Select source tract and normalize to target year
4. **Cohort Builder**: Build filters and search for persons
5. **Data Sources**: View and manage data sources

## Testing GraphQL Queries

You can test queries directly at `http://localhost:8080/graphql`:

### Get Available Years
```graphql
query {
  getAvailableYears(objectType: "census_tract_vintage")
}
```

### Search Tracts
```graphql
query {
  searchObjects(
    objectType: "census_tract_vintage"
    limit: 5
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
  }
}
```

## Expected Behavior

### Tract Map Page
- Should display a map placeholder (full map requires Leaflet/Mapbox integration)
- Time slider should show years: 1990, 2000, 2010, 2020
- Selecting a year should filter tracts

### PUMS Analysis Page
- Object browser should allow searching for tracts
- Graph traversal should show connected objects
- Adjusting max hops should change traversal depth

### Crosswalk View
- Should allow selecting source tract and year
- Should allow selecting target year
- "Normalize Boundaries" button should trigger crosswalk operation

### Cohort Builder
- Filter builder should allow adding multiple filters
- Search should return filtered person results
- Results should display person properties

## Troubleshooting

### Server Won't Start
- Check that ontology file exists: `examples/census/config/census_ontology.yaml`
- Check that port 8080 is not in use
- Check Rust compilation: `cargo check --bin server --package graphql-api`

### Frontend Won't Connect
- Verify GraphQL server is running on port 8080
- Check browser console for errors
- Verify proxy configuration in `vite.config.ts`

### No Data Showing
- Verify sample data was generated: `ls examples/census/data/`
- Check that data files contain JSON arrays
- Check server logs for errors

### GraphQL Errors
- Check that ontology is valid YAML
- Verify object types and link types are correctly defined
- Check server logs for detailed error messages

## Next Steps for Full Implementation

1. **Connect Real Backends**:
   - Implement Elasticsearch for search
   - Implement Dgraph/Neo4j for graph
   - Implement Parquet writer for columnar

2. **Enhance Map Visualization**:
   - Integrate Leaflet or Mapbox GL
   - Render actual GeoJSON polygons
   - Add choropleth coloring

3. **Add Real Data**:
   - Connect to Census API
   - Load TIGER/Line shapefiles
   - Import PUMS microdata

4. **Improve Graph Visualization**:
   - Use react-force-graph or vis-network
   - Show link properties
   - Interactive node selection

5. **Add Authentication**:
   - Implement OLS (Object Level Security)
   - Add user roles and badges
   - Secure GraphQL endpoints




