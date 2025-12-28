# Financial Portfolio Manager

A Palantir-like application for managing financial portfolios, assets, and transactions

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

## Example Ontology

This application expects the following object types in your ontology:

- **Portfolio**: Represents a financial portfolio
  - Properties: name (string), description (string), owner (string), created_date (datetime)
  
- **Asset**: Represents a financial asset (stock, bond, etc.)
  - Properties: symbol (string), name (string), asset_type (string), current_price (double)
  
- **Transaction**: Represents a portfolio transaction
  - Properties: date (datetime), type (string), quantity (double), price (double), total (double), portfolio (object_reference), asset (object_reference)

- **Link Types**:
  - `portfolio_holding`: Links Portfolio to Asset (properties: quantity, purchase_price)

## Customization

1. The app demonstrates patterns using Portfolio, Asset, and Transaction object types
2. Update object types in `src/pages/` to match your specific ontology
3. Add new pages in `src/pages/` and update the navigation in `src/App.tsx`
4. Use components from `@ontology/core`, `@ontology/forms`, `@ontology/graph`, and `@ontology/map` packages
5. Customize styling using Tailwind CSS classes

## Available Components

- `@ontology/core`: ObjectBrowser, ObjectSearch, PropertyEditor, VisualizationManager
- `@ontology/forms`: DynamicForm, FilterBuilder
- `@ontology/graph`: GraphVisualization
- `@ontology/map`: MapView, TimeSlider

See the APPLICATION_GUIDE.md in the ui-framework directory for detailed documentation.
