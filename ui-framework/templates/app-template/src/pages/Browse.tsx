import React from 'react';
import { ObjectBrowser } from '@ontology/core';

export default function Browse() {
  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Browse Objects</h1>
      <div className="bg-white p-6 rounded-lg shadow">
        <p className="text-gray-600 mb-4">
          Replace 'your-object-type' with an actual object type from your ontology.
        </p>
        <ObjectBrowser objectType="your-object-type" />
      </div>
    </div>
  );
}

