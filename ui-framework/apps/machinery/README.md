# Machinery

Asset and equipment management application

## Getting Started

### Prerequisites

- Node.js 18+ and npm
- The ontology backend server running (default: http://localhost:8080)

### Installation

Install dependencies:

```bash
npm install
```

### Configuration

Create a `.env` file in the app directory to configure the GraphQL endpoint:

```bash
VITE_GRAPHQL_URL=http://localhost:8080/graphql
```

### Development

Start the development server:

```bash
npm run dev
```

The app will be available at http://localhost:3000

### Build

Build for production:

```bash
npm run build
```

Preview the production build:

```bash
npm run preview
```

## Customization

1. Update the object types in `src/pages/Search.tsx` and `src/pages/Browse.tsx` to match your ontology
2. Add new pages in `src/pages/` and update the navigation in `src/App.tsx`
3. Use components from `@ontology/core`, `@ontology/forms`, `@ontology/graph`, and `@ontology/map` packages
4. Customize styling using Tailwind CSS classes

## Available Components

- `@ontology/core`: ObjectBrowser, ObjectSearch, PropertyEditor, VisualizationManager
- `@ontology/forms`: DynamicForm, FilterBuilder
- `@ontology/graph`: GraphVisualization
- `@ontology/map`: MapView, TimeSlider

See the APPLICATION_GUIDE.md in the ui-framework directory for detailed documentation.
