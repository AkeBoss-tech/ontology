#!/usr/bin/env python3
"""
Geography Ingestion Script

Fetches TIGER/Line shapefiles from TIGERweb REST API and converts to GeoJSON.
"""
import requests
import json
from pathlib import Path
from typing import Dict, Optional, List
import sys


class TIGERwebAdapter:
    """Adapter for TIGERweb REST API."""
    
    BASE_URL = "https://tigerweb.geo.census.gov/arcgis/rest/services/TIGERweb"
    
    def __init__(self):
        """Initialize TIGERweb adapter."""
        pass
    
    def fetch_tract_geojson(
        self,
        geoid: str,
        year: int = 2020
    ) -> Optional[Dict]:
        """
        Fetch GeoJSON for a specific census tract.
        
        Args:
            geoid: Full GEOID (e.g., "36061010000")
            year: Census year (affects which service to use)
            
        Returns:
            GeoJSON feature or None if not found
        """
        # Determine service based on year
        if year >= 2020:
            service = "Tracts_Blocks/MapServer/0"
        elif year >= 2010:
            service = "tigerWMS_Current/MapServer/0"
        else:
            service = "tigerWMS_ACS{year}/MapServer/0".format(year=year)
        
        url = f"{self.BASE_URL}/{service}/query"
        params = {
            "where": f"GEOID='{geoid}'",
            "f": "geojson",
            "outFields": "*",
            "outSR": "4326",  # WGS84
        }
        
        try:
            response = requests.get(url, params=params, timeout=30)
            response.raise_for_status()
            data = response.json()
            
            # Return first feature if available
            if "features" in data and len(data["features"]) > 0:
                return data["features"][0]
            return None
            
        except requests.RequestException as e:
            print(f"Error fetching tract geometry: {e}")
            return None
    
    def fetch_county_tracts(
        self,
        state_fips: str,
        county_fips: str,
        year: int = 2020
    ) -> List[Dict]:
        """
        Fetch all tracts in a county.
        
        Args:
            state_fips: State FIPS code
            county_fips: County FIPS code
            year: Census year
            
        Returns:
            List of GeoJSON features
        """
        # Determine service
        if year >= 2020:
            service = "Tracts_Blocks/MapServer/0"
        else:
            service = f"tigerWMS_ACS{year}/MapServer/0"
        
        url = f"{self.BASE_URL}/{service}/query"
        params = {
            "where": f"STATE='{state_fips}' AND COUNTY='{county_fips}'",
            "f": "geojson",
            "outFields": "*",
            "outSR": "4326",
        }
        
        try:
            response = requests.get(url, params=params, timeout=60)
            response.raise_for_status()
            data = response.json()
            
            return data.get("features", [])
            
        except requests.RequestException as e:
            print(f"Error fetching county tracts: {e}")
            return []


def save_geojson(geojson: Dict, output_path: str) -> None:
    """Save GeoJSON to file."""
    path = Path(output_path)
    path.parent.mkdir(parents=True, exist_ok=True)
    
    with open(path, "w") as f:
        json.dump(geojson, f, indent=2)


def ingest_tract_geography(
    geoid: str,
    year: int,
    output_dir: str
) -> Optional[str]:
    """
    Ingest geography for a single tract.
    
    Args:
        geoid: Tract GEOID
        year: Census year
        output_dir: Output directory
        
    Returns:
        Path to saved GeoJSON file or None
    """
    adapter = TIGERwebAdapter()
    feature = adapter.fetch_tract_geojson(geoid, year)
    
    if feature:
        output_path = Path(output_dir) / f"tract_{geoid}_{year}.geojson"
        save_geojson(feature, str(output_path))
        print(f"Saved geometry for tract {geoid} ({year})")
        return str(output_path)
    else:
        print(f"No geometry found for tract {geoid} ({year})")
        return None


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Ingest TIGER/Line geography")
    parser.add_argument("--geoid", type=str, help="Tract GEOID")
    parser.add_argument("--state", type=str, help="State FIPS code")
    parser.add_argument("--county", type=str, help="County FIPS code")
    parser.add_argument("--year", type=int, default=2020, help="Census year")
    parser.add_argument("--output", type=str, default="data/geography", help="Output directory")
    
    args = parser.parse_args()
    
    if args.geoid:
        ingest_tract_geography(args.geoid, args.year, args.output)
    elif args.state and args.county:
        adapter = TIGERwebAdapter()
        features = adapter.fetch_county_tracts(args.state, args.county, args.year)
        print(f"Fetched {len(features)} tracts")
        
        output_dir = Path(args.output)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        for feature in features:
            props = feature.get("properties", {})
            geoid = props.get("GEOID", "unknown")
            output_path = output_dir / f"tract_{geoid}_{args.year}.geojson"
            save_geojson(feature, str(output_path))
    else:
        parser.print_help()



