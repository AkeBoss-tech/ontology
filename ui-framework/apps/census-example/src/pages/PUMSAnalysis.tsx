import React, { useState } from 'react';
import { GraphVisualization } from '@ontology/graph';
import { ObjectBrowser, useVisualizationManager, VisualizationConfig } from '@ontology/core';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';
import LoadVisualizationButton from '../components/LoadVisualizationButton';

const TRAVERSE_GRAPH = gql`
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
    }
  }
`;

export default function PUMSAnalysis() {
  const { client } = useOntology();
  const { saveVisualization } = useVisualizationManager();
  const [selectedTractId, setSelectedTractId] = useState<string>('');
  const [maxHops, setMaxHops] = useState(3);

  const { data, loading } = useQuery(TRAVERSE_GRAPH, {
    client,
    variables: {
      objectType: 'census_tract_vintage',
      objectId: selectedTractId,
      linkTypes: ['tract_to_puma', 'puma_to_household', 'household_to_person'],
      maxHops,
    },
    skip: !selectedTractId,
  });

  const handleSaveVisualization = () => {
    const vizId = saveVisualization({
      name: `PUMS Analysis - ${selectedTractId || 'No Tract Selected'}`,
      type: 'graph',
      objectType: 'census_tract_vintage',
      settings: {
        startObjectId: selectedTractId,
        linkTypes: ['tract_to_puma', 'puma_to_household', 'household_to_person'],
        maxHops,
      },
    });
    alert(`Visualization saved! ID: ${vizId}`);
  };

  const handleLoadVisualization = (config: VisualizationConfig) => {
    if (config.settings?.startObjectId) {
      setSelectedTractId(config.settings.startObjectId);
    }
    if (config.settings?.maxHops) {
      setMaxHops(config.settings.maxHops);
    }
    alert(`Loaded visualization: ${config.name}`);
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">PUMS Analysis</h1>
        <div className="flex gap-2">
          <LoadVisualizationButton
            onLoad={handleLoadVisualization}
            objectType="census_tract_vintage"
            type="graph"
          />
          <button
            onClick={handleSaveVisualization}
            disabled={!selectedTractId}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed"
          >
            Save Visualization
          </button>
        </div>
      </div>
      
      <div className="grid grid-cols-2 gap-4">
        <div>
          <h2 className="text-lg font-semibold mb-2">Select Tract</h2>
          <ObjectBrowser
            objectType="census_tract_vintage"
            initialObjectId={selectedTractId}
          />
        </div>
        
        <div>
          <h2 className="text-lg font-semibold mb-2">Graph Traversal</h2>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-1">
              Max Hops: {maxHops}
            </label>
            <input
              type="range"
              min="1"
              max="5"
              value={maxHops}
              onChange={(e) => setMaxHops(parseInt(e.target.value))}
              className="w-full"
            />
          </div>
          
          {selectedTractId && (
            <GraphVisualization
              objectType="census_tract_vintage"
              objectId={selectedTractId}
              linkTypes={['tract_to_puma', 'puma_to_household', 'household_to_person']}
              maxHops={maxHops}
              onNodeClick={(objectId) => {
                console.log('Clicked node:', objectId);
              }}
            />
          )}
          
          {loading && <div>Loading traversal...</div>}
          {data && (
            <div className="mt-4 p-4 bg-white border rounded">
              <p className="text-sm">
                Traversed {data.traverseGraph.count} objects in {maxHops} hops
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

