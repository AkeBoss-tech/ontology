import React, { useState } from 'react';
import { FilterBuilder, Filter } from '@ontology/forms';
import { useQuery, gql } from '@apollo/client';
import { useOntology, useVisualizationManager, VisualizationConfig } from '@ontology/core';
import LoadVisualizationButton from '../components/LoadVisualizationButton';

const SEARCH_OBJECTS = gql`
  query SearchObjects($objectType: String!, $filters: [FilterInput!], $limit: Int, $offset: Int) {
    searchObjects(objectType: $objectType, filters: $filters, limit: $limit, offset: $offset) {
      objectType
      objectId
      title
      properties
    }
  }
`;

const AGGREGATE_OBJECTS = gql`
  query AggregateObjects(
    $objectType: String!
    $aggregations: [AggregationInput!]!
    $filters: [FilterInput!]
    $groupBy: [String!]
  ) {
    aggregateObjects(
      objectType: $objectType
      aggregations: $aggregations
      filters: $filters
      groupBy: $groupBy
    ) {
      rows
      total
    }
  }
`;

export default function EnhancedPersonSearch() {
  const { client } = useOntology();
  const { saveVisualization } = useVisualizationManager();
  const [filters, setFilters] = useState<Filter[]>([]);
  const [objectType] = useState('pums_person');
  const [searchText, setSearchText] = useState('');
  const [page, setPage] = useState(0);
  const [pageSize] = useState(50);
  const [showAggregations, setShowAggregations] = useState(false);

  const availableProperties = [
    'age',
    'sex',
    'race_code',
    'occupation_code',
    'industry_code',
    'wages',
    'education_attainment',
    'hours_worked',
  ];

  const { data, loading, refetch } = useQuery(SEARCH_OBJECTS, {
    client,
    variables: {
      objectType,
      filters: filters.length > 0 ? filters : undefined,
      limit: pageSize,
      offset: page * pageSize,
    },
  });

  const { data: aggData } = useQuery(AGGREGATE_OBJECTS, {
    client,
    variables: {
      objectType,
      aggregations: [
        { property: 'wages', operation: 'avg' },
        { property: 'wages', operation: 'max' },
        { property: 'wages', operation: 'min' },
        { property: 'age', operation: 'avg' },
        { property: 'age', operation: 'max' },
      ],
      filters: filters.length > 0 ? filters : undefined,
    },
    skip: !showAggregations,
  });

  const handleSearch = () => {
    setPage(0);
    refetch();
  };

  const handleSaveVisualization = () => {
    const vizId = saveVisualization({
      name: `Person Search - ${filters.length} filters`,
      type: 'table',
      objectType: 'pums_person',
      filters: filters.map((f) => ({
        property: f.property,
        operator: f.operator,
        value: f.value,
      })),
      properties: availableProperties,
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

  // Filter results by search text
  const filteredResults = data?.searchObjects?.filter((obj: any) => {
    if (!searchText) return true;
    const props = typeof obj.properties === 'string' ? JSON.parse(obj.properties) : obj.properties;
    const searchLower = searchText.toLowerCase();
    return (
      obj.title?.toLowerCase().includes(searchLower) ||
      props.race_code?.toLowerCase().includes(searchLower) ||
      props.occupation_code?.toLowerCase().includes(searchLower) ||
      props.education_attainment?.toLowerCase().includes(searchLower) ||
      props.sex?.toLowerCase().includes(searchLower)
    );
  }) || [];

  const totalResults = data?.searchObjects?.length || 0;
  const totalPages = Math.ceil(totalResults / pageSize);

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Person Search & Analysis</h1>
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
            Save Search
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
        {/* Filters Panel */}
        <div className="lg:col-span-1">
          <div className="p-4 bg-white border rounded space-y-4">
            <h2 className="text-lg font-semibold">Filters</h2>
            <FilterBuilder
              availableProperties={availableProperties}
              filters={filters}
              onChange={setFilters}
            />
            <button
              onClick={handleSearch}
              className="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              Apply Filters
            </button>
            <button
              onClick={() => {
                setFilters([]);
                setPage(0);
                refetch();
              }}
              className="w-full px-4 py-2 bg-gray-300 rounded hover:bg-gray-400"
            >
              Clear All
            </button>
          </div>

          {/* Aggregations */}
          <div className="mt-4 p-4 bg-white border rounded">
            <div className="flex justify-between items-center mb-2">
              <h3 className="font-semibold">Statistics</h3>
              <button
                onClick={() => setShowAggregations(!showAggregations)}
                className="text-sm text-blue-600 hover:underline"
              >
                {showAggregations ? 'Hide' : 'Show'}
              </button>
            </div>
            {showAggregations && aggData && (
              <div className="text-sm space-y-1">
                {(() => {
                  try {
                    const rows = JSON.parse(aggData.aggregateObjects.rows);
                    return rows.map((row: any, idx: number) => (
                      <div key={idx} className="text-gray-600">
                        {Object.entries(row).map(([key, value]) => (
                          <div key={key}>
                            <strong>{key}:</strong> {typeof value === 'number' ? value.toLocaleString() : value}
                          </div>
                        ))}
                      </div>
                    ));
                  } catch (e) {
                    return <div>Error parsing aggregations</div>;
                  }
                })()}
              </div>
            )}
          </div>
        </div>

        {/* Results Panel */}
        <div className="lg:col-span-2">
          <div className="p-4 bg-white border rounded space-y-4">
            {/* Search Bar */}
            <div>
              <input
                type="text"
                value={searchText}
                onChange={(e) => setSearchText(e.target.value)}
                placeholder="Search by name, race, occupation, education..."
                className="w-full px-3 py-2 border rounded"
              />
            </div>

            {/* Results Count */}
            <div className="flex justify-between items-center">
              <div>
                <strong>{filteredResults.length}</strong> persons found
                {filters.length > 0 && ` (with ${filters.length} filter${filters.length > 1 ? 's' : ''})`}
              </div>
              <div className="text-sm text-gray-500">
                Page {page + 1} of {totalPages || 1}
              </div>
            </div>

            {/* Results List */}
            {loading && <div className="text-center py-8">Loading...</div>}
            {!loading && filteredResults.length === 0 && (
              <div className="text-center py-8 text-gray-500">
                No persons found. Try adjusting your filters.
              </div>
            )}
            {!loading && filteredResults.length > 0 && (
              <>
                <div className="max-h-96 overflow-y-auto space-y-2">
                  {filteredResults.map((obj: any) => {
                    const props = typeof obj.properties === 'string' ? JSON.parse(obj.properties) : obj.properties;
                    return (
                      <div key={obj.objectId} className="p-3 border rounded hover:bg-gray-50">
                        <div className="flex justify-between items-start">
                          <div className="flex-1">
                            <div className="font-medium">{obj.title || obj.objectId}</div>
                            <div className="text-sm text-gray-600 mt-1 space-y-1">
                              <div className="flex gap-4">
                                <span><strong>Age:</strong> {props.age || 'N/A'}</span>
                                <span><strong>Sex:</strong> {props.sex || 'N/A'}</span>
                                <span><strong>Race:</strong> {props.race_code || 'N/A'}</span>
                              </div>
                              <div className="flex gap-4">
                                <span><strong>Occupation:</strong> {props.occupation_code || 'N/A'}</span>
                                <span><strong>Education:</strong> {props.education_attainment || 'N/A'}</span>
                              </div>
                              <div className="flex gap-4">
                                <span><strong>Wages:</strong> ${props.wages?.toLocaleString() || '0'}</span>
                                <span><strong>Hours:</strong> {props.hours_worked || '0'}/week</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      </div>
                    );
                  })}
                </div>

                {/* Pagination */}
                {totalPages > 1 && (
                  <div className="flex justify-center gap-2 mt-4">
                    <button
                      onClick={() => setPage(Math.max(0, page - 1))}
                      disabled={page === 0}
                      className="px-3 py-1 border rounded disabled:opacity-50"
                    >
                      Previous
                    </button>
                    <span className="px-3 py-1">
                      Page {page + 1} of {totalPages}
                    </span>
                    <button
                      onClick={() => setPage(Math.min(totalPages - 1, page + 1))}
                      disabled={page >= totalPages - 1}
                      className="px-3 py-1 border rounded disabled:opacity-50"
                    >
                      Next
                    </button>
                  </div>
                )}
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}



