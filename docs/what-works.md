# What Works (Inventory)

This document provides a verified inventory of the current system capabilities.

## 🟢 Fully Functional Apps
These applications are built, running, and connected to the live backend.

| App Name | Port | Description | Status |
|---|---|---|---|
| **Model Manager** | `5185` | AI/ML Registry. Register models, view metrics, and **bind models to object properties**. | ✅ **Verified** |
| **Object Explorer** | `3000`* | Main interface for searching and browsing the ontology graph. | ✅ **Verified** |
| **Function Executor** | `5180` | Interface to discover and execute ontology functions (Actions). | ✅ **Verified** |
| **Interface Explorer**| `5181` | Browse Object Interfaces and their implementations. | ✅ **Verified** |
| **Ontology Viewer** | `5182` | Visual schema browser for Object Types and Links. | ✅ **Verified** |
| **Census Example** | `3005` | Demo dashboard for the loaded Census dataset. | ✅ **Verified** |

> *Note: Ports may vary based on `Makefile` defaults, but these are standard.*

## 🟢 Backend Services
The core infrastructure managing data and logic.

*   **GraphQL API** (`:8080`): The central brain. Handles all queries, mutations, and orchestration.
*   **Data Engine**:
    *   **Ontology Loading**: Successfully parses `ontology.yaml`.
    *   **Data Ingestion**: Loads JSON datasets (Census) into memory/graph.
    *   **Link Traversal**: Graph navigation works (links between objects).
*   **Search**: In-memory text search and filtering is active.
*   **Model Registry**: In-memory registry for ML models is active.

## 🟡 Partially Implemented / Prototypes
These components exist in code but may be incomplete or using mock data.

*   **Python Integration**: The `python-pipeline` code exists for TensorFlow/PyTorch serving, but the Backend currently returns a "Not Implemented" or Mock response for actual predictions.
*   **Vertex**: `ui-framework/apps/vertex` exists as a skeleton for simulation scenarios but lacks deep logic.
*   **Maps**: `map-app` is a basic template, not a full GIS suite yet.

## 🔴 Skeleton / Planned
*   `financial-portfolio`: Placeholder.
*   `foundry-rules`: Placeholder logic.
*   `machinery`: Placeholder.

---

## How to Verify
Run `make start` (or `make model-manager`, `make object-explorer` etc) to launch the stack.
- Access **GraphQL Playground**: `http://localhost:8080/graphql`
- Access **Model Manager**: `http://localhost:5185`
