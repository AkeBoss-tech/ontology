import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from './OntologyProvider';
import { ObjectSearch } from './ObjectSearch';

const GET_INTERFACES = gql`
  query GetInterfaces {
    getInterfaces {
      id
      displayName
      properties {
        id
        displayName
        type
        required
      }
      implementers {
        objectType
        count
      }
    }
  }
`;

const QUERY_BY_INTERFACE = gql`
  query QueryByInterface($interfaceId: String!, $filters: [FilterInput!], $limit: Int) {
    queryByInterface(interfaceId: $interfaceId, filters: $filters, limit: $limit) {
      objectType
      objectId
      title
      properties
    }
  }
`;

export interface InterfaceQueryProps {
  onSelectObject?: (objectId: string, objectType: string) => void;
}

export function InterfaceQuery({ onSelectObject }: InterfaceQueryProps) {
  const { client } = useOntology();
  const [selectedInterfaceId, setSelectedInterfaceId] = useState<string>();

  const { data: interfacesData, loading: interfacesLoading } = useQuery(GET_INTERFACES, {
    client,
  });

  const { data: objectsData, loading: objectsLoading } = useQuery(QUERY_BY_INTERFACE, {
    client,
    variables: {
      interfaceId: selectedInterfaceId,
      limit: 100,
    },
    skip: !selectedInterfaceId,
  });

  const interfaces = interfacesData?.getInterfaces || [];
  const objects = objectsData?.queryByInterface || [];
  const selectedInterface = interfaces.find((i: any) => i.id === selectedInterfaceId);

  return (
    <div className="interface-query space-y-6">
      <div>
        <h2 className="text-xl font-bold mb-4">Query by Interface</h2>
        {interfacesLoading ? (
          <div>Loading interfaces...</div>
        ) : interfaces.length === 0 ? (
          <div className="text-gray-500">No interfaces available</div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {interfaces.map((iface: any) => (
              <div
                key={iface.id}
                onClick={() => setSelectedInterfaceId(iface.id)}
                className={`p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 transition-colors ${
                  selectedInterfaceId === iface.id
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200'
                }`}
              >
                <div className="font-semibold">{iface.displayName}</div>
                <div className="text-sm text-gray-500 mt-1">{iface.id}</div>
                <div className="text-xs text-gray-400 mt-2">
                  {iface.properties.length} properties, {iface.implementers.length} implementers
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {selectedInterface && (
        <div className="bg-white p-6 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">
            Objects Implementing: {selectedInterface.displayName}
          </h3>

          {selectedInterface.properties.length > 0 && (
            <div className="mb-4">
              <h4 className="font-medium mb-2">Required Properties</h4>
              <div className="space-y-1">
                {selectedInterface.properties.map((prop: any) => (
                  <div key={prop.id} className="text-sm">
                    <span className="font-medium">{prop.displayName || prop.id}</span>
                    <span className="text-gray-500 ml-2">({prop.type})</span>
                    {prop.required && <span className="text-red-500 ml-1">*</span>}
                  </div>
                ))}
              </div>
            </div>
          )}

          {selectedInterface.implementers.length > 0 && (
            <div className="mb-4">
              <h4 className="font-medium mb-2">Implementing Object Types</h4>
              <div className="flex flex-wrap gap-2">
                {selectedInterface.implementers.map((impl: any) => (
                  <span
                    key={impl.objectType}
                    className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm"
                  >
                    {impl.objectType} ({impl.count})
                  </span>
                ))}
              </div>
            </div>
          )}

          {objectsLoading ? (
            <div>Loading objects...</div>
          ) : objects.length > 0 ? (
            <div className="space-y-2">
              <h4 className="font-medium mb-2">Results ({objects.length})</h4>
              {objects.map((obj: any) => (
                <div
                  key={obj.objectId}
                  onClick={() => onSelectObject?.(obj.objectId, obj.objectType)}
                  className="p-3 border rounded cursor-pointer hover:bg-gray-50"
                >
                  <div className="font-semibold">{obj.title}</div>
                  <div className="text-sm text-gray-500">{obj.objectType} • {obj.objectId}</div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-gray-500">No objects found implementing this interface</div>
          )}
        </div>
      )}
    </div>
  );
}

