# Palantir Foundry Application Examples

This directory contains example applications demonstrating different Palantir Foundry application patterns.

## Available Applications

### 1. Object Explorer (`object-explorer`)
**Type:** Discovery & Analysis, Exploratory, Walk-up usable

Generic object browsing and exploration application. Browse objects by type, search and filter, and explore object details and relationships.

**Features:**
- Object type selection
- Object search and filtering
- Property viewing
- Link type exploration

**Usage:**
```bash
cd ui-framework/apps/object-explorer
npm install
npm run dev
```

### 2. Object Views (`object-views`)
**Type:** Discovery, Workflow-specific, Walk-up usable

Create custom views of objects with specific layouts, filters, and workflows. Save views for quick access and team sharing.

**Features:**
- View builder interface
- Saved view management
- Custom filters and layouts
- View sharing

**Usage:**
```bash
cd ui-framework/apps/object-views
npm install
npm run dev
```

### 3. Vertex (`vertex`)
**Type:** Analysis & Dashboards, Exploratory

Graph visualization and relationship traversal. Explore object relationships through interactive graph visualizations.

**Features:**
- Graph traversal from any object
- Multi-link type support
- Configurable hop depth
- Interactive node exploration

**Usage:**
```bash
cd ui-framework/apps/vertex
npm install
npm run dev
```

### 4. Map (`map-app`)
**Type:** Geospatial, Exploratory or Workflow-specific, Walk-up usable

Geospatial visualization and mapping application. Visualize objects with geographic properties on interactive maps.

**Features:**
- GeoJSON support
- Choropleth visualization
- Time filtering
- Interactive map exploration

**Usage:**
```bash
cd ui-framework/apps/map-app
npm install
npm run dev
```

### 5. Ontology Manager (`ontology-manager`)
**Type:** Administration

Manage and configure ontology definitions. View, edit, and maintain object types, link types, and action types.

**Features:**
- Ontology definition viewing
- Object type management
- Link type configuration
- Action type management

**Usage:**
```bash
cd ui-framework/apps/ontology-manager
npm install
npm run dev
```

### 6. Foundry Rules (`foundry-rules`)
**Type:** Administration

Manage sharing rules and access controls. Configure fine-grained sharing rules for objects and links.

**Features:**
- Sharing rule creation
- Access control management
- Rule evaluation
- Audit logging

**Usage:**
```bash
cd ui-framework/apps/foundry-rules
npm install
npm run dev
```

### 7. Machinery (`machinery`)
**Type:** Workflow-specific

Asset and equipment management application. Track machinery, maintenance schedules, and relationships.

**Features:**
- Asset tracking
- Maintenance scheduling
- Location management
- Equipment relationships

**Usage:**
```bash
cd ui-framework/apps/machinery
npm install
npm run dev
```

### 8. Dynamic Scheduling (`dynamic-scheduling`)
**Type:** Applications & Dashboards, Workflow-specific, Customizable

Task scheduling and workflow management. Create, manage, and track tasks and workflows.

**Features:**
- Task creation and management
- Workflow scheduling
- Dependency tracking
- Status monitoring

**Usage:**
```bash
cd ui-framework/apps/dynamic-scheduling
npm install
npm run dev
```

### 9. Financial Portfolio (`financial-portfolio`)
**Type:** Workflow-specific

Financial portfolio management application. Manage portfolios, assets, and transactions.

**Features:**
- Portfolio browsing
- Asset search
- Transaction history
- Relationship tracking

**Usage:**
```bash
cd ui-framework/apps/financial-portfolio
npm install
npm run dev
```

### 10. Census Example (`census-example`)
**Type:** Analysis & Dashboards, Exploratory

Census data explorer with geospatial visualization. Demonstrates complex data exploration patterns.

**Features:**
- Census data browsing
- Geospatial mapping
- Cohort building
- Crosswalk visualization

**Usage:**
```bash
cd ui-framework/apps/census-example
npm install
npm run dev
```

## Application Patterns

These applications demonstrate various Palantir Foundry application patterns:

- **Discovery & Analysis**: Object Explorer, Vertex
- **Workflow-specific**: Object Views, Machinery, Dynamic Scheduling, Financial Portfolio
- **Geospatial**: Map, Census Example
- **Administration**: Ontology Manager, Foundry Rules
- **Exploratory**: Object Explorer, Vertex, Map
- **Walk-up usable**: Object Explorer, Object Views, Map

## Creating New Applications

To create a new application, use the generator script:

```bash
./scripts/create-app.sh my-app-name "My App Display Name" "Description"
```

See [APPLICATION_GUIDE.md](./APPLICATION_GUIDE.md) for detailed documentation on building applications.

## Shared Components

All applications use shared packages from `packages/`:

- `@ontology/core`: GraphQL client, ObjectBrowser, ObjectSearch, PropertyEditor
- `@ontology/forms`: DynamicForm, FilterBuilder
- `@ontology/graph`: GraphVisualization
- `@ontology/map`: MapView, TimeSlider

See the [APPLICATION_GUIDE.md](./APPLICATION_GUIDE.md) for detailed API documentation.





