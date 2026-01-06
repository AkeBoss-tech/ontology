import React, { useState } from 'react';
import { useVisualizationManager, VisualizationConfig } from '@ontology/core';

export default function VisualizationManager() {
  const {
    visualizations,
    saveVisualization,
    loadVisualization,
    deleteVisualization,
    updateVisualization,
    exportVisualizations,
    importVisualizations,
  } = useVisualizationManager();

  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [showLoadDialog, setShowLoadDialog] = useState(false);
  const [newVizName, setNewVizName] = useState('');
  const [selectedViz, setSelectedViz] = useState<VisualizationConfig | null>(null);
  const [importJson, setImportJson] = useState('');

  const handleSave = () => {
    if (!selectedViz || !newVizName.trim()) return;
    
    saveVisualization({
      name: newVizName,
      type: selectedViz.type,
      objectType: selectedViz.objectType,
      filters: selectedViz.filters,
      properties: selectedViz.properties,
      settings: selectedViz.settings,
    });
    
    setShowSaveDialog(false);
    setNewVizName('');
  };

  const handleExport = () => {
    const json = exportVisualizations();
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `visualizations_${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const handleImport = () => {
    try {
      importVisualizations(importJson);
      setImportJson('');
      setShowLoadDialog(false);
      alert('Visualizations imported successfully!');
    } catch (e) {
      alert('Failed to import: ' + (e as Error).message);
    }
  };

  const handleFileImport = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const content = event.target?.result as string;
        importVisualizations(content);
        alert('Visualizations imported successfully!');
      } catch (e) {
        alert('Failed to import: ' + (e as Error).message);
      }
    };
    reader.readAsText(file);
  };

  return (
    <div className="visualization-manager space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-bold">Saved Visualizations</h2>
        <div className="flex gap-2">
          <button
            onClick={() => setShowSaveDialog(true)}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Save Current
          </button>
          <button
            onClick={handleExport}
            className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
          >
            Export All
          </button>
          <button
            onClick={() => setShowLoadDialog(true)}
            className="px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600"
          >
            Import
          </button>
        </div>
      </div>

      {showSaveDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-md w-full">
            <h3 className="text-lg font-semibold mb-4">Save Visualization</h3>
            <input
              type="text"
              value={newVizName}
              onChange={(e) => setNewVizName(e.target.value)}
              placeholder="Enter visualization name..."
              className="w-full px-3 py-2 border rounded mb-4"
            />
            <div className="flex gap-2">
              <button
                onClick={handleSave}
                className="flex-1 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                Save
              </button>
              <button
                onClick={() => {
                  setShowSaveDialog(false);
                  setNewVizName('');
                }}
                className="flex-1 px-4 py-2 bg-gray-300 rounded hover:bg-gray-400"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {showLoadDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg max-w-2xl w-full max-h-96 overflow-y-auto">
            <h3 className="text-lg font-semibold mb-4">Import Visualizations</h3>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2">Paste JSON:</label>
              <textarea
                value={importJson}
                onChange={(e) => setImportJson(e.target.value)}
                className="w-full px-3 py-2 border rounded font-mono text-sm"
                rows={10}
                placeholder='[{"name": "My Viz", "type": "map", ...}]'
              />
            </div>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2">Or upload file:</label>
              <input
                type="file"
                accept=".json"
                onChange={handleFileImport}
                className="w-full px-3 py-2 border rounded"
              />
            </div>
            <div className="flex gap-2">
              <button
                onClick={handleImport}
                className="flex-1 px-4 py-2 bg-purple-500 text-white rounded hover:bg-purple-600"
              >
                Import
              </button>
              <button
                onClick={() => {
                  setShowLoadDialog(false);
                  setImportJson('');
                }}
                className="flex-1 px-4 py-2 bg-gray-300 rounded hover:bg-gray-400"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      <div className="space-y-2">
        {visualizations.length === 0 ? (
          <div className="p-4 bg-gray-100 rounded text-center text-gray-500">
            No saved visualizations. Create one to get started!
          </div>
        ) : (
          visualizations.map((viz) => (
            <div
              key={viz.id}
              className="p-4 bg-white border rounded flex justify-between items-center"
            >
              <div>
                <h3 className="font-semibold">{viz.name}</h3>
                <p className="text-sm text-gray-500">
                  {viz.type} • {viz.objectType} • {new Date(viz.updatedAt).toLocaleDateString()}
                </p>
              </div>
              <div className="flex gap-2">
                <button
                  onClick={() => {
                    const loaded = loadVisualization(viz.id);
                    if (loaded) {
                      setSelectedViz(loaded);
                      // In a real implementation, this would restore the visualization
                      alert(`Loaded: ${loaded.name}`);
                    }
                  }}
                  className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600"
                >
                  Load
                </button>
                <button
                  onClick={() => {
                    if (confirm(`Delete "${viz.name}"?`)) {
                      deleteVisualization(viz.id);
                    }
                  }}
                  className="px-3 py-1 text-sm bg-red-500 text-white rounded hover:bg-red-600"
                >
                  Delete
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}







