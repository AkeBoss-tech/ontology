# Visualization Save/Load Features

## Overview

The census example now includes comprehensive visualization management features that allow you to save, load, export, and import visualization configurations locally.

## Features

### 1. Save Visualizations
- **Location**: Available on all visualization pages (Tract Map, PUMS Analysis, Cohort Builder)
- **What's Saved**:
  - Visualization type (map, graph, table, chart)
  - Object type being visualized
  - Filters applied
  - Properties displayed
  - Settings (year, center, zoom, max hops, etc.)
  - Timestamp (created/updated)

### 2. Load Visualizations
- **Quick Load Button**: "Load Saved" button on each page
  - Filters by visualization type and object type
  - Shows count of available visualizations
  - Click to see list and load
- **Visualization Manager Page**: Full management interface
  - View all saved visualizations
  - Load, delete, or update visualizations
  - Export/import functionality

### 3. Export/Import
- **Export**: Download all visualizations as JSON file
- **Import**: 
  - Paste JSON directly
  - Upload JSON file
  - Merges with existing visualizations

### 4. Local Storage
- All visualizations are automatically saved to browser localStorage
- Persists across browser sessions
- No server required - completely local

## Usage

### Saving a Visualization

1. Navigate to any visualization page (Tract Map, PUMS Analysis, Cohort Builder)
2. Configure your visualization:
   - Set filters
   - Adjust settings (year, max hops, etc.)
   - Select properties to display
3. Click "Save Visualization" button
4. Enter a name for the visualization
5. Click "Save"

### Loading a Visualization

**Method 1: Quick Load Button**
1. On any visualization page, click "Load Saved (N)" button
2. Select a visualization from the list
3. Click "Load" - the visualization settings will be restored

**Method 2: Visualization Manager**
1. Navigate to "Visualizations" page
2. Find the visualization you want
3. Click "Load" button

### Exporting Visualizations

1. Go to "Visualizations" page
2. Click "Export All" button
3. A JSON file will be downloaded with all your visualizations

### Importing Visualizations

1. Go to "Visualizations" page
2. Click "Import" button
3. Either:
   - Paste JSON into the text area, OR
   - Upload a JSON file
4. Click "Import"
5. Visualizations will be merged with existing ones

### Managing Visualizations

On the "Visualizations" page, you can:
- View all saved visualizations with metadata
- Load a visualization (restores its settings)
- Delete a visualization
- Export all visualizations
- Import visualizations from file or JSON

## Visualization Configuration Structure

Each saved visualization contains:

```typescript
{
  id: string;                    // Unique identifier
  name: string;                  // User-provided name
  type: 'map' | 'graph' | 'table' | 'chart';
  objectType: string;            // e.g., 'census_tract_vintage'
  filters?: Array<{              // Applied filters
    property: string;
    operator: string;
    value: any;
  }>;
  properties?: string[];         // Properties to display
  settings?: {                   // Type-specific settings
    selectedYear?: number;
    center?: [number, number];
    zoom?: number;
    maxHops?: number;
    startObjectId?: string;
    linkTypes?: string[];
    // ... other settings
  };
  createdAt: string;             // ISO timestamp
  updatedAt: string;             // ISO timestamp
}
```

## Examples

### Example 1: Save a Map Visualization
1. Go to Tract Map
2. Set year to 2010
3. Click "Save Visualization"
4. Name it "2010 Census Tracts"
5. Later, load it to quickly view 2010 data

### Example 2: Save a Cohort Query
1. Go to Cohort Builder
2. Add filters: age > 30, wages > 50000
3. Click "Save Visualization"
4. Name it "High Earners 30+"
5. Load it later to quickly rerun the same query

### Example 3: Export and Share
1. Create several visualizations
2. Go to Visualizations page
3. Click "Export All"
4. Share the JSON file with colleagues
5. They can import it to get the same visualizations

## Technical Details

### Storage
- Uses browser `localStorage` API
- Key: `ontology_visualizations`
- Automatically saves on every change
- Automatically loads on app start

### Data Format
- JSON format for easy sharing
- Human-readable structure
- Includes all necessary metadata

### Integration
- Integrated into all visualization pages
- Uses React Context for state management
- Type-safe with TypeScript

## Benefits

1. **Learn from Data**: Save interesting visualizations to revisit later
2. **Share Insights**: Export and share visualization configurations
3. **Quick Access**: Load saved visualizations with one click
4. **No Server Required**: Everything stored locally
5. **Persistent**: Visualizations survive browser restarts

## Future Enhancements

Potential improvements:
- Visualization templates
- Scheduled exports
- Cloud sync (optional)
- Visualization sharing via URL
- Version history
- Visualization collections/folders



