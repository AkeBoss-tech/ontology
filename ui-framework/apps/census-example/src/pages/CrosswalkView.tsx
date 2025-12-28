import React, { useState } from 'react';
import { ObjectBrowser } from '@ontology/core';
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

export default function CrosswalkView() {
  const { client } = useOntology();
  const [sourceTractId, setSourceTractId] = useState<string>('');
  const [targetYear, setTargetYear] = useState<number>(2010);
  const [sourceYear, setSourceYear] = useState<number>(1990);

  const { data: sourceData } = useQuery(GET_OBJECT, {
    client,
    variables: {
      objectType: 'census_tract_vintage',
      objectId: sourceTractId,
    },
    skip: !sourceTractId,
  });

  const sourceTract = sourceData?.getObject;
  const sourceProperties = sourceTract ? JSON.parse(sourceTract.properties) : {};

  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-bold">Boundary Crosswalk</h1>
      
      <div className="grid grid-cols-2 gap-4">
        <div>
          <h2 className="text-lg font-semibold mb-2">Source Tract</h2>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-1">Source Year</label>
            <input
              type="number"
              value={sourceYear}
              onChange={(e) => setSourceYear(parseInt(e.target.value))}
              className="w-full px-3 py-2 border rounded"
            />
          </div>
          <ObjectBrowser
            objectType="census_tract_vintage"
            initialObjectId={sourceTractId}
          />
        </div>
        
        <div>
          <h2 className="text-lg font-semibold mb-2">Normalized to Target Year</h2>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-1">Target Year</label>
            <input
              type="number"
              value={targetYear}
              onChange={(e) => setTargetYear(parseInt(e.target.value))}
              className="w-full px-3 py-2 border rounded"
            />
          </div>
          
          {sourceTract && (
            <div className="p-4 bg-white border rounded">
              <h3 className="font-semibold mb-2">Normalization Result</h3>
              <p className="text-sm text-gray-600 mb-2">
                Source: {sourceTract.title} ({sourceYear})
              </p>
              <p className="text-sm text-gray-600 mb-4">
                Target: {targetYear}
              </p>
              
              {sourceProperties.total_population && (
                <div className="space-y-2">
                  <div>
                    <span className="font-medium">Source Population: </span>
                    <span>{sourceProperties.total_population}</span>
                  </div>
                  <div className="text-sm text-gray-500">
                    This value will be distributed to target tracts based on overlap percentage
                  </div>
                </div>
              )}
              
              <button
                className="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                onClick={() => {
                  // In a real implementation, this would call the crosswalk normalization
                  alert('Crosswalk normalization would be executed here');
                }}
              >
                Normalize Boundaries
              </button>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}



