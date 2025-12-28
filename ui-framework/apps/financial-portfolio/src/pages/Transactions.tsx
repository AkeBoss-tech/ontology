import React, { useState } from 'react';
import { ObjectSearch } from '@ontology/core';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

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

export default function Transactions() {
  const { client } = useOntology();
  const [selectedPortfolioId, setSelectedPortfolioId] = useState<string>();

  const { data, loading } = useQuery(SEARCH_OBJECTS, {
    client,
    variables: {
      objectType: 'Transaction',
      filters: selectedPortfolioId
        ? [
            {
              property: 'portfolio',
              operator: 'equals',
              value: JSON.stringify(selectedPortfolioId),
            },
          ]
        : undefined,
      limit: 100,
    },
  });

  const transactions = data?.searchObjects || [];

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Transactions</h1>
      
      <div className="mb-6 bg-white p-4 rounded-lg shadow">
        <h2 className="text-lg font-semibold mb-2">Filter by Portfolio</h2>
        <ObjectSearch
          objectType="Portfolio"
          onSelectObject={setSelectedPortfolioId}
        />
        {selectedPortfolioId && (
          <button
            onClick={() => setSelectedPortfolioId(undefined)}
            className="mt-2 px-3 py-1 text-sm bg-gray-200 rounded hover:bg-gray-300"
          >
            Clear filter
          </button>
        )}
      </div>

      {loading ? (
        <div>Loading transactions...</div>
      ) : (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Date
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Type
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Asset
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Quantity
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Price
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Total
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {transactions.map((transaction: any) => {
                  const props = JSON.parse(transaction.properties);
                  return (
                    <tr key={transaction.objectId} className="hover:bg-gray-50">
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {props.date || 'N/A'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {props.type || 'N/A'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {props.asset || 'N/A'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {props.quantity || 'N/A'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        {props.price ? `$${Number(props.price).toFixed(2)}` : 'N/A'}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                        {props.total ? `$${Number(props.total).toFixed(2)}` : 'N/A'}
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
          {transactions.length === 0 && (
            <div className="text-center py-8 text-gray-500">
              No transactions found. {selectedPortfolioId && 'Try selecting a different portfolio.'}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
