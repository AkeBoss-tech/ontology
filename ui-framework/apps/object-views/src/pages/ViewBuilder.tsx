import React, { useState } from 'react';
import { ObjectBrowser } from '@ontology/core';
import { FilterBuilder } from '@ontology/forms';

export default function ViewBuilder() {
  const [objectType, setObjectType] = useState<string>('');
  const [viewName, setViewName] = useState<string>('');
  const [selectedProperties, setSelectedProperties] = useState<string[]>([]);

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Create Object View</h1>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">View Configuration</h2>
          
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">View Name</label>
            <input
              type="text"
              value={viewName}
              onChange={(e) => setViewName(e.target.value)}
              placeholder="My Custom View"
              className="w-full px-4 py-2 border rounded"
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Object Type</label>
            <input
              type="text"
              value={objectType}
              onChange={(e) => setObjectType(e.target.value)}
              placeholder="Person, Asset, etc."
              className="w-full px-4 py-2 border rounded"
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Filters</label>
            <div className="border rounded p-4 bg-gray-50">
              <p className="text-sm text-gray-600">
                Filter builder component would go here. Configure filters to show only objects matching specific criteria.
              </p>
            </div>
          </div>

          <button className="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
            Save View
          </button>
        </div>

        <div className="bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Preview</h2>
          {objectType ? (
            <ObjectBrowser objectType={objectType} />
          ) : (
            <div className="text-center py-12 text-gray-500">
              Enter an object type to preview the view
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
