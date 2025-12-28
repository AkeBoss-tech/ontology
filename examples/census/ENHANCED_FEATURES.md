# Enhanced Census Data Explorer Features

## Overview

The census example now includes comprehensive data and interactive features for exploring census data at multiple geographic levels with advanced search and filtering capabilities.

## New Data

### Comprehensive Geographic Data
- **88 States** (44 states × 2 years: 2010, 2020)
- **140 Counties** across 10 states
- **310 Census Tracts** across 5 states
- **481 Households** linked to tracts
- **1,717 Persons** with diverse attributes
- **2,648 Relationship Links** connecting all entities

### Geographic Hierarchy
- **State → County → Tract → Household → Person**
- Full drill-down navigation supported
- Breadcrumb navigation for easy navigation back up

## New Features

### 1. Interactive Map (`/enhanced-map`)

**Features:**
- **Real Leaflet Map** with real GeoJSON rendering
- **Multi-level Geography**: Switch between State, County, and Tract views
- **Variable Selection**: Choose what data to visualize:
  - Total Population
  - Median Household Income
  - Median Rent
- **Time Slider**: View different census years (2010, 2020)
- **Drill-down Navigation**: Click on areas to drill down:
  - Click State → See Counties
  - Click County → See Tracts
- **Breadcrumb Navigation**: Navigate back up the hierarchy
- **Choropleth Coloring**: Areas colored by selected variable
- **Interactive Popups**: Click areas to see details
- **Auto-zoom**: Map automatically fits to data bounds

**Usage:**
1. Select geography level (State/County/Tract)
2. Choose variable to visualize
3. Select year using time slider
4. Click on areas to drill down
5. Use breadcrumbs to navigate back

### 2. Enhanced Person Search (`/person-search`)

**Features:**
- **Advanced Filtering**: Filter by:
  - Age
  - Sex
  - Race
  - Occupation
  - Industry
  - Wages
  - Education Attainment
  - Hours Worked
- **Text Search**: Search across all person attributes
- **Pagination**: Navigate through large result sets (50 per page)
- **Statistics Panel**: View aggregated statistics:
  - Average/Max/Min Wages
  - Average/Max Age
- **Save/Load Searches**: Save filter configurations for later
- **Detailed Results**: View comprehensive person information

**Usage:**
1. Add filters using the Filter Builder
2. Optionally use text search for quick filtering
3. View statistics for filtered results
4. Navigate through pages of results
5. Save search configurations for reuse

## Data Structure

### States
- FIPS codes for all 50 states
- Approximate geographic centers
- GeoJSON polygons for visualization
- Population and income data

### Counties
- Linked to states via state_fips
- Geographic boundaries
- Economic indicators

### Tracts
- Linked to counties
- Smallest geographic unit
- Detailed demographic data

### Persons
- Diverse attributes:
  - Demographics (age, sex, race)
  - Economic (wages, occupation, industry)
  - Education
  - Work hours
- Linked to households
- Filterable and searchable

## Technical Implementation

### Map Component
- Uses **Leaflet** for map rendering
- Renders GeoJSON polygons
- Choropleth coloring based on variable values
- Interactive popups and click handlers
- Auto-fitting bounds

### Search Component
- GraphQL queries with filters
- Client-side text search
- Pagination support
- Aggregation queries for statistics

### Data Generation
- Python script generates comprehensive sample data
- Includes realistic geographic shapes
- Diverse person attributes
- Proper hierarchical relationships

## Usage Examples

### Example 1: Explore State-Level Population
1. Go to Interactive Map
2. Select "State" geography level
3. Select "Total Population" variable
4. Select year 2020
5. View choropleth map of US states
6. Click on a state to see its counties

### Example 2: Find High-Earning Professionals
1. Go to Person Search
2. Add filters:
   - Age > 30
   - Wages > 75000
   - Education: "Bachelor's degree" or higher
   - Occupation: "Management" or "Engineering"
3. View results
4. Check statistics panel for averages
5. Save search for later

### Example 3: Compare Counties in a State
1. Go to Interactive Map
2. Select "County" geography level
3. Click on a state to filter to that state's counties
4. Change variable to "Median Household Income"
5. Compare counties visually
6. Click on a county to drill to tracts

## Benefits

1. **Comprehensive Data**: Large dataset for realistic exploration
2. **Multi-level Analysis**: Explore at state, county, and tract levels
3. **Interactive Exploration**: Click to drill down, navigate back
4. **Flexible Visualization**: Change variables and years easily
5. **Advanced Search**: Find specific people and groups
6. **Save/Load**: Preserve interesting visualizations and searches

## Next Steps

Potential enhancements:
- Real TIGER/Line shapefiles for accurate boundaries
- More census variables
- Comparison mode (side-by-side years)
- Export map images
- Share visualizations via URL
- Real-time data updates




