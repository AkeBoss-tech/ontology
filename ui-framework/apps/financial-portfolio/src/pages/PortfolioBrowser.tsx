import React from 'react';
import { ObjectBrowser } from '@ontology/core';

export default function PortfolioBrowser() {
  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Portfolio Browser</h1>
      <p className="text-gray-600 mb-4">
        Browse portfolios and their holdings. Select a portfolio to view details and linked assets.
      </p>
      <div className="bg-white p-6 rounded-lg shadow">
        <ObjectBrowser objectType="Portfolio" />
      </div>
    </div>
  );
}


