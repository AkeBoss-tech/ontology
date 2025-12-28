import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology, BreadcrumbNav, PropertyMetadataDisplay, DataTable, parseProperties } from '@ontology/core';

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

interface ObjectDetailProps {
  objectType: string;
  objectId: string;
  onBack: () => void;
}

export default function ObjectDetail({ objectType, objectId, onBack }: ObjectDetailProps) {
  const { client } = useOntology();
  const [selectedLinkType, setSelectedLinkType] = useState<string>('');

  if (!client) {
    return <div>Ontology client not available</div>;
  }

  const { data: objectData, loading: objectLoading } = useQuery(GET_OBJECT, {
    client,
    variables: {
      objectType,
      objectId,
    },
  });

  const { data: linkedData, loading: linkedLoading } = useQuery(GET_LINKED_OBJECTS, {
    client,
    variables: {
      objectType,
      objectId,
      linkType: selectedLinkType,
    },
    skip: !selectedLinkType,
  });

  const object = objectData?.getObject;
  const properties = object ? parseProperties(object.properties) : {};
  const linkedObjects = linkedData?.getLinkedObjects || [];

  const breadcrumbs = [
    { label: 'Explorer', onClick: onBack },
    { label: objectType },
    { label: object?.title || objectId },
  ];

  return (
    <div className="px-4 py-8">
      <BreadcrumbNav items={breadcrumbs} className="mb-4" />

      {objectLoading ? (
        <div>Loading object...</div>
      ) : object ? (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          {/* Header */}
          <div className="bg-gray-50 px-6 py-4 border-b">
            <h1 className="text-2xl font-bold">{object.title}</h1>
            <div className="text-sm text-gray-500 mt-1">
              {object.objectType} • {object.objectId}
            </div>
          </div>

          {/* Properties */}
          <div className="p-6">
            <h2 className="text-lg font-semibold mb-4">Properties</h2>
            <div className="space-y-4">
              {Object.entries(properties).map(([key, value]) => (
                <div key={key} className="p-4 border rounded-lg bg-gray-50">
                  <PropertyMetadataDisplay
                    property={{
                      id: key,
                      displayName: key,
                      type: typeof value === 'string' ? 'string' : typeof value === 'number' ? 'number' : 'unknown',
                    }}
                    value={value}
                  />
                </div>
              ))}
            </div>
          </div>

          {/* Links */}
          <div className="p-6 border-t">
            <h2 className="text-lg font-semibold mb-4">Linked Objects</h2>
            <div className="mb-4">
              <input
                type="text"
                value={selectedLinkType}
                onChange={(e) => setSelectedLinkType(e.target.value)}
                placeholder="Enter link type to explore (e.g., 'knows', 'owns', 'located_in')"
                className="w-full px-4 py-2 border rounded"
              />
            </div>
            {selectedLinkType && (
              <div>
                {linkedLoading ? (
                  <div>Loading linked objects...</div>
                ) : linkedObjects.length > 0 ? (
                  <DataTable
                    data={linkedObjects}
                    columns={[
                      { key: 'title', label: 'Title', sortable: true },
                      { key: 'objectType', label: 'Type', sortable: true },
                      { key: 'objectId', label: 'ID', sortable: true },
                    ]}
                    keyExtractor={(row) => row.objectId}
                    onRowClick={(row) => {
                      // Navigate to linked object detail
                      window.location.hash = `#/object/${row.objectType}/${row.objectId}`;
                    }}
                    filterable
                    exportable
                    exportFilename={`${objectType}_${objectId}_linked_${selectedLinkType}`}
                  />
                ) : (
                  <div className="text-gray-500">No linked objects found for link type: {selectedLinkType}</div>
                )}
              </div>
            )}
          </div>
        </div>
      ) : (
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <p className="text-red-800">Object not found</p>
        </div>
      )}
    </div>
  );
}
