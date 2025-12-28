import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from './OntologyProvider';
import { parseProperties } from './utils';

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

export interface ObjectSearchProps {
  objectType: string;
  onSelectObject?: (objectId: string) => void;
}

export function ObjectSearch({ objectType, onSelectObject }: ObjectSearchProps) {
  const { client } = useOntology();
  const [searchTerm, setSearchTerm] = useState('');
  const [filters, setFilters] = useState<any[]>([]);

  const { data, loading, error, refetch } = useQuery(SEARCH_OBJECTS, {
    client,
    variables: {
      objectType,
      filters: filters.length > 0 ? filters : undefined,
      limit: 20,
    },
    skip: !objectType,
  });

  const handleSearch = () => {
    if (searchTerm) {
      setFilters([{
        property: 'title',
        operator: 'contains',
        value: JSON.stringify(searchTerm),
      }]);
    } else {
      setFilters([]);
    }
    refetch();
  };

  return (
    <div className="object-search">
      <div className="flex gap-2 mb-4">
        <input
          type="text"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          placeholder="Search objects..."
          className="flex-1 px-4 py-2 border rounded"
        />
        <button
          onClick={handleSearch}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Search
        </button>
      </div>

      {loading && <div>Loading...</div>}
      {error && <div className="text-red-500">Error: {error.message}</div>}

      {data && (
        <div className="space-y-2">
          {data.searchObjects.map((obj: any) => (
            <div
              key={obj.objectId}
              onClick={() => onSelectObject?.(obj.objectId)}
              className="p-3 border rounded cursor-pointer hover:bg-gray-50"
            >
              <div className="font-semibold">{obj.title}</div>
              <div className="text-sm text-gray-500">{obj.objectType}</div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}




