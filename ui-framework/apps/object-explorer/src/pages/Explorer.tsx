import React, { useState } from 'react';
import { ObjectSearch, InterfaceQuery } from '@ontology/core'; // Importing InterfaceQuery
import { useOntology } from '@ontology/core';

interface ExplorerProps {
  onSelectObject: (obj: { type: string; id: string }) => void;
}

export default function Explorer({ onSelectObject }: ExplorerProps) {
  const { client } = useOntology();
  const [selectedObjectType, setSelectedObjectType] = useState<string>('');
  const [queryMode, setQueryMode] = useState<'object' | 'interface'>('object'); // New query mode state

  if (!client) {
    return <div>Ontology client not available</div>;
  }

  const objectTypes = [
    { id: 'Person', displayName: 'Person', icon: '👤' },
    { id: 'Asset', displayName: 'Asset', icon: '🏦' },
    { id: 'Location', displayName: 'Location', icon: '📍' },
    { id: 'Transaction', displayName: 'Transaction', icon: '💳' },
    { id: 'Portfolio', displayName: 'Portfolio', icon: '📁' },
  ];

  return (
    <div className="flex flex-col h-full">
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-bold text-dark-300">Object Explorer</h1>

        {/* Query Mode Toggle */}
        <div className="bg-white border border-light-300 rounded-sm p-1 flex">
          <button
            onClick={() => setQueryMode('object')}
            className={`px-3 py-1 text-xs font-medium rounded-sm transition-colors ${queryMode === 'object' ? 'bg-foundry-core text-white' : 'text-gray-500 hover:bg-light-200'
              }`}
          >
            By Object Type
          </button>
          <button
            onClick={() => setQueryMode('interface')}
            className={`px-3 py-1 text-xs font-medium rounded-sm transition-colors ${queryMode === 'interface' ? 'bg-foundry-core text-white' : 'text-gray-500 hover:bg-light-200'
              }`}
          >
            By Interface
          </button>
        </div>
      </div>

      {queryMode === 'object' ? (
        <>
          {/* Object Type Selector */}
          <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm mb-6">
            <h2 className="text-xs font-bold text-gray-500 uppercase tracking-wide mb-3">Select Object Type</h2>
            <div className="grid grid-cols-2 md:grid-cols-5 gap-3">
              {objectTypes.map((type: any) => (
                <button
                  key={type.id}
                  onClick={() => setSelectedObjectType(type.id)}
                  className={`p-3 border rounded-sm text-left hover:bg-light-100 transition-all flex items-center ${selectedObjectType === type.id
                      ? 'border-foundry-core bg-blue-50 ring-1 ring-foundry-core'
                      : 'border-light-300'
                    }`}
                >
                  <span className="text-lg mr-3">{type.icon}</span>
                  <div>
                    <div className="text-sm font-semibold text-dark-300">{type.displayName}</div>
                    <div className="text-xs text-gray-500">{type.id}</div>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Results Area */}
          {selectedObjectType ? (
            <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm flex-1">
              <div className="flex items-center justify-between mb-4 border-b border-light-300 pb-3">
                <h2 className="text-sm font-bold text-dark-300">
                  Browsing: <span className="text-foundry-core">{objectTypes.find((t: any) => t.id === selectedObjectType)?.displayName || selectedObjectType}</span>
                </h2>
                <div className="flex space-x-2">
                  <button className="text-xs px-2 py-1 bg-light-200 border border-light-300 rounded-sm text-dark-400">Save as set</button>
                  <button className="text-xs px-2 py-1 bg-light-200 border border-light-300 rounded-sm text-dark-400">Export</button>
                </div>
              </div>
              <ObjectSearch
                objectType={selectedObjectType}
                onSelectObject={(objectId) => {
                  onSelectObject({ type: selectedObjectType, id: objectId });
                }}
              />
            </div>
          ) : (
            <div className="bg-light-200 border border-dashed border-light-400 rounded-sm p-8 text-center">
              <p className="text-dark-300 font-medium">Select an object type above to start exploring.</p>
              <p className="text-gray-500 text-sm mt-1">You can filter, sort, and analyze objects from here.</p>
            </div>
          )}
        </>
      ) : (
        /* Interface Query Mode */
        <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm flex-1">
          <div className="mb-4">
            <h2 className="text-sm font-bold text-dark-300 mb-1">Interface Query</h2>
            <p className="text-xs text-gray-500">Find objects across different types that implement specific interfaces.</p>
          </div>
          <InterfaceQuery
            onSelectObject={(obj) => onSelectObject({ type: obj.__typename, id: obj.id })}
          />
        </div>
      )}
    </div>
  );
}
