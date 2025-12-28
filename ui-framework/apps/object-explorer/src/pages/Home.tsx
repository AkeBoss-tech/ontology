import React from 'react';

interface HomeProps {
  onNavigateToExplorer: () => void;
}

export default function Home({ onNavigateToExplorer }: HomeProps) {
  return (
    <div className="px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome to Object Explorer</h1>
        <p className="text-gray-600 text-lg">
          Browse and explore objects in your ontology. Similar to Palantir Foundry's Object Explorer,
          this application provides a generic interface for discovering and navigating ontology objects.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Browse by Type</h2>
          <p className="text-gray-600 mb-4">
            Select an object type and browse all instances. Search and filter to find specific objects.
          </p>
          <button
            onClick={onNavigateToExplorer}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Open Explorer →
          </button>
        </div>
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Object Details</h2>
          <p className="text-gray-600 mb-4">
            View complete object properties and explore relationships through link types.
          </p>
        </div>
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-blue-900 mb-2">Features</h3>
        <ul className="list-disc list-inside space-y-2 text-blue-800">
          <li><strong>Object Type Selection:</strong> Choose from available object types in your ontology</li>
          <li><strong>Object Search:</strong> Search and filter objects by properties</li>
          <li><strong>Property Viewing:</strong> View all properties of selected objects</li>
          <li><strong>Link Exploration:</strong> Explore relationships by traversing link types</li>
        </ul>
      </div>
    </div>
  );
}