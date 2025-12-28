import React, { useState, useEffect } from 'react';
import { MapView } from '@ontology/map';
import { TimeSlider } from '@ontology/map';
import { useQuery, gql } from '@apollo/client';
import { useOntology, useVisualizationManager } from '@ontology/core';
import LoadVisualizationButton from '../components/LoadVisualizationButton';
import { VisualizationConfig } from '@ontology/core';

const GET_AVAILABLE_YEARS = gql`
  query GetAvailableYears($objectType: String!) {
    getAvailableYears(objectType: $objectType)
  }
`;

const SEARCH_OBJECTS = gql`
  query SearchObjects($objectType: String!, $filters: [FilterInput!], $limit: Int) {
    searchObjects(objectType: $objectType, filters: $filters, limit: $limit) {
      objectType
      objectId
      title
      properties
    }
  }
`;

type GeographyLevel = 'state' | 'county' | 'tract';
type ObjectTypeMap = {
  state: string;
  county: string;
  tract: string;
};

const OBJECT_TYPES: ObjectTypeMap = {
  state: 'state_vintage',
  county: 'county_vintage',
  tract: 'census_tract_vintage',
};

// Note: If state_vintage is not in ontology yet, use census_tract_vintage as fallback

const VARIABLE_OPTIONS = [
  { value: 'total_population', label: 'Total Population' },
  { value: 'median_household_income', label: 'Median Household Income' },
  { value: 'median_rent', label: 'Median Rent' },
];

export default function EnhancedMap() {
  const { client } = useOntology();
  const { saveVisualization } = useVisualizationManager();
  const [selectedYear, setSelectedYear] = useState<number | null>(null);
  const [geographyLevel, setGeographyLevel] = useState<GeographyLevel>('state');
  const [selectedVariable, setSelectedVariable] = useState<string>('total_population');
  const [selectedObjectId, setSelectedObjectId] = useState<string | null>(null);
  const [drillDownPath, setDrillDownPath] = useState<Array<{ level: GeographyLevel; id: string; name: string }>>([]);
  const [filters, setFilters] = useState<any[]>([]);

  const objectType = OBJECT_TYPES[geographyLevel];

  const { data: yearsData } = useQuery(GET_AVAILABLE_YEARS, {
    client,
    variables: { objectType },
  });

  const { data: selectedObjectData } = useQuery(SEARCH_OBJECTS, {
    client,
    variables: {
      objectType,
      filters: selectedObjectId ? [{ property: 'geoid_year', operator: 'equals', value: selectedObjectId }] : [],
      limit: 1,
    },
    skip: !selectedObjectId,
  });

  const years = yearsData?.getAvailableYears || [];
  const defaultYear = years.length > 0 ? years[0] : null;

  useEffect(() => {
    if (defaultYear && !selectedYear) {
      setSelectedYear(defaultYear);
    }
  }, [defaultYear, selectedYear]);

  const handleObjectClick = (objectId: string, properties: any) => {
    setSelectedObjectId(objectId);
    
    // Add to drill-down path
    const name = properties.name || properties.geoid_year || objectId;
    setDrillDownPath((prev) => [...prev, { level: geographyLevel, id: objectId, name }]);

    // Auto-drill down if possible
    if (geographyLevel === 'state' && properties.county_fips) {
      // Can drill to county
      setTimeout(() => {
        setGeographyLevel('county');
        setFilters([{ property: 'state_fips', operator: 'equals', value: properties.state_fips }]);
        setSelectedObjectId(null);
      }, 500);
    } else if (geographyLevel === 'county' && properties.tract_ce) {
      // Can drill to tract
      setTimeout(() => {
        setGeographyLevel('tract');
        setFilters([
          { property: 'state_fips', operator: 'equals', value: properties.state_fips },
          { property: 'county_fips', operator: 'equals', value: properties.county_fips },
        ]);
        setSelectedObjectId(null);
      }, 500);
    }
  };

  const handleBreadcrumbClick = (index: number) => {
    const target = drillDownPath[index];
    setDrillDownPath(drillDownPath.slice(0, index + 1));
    
    // Reset to that level
    if (target.level === 'state') {
      setGeographyLevel('state');
      setFilters([]);
    } else if (target.level === 'county') {
      setGeographyLevel('county');
      const stateFips = target.id.split('_')[0].replace('05000US', '').substring(0, 2);
      setFilters([{ property: 'state_fips', operator: 'equals', value: stateFips }]);
    } else {
      setGeographyLevel('tract');
      const parts = target.id.split('_')[0].replace('14000US', '');
      const stateFips = parts.substring(0, 2);
      const countyFips = parts.substring(2, 5);
      setFilters([
        { property: 'state_fips', operator: 'equals', value: stateFips },
        { property: 'county_fips', operator: 'equals', value: `0${stateFips}${countyFips}` },
      ]);
    }
    setSelectedObjectId(null);
  };

  const handleSaveVisualization = () => {
    const vizId = saveVisualization({
      name: `${geographyLevel.charAt(0).toUpperCase() + geographyLevel.slice(1)} Map - ${selectedVariable} - ${selectedYear || 'All Years'}`,
      type: 'map',
      objectType,
      filters: filters.length > 0 ? filters : undefined,
      properties: ['geoshape', selectedVariable],
      settings: {
        selectedYear,
        geographyLevel,
        selectedVariable,
        drillDownPath,
      },
    });
    alert(`Visualization saved! ID: ${vizId}`);
  };

  const handleLoadVisualization = (config: VisualizationConfig) => {
    if (config.settings?.selectedYear) {
      setSelectedYear(config.settings.selectedYear);
    }
    if (config.settings?.geographyLevel) {
      setGeographyLevel(config.settings.geographyLevel as GeographyLevel);
    }
    if (config.settings?.selectedVariable) {
      setSelectedVariable(config.settings.selectedVariable);
    }
    if (config.settings?.drillDownPath) {
      setDrillDownPath(config.settings.drillDownPath as any);
    }
    if (config.filters) {
      setFilters(config.filters);
    }
    alert(`Loaded visualization: ${config.name}`);
  };

  // Calculate center based on selected object or default
  const getMapCenter = (): [number, number] => {
    if (selectedObjectData?.searchObjects?.[0]) {
      const props = typeof selectedObjectData.searchObjects[0].properties === 'string'
        ? JSON.parse(selectedObjectData.searchObjects[0].properties)
        : selectedObjectData.searchObjects[0].properties;
      if (props.centroid_lon && props.centroid_lat) {
        return [props.centroid_lon, props.centroid_lat];
      }
    }
    return [-98.5795, 39.8283]; // Center of USA
  };

  const getMapZoom = (): number => {
    if (geographyLevel === 'state') return 4;
    if (geographyLevel === 'county') return 6;
    return 8;
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Interactive Census Map</h1>
        <div className="flex gap-2">
          <LoadVisualizationButton
            onLoad={handleLoadVisualization}
            objectType={objectType}
            type="map"
          />
          <button
            onClick={handleSaveVisualization}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Save Visualization
          </button>
        </div>
      </div>

      {/* Breadcrumb Navigation */}
      {drillDownPath.length > 0 && (
        <div className="flex items-center gap-2 text-sm">
          <button
            onClick={() => {
              setDrillDownPath([]);
              setGeographyLevel('state');
              setFilters([]);
              setSelectedObjectId(null);
            }}
            className="text-blue-600 hover:underline"
          >
            United States
          </button>
          {drillDownPath.map((item, index) => (
            <React.Fragment key={index}>
              <span>/</span>
              <button
                onClick={() => handleBreadcrumbClick(index)}
                className="text-blue-600 hover:underline"
              >
                {item.name}
              </button>
            </React.Fragment>
          ))}
        </div>
      )}

      {/* Controls */}
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4">
        <div className="lg:col-span-3">
          <MapView
            objectType={objectType}
            geojsonProperty="geoshape"
            valueProperty={selectedVariable}
            center={getMapCenter()}
            zoom={getMapZoom()}
            onObjectClick={handleObjectClick}
            filters={filters}
            selectedYear={selectedYear || undefined}
          />
        </div>

        <div className="space-y-4">
          {/* Geography Level Selector */}
          <div className="p-4 bg-white border rounded">
            <h3 className="font-semibold mb-2">Geography Level</h3>
            <div className="space-y-2">
              {(['state', 'county', 'tract'] as GeographyLevel[]).map((level) => (
                <label key={level} className="flex items-center">
                  <input
                    type="radio"
                    name="geography"
                    value={level}
                    checked={geographyLevel === level}
                    onChange={() => {
                      setGeographyLevel(level);
                      setSelectedObjectId(null);
                      if (level === 'state') {
                        setFilters([]);
                        setDrillDownPath([]);
                      }
                    }}
                    className="mr-2"
                  />
                  <span className="capitalize">{level}</span>
                </label>
              ))}
            </div>
          </div>

          {/* Variable Selector */}
          <div className="p-4 bg-white border rounded">
            <h3 className="font-semibold mb-2">Variable</h3>
            <select
              value={selectedVariable}
              onChange={(e) => setSelectedVariable(e.target.value)}
              className="w-full px-3 py-2 border rounded"
            >
              {VARIABLE_OPTIONS.map((opt) => (
                <option key={opt.value} value={opt.value}>
                  {opt.label}
                </option>
              ))}
            </select>
          </div>

          {/* Time Slider */}
          <div className="p-4 bg-white border rounded">
            <h3 className="font-semibold mb-2">Year</h3>
            <TimeSlider
              objectType={objectType}
              onYearChange={(year) => {
                setSelectedYear(year);
                setSelectedObjectId(null);
              }}
            />
          </div>

          {/* Selected Object Info */}
          {selectedObjectData?.searchObjects?.[0] && (
            <div className="p-4 bg-white border rounded">
              <h3 className="font-semibold mb-2">Selected Area</h3>
              {(() => {
                const props = typeof selectedObjectData.searchObjects[0].properties === 'string'
                  ? JSON.parse(selectedObjectData.searchObjects[0].properties)
                  : selectedObjectData.searchObjects[0].properties;
                return (
                  <div className="text-sm space-y-1">
                    <div><strong>Name:</strong> {props.name || selectedObjectData.searchObjects[0].title}</div>
                    <div><strong>Population:</strong> {props.total_population?.toLocaleString() || 'N/A'}</div>
                    <div><strong>Income:</strong> ${props.median_household_income?.toLocaleString() || 'N/A'}</div>
                    <div><strong>Year:</strong> {props.year || 'N/A'}</div>
                  </div>
                );
              })()}
            </div>
          )}

          {/* Instructions */}
          <div className="p-4 bg-gray-50 border rounded text-sm">
            <h3 className="font-semibold mb-2">Instructions</h3>
            <ul className="list-disc list-inside space-y-1 text-gray-600">
              <li>Click on areas to drill down</li>
              <li>Use breadcrumbs to navigate back</li>
              <li>Change variable to see different data</li>
              <li>Use time slider to view different years</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

