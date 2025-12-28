import React, { useState } from 'react';
import { FilterBuilder, Filter } from '@ontology/forms';
import { useQuery, gql } from '@apollo/client';
import { useOntology, useVisualizationManager, VisualizationConfig } from '@ontology/core';
import LoadVisualizationButton from '../components/LoadVisualizationButton';

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

export default function CohortBuilder() {
  const { client } = useOntology();
  const { saveVisualization } = useVisualizationManager();
  const [filters, setFilters] = useState<Filter[]>([]);
  const [objectType] = useState('pums_person');

  const availableProperties = [
    'age',
    'sex',
    'race_code',
    'occupation_code',
    'industry_code',
    'wages',
    'education_attainment',
  ];

  const { data, loading, refetch } = useQuery(SEARCH_OBJECTS, {
    client,
    variables: {
      objectType,
      filters: filters.length > 0 ? filters : undefined,
      limit: 100,
    },
  });

  const handleSearch = () => {
    refetch();
  };

  const handleSaveVisualization = () => {
    const vizId = saveVisualization({
      name: `Cohort - ${filters.length} filters`,
      type: 'table',
      objectType: 'pums_person',
      filters: filters.map((f) => ({
        property: f.property,
        operator: f.operator,
        value: f.value,
      })),
      properties: ['age', 'sex', 'race_code', 'wages', 'education_attainment'],
    });
    alert(`Visualization saved! ID: ${vizId}`);
  };

  const handleLoadVisualization = (config: VisualizationConfig) => {
    if (config.filters) {
      setFilters(config.filters as Filter[]);
      refetch();
    }
    alert(`Loaded visualization: ${config.name}`);
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Cohort Builder</h1>
        <div className="flex gap-2">
          <LoadVisualizationButton
            onLoad={handleLoadVisualization}
            objectType="pums_person"
            type="table"
          />
          <button
            onClick={handleSaveVisualization}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Save Visualization
          </button>
        </div>
      </div>
      
      <div className="grid grid-cols-2 gap-4">
        <div>
          <h2 className="text-lg font-semibold mb-2">Filters</h2>
          <FilterBuilder
            availableProperties={availableProperties}
            filters={filters}
            onChange={setFilters}
          />
          <button
            onClick={handleSearch}
            className="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Search Cohort
          </button>
        </div>
        
        <div>
          <h2 className="text-lg font-semibold mb-2">Results</h2>
          {loading && <div>Loading...</div>}
          {data && (
            <div className="space-y-2">
              <div className="p-2 bg-gray-100 rounded">
                Found {data.searchObjects.length} persons matching filters
              </div>
              <div className="max-h-96 overflow-y-auto space-y-1">
                {data.searchObjects.map((obj: any) => {
                  const props = JSON.parse(obj.properties);
                  return (
                    <div key={obj.objectId} className="p-2 border rounded text-sm">
                      <div className="font-medium">{obj.title}</div>
                      <div className="text-gray-500">
                        Age: {props.age}, Sex: {props.sex}, Wages: ${props.wages}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

