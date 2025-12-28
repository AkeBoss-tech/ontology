# Ontology Framework

A generic, Palantir-like Ontology-Driven Data Operating System built with Rust (for performance) and Python (for data science workloads).

## Architecture

The system follows a hub-and-spoke architecture with three main layers:

1. **Hydration Pipeline (Python)**: Ingest and transform data from various sources (SQL, CSV, APIs, Kafka)
2. **Indexing Core (Rust)**: Multi-store synchronization (Elasticsearch for search, Dgraph/Neo4j for graph, Parquet/S3 for columnar)
3. **Object API (Rust)**: GraphQL API with Object Level Security (OLS)

**For detailed architecture documentation, design choices, and rationale, see [ARCHITECTURE.md](./ARCHITECTURE.md)**

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
├── ui-framework/           # React-based UI framework
│   ├── packages/           # Shared UI packages
│   │   ├── core/           # Core components (ObjectBrowser, ObjectSearch, etc.)
│   │   ├── forms/          # Form components (DynamicForm, FilterBuilder)
│   │   ├── graph/          # Graph visualization components
│   │   └── map/            # Geospatial visualization components
│   ├── apps/               # Domain-specific applications
│   │   ├── census-example/ # Census data explorer example
│   │   └── financial-portfolio/ # Financial portfolio manager example
│   └── templates/          # Application templates
│       └── app-template/   # Template for creating new applications
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

### Quick Start with Makefile

The easiest way to get started is using the Makefile:

```bash
# Install all dependencies
make install

# Run backend and frontend together
make dev

# Or run specific app
make object-explorer
make vertex
make map-app
```

See `README_MAKEFILE.md` for all available commands.

### Manual Setup

#### Rust Components

```bash
cd rust-core
cargo build --workspace
cargo test --workspace
```

#### Python Components

```bash
cd python-pipeline
pip install -e .
```

### Example Ontology

See `config/example_ontology.yaml` for an example ontology definition.

### UI Framework and Applications

The project includes a React-based UI framework for building Palantir-like applications:

- **Shared Packages**: Reusable components for ontology interactions
  - `@ontology/core`: GraphQL client, ObjectBrowser, ObjectSearch, PropertyEditor
  - `@ontology/forms`: DynamicForm, FilterBuilder
  - `@ontology/graph`: GraphVisualization
  - `@ontology/map`: MapView, TimeSlider

- **Example Applications**: 
  - See `ui-framework/APPLICATIONS.md` for a complete list of example applications
  - Includes: Object Explorer, Object Views, Vertex (graph), Map, Ontology Manager, Foundry Rules, Machinery, Dynamic Scheduling, Financial Portfolio, and Census Example

- **Creating New Applications**: 
  - Use the application template: `./scripts/create-app.sh my-app-name "Display Name" "Description"`
  - See `ui-framework/APPLICATION_GUIDE.md` for detailed documentation
  - See `ui-framework/APPLICATIONS.md` for examples of different application patterns

#### Quick Start for UI Development

**Using Makefile (Recommended):**
```bash
# Run backend + frontend together
make dev

# Run specific app
make object-explorer
make vertex
make map-app
```

**Manual Setup:**
```bash
# Navigate to UI framework
cd ui-framework

# Install dependencies
npm install

# Run an example app
npm run dev --workspace=census-example

# Or create a new app
./scripts/create-app.sh my-app "My App" "Description"
cd apps/my-app
npm install
npm run dev
```

## Making It More Palantir-Like

The system already implements many Palantir Foundry features in the backend. To make it more Palantir-like:

1. **Quick Start:** See `QUICK_START_PALANTIR.md` for immediate next steps
2. **Detailed Guide:** See `HOW_TO_MAKE_MORE_PALANTIR_LIKE.md` for comprehensive improvements
3. **Roadmap:** See `PALANTIR_ROADMAP.md` for prioritized implementation plan
4. **Feature Status:** See `PALANTIR_FOUNDRY_FEATURES.md` for what's implemented vs. missing

**Key Priorities:**
- Expose Functions and Interfaces in UI (backend already implemented)
- Enhance property display with metadata (units, formats, descriptions)
- Create data table component with Palantir-like features
- Implement sharing rules (beyond basic OLS)

## License

MIT



