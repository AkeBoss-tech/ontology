import React, { useEffect, useRef, useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';
import L from 'leaflet';
import 'leaflet/dist/leaflet.css';

// Fix for default marker icons in Leaflet
delete (L.Icon.Default.prototype as any)._getIconUrl;
L.Icon.Default.mergeOptions({
  iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-icon-2x.png',
  iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-icon.png',
  shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.7.1/images/marker-shadow.png',
});

const SEARCH_OBJECTS = gql`
  query SearchObjects($objectType: String!, $filters: [FilterInput!], $limit: Int) {
    searchObjects(objectType: $objectType, filters: $filters, limit: $limit) {
      objectType
      objectId
      title
      properties
    }
  }
`;

export interface MapViewProps {
  objectType: string;
  geojsonProperty: string;
  valueProperty?: string; // For choropleth
  center?: [number, number];
  zoom?: number;
  onObjectClick?: (objectId: string, properties?: any) => void;
  filters?: any[];
  selectedYear?: number;
}

export function MapView({
  objectType,
  geojsonProperty,
  valueProperty = 'total_population',
  center = [-98.5795, 39.8283], // Center of USA
  zoom = 4,
  onObjectClick,
  filters = [],
  selectedYear,
}: MapViewProps) {
  const { client } = useOntology();
  const mapRef = useRef<HTMLDivElement>(null);
  const mapInstanceRef = useRef<L.Map | null>(null);
  const layersRef = useRef<L.GeoJSON[]>([]);

  // Build filters including year if provided
  const queryFilters = [
    ...filters,
    ...(selectedYear ? [{ property: 'year', operator: 'equals', value: selectedYear.toString() }] : []),
  ];

  const { data, loading } = useQuery(SEARCH_OBJECTS, {
    client,
    variables: {
      objectType,
      filters: queryFilters.length > 0 ? queryFilters : undefined,
      limit: 10000, // Get all objects for map
    },
    skip: false,
  });

  useEffect(() => {
    if (!mapRef.current || mapInstanceRef.current) return;

    // Initialize map
    const map = L.map(mapRef.current).setView(center, zoom);
    mapInstanceRef.current = map;

    // Add tile layer
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '© OpenStreetMap contributors',
      maxZoom: 19,
    }).addTo(map);

    return () => {
      map.remove();
      mapInstanceRef.current = null;
    };
  }, []);

  useEffect(() => {
    if (!mapInstanceRef.current || !data) return;

    // Clear existing layers
    layersRef.current.forEach((layer) => {
      mapInstanceRef.current?.removeLayer(layer);
    });
    layersRef.current = [];

    // Get min/max values for choropleth
    const values: number[] = [];
    data.searchObjects.forEach((obj: any) => {
      try {
        const props = typeof obj.properties === 'string' ? JSON.parse(obj.properties) : obj.properties;
        const geojsonStr = props[geojsonProperty];
        const value = props[valueProperty];
        if (geojsonStr && value !== undefined && value !== null) {
          values.push(Number(value));
        }
      } catch (e) {
        // Skip invalid objects
      }
    });

    const minValue = Math.min(...values);
    const maxValue = Math.max(...values);
    const range = maxValue - minValue || 1;

    // Create color scale
    const getColor = (value: number) => {
      const normalized = (value - minValue) / range;
      const hue = (1 - normalized) * 120; // Green to red
      return `hsl(${hue}, 70%, 50%)`;
    };

    // Add GeoJSON layers
    data.searchObjects.forEach((obj: any) => {
      try {
        const props = typeof obj.properties === 'string' ? JSON.parse(obj.properties) : obj.properties;
        const geojsonStr = props[geojsonProperty];
        if (!geojsonStr) return;

        const geojson = typeof geojsonStr === 'string' ? JSON.parse(geojsonStr) : geojsonStr;
        const value = props[valueProperty];
        const color = value !== undefined && value !== null ? getColor(Number(value)) : '#3388ff';

        const geoJsonLayer = L.geoJSON(geojson as any, {
          style: {
            fillColor: color,
            color: '#333',
            weight: 1,
            opacity: 0.8,
            fillOpacity: 0.6,
          },
        });

        // Add popup with object info
        geoJsonLayer.bindPopup(`
          <div>
            <strong>${obj.title}</strong><br/>
            ${valueProperty}: ${value !== undefined ? value.toLocaleString() : 'N/A'}<br/>
            <button onclick="window.mapClickHandler && window.mapClickHandler('${obj.objectId}')" 
                    style="margin-top: 5px; padding: 5px 10px; background: #007bff; color: white; border: none; border-radius: 3px; cursor: pointer;">
              View Details
            </button>
          </div>
        `);

        // Add click handler
        geoJsonLayer.on('click', () => {
          if (onObjectClick) {
            onObjectClick(obj.objectId, props);
          }
        });

        geoJsonLayer.addTo(mapInstanceRef.current!);
        layersRef.current.push(geoJsonLayer);
      } catch (e) {
        console.error('Error rendering GeoJSON:', e, obj);
      }
    });

    // Fit map to bounds if we have data
    if (layersRef.current.length > 0) {
      const group = new L.FeatureGroup(layersRef.current);
      mapInstanceRef.current.fitBounds(group.getBounds().pad(0.1));
    }
  }, [data, geojsonProperty, valueProperty, onObjectClick]);

  // Update center/zoom
  useEffect(() => {
    if (mapInstanceRef.current) {
      mapInstanceRef.current.setView(center, zoom);
    }
  }, [center, zoom]);

  return (
    <div className="map-view w-full h-full border rounded overflow-hidden relative">
      {loading && (
        <div className="absolute top-2 left-2 z-[1000] bg-white px-3 py-1 rounded shadow">
          Loading map data...
        </div>
      )}
      <div ref={mapRef} className="w-full h-full" style={{ minHeight: '500px' }} />
      <div className="absolute bottom-2 left-2 bg-white px-3 py-2 rounded shadow text-sm">
        <div>
          <strong>{objectType}</strong>
        </div>
        {valueProperty && (
          <div className="text-gray-600">
            Choropleth: {valueProperty}
          </div>
        )}
        {data && (
          <div className="text-gray-500">
            {data.searchObjects.length} objects displayed
          </div>
        )}
      </div>
    </div>
  );
}
