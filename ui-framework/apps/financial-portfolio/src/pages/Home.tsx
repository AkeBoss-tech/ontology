import React from 'react';

export default function Home() {
  return (
    <div className="px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome to Financial Portfolio Manager</h1>
        <p className="text-gray-600 text-lg">
          A Palantir-like application for managing financial portfolios, assets, and transactions.
          This example demonstrates object browsing, search, relationships, and graph visualizations.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Portfolios</h2>
          <p className="text-gray-600 mb-4">
            Browse portfolios and explore their holdings. View portfolio details and linked assets.
          </p>
          <div className="text-sm text-blue-600 font-medium">
            → Browse Portfolios
          </div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Asset Search</h2>
          <p className="text-gray-600 mb-4">
            Search for assets and view their details, including which portfolios hold them.
          </p>
          <div className="text-sm text-blue-600 font-medium">
            → Search Assets
          </div>
        </div>
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Transactions</h2>
          <p className="text-gray-600 mb-4">
            View transaction history. Filter by portfolio to see specific transaction records.
          </p>
          <div className="text-sm text-blue-600 font-medium">
            → View Transactions
          </div>
        </div>
      </div>

      <div className="mt-8 bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-blue-900 mb-2">Getting Started</h3>
        <p className="text-blue-800 mb-4">
          This application demonstrates key Palantir Foundry patterns:
        </p>
        <ul className="list-disc list-inside space-y-2 text-blue-800">
          <li><strong>Object Browsing:</strong> Navigate through portfolios and view linked objects</li>
          <li><strong>Object Search:</strong> Search for assets with filtering capabilities</li>
          <li><strong>Relationship Traversal:</strong> View which portfolios hold specific assets</li>
          <li><strong>Query Filtering:</strong> Filter transactions by portfolio</li>
        </ul>
        <p className="text-blue-800 mt-4">
          To use this app, ensure your backend ontology defines: <code className="bg-blue-100 px-2 py-1 rounded">Portfolio</code>, <code className="bg-blue-100 px-2 py-1 rounded">Asset</code>, and <code className="bg-blue-100 px-2 py-1 rounded">Transaction</code> object types with appropriate link types.
        </p>
      </div>
    </div>
  );
}
