import React from 'react';

export default function Home() {
  return (
    <div className="px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome to Object Views</h1>
        <p className="text-gray-600 text-lg">
          Create custom views of objects with specific layouts, filters, and workflows.
          Similar to Palantir Foundry's Object Views application.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Create Views</h2>
          <p className="text-gray-600 mb-4">
            Build custom views by selecting object types, properties, and filters.
            Save views for quick access to frequently used object configurations.
          </p>
        </div>
        <div className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
          <h2 className="text-xl font-semibold mb-2">Saved Views</h2>
          <p className="text-gray-600 mb-4">
            Access your saved views and share them with your team.
            Views can be workflow-specific or general-purpose.
          </p>
        </div>
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-6">
        <h3 className="text-lg font-semibold text-blue-900 mb-2">Use Cases</h3>
        <ul className="list-disc list-inside space-y-2 text-blue-800">
          <li><strong>Workflow-Specific Views:</strong> Create views optimized for specific business processes</li>
          <li><strong>Filtered Object Lists:</strong> Save frequently used filter combinations</li>
          <li><strong>Custom Layouts:</strong> Arrange properties and relationships in meaningful ways</li>
          <li><strong>Team Sharing:</strong> Share views with team members for consistent workflows</li>
        </ul>
      </div>
    </div>
  );
}