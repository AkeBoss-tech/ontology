import React, { useState, useEffect } from 'react';
import { MapView, TimeSlider } from '@ontology/map';
import { useQuery, gql } from '@apollo/client';
import { useOntology, useVisualizationManager, VisualizationConfig } from '@ontology/core';
import LoadVisualizationButton from '../components/LoadVisualizationButton';

const GET_AVAILABLE_YEARS = gql`
  query GetAvailableYears($objectType: String!) {
    getAvailableYears(objectType: $objectType)
  }
`;

export default function TractMap() {
  const { client } = useOntology();
  const { saveVisualization } = useVisualizationManager();
  const [selectedYear, setSelectedYear] = useState<number | null>(null);
  const [filters, setFilters] = useState<any[]>([]);

  const { data: yearsData } = useQuery(GET_AVAILABLE_YEARS, {
    client,
    variables: { objectType: 'census_tract_vintage' },
  });

  const years = yearsData?.getAvailableYears || [];
  const defaultYear = years.length > 0 ? years[0] : null;

  React.useEffect(() => {
    if (defaultYear && !selectedYear) {
      setSelectedYear(defaultYear);
    }
  }, [defaultYear, selectedYear]);

  const handleSaveVisualization = () => {
    const vizId = saveVisualization({
      name: `Tract Map - ${selectedYear || 'All Years'}`,
      type: 'map',
      objectType: 'census_tract_vintage',
      filters: filters.length > 0 ? filters : undefined,
      properties: ['geoshape', 'total_population', 'median_household_income'],
      settings: {
        selectedYear,
        center: [-98.5795, 39.8283],
        zoom: 4,
        valueProperty: 'total_population',
      },
    });
    alert(`Visualization saved! ID: ${vizId}`);
  };

  const handleLoadVisualization = (config: VisualizationConfig) => {
    if (config.settings?.selectedYear) {
      setSelectedYear(config.settings.selectedYear);
    }
    if (config.filters) {
      setFilters(config.filters);
    }
    alert(`Loaded visualization: ${config.name}`);
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Census Tract Map</h1>
        <div className="flex gap-2">
          <LoadVisualizationButton
            onLoad={handleLoadVisualization}
            objectType="census_tract_vintage"
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
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div className="lg:col-span-2">
          <MapView
            objectType="census_tract_vintage"
            geojsonProperty="geoshape"
            valueProperty="total_population"
            center={[-98.5795, 39.8283]} // Center of USA
            zoom={4}
            onObjectClick={(objectId) => {
              console.log('Clicked tract:', objectId);
            }}
          />
        </div>
        
        <div>
          <TimeSlider
            objectType="census_tract_vintage"
            onYearChange={(year, objects) => {
              console.log(`Year ${year}: ${objects.length} tracts`);
            }}
          />
          
          <div className="mt-4 p-4 bg-white border rounded">
            <h3 className="font-semibold mb-2">Map Controls</h3>
            <p className="text-sm text-gray-600">
              Use the time slider to view different census vintages.
              Click on tracts to view details.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

