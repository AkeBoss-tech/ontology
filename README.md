# Ontology Framework

An ontology-driven data operating system inspired by Palantir Foundry. You define your domain as an **ontology** — object types, links between them, and actions — in a YAML file, and the system gives you a GraphQL API, full-text search, graph traversal, time-travel queries, and a suite of React UIs on top of any data source (SQL, CSV, APIs, Parquet/S3).

## What It Does

Most data platforms require you to model your data around the storage layer. This system inverts that: you model your **domain** first (people, assets, transactions, locations, whatever) and the platform handles where and how data is stored.

**Concretely:**
- Write a YAML ontology that declares object types, their properties, and how they link to each other
- The Rust backend loads that ontology, exposes a GraphQL API (with a playground at `/graphql`), handles search and graph traversal, and applies per-object access rules
- The React frontend apps connect to that API and let users browse objects, traverse relationships, visualize data on maps, and execute actions — without you writing custom API or UI code for each object type

## Current Status

| Component | Status |
|---|---|
| GraphQL API (`:8080`) | Working — ontology loading, object search, graph traversal, link queries, ML model registry |
| Object Explorer | Working |
| Model Manager (`:5185`) | Working — register/bind ML models to object properties |
| Function Executor (`:5180`) | Working |
| Interface Explorer (`:5181`) | Working |
| Ontology Viewer (`:5182`) | Working |
| Census Example (`:3005`) | Working |
| Python pipeline | Partial — code exists, backend returns mock predictions |
| Elasticsearch / Dgraph / Parquet stores | Partial — trait interfaces done, implementations are stubs |
| Financial Portfolio, Foundry Rules, Machinery apps | Skeleton / placeholder |

See [`docs/what-works.md`](docs/what-works.md) for details.

## Getting Started

### Prerequisites

- Rust (stable, via `rustup`)
- Node.js 18+
- Docker (only needed for `make start` which brings up Elasticsearch + Dgraph)

### Install and Run

```bash
# Install all dependencies
make install

# Run the GraphQL backend + Object Explorer together
make dev
# Backend:  http://localhost:8080/graphql (includes GraphQL playground)
# Frontend: http://localhost:3000

# Or run a specific app
make object-explorer     # port 3000
make census-example      # port 3005
make function-executor   # port 5180
make interface-explorer  # port 5181
make ontology-viewer     # port 5182

# Start the full stack including Docker services (Elasticsearch, Dgraph)
make start
```

Ports and the ontology file path are configurable:

```bash
make dev BACKEND_PORT=9090 FRONTEND_PORT=4000
make dev ONTOLOGY_PATH=examples/census/config/census_ontology.yaml
```

### Run backend or frontend independently

```bash
# Backend only
make backend

# Specific frontend app only
make frontend APP_NAME=vertex
```

## Defining an Ontology

Everything starts with a YAML ontology file. The backend loads this at startup and generates the GraphQL schema from it dynamically. Here's the structure:

```yaml
ontology:
  objectTypes:
    - id: "person"
      displayName: "Person"
      primaryKey: "id"
      titleKey: "name"
      backingDatasource: "postgres://people_table"
      properties:
        - id: "name"
          type: "string"
          required: true
        - id: "age"
          type: "integer"
        - id: "location"
          type: "geojson"

    - id: "organization"
      displayName: "Organization"
      primaryKey: "org_id"
      titleKey: "name"
      properties:
        - id: "name"
          type: "string"
          required: true
        - id: "industry"
          type: "string"

  linkTypes:
    - id: "person_works_at"
      displayName: "Works At"
      sourceObjectType: "person"
      targetObjectType: "organization"
      cardinality: "many_to_one"

  actionTypes:
    - id: "send_notification"
      displayName: "Send Notification"
      targetObjectType: "person"
      parameters:
        - id: "message"
          type: "string"
          required: true
```

See `config/example_ontology.yaml` and `examples/census/config/census_ontology.yaml` for complete real-world examples. Property types: `string`, `integer`, `double`, `boolean`, `date`, `geojson`, `json`.

## GraphQL API

The backend runs at `http://localhost:8080/graphql` with an interactive playground. The schema is generated from your ontology.

**Core queries:**

```graphql
# Search objects with filters
query {
  searchObjects(
    objectType: "person"
    filters: [{ property: "age", operator: "greaterThan", value: "30" }]
    limit: 10
  ) {
    objectId
    title
    properties
  }
}

# Get a single object
query {
  getObject(objectType: "person", objectId: "p-123") {
    objectId
    title
    properties
  }
}

# Follow links between objects
query {
  getLinkedObjects(objectType: "person", objectId: "p-123", linkType: "person_works_at") {
    objectId
    title
    properties
  }
}

# Graph traversal
query {
  traverseGraph(
    objectType: "person"
    objectId: "p-123"
    linkTypes: ["person_works_at", "org_has_location"]
    maxHops: 3
  ) {
    objectIds
    count
  }
}

# Aggregate
query {
  aggregate(
    objectType: "person"
    aggregations: [{ property: "age", operation: "avg" }]
  ) {
    rows
    total
  }
}
```

**Mutations:**

```graphql
mutation { createObject(objectType: "person", properties: {name: "Alice", age: 30}) { objectId } }
mutation { updateObject(objectType: "person", objectId: "p-123", properties: {age: 31}) { objectId } }
mutation { createLink(linkType: "person_works_at", sourceObjectType: "person", sourceObjectId: "p-123", targetObjectType: "organization", targetObjectId: "org-456", properties: {}) { linkId } }
```

**Filter operators:** `equals`, `contains`, `greaterThan`, `lessThan`, `in`, `between`

## Architecture

```
UI Apps (React/TypeScript/Vite)
          │ GraphQL over HTTP
          ▼
GraphQL API — Rust/Axum (rust-core/graphql-api)
          │
    ┌─────┴──────────────────────┐
    │                            │
Ontology Engine            Indexing Service
(rust-core/ontology-engine) (rust-core/indexing)
    │                            │
    │                   ┌────────┼────────┐
    │                   ▼        ▼        ▼
    │             Elasticsearch Dgraph  Parquet
    │             (search)    (graph)  (analytics)
    │
Security (OLS) ─ rust-core/security
Versioning     ─ rust-core/versioning  (time-travel queries)
Writeback      ─ rust-core/writeback   (user edits overlay source data)
Compiler       ─ rust-core/ontology-compiler

Python Pipeline (python-pipeline/)
  hydration/ ─ adapters for SQL, CSV, API, Kafka → pushes to Indexing Service
```

The **ontology engine** is the core — it defines `ObjectType`, `LinkType`, `ActionType`, `Interface`, `Function`, and computed/derived properties. All other components depend on it.

The **writeback queue** stores user edits in PostgreSQL separately from source data. When you query an object, edits are merged in. Source data is never modified.

**Versioning** uses event sourcing: every change is an append-only event, enabling time-travel queries (`temporalQuery`).

**OLS (Object Level Security)** evaluates per-object access rules inside GraphQL resolvers before returning data.

See [`docs/architecture.md`](docs/architecture.md) for a full deep-dive.

## Frontend Applications

The `ui-framework/` directory is an npm workspace. `packages/` contains shared React component libraries; `apps/` contains runnable applications that consume them.

### Shared packages

| Package | Contents |
|---|---|
| `@ontology/core` | `OntologyProvider`, `useOntology`, `ObjectBrowser`, `ObjectSearch`, `PropertyEditor` |
| `@ontology/forms` | `DynamicForm`, `FilterBuilder` |
| `@ontology/graph` | `GraphVisualization` |
| `@ontology/map` | `MapView`, `TimeSlider` |

Every app wraps its root in `<OntologyProvider client={apolloClient}>` to get access to the GraphQL client via `useOntology()`.

### Available apps

| App | Makefile target | Default port | Description |
|---|---|---|---|
| Object Explorer | `make object-explorer` | 3000 | Browse and search all object types |
| Census Example | `make census-example` | 3005 | Geospatial + temporal census data demo |
| Function Executor | `make function-executor` | 5180 | Discover and execute ontology Actions |
| Interface Explorer | `make interface-explorer` | 5181 | Browse Object Interfaces |
| Ontology Viewer | `make ontology-viewer` | 5182 | Visual schema browser |
| Model Manager | *(no make target)* | 5185 | AI/ML model registry, bind models to properties |
| Vertex | `make vertex` | 3002 | Graph traversal visualization |
| Map | `make map-app` | 3003 | GeoJSON map visualization |
| Platform | `make platform` | 5200 | Unified workstation (all apps in one) |

### Creating a new app

```bash
./scripts/create-app.sh my-app "My App" "Description"
cd ui-framework/apps/my-app
npm install
npm run dev
```

The generated app is pre-wired with `OntologyProvider`, Tailwind CSS, and Apollo Client pointing at `VITE_GRAPHQL_URL` (defaults to `http://localhost:8080`). See [`docs/ui-application-guide.md`](docs/ui-application-guide.md) for component API docs and patterns.

## Development

```bash
# Build
make build              # build everything
make build-backend      # Rust release build only
make build-frontend     # all frontend apps

# Test
make test                               # all tests
cd rust-core && cargo test --workspace  # Rust unit tests only
cd rust-core && cargo test -p indexing  # single crate

# Integration tests (require Docker services running)
make services-up
cd rust-core && cargo test --package indexing --test store_test -- --ignored

# Stop everything
make stop   # graceful
make kill   # force
```

## Example: Census Dataset

The census example (`examples/census/`) demonstrates the full system with real data:

```bash
# Generate sample data
python3 examples/census/scripts/load_sample_data.py

# Start with census ontology
make dev ONTOLOGY_PATH=examples/census/config/census_ontology.yaml
make census-example
```

This loads census tracts, counties, PUMAs, households, and persons with GeoJSON geometries. The census-example app shows choropleth maps, time-slider queries (1990/2000/2010/2020 vintages), graph traversal (Tract → PUMA → Household → Person), cohort building, and crosswalk boundary normalization.

## License

MIT
