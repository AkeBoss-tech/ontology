import React, { useState } from 'react';
import { useVisualizationManager, VisualizationConfig } from '@ontology/core';

interface LoadVisualizationButtonProps {
  onLoad: (config: VisualizationConfig) => void;
  objectType?: string;
  type?: 'map' | 'graph' | 'chart' | 'table';
}

export default function LoadVisualizationButton({ onLoad, objectType, type }: LoadVisualizationButtonProps) {
  const { visualizations, loadVisualization } = useVisualizationManager();
  const [showDialog, setShowDialog] = useState(false);

  // Filter visualizations by type and objectType if provided
  const filtered = visualizations.filter((viz) => {
    if (type && viz.type !== type) return false;
    if (objectType && viz.objectType !== objectType) return false;
    return true;
  });

  const handleLoad = (id: string) => {
    const config = loadVisualization(id);
    if (config) {
      onLoad(config);
      setShowDialog(false);
    }
  };

  if (filtered.length === 0) {
    return null;
  }

  return (
    <>
      <button
        onClick={() => setShowDialog(true)}
        className="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600"
      >
        Load Saved ({filtered.length})
      </button>

      {showDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-2xl w-full max-h-96 overflow-y-auto">
            <h3 className="text-lg font-semibold mb-4">Load Visualization</h3>
            <div className="space-y-2">
              {filtered.map((viz) => (
                <div
                  key={viz.id}
                  className="p-3 border rounded flex justify-between items-center hover:bg-gray-50"
                >
                  <div>
                    <h4 className="font-semibold">{viz.name}</h4>
                    <p className="text-sm text-gray-500">
                      {viz.type} • {viz.objectType} • {new Date(viz.updatedAt).toLocaleDateString()}
                    </p>
                  </div>
                  <button
                    onClick={() => handleLoad(viz.id)}
                    className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600"
                  >
                    Load
                  </button>
                </div>
              ))}
            </div>
            <button
              onClick={() => setShowDialog(false)}
              className="mt-4 w-full px-4 py-2 bg-gray-300 rounded hover:bg-gray-400"
            >
              Cancel
            </button>
          </div>
        </div>
      )}
    </>
  );
}



