#!/usr/bin/env python3
"""
Simple Map View for Census Data

A basic HTML/JavaScript map visualization using Leaflet to display census tracts.
"""
import json
from pathlib import Path
from typing import List, Dict, Optional


def create_map_html(
    tracts: List[Dict],
    output_path: str,
    center_lat: float = 40.7128,
    center_lon: float = -74.0060,
    zoom: int = 10
) -> None:
    """
    Create an HTML file with an interactive map showing census tracts.
    
    Args:
        tracts: List of tract dictionaries with geoid, year, properties, and optional geoshape
        output_path: Path to output HTML file
        center_lat: Map center latitude
        center_lon: Map center longitude
        zoom: Initial zoom level
    """
    html_template = """<!DOCTYPE html>
<html>
<head>
    <title>Census Tract Map</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: Arial, sans-serif;
        }
        #map {
            width: 100%;
            height: 100vh;
        }
        .info {
            position: absolute;
            top: 10px;
            right: 10px;
            background: white;
            padding: 15px;
            border-radius: 5px;
            box-shadow: 0 2px 5px rgba(0,0,0,0.2);
            z-index: 1000;
            max-width: 300px;
        }
        .tract-info {
            margin: 5px 0;
            padding: 5px;
            border-bottom: 1px solid #eee;
        }
        .year-selector {
            margin-bottom: 10px;
        }
        select {
            width: 100%;
            padding: 5px;
        }
    </style>
</head>
<body>
    <div id="map"></div>
    <div class="info">
        <h3>Census Tract Viewer</h3>
        <div class="year-selector">
            <label>Year:</label>
            <select id="yearSelect" onchange="filterByYear()">
                <option value="all">All Years</option>
            </select>
        </div>
        <div id="tractInfo">Click on a tract to see details</div>
    </div>

    <script>
        // Initialize map
        var map = L.map('map').setView([{center_lat}, {center_lon}], {zoom});
        
        // Add base tile layer
        L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
            attribution: '© OpenStreetMap contributors'
        }}).addTo(map);
        
        // Tract data
        var tractsData = {tracts_json};
        
        // Store layer references
        var tractLayers = {{}};
        
        // Available years
        var years = [...new Set(tractsData.map(t => t.year))].sort();
        var yearSelect = document.getElementById('yearSelect');
        years.forEach(year => {{
            var option = document.createElement('option');
            option.value = year;
            option.textContent = year;
            yearSelect.appendChild(option);
        }});
        
        // Color scale for median income (if available)
        function getColor(income) {{
            if (!income) return '#808080';
            if (income < 30000) return '#d73027';
            if (income < 50000) return '#f46d43';
            if (income < 70000) return '#fdae61';
            if (income < 100000) return '#fee08b';
            return '#1a9850';
        }}
        
        // Add tracts to map
        function addTractsToMap(filterYear) {{
            // Remove existing layers
            Object.values(tractLayers).forEach(layer => map.removeLayer(layer));
            tractLayers = {{}};
            
            tractsData.forEach(tract => {{
                if (filterYear && filterYear !== 'all' && tract.year != filterYear) {{
                    return;
                }}
                
                var geojson = tract.geoshape;
                if (!geojson) {{
                    // Create a simple point if no geometry
                    geojson = {{
                        type: 'Feature',
                        geometry: {{
                            type: 'Point',
                            coordinates: [tract.centroid_lon || 0, tract.centroid_lat || 0]
                        }},
                        properties: {{}}
                    }};
                }}
                
                try {{
                    var geoJsonObj = typeof geojson === 'string' ? JSON.parse(geojson) : geojson;
                    
                    var income = tract.median_household_income || 0;
                    var fillColor = getColor(income);
                    
                    var layer = L.geoJSON(geoJsonObj, {{
                        style: {{
                            fillColor: fillColor,
                            fillOpacity: 0.6,
                            color: '#333',
                            weight: 1
                        }},
                        onEachFeature: function(feature, layer) {{
                            layer.bindPopup(createPopupContent(tract));
                            layer.on('click', function() {{
                                showTractInfo(tract);
                            }});
                        }}
                    }});
                    
                    layer.addTo(map);
                    tractLayers[tract.geoid_year] = layer;
                }} catch (e) {{
                    console.error('Error parsing geojson for tract', tract.geoid_year, e);
                }}
            }});
        }}
        
        function createPopupContent(tract) {{
            var content = '<b>Tract: ' + tract.geoid + '</b><br>';
            content += 'Year: ' + tract.year + '<br>';
            if (tract.total_population) {{
                content += 'Population: ' + tract.total_population.toLocaleString() + '<br>';
            }}
            if (tract.median_household_income) {{
                content += 'Median Income: $' + tract.median_household_income.toLocaleString() + '<br>';
            }}
            if (tract.median_rent) {{
                content += 'Median Rent: $' + tract.median_rent.toLocaleString() + '<br>';
            }}
            return content;
        }}
        
        function showTractInfo(tract) {{
            var infoDiv = document.getElementById('tractInfo');
            infoDiv.innerHTML = '<div class="tract-info">' +
                '<strong>' + tract.geoid + '</strong><br>' +
                'Year: ' + tract.year + '<br>' +
                (tract.total_population ? 'Population: ' + tract.total_population.toLocaleString() + '<br>' : '') +
                (tract.median_household_income ? 'Median Income: $' + tract.median_household_income.toLocaleString() + '<br>' : '') +
                (tract.median_rent ? 'Median Rent: $' + tract.median_rent.toLocaleString() + '<br>' : '') +
                '</div>';
        }}
        
        function filterByYear() {{
            var selectedYear = document.getElementById('yearSelect').value;
            addTractsToMap(selectedYear);
        }}
        
        // Initial load
        addTractsToMap('all');
    </script>
</body>
</html>"""
    
    # Prepare tracts data for JSON
    tracts_for_json = []
    for tract in tracts:
        tract_copy = dict(tract)
        # Ensure geoshape is properly formatted
        if 'geoshape' in tract_copy and isinstance(tract_copy['geoshape'], str):
            try:
                # If it's a JSON string, parse it
                tract_copy['geoshape'] = json.loads(tract_copy['geoshape'])
            except:
                # If parsing fails, keep as string and let JS handle it
                pass
        tracts_for_json.append(tract_copy)
    
    html_content = html_template.format(
        center_lat=center_lat,
        center_lon=center_lon,
        zoom=zoom,
        tracts_json=json.dumps(tracts_for_json, indent=2)
    )
    
    output = Path(output_path)
    output.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output, "w") as f:
        f.write(html_content)
    
    print(f"Map HTML created at {output_path}")


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Create map view for census tracts")
    parser.add_argument("--data", type=str, required=True, help="JSON file with tract data")
    parser.add_argument("--output", type=str, default="map_view.html", help="Output HTML file")
    parser.add_argument("--center-lat", type=float, default=40.7128, help="Map center latitude")
    parser.add_argument("--center-lon", type=float, default=-74.0060, help="Map center longitude")
    parser.add_argument("--zoom", type=int, default=10, help="Initial zoom level")
    
    args = parser.parse_args()
    
    # Load tract data
    with open(args.data, "r") as f:
        tracts = json.load(f)
    
    create_map_html(
        tracts,
        args.output,
        args.center_lat,
        args.center_lon,
        args.zoom
    )







