import React from 'react';

export default function SavedViews() {
  // Mock saved views - in production, these would come from the backend
  const savedViews = [
    { id: '1', name: 'Active Users', objectType: 'User', description: 'Users with active status' },
    { id: '2', name: 'High Value Assets', objectType: 'Asset', description: 'Assets worth more than $100k' },
    { id: '3', name: 'Recent Transactions', objectType: 'Transaction', description: 'Transactions from last 30 days' },
  ];

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Saved Views</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {savedViews.map((view) => (
          <div key={view.id} className="bg-white p-6 rounded-lg shadow hover:shadow-lg transition-shadow">
            <h2 className="text-xl font-semibold mb-2">{view.name}</h2>
            <div className="text-sm text-gray-500 mb-2">{view.objectType}</div>
            <p className="text-gray-600 mb-4">{view.description}</p>
            <button className="w-full px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
              Open View
            </button>
          </div>
        ))}
      </div>

      {savedViews.length === 0 && (
        <div className="bg-gray-50 border border-gray-200 rounded-lg p-8 text-center">
          <p className="text-gray-600">No saved views yet. Create one using the View Builder.</p>
        </div>
      )}
    </div>
  );
}


