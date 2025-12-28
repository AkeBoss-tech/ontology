import React from 'react';

export default function Home() {
  return (
    <div className="px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome to Vertex</h1>
        <p className="text-gray-600 text-lg">
          Graph visualization and relationship traversal. Explore object relationships
          through interactive graph visualizations, similar to Palantir Foundry's Vertex application.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Graph Traversal</h2>
          <p className="text-gray-600 mb-4">
            Start from any object and traverse relationships through link types.
            Control the depth of traversal with configurable hop limits.
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Relationship Exploration</h2>
          <p className="text-gray-600 mb-4">
            Visualize complex object networks. Follow multiple link types simultaneously
            to understand how objects are connected.
          </p>
        </div>
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-blue-900 mb-2">Features</h3>
        <ul className="list-disc list-inside space-y-2 text-blue-800">
          <li><strong>Multi-Link Traversal:</strong> Follow multiple link types in a single visualization</li>
          <li><strong>Configurable Depth:</strong> Control how many hops to traverse from the starting object</li>
          <li><strong>Interactive Exploration:</strong> Click nodes to explore further relationships</li>
          <li><strong>Relationship Discovery:</strong> Discover hidden connections between objects</li>
        </ul>
      </div>
    </div>
  );
}