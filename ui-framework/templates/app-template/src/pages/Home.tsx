import React from 'react';

export default function Home() {
  return (
    <div className="px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome to {{APP_DISPLAY_NAME}}</h1>
        <p className="text-gray-600">
          {{APP_DESCRIPTION}}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">Search Objects</h2>
          <p className="text-gray-600 mb-4">
            Search and find objects in your ontology.
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">Browse Data</h2>
          <p className="text-gray-600 mb-4">
            Browse objects by type and explore relationships.
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-xl font-semibold mb-2">Get Started</h2>
          <p className="text-gray-600 mb-4">
            Configure your ontology endpoint and start exploring.
          </p>
        </div>
      </div>
    </div>
  );
}




