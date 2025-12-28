import React, { useState } from 'react';
import { ObjectSearch } from '@ontology/core';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

// Note: This query may need to be adjusted based on your GraphQL schema
// If getObjectTypes is not available, use a hardcoded list or fetch from ontology definition
const GET_OBJECT_TYPES = gql`
  query GetObjectTypes {
    getObjectTypes {
      id
      displayName
    }
  }
`;

interface ExplorerProps {
  onSelectObject: (obj: { type: string; id: string }) => void;
}

export default function Explorer({ onSelectObject }: ExplorerProps) {
  const { client } = useOntology();
  const [selectedObjectType, setSelectedObjectType] = useState<string>('');

  if (!client) {
    return <div>Ontology client not available</div>;
  }

  // For now, use a hardcoded list or fetch from a known endpoint
  // In production, this would come from the GraphQL schema
  const objectTypes = [
    { id: 'Person', displayName: 'Person' },
    { id: 'Asset', displayName: 'Asset' },
    { id: 'Location', displayName: 'Location' },
    { id: 'Transaction', displayName: 'Transaction' },
    { id: 'Portfolio', displayName: 'Portfolio' },
  ];
  
  // Uncomment when getObjectTypes query is available:
  // const { data: typesData, loading: typesLoading } = useQuery(GET_OBJECT_TYPES, { client });
  // const objectTypes = typesData?.getObjectTypes || [];
  const typesLoading = false;

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Object Explorer</h1>
      
      <div className="bg-white p-6 rounded-lg shadow mb-6">
        <h2 className="text-lg font-semibold mb-4">Select Object Type</h2>
        {typesLoading ? (
          <div>Loading object types...</div>
        ) : objectTypes.length > 0 ? (
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {objectTypes.map((type: any) => (
              <button
                key={type.id}
                onClick={() => setSelectedObjectType(type.id)}
                className={`p-4 border-2 rounded-lg text-left hover:bg-gray-50 transition-colors ${
                  selectedObjectType === type.id
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200'
                }`}
              >
                <div className="font-semibold">{type.displayName || type.id}</div>
                <div className="text-sm text-gray-500 mt-1">{type.id}</div>
              </button>
            ))}
          </div>
        ) : (
          <div className="text-gray-500">No object types available</div>
        )}
      </div>

      {selectedObjectType && (
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">
            Browse {objectTypes.find((t: any) => t.id === selectedObjectType)?.displayName || selectedObjectType}
          </h2>
          <ObjectSearch
            objectType={selectedObjectType}
            onSelectObject={(objectId) => {
              onSelectObject({ type: selectedObjectType, id: objectId });
            }}
          />
        </div>
      )}

      {!selectedObjectType && objectTypes.length > 0 && (
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
          <p className="text-blue-800">
            Select an object type above to start exploring objects in your ontology.
          </p>
        </div>
      )}
    </div>
  );
}
