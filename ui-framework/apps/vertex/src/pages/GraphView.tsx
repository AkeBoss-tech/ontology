import React, { useState } from 'react';
import { GraphVisualization } from '@ontology/graph';
import { ObjectSearch } from '@ontology/core';

export default function GraphView() {
  const [objectType, setObjectType] = useState<string>('');
  const [objectId, setObjectId] = useState<string>('');
  const [linkTypes, setLinkTypes] = useState<string[]>(['']);
  const [maxHops, setMaxHops] = useState<number>(2);

  const handleLinkTypeChange = (index: number, value: string) => {
    const newLinkTypes = [...linkTypes];
    newLinkTypes[index] = value;
    setLinkTypes(newLinkTypes);
  };

  const addLinkType = () => {
    setLinkTypes([...linkTypes, '']);
  };

  const removeLinkType = (index: number) => {
    setLinkTypes(linkTypes.filter((_, i) => i !== index));
  };

  const validLinkTypes = linkTypes.filter(lt => lt.trim() !== '');

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Graph Visualization</h1>
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-1 bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Configuration</h2>
          
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
            <label className="block text-sm font-medium mb-2">Start Object</label>
            {objectType ? (
              <ObjectSearch
                objectType={objectType}
                onSelectObject={setObjectId}
              />
            ) : (
              <input
                type="text"
                value={objectId}
                onChange={(e) => setObjectId(e.target.value)}
                placeholder="Enter object ID"
                className="w-full px-4 py-2 border rounded"
              />
            )}
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Link Types</label>
            {linkTypes.map((linkType, index) => (
              <div key={index} className="flex gap-2 mb-2">
                <input
                  type="text"
                  value={linkType}
                  onChange={(e) => handleLinkTypeChange(index, e.target.value)}
                  placeholder="knows, owns, etc."
                  className="flex-1 px-4 py-2 border rounded"
                />
                {linkTypes.length > 1 && (
                  <button
                    onClick={() => removeLinkType(index)}
                    className="px-3 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                  >
                    ×
                  </button>
                )}
              </div>
            ))}
            <button
              onClick={addLinkType}
              className="text-sm text-blue-600 hover:text-blue-800"
            >
              + Add Link Type
            </button>
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">
              Max Hops: {maxHops}
            </label>
            <input
              type="range"
              min="1"
              max="5"
              value={maxHops}
              onChange={(e) => setMaxHops(parseInt(e.target.value))}
              className="w-full"
            />
          </div>
        </div>

        <div className="lg:col-span-2 bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Graph Visualization</h2>
          {objectId && validLinkTypes.length > 0 ? (
            <GraphVisualization
              objectType={objectType}
              objectId={objectId}
              linkTypes={validLinkTypes}
              maxHops={maxHops}
              onNodeClick={(nodeId) => {
                console.log('Clicked node:', nodeId);
              }}
            />
          ) : (
            <div className="text-center py-12 text-gray-500">
              Configure object type, start object, and link types to visualize the graph
            </div>
          )}
        </div>
      </div>
    </div>
  );
}


