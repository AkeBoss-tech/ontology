# Ontology Framework

A generic, Palantir-like Ontology-Driven Data Operating System built with Rust (for performance) and Python (for data science workloads).

## Architecture

The system follows a hub-and-spoke architecture with three main layers:

1. **Hydration Pipeline (Python)**: Ingest and transform data from various sources (SQL, CSV, APIs, Kafka)
2. **Indexing Core (Rust)**: Multi-store synchronization (Elasticsearch for search, Dgraph/Neo4j for graph, Parquet/S3 for columnar)
3. **Object API (Rust)**: GraphQL API with Object Level Security (OLS)

## Project Structure

```
ontology/
├── rust-core/              # Rust performance-critical components
│   ├── ontology-engine/    # Core meta-model (ObjectType, LinkType, ActionType)
│   ├── indexing/           # Multi-store indexer and sync service
│   ├── graphql-api/        # GraphQL API server
│   ├── security/           # Object Level Security (OLS)
│   ├── versioning/         # Time-travel and event logging
│   └── writeback/          # Write-back queue implementation
├── python-pipeline/        # Python data science components
│   ├── hydration/          # Schema mapping and data ingestion
│   ├── transformers/       # Data transformation utilities
│   └── analytics/          # Columnar analytics helpers
├── config/                 # Ontology YAML/JSON definitions
├── scripts/                # Utility scripts
└── docs/                   # Architecture documentation
```

## Key Features

- **Backing Dataset Independence**: Object types can be backed by any data source (SQL, APIs, CSV, etc.)
- **First-Class Links**: Links between objects are first-class citizens with their own properties
- **Time Travel**: Query objects at any point in time using event sourcing
- **Dynamic Ontology Editing**: Add/modify object types, link types, and action types at runtime
- **Write-Back Queue**: User edits overlay source data without modifying the original
- **Multi-Store Architecture**: Separate optimized stores for search, graph traversal, and analytics

## Getting Started

### Rust Components

```bash
cd rust-core
cargo build --workspace
cargo test --workspace
```

### Python Components

```bash
cd python-pipeline
pip install -e .
```

### Example Ontology

See `config/example_ontology.yaml` for an example ontology definition.

## License

MIT



