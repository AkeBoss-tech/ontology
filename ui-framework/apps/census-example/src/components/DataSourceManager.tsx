import React, { useState } from 'react';

interface DataSource {
  id: string;
  name: string;
  type: string;
  connectionString: string;
  status: 'connected' | 'disconnected' | 'error';
}

export default function DataSourceManager() {
  const [dataSources, setDataSources] = useState<DataSource[]>([
    {
      id: '1',
      name: 'Census Parquet Files',
      type: 'parquet',
      connectionString: 'parquet://census_data/tracts',
      status: 'connected',
    },
  ]);
  const [showAddForm, setShowAddForm] = useState(false);
  const [newSource, setNewSource] = useState<Partial<DataSource>>({
    name: '',
    type: 'parquet',
    connectionString: '',
  });

  const handleAdd = () => {
    if (newSource.name && newSource.connectionString) {
      setDataSources([
        ...dataSources,
        {
          id: Date.now().toString(),
          name: newSource.name,
          type: newSource.type || 'parquet',
          connectionString: newSource.connectionString,
          status: 'disconnected',
        },
      ]);
      setNewSource({ name: '', type: 'parquet', connectionString: '' });
      setShowAddForm(false);
    }
  };

  const handleTest = (id: string) => {
    setDataSources(
      dataSources.map((ds) =>
        ds.id === id ? { ...ds, status: 'connected' } : ds
      )
    );
  };

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold">Data Source Management</h1>
        <button
          onClick={() => setShowAddForm(!showAddForm)}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          {showAddForm ? 'Cancel' : 'Add Data Source'}
        </button>
      </div>

      {showAddForm && (
        <div className="p-4 bg-white border rounded">
          <h2 className="text-lg font-semibold mb-4">Add New Data Source</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium mb-1">Name</label>
              <input
                type="text"
                value={newSource.name}
                onChange={(e) => setNewSource({ ...newSource, name: e.target.value })}
                className="w-full px-3 py-2 border rounded"
                placeholder="e.g., Census Parquet Files"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Type</label>
              <select
                value={newSource.type}
                onChange={(e) => setNewSource({ ...newSource, type: e.target.value })}
                className="w-full px-3 py-2 border rounded"
              >
                <option value="parquet">Parquet</option>
                <option value="csv">CSV</option>
                <option value="sql">SQL Database</option>
                <option value="api">API</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">Connection String</label>
              <input
                type="text"
                value={newSource.connectionString}
                onChange={(e) =>
                  setNewSource({ ...newSource, connectionString: e.target.value })
                }
                className="w-full px-3 py-2 border rounded"
                placeholder="e.g., parquet://census_data/tracts"
              />
            </div>
            <button
              onClick={handleAdd}
              className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
            >
              Add Data Source
            </button>
          </div>
        </div>
      )}

      <div className="space-y-2">
        {dataSources.map((ds) => (
          <div key={ds.id} className="p-4 bg-white border rounded flex justify-between items-center">
            <div>
              <h3 className="font-semibold">{ds.name}</h3>
              <p className="text-sm text-gray-500">{ds.type} - {ds.connectionString}</p>
            </div>
            <div className="flex items-center gap-4">
              <span
                className={`px-3 py-1 rounded text-sm ${
                  ds.status === 'connected'
                    ? 'bg-green-100 text-green-800'
                    : ds.status === 'error'
                    ? 'bg-red-100 text-red-800'
                    : 'bg-gray-100 text-gray-800'
                }`}
              >
                {ds.status}
              </span>
              <button
                onClick={() => handleTest(ds.id)}
                className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                Test Connection
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}







