# Makefile Usage Guide

The Makefile provides convenient commands to run, build, and manage the ontology framework.

## Quick Start

### Run Everything (Backend + Frontend)
```bash
make dev
```
This will start:
- GraphQL backend on `http://localhost:8080`
- Frontend app (default: `object-explorer`) on `http://localhost:3000`

### Run Specific App
```bash
make object-explorer    # Run object-explorer
make vertex            # Run vertex app
make map-app           # Run map app
make financial-portfolio # Run financial portfolio
```

Or specify the app name:
```bash
make frontend APP=vertex
```

## Available Commands

### Setup
```bash
make install              # Install all dependencies
make install-backend      # Install Rust dependencies only
make install-frontend     # Install Node.js dependencies only
```

### Development
```bash
make dev                  # Run backend + frontend (default: object-explorer)
make backend              # Run only the GraphQL backend
make frontend             # Run only frontend (default: object-explorer)
make frontend APP=app-name # Run specific frontend app
```

### Build
```bash
make build                # Build both backend and frontend
make build-backend        # Build Rust backend only
make build-frontend       # Build all frontend apps
```

### Utilities
```bash
make clean                # Clean all build artifacts
make stop                 # Stop all running servers
make test                 # Run tests
make help                 # Show help message
```

## Available Frontend Apps

- `object-explorer` (default)
- `object-views`
- `vertex`
- `map-app`
- `financial-portfolio`
- `census-example`
- `ontology-manager`
- `foundry-rules`
- `machinery`
- `dynamic-scheduling`

## Configuration

You can override default ports and app name:

```bash
BACKEND_PORT=8080 FRONTEND_PORT=3001 make dev APP=vertex
```

## Examples

### Run object-explorer with backend
```bash
make object-explorer
```

### Run only the backend
```bash
make backend
```

### Run only a specific frontend app
```bash
make frontend APP=vertex
```

### Install dependencies for a new app
```bash
cd ui-framework/apps/my-app
npm install
```

### Clean everything and start fresh
```bash
make clean
make install
make dev
```





