# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Development

```bash
make dev                        # Run backend (port 8080) + default frontend (port 3000) together
make object-explorer            # Run object-explorer app + backend
make backend                    # Only the Rust GraphQL server
make frontend APP_NAME=vertex   # Only a specific frontend app
make start                      # Full stack including Docker services (Elasticsearch, Dgraph)
```

Available frontend apps: `object-explorer`, `object-views`, `vertex`, `map-app`, `financial-portfolio`, `census-example`, `ontology-manager`, `foundry-rules`, `machinery`, `dynamic-scheduling`, `function-executor`, `interface-explorer`, `ontology-viewer`, `model-manager`, `platform`

Ports can be overridden: `make dev BACKEND_PORT=9090 FRONTEND_PORT=4000`

### Build & Test

```bash
make install                    # Install all deps (Rust + Node)
make build                      # Build backend (release) + all frontend apps
make test                       # Run Rust tests + frontend tests

# Rust only
cd rust-core && cargo test --workspace
cd rust-core && cargo test -p ontology-engine <test_name>   # Single test
cd rust-core/graphql-api && cargo run --bin server          # Run backend directly

# Frontend only
cd ui-framework && npm install
cd ui-framework/apps/object-explorer && npm run dev
```

### Cleanup

```bash
make stop    # Graceful stop
make kill    # Force kill all processes and Docker containers
```

## Architecture

This is a Palantir Foundry-inspired ontology-driven data operating system. The ontology (object types, link types, action types) is defined in YAML (`config/` or `examples/`) and drives the entire system dynamically.

### Backend (Rust workspace at `rust-core/`)

Seven crates in the Cargo workspace:
- **`ontology-engine`** – Core meta-model: `ObjectType`, `LinkType`, `ActionType`, `Interface`, `Function`, computed properties, crosswalk navigation. Everything else depends on this.
- **`graphql-api`** – Axum + async-graphql server. Schema is generated dynamically from the loaded ontology. Entry point: `src/bin/server.rs`. GraphQL playground at `http://localhost:8080/graphql`.
- **`indexing`** – Trait-based multi-store sync (`SearchStore`, `GraphStore`, `ColumnarStore`). Implementations for Elasticsearch, Dgraph, and Parquet.
- **`security`** – Object Level Security (OLS) engine; rule-based per-object access control evaluated inside GraphQL resolvers.
- **`versioning`** – Event sourcing for time-travel queries.
- **`writeback`** – PostgreSQL-backed overlay queue; user edits are stored separately from source data and merged at query time.
- **`ontology-compiler`** – Compiles/validates ontology definitions.

The runtime ontology path is set via the `ONTOLOGY_PATH` env var (default: `examples/census/config/census_ontology.yaml`).

### Frontend (npm workspaces at `ui-framework/`)

Structure: `packages/*` contains shared libraries; `apps/*` contains individual applications. All apps use React + TypeScript + Vite + Tailwind CSS + Apollo Client.

**Shared packages** (consumed by all apps via `@ontology/*` aliases):
- **`packages/core`** – `OntologyProvider` (React context wrapping Apollo), `useOntology()` hook, ontology switching via localStorage.
- **`packages/forms`** – `DynamicForm`, `FilterBuilder`
- **`packages/graph`** – `GraphVisualization`
- **`packages/map`** – `MapView`, `TimeSlider`

Every app wraps its root in `<OntologyProvider client={apolloClient}>`. The Apollo client points to `VITE_GRAPHQL_URL` (default `http://localhost:8080`).

**Working apps** (per `WHAT_WORKS.md`): Object Explorer, Model Manager (5185), Function Executor (5180), Interface Explorer (5181), Ontology Viewer (5182), Census Example (3005). Apps like `financial-portfolio`, `foundry-rules`, `machinery` are skeleton placeholders.

### Data Flow

```
Source data (SQL/CSV/API)
  → Python hydration pipeline (python-pipeline/)
  → Rust Indexing Service
  → Elasticsearch / Dgraph / Parquet

GraphQL query → Resolver → OLS check → writeback merge → store query → response
```

### Creating a New App

```bash
./scripts/create-app.sh my-app "My App" "Description"
cd ui-framework/apps/my-app && npm install && npm run dev
```
