import React, { useState } from 'react';
import { ObjectSearch } from '@ontology/core';
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

export default function AssetSearch() {
  const { client } = useOntology();
  const [selectedAssetId, setSelectedAssetId] = useState<string>();

  const { data: assetData, loading: assetLoading } = useQuery(GET_OBJECT, {
    client,
    variables: {
      objectType: 'Asset',
      objectId: selectedAssetId,
    },
    skip: !selectedAssetId,
  });

  const { data: portfolioData } = useQuery(GET_LINKED_OBJECTS, {
    client,
    variables: {
      objectType: 'Asset',
      objectId: selectedAssetId,
      linkType: 'portfolio_holding',
    },
    skip: !selectedAssetId,
  });

  const asset = assetData?.getObject;
  const properties = asset ? JSON.parse(asset.properties) : {};
  const portfolios = portfolioData?.getLinkedObjects || [];

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Asset Search</h1>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-4">Search Assets</h2>
          <ObjectSearch
            objectType="Asset"
            onSelectObject={setSelectedAssetId}
          />
        </div>

        {selectedAssetId && (
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-xl font-semibold mb-4">Asset Details</h2>
            {assetLoading ? (
              <div>Loading...</div>
            ) : asset ? (
              <div className="space-y-4">
                <div>
                  <h3 className="text-lg font-semibold">{asset.title}</h3>
                  <div className="text-sm text-gray-500">{asset.objectType}</div>
                </div>

                <div>
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

                {portfolios.length > 0 && (
                  <div>
                    <h4 className="font-semibold mb-2">Held in Portfolios</h4>
                    <div className="space-y-2">
                      {portfolios.map((portfolio: any) => (
                        <div
                          key={portfolio.objectId}
                          className="p-2 border rounded hover:bg-gray-50"
                        >
                          <div className="font-medium">{portfolio.title}</div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <div>Asset not found</div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
