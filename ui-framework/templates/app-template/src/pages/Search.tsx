import React from 'react';
import { ObjectSearch } from '@ontology/core';

export default function Search() {
  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Search Objects</h1>
      <div className="bg-white p-6 rounded-lg shadow">
        <p className="text-gray-600 mb-4">
          Replace 'your-object-type' with an actual object type from your ontology.
        </p>
        <ObjectSearch
          objectType="your-object-type"
          onSelectObject={(objectId) => {
            console.log('Selected object:', objectId);
            // Navigate to object detail page or show object browser
          }}
        />
      </div>
    </div>
  );
}




