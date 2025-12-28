import React from 'react';

export default function Home() {
  return (
    <div className="px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome to Map</h1>
        <p className="text-gray-600 text-lg">
          Geospatial visualization and mapping application. Visualize objects with geographic properties
          on interactive maps, similar to Palantir Foundry's Map application.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Geographic Visualization</h2>
          <p className="text-gray-600 mb-4">
            Display objects with GeoJSON geometry properties on interactive maps.
            Supports point, line, and polygon geometries.
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Choropleth Maps</h2>
          <p className="text-gray-600 mb-4">
            Color-code regions based on property values. Ideal for visualizing
            demographic data, sales territories, or other geographic distributions.
          </p>
        </div>
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-blue-900 mb-2">Features</h3>
        <ul className="list-disc list-inside space-y-2 text-blue-800">
          <li><strong>GeoJSON Support:</strong> Display objects with geographic coordinates</li>
          <li><strong>Choropleth Visualization:</strong> Color regions by property values</li>
          <li><strong>Time Filtering:</strong> Filter objects by time periods</li>
          <li><strong>Interactive Exploration:</strong> Click map features to view object details</li>
        </ul>
      </div>
    </div>
  );
}