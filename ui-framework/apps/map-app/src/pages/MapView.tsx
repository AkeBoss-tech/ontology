import React, { useState } from 'react';
import { MapView as MapViewComponent } from '@ontology/map';
import { TimeSlider } from '@ontology/map';

export default function MapViewPage() {
  const [objectType, setObjectType] = useState<string>('Location');
  const [geojsonProperty, setGeojsonProperty] = useState<string>('geometry');
  const [valueProperty, setValueProperty] = useState<string>('population');
  const [selectedYear, setSelectedYear] = useState<number>(2020);

  return (
    <div className="px-4 py-8">
      <h1 className="text-2xl font-bold mb-6">Geospatial Map</h1>
      
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        <div className="lg:col-span-1 bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Configuration</h2>
          
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Object Type</label>
            <input
              type="text"
              value={objectType}
              onChange={(e) => setObjectType(e.target.value)}
              placeholder="Location, Tract, etc."
              className="w-full px-4 py-2 border rounded"
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">GeoJSON Property</label>
            <input
              type="text"
              value={geojsonProperty}
              onChange={(e) => setGeojsonProperty(e.target.value)}
              placeholder="geometry"
              className="w-full px-4 py-2 border rounded"
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Value Property (for choropleth)</label>
            <input
              type="text"
              value={valueProperty}
              onChange={(e) => setValueProperty(e.target.value)}
              placeholder="population, value, etc."
              className="w-full px-4 py-2 border rounded"
            />
          </div>

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2">Time Filter</label>
            <TimeSlider
              minYear={2010}
              maxYear={2024}
              selectedYear={selectedYear}
              onChange={setSelectedYear}
            />
          </div>
        </div>

        <div className="lg:col-span-3 bg-white p-6 rounded-lg shadow">
          <h2 className="text-lg font-semibold mb-4">Map Visualization</h2>
          <div className="h-[600px]">
            <MapViewComponent
              objectType={objectType}
              geojsonProperty={geojsonProperty}
              valueProperty={valueProperty}
              selectedYear={selectedYear}
              onObjectClick={(objectId, properties) => {
                console.log('Clicked object:', objectId, properties);
              }}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
