import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from './OntologyProvider';
import { ObjectSearch } from './ObjectSearch';

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

const GET_LINKED_OBJECTS = gql`
  query GetLinkedObjects($objectType: String!, $objectId: String!, $linkType: String!) {
    getLinkedObjects(objectType: $objectType, objectId: $objectId, linkType: $linkType) {
      objectType
      objectId
      title
      properties
    }
  }
`;

export interface ObjectBrowserProps {
  objectType: string;
  initialObjectId?: string;
}

export function ObjectBrowser({ objectType, initialObjectId }: ObjectBrowserProps) {
  const { client } = useOntology();
  const [selectedObjectId, setSelectedObjectId] = useState<string | undefined>(initialObjectId);
  const [selectedLinkType, setSelectedLinkType] = useState<string>('');

  const { data: objectData, loading: objectLoading } = useQuery(GET_OBJECT, {
    client,
    variables: {
      objectType,
      objectId: selectedObjectId,
    },
    skip: !selectedObjectId,
  });

  const { data: linkedData, loading: linkedLoading } = useQuery(GET_LINKED_OBJECTS, {
    client,
    variables: {
      objectType,
      objectId: selectedObjectId,
      linkType: selectedLinkType,
    },
    skip: !selectedObjectId || !selectedLinkType,
  });

  const object = objectData?.getObject;
  const properties = object ? JSON.parse(object.properties) : {};

  return (
    <div className="object-browser grid grid-cols-2 gap-4">
      <div>
        <h2 className="text-xl font-bold mb-4">Search {objectType}</h2>
        <ObjectSearch objectType={objectType} onSelectObject={setSelectedObjectId} />
      </div>

      <div>
        {selectedObjectId && (
          <>
            <h2 className="text-xl font-bold mb-4">Object Details</h2>
            {objectLoading ? (
              <div>Loading...</div>
            ) : object ? (
              <div className="border rounded p-4">
                <h3 className="text-lg font-semibold mb-2">{object.title}</h3>
                <div className="text-sm text-gray-500 mb-4">{object.objectType}</div>
                
                <div className="mb-4">
                  <h4 className="font-semibold mb-2">Properties</h4>
                  <div className="space-y-1">
                    {Object.entries(properties).map(([key, value]) => (
                      <div key={key} className="flex">
                        <span className="font-medium w-32">{key}:</span>
                        <span>{String(value)}</span>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="mb-4">
                  <h4 className="font-semibold mb-2">Links</h4>
                  <input
                    type="text"
                    value={selectedLinkType}
                    onChange={(e) => setSelectedLinkType(e.target.value)}
                    placeholder="Enter link type..."
                    className="w-full px-2 py-1 border rounded mb-2"
                  />
                  {selectedLinkType && (
                    <div>
                      {linkedLoading ? (
                        <div>Loading linked objects...</div>
                      ) : linkedData?.getLinkedObjects ? (
                        <div className="space-y-2">
                          {linkedData.getLinkedObjects.map((linked: any) => (
                            <div
                              key={linked.objectId}
                              className="p-2 border rounded cursor-pointer hover:bg-gray-50"
                              onClick={() => setSelectedObjectId(linked.objectId)}
                            >
                              {linked.title}
                            </div>
                          ))}
                        </div>
                      ) : null}
                    </div>
                  )}
                </div>
              </div>
            ) : (
              <div>Object not found</div>
            )}
          </>
        )}
      </div>
    </div>
  );
}



