# Ontology Application Development Guide

This guide explains how to create and develop Palantir-like applications using the ontology framework.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Architecture Overview](#architecture-overview)
3. [Available Packages](#available-packages)
4. [Common Patterns](#common-patterns)
5. [GraphQL Integration](#graphql-integration)
6. [Styling Guidelines](#styling-guidelines)
7. [Testing](#testing)
8. [Deployment](#deployment)

## Quick Start

### Creating a New Application

Create a new application from the template:

```bash
cd ui-framework
npm run create-app my-app-name "My App Display Name" "Description of my app"
```

Or use the script directly:

```bash
./scripts/create-app.sh my-app-name "My App Display Name" "Description of my app"
```

### Setup and Development

1. Navigate to your app:
   ```bash
   cd apps/my-app-name
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. (Optional) Create a `.env` file to configure the GraphQL endpoint:
   ```bash
   VITE_GRAPHQL_URL=http://localhost:8080/graphql
   ```

4. Start the development server:
   ```bash
   npm run dev
   ```

5. Open http://localhost:3000 in your browser

### Customization Checklist

- [ ] Update object types in `src/pages/Search.tsx` and `src/pages/Browse.tsx`
- [ ] Customize pages in `src/pages/`
- [ ] Add new navigation items in `src/App.tsx`
- [ ] Configure your ontology backend endpoint
- [ ] Add domain-specific components

## Architecture Overview

### System Components

```
┌─────────────────────────────────────────────────────────┐
│                    Your Application                      │
│  (React + TypeScript + Vite + Tailwind CSS)            │
└─────────────────────────────────────────────────────────┘
                         │
                         │ Uses
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Shared Packages (@ontology/*)               │
│  • @ontology/core: GraphQL client, ObjectBrowser, etc.  │
│  • @ontology/forms: DynamicForm, FilterBuilder          │
│  • @ontology/graph: GraphVisualization                  │
│  • @ontology/map: MapView, TimeSlider                   │
└─────────────────────────────────────────────────────────┘
                         │
                         │ GraphQL Queries
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Rust GraphQL API (/graphql)                 │
│  • Ontology Engine (ObjectType, LinkType, ActionType)   │
│  • Multi-store Indexing (Elasticsearch, Graph, Columnar)│
│  • Object Level Security (OLS)                          │
│  • Time-travel Queries                                  │
└─────────────────────────────────────────────────────────┘
```

### Application Structure

```
apps/my-app-name/
├── package.json          # App dependencies and scripts
├── vite.config.ts        # Vite configuration with GraphQL proxy
├── tsconfig.json         # TypeScript configuration
├── tailwind.config.js    # Tailwind CSS configuration
├── index.html            # HTML entry point
└── src/
    ├── main.tsx          # React entry with OntologyProvider
    ├── App.tsx           # Main app component with navigation
    ├── index.css         # Global styles
    └── pages/            # Application pages
        ├── Home.tsx
        ├── Search.tsx
        └── Browse.tsx
```

## Available Packages

### @ontology/core

Core components for ontology interactions.

#### Components

**OntologyProvider & useOntology**

Wrap your app and access the GraphQL client:

```tsx
import { OntologyProvider, useOntology, createOntologyClient } from '@ontology/core';

const client = createOntologyClient('/graphql');

function App() {
  return (
    <OntologyProvider client={client}>
      {/* Your app */}
    </OntologyProvider>
  );
}

function MyComponent() {
  const { client } = useOntology();
  // Use client for GraphQL queries
}
```

**ObjectSearch**

Search for objects by type:

```tsx
import { ObjectSearch } from '@ontology/core';

<ObjectSearch
  objectType="Person"
  onSelectObject={(objectId) => {
    console.log('Selected:', objectId);
  }}
/>
```

**ObjectBrowser**

Browse objects with details and links:

```tsx
import { ObjectBrowser } from '@ontology/core';

<ObjectBrowser 
  objectType="Person"
  initialObjectId="optional-initial-id"
/>
```

**PropertyEditor**

Edit object properties dynamically:

```tsx
import { PropertyEditor, PropertyDefinition } from '@ontology/core';

const properties: PropertyDefinition[] = [
  {
    id: 'name',
    displayName: 'Name',
    type: 'string',
    required: true,
  },
  {
    id: 'age',
    displayName: 'Age',
    type: 'integer',
    validation: { min: 0, max: 150 },
  },
];

<PropertyEditor
  properties={properties}
  values={{ name: 'John', age: 30 }}
  onChange={(newValues) => console.log(newValues)}
/>
```

**VisualizationManager**

Manage and persist visualizations:

```tsx
import { VisualizationManagerProvider, useVisualizationManager } from '@ontology/core';

function App() {
  return (
    <VisualizationManagerProvider>
      {/* Your app */}
    </VisualizationManagerProvider>
  );
}

function MyComponent() {
  const { visualizations, saveVisualization, deleteVisualization } = useVisualizationManager();
  // Use visualization manager
}
```

### @ontology/forms

Form components for ontology objects.

**DynamicForm**

Create forms from property definitions:

```tsx
import { DynamicForm } from '@ontology/forms';

const properties = [
  { id: 'name', type: 'string', required: true },
  { id: 'email', type: 'string', required: true },
];

<DynamicForm
  properties={properties}
  initialValues={{}}
  onSubmit={(values) => {
    console.log('Submitted:', values);
  }}
  submitLabel="Create Person"
/>
```

**FilterBuilder**

Build complex filter queries:

```tsx
import { FilterBuilder, Filter } from '@ontology/forms';

const [filters, setFilters] = useState<Filter[]>([]);

<FilterBuilder
  properties={properties}
  filters={filters}
  onChange={setFilters}
/>
```

### @ontology/graph

Graph visualization components.

**GraphVisualization**

Visualize object relationships:

```tsx
import { GraphVisualization } from '@ontology/graph';

<GraphVisualization
  objectType="Person"
  objectId="person-123"
  linkTypes={['knows', 'works_with']}
  maxHops={3}
  onNodeClick={(objectId) => console.log('Clicked:', objectId)}
/>
```

### @ontology/map

Geospatial visualization components.

**MapView**

Display objects on a map:

```tsx
import { MapView } from '@ontology/map';

<MapView
  objectType="Location"
  geojsonProperty="geometry"
  valueProperty="population"  // For choropleth coloring
  center={[-98.5795, 39.8283]}
  zoom={4}
  onObjectClick={(objectId, properties) => {
    console.log('Clicked object:', objectId);
  }}
  filters={[]}
  selectedYear={2020}
/>
```

**TimeSlider**

Filter by time periods:

```tsx
import { TimeSlider } from '@ontology/map';

<TimeSlider
  minYear={2010}
  maxYear={2020}
  selectedYear={2020}
  onChange={(year) => setSelectedYear(year)}
/>
```

## Common Patterns

### Pattern 1: Object Search and Display

```tsx
import { ObjectSearch, ObjectBrowser } from '@ontology/core';

function SearchPage() {
  const [selectedObjectId, setSelectedObjectId] = useState<string>();

  return (
    <div className="grid grid-cols-2 gap-4">
      <div>
        <h2>Search</h2>
        <ObjectSearch
          objectType="Person"
          onSelectObject={setSelectedObjectId}
        />
      </div>
      <div>
        {selectedObjectId && (
          <ObjectBrowser
            objectType="Person"
            initialObjectId={selectedObjectId}
          />
        )}
      </div>
    </div>
  );
}
```

### Pattern 2: Custom GraphQL Queries

```tsx
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const GET_OBJECT = gql`
  query GetObject($objectType: String!, $objectId: String!) {
    getObject(objectType: $objectType, objectId: $objectId) {
      objectType
      objectId
      title
      properties
    }
  }
`;

function MyComponent() {
  const { client } = useOntology();
  const { data, loading, error } = useQuery(GET_OBJECT, {
    client,
    variables: {
      objectType: 'Person',
      objectId: 'person-123',
    },
  });

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  const object = data?.getObject;
  const properties = object ? JSON.parse(object.properties) : {};

  return (
    <div>
      <h2>{object.title}</h2>
      {Object.entries(properties).map(([key, value]) => (
        <div key={key}>
          <strong>{key}:</strong> {String(value)}
        </div>
      ))}
    </div>
  );
}
```

### Pattern 3: Creating Objects with Forms

```tsx
import { DynamicForm } from '@ontology/forms';
import { useMutation, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const CREATE_OBJECT = gql`
  mutation CreateObject($objectType: String!, $properties: JSON!) {
    createObject(objectType: $objectType, properties: $properties) {
      objectId
    }
  }
`;

function CreatePersonPage() {
  const { client } = useOntology();
  const [createObject, { loading }] = useMutation(CREATE_OBJECT, { client });

  const properties = [
    { id: 'name', type: 'string', required: true },
    { id: 'email', type: 'string', required: true },
    { id: 'age', type: 'integer' },
  ];

  const handleSubmit = async (values: Record<string, any>) => {
    try {
      const { data } = await createObject({
        variables: {
          objectType: 'Person',
          properties: values,
        },
      });
      console.log('Created:', data.createObject.objectId);
    } catch (error) {
      console.error('Error creating object:', error);
    }
  };

  return (
    <div>
      <h1>Create Person</h1>
      <DynamicForm
        properties={properties}
        onSubmit={handleSubmit}
        submitLabel={loading ? 'Creating...' : 'Create'}
      />
    </div>
  );
}
```

### Pattern 4: Map Visualization with Time Filtering

```tsx
import { MapView, TimeSlider } from '@ontology/map';
import { useState } from 'react';

function MapPage() {
  const [selectedYear, setSelectedYear] = useState(2020);

  return (
    <div>
      <TimeSlider
        minYear={2010}
        maxYear={2020}
        selectedYear={selectedYear}
        onChange={setSelectedYear}
      />
      <div className="h-96 mt-4">
        <MapView
          objectType="Tract"
          geojsonProperty="geometry"
          valueProperty="total_population"
          selectedYear={selectedYear}
          onObjectClick={(objectId, properties) => {
            console.log('Clicked tract:', objectId, properties);
          }}
        />
      </div>
    </div>
  );
}
```

### Pattern 5: Graph Traversal

```tsx
import { GraphVisualization } from '@ontology/graph';

function NetworkPage() {
  const [selectedObjectId, setSelectedObjectId] = useState('person-123');

  return (
    <div>
      <GraphVisualization
        objectType="Person"
        objectId={selectedObjectId}
        linkTypes={['knows', 'works_with', 'reports_to']}
        maxHops={2}
        onNodeClick={(objectId) => {
          setSelectedObjectId(objectId);
        }}
      />
    </div>
  );
}
```

## GraphQL Integration

### Common Queries

**Search Objects**

```graphql
query SearchObjects($objectType: String!, $filters: [FilterInput!], $limit: Int) {
  searchObjects(objectType: $objectType, filters: $filters, limit: $limit) {
    objectType
    objectId
    title
    properties
  }
}
```

**Get Object**

```graphql
query GetObject($objectType: String!, $objectId: String!) {
  getObject(objectType: $objectType, objectId: $objectId) {
    objectType
    objectId
    title
    properties
  }
}
```

**Get Linked Objects**

```graphql
query GetLinkedObjects(
  $objectType: String!
  $objectId: String!
  $linkType: String!
) {
  getLinkedObjects(
    objectType: $objectType
    objectId: $objectId
    linkType: $linkType
  ) {
    objectType
    objectId
    title
    properties
  }
}
```

**Traverse Graph**

```graphql
query TraverseGraph(
  $objectType: String!
  $objectId: String!
  $linkTypes: [String!]!
  $maxHops: Int!
) {
  traverseGraph(
    objectType: $objectType
    objectId: $objectId
    linkTypes: $linkTypes
    maxHops: $maxHops
  ) {
    objectIds
    count
    aggregatedValue
  }
}
```

### Common Mutations

**Create Object**

```graphql
mutation CreateObject($objectType: String!, $properties: JSON!) {
  createObject(objectType: $objectType, properties: $properties) {
    objectId
  }
}
```

**Update Object**

```graphql
mutation UpdateObject(
  $objectType: String!
  $objectId: String!
  $properties: JSON!
) {
  updateObject(
    objectType: $objectType
    objectId: $objectId
    properties: $properties
  ) {
    objectId
  }
}
```

**Create Link**

```graphql
mutation CreateLink(
  $linkType: String!
  $sourceObjectType: String!
  $sourceObjectId: String!
  $targetObjectType: String!
  $targetObjectId: String!
  $properties: JSON
) {
  createLink(
    linkType: $linkType
    sourceObjectType: $sourceObjectType
    sourceObjectId: $sourceObjectId
    targetObjectType: $targetObjectType
    targetObjectId: $targetObjectId
    properties: $properties
  ) {
    linkId
  }
}
```

### Filter Syntax

Filters use the following structure:

```typescript
interface Filter {
  property: string;
  operator: 'equals' | 'contains' | 'greaterThan' | 'lessThan' | 'in' | 'between';
  value: string; // JSON stringified for complex values
}
```

Examples:

```typescript
// Exact match
{ property: 'status', operator: 'equals', value: JSON.stringify('active') }

// Contains (for strings)
{ property: 'name', operator: 'contains', value: JSON.stringify('John') }

// Greater than
{ property: 'age', operator: 'greaterThan', value: JSON.stringify(18) }

// In array
{ property: 'category', operator: 'in', value: JSON.stringify(['A', 'B', 'C']) }

// Between (for numbers/dates)
{ property: 'date', operator: 'between', value: JSON.stringify({ start: '2020-01-01', end: '2020-12-31' }) }
```

## Styling Guidelines

### Tailwind CSS

All applications use Tailwind CSS for styling. The template includes a default configuration.

**Common Patterns:**

```tsx
// Cards
<div className="bg-white p-6 rounded-lg shadow">
  {/* Content */}
</div>

// Buttons
<button className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
  Click me
</button>

// Forms
<input className="w-full px-3 py-2 border rounded" />

// Grid layouts
<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  {/* Items */}
</div>

// Navigation tabs
<button className={`border-b-2 ${
  active ? 'border-blue-500 text-gray-900' : 'border-transparent text-gray-500'
}`}>
  Tab
</button>
```

### Color Scheme

The framework uses a neutral color scheme with blue accents:

- Primary: `bg-blue-500`, `text-blue-500`
- Background: `bg-gray-50`, `bg-white`
- Text: `text-gray-900`, `text-gray-600`, `text-gray-500`
- Borders: `border-gray-300`

Customize colors in `tailwind.config.js`:

```js
export default {
  theme: {
    extend: {
      colors: {
        primary: '#your-color',
      },
    },
  },
}
```

## Testing

### Running Tests

Run tests for all packages and apps:

```bash
cd ui-framework
npm test
```

Run tests for a specific app:

```bash
cd ui-framework/apps/my-app
npm test
```

### Testing Components

Example test structure:

```tsx
import { render, screen } from '@testing-library/react';
import { OntologyProvider, createOntologyClient } from '@ontology/core';
import MyComponent from './MyComponent';

const client = createOntologyClient();

test('renders component', () => {
  render(
    <OntologyProvider client={client}>
      <MyComponent />
    </OntologyProvider>
  );
  expect(screen.getByText('Expected Text')).toBeInTheDocument();
});
```

## Deployment

### Building for Production

Build the application:

```bash
cd ui-framework/apps/my-app
npm run build
```

The build output will be in the `dist/` directory.

### Environment Variables

Configure production environment variables:

```bash
VITE_GRAPHQL_URL=https://your-api.example.com/graphql
```

### Serving the Build

The built application can be served by any static file server:

- **Nginx**: Configure to serve `dist/` directory
- **Apache**: Serve `dist/` directory
- **Vercel/Netlify**: Deploy `dist/` directory
- **Docker**: Use a simple nginx container

### Proxy Configuration

If your API is on a different domain, configure your server to proxy `/graphql` requests, or use the `VITE_GRAPHQL_URL` environment variable in your build.

## Best Practices

1. **Error Handling**: Always handle loading and error states in GraphQL queries
2. **Type Safety**: Use TypeScript types for object properties and GraphQL responses
3. **Component Reusability**: Extract common patterns into reusable components
4. **Performance**: Use pagination for large result sets, implement debouncing for search
5. **Accessibility**: Use semantic HTML, proper ARIA labels, keyboard navigation
6. **Security**: Never expose sensitive data in client-side code, validate all user inputs

## Getting Help

- Check existing example apps: `apps/census-example`
- Review package source code: `packages/*/src`
- Check GraphQL API documentation
- Review ontology definitions: `config/*.yaml`

## Next Steps

- Explore the census example application to see patterns in action
- Review the shared packages to understand available components
- Define your ontology in YAML format
- Build custom pages for your domain
- Extend with domain-specific visualizations and workflows




