#!/usr/bin/env python3
"""
Census Data Ingestion Script

Ingests aggregate statistics from the Census Data API (ACS and Decennial)
and stores them in a format ready for the ontology framework.
"""
import requests
import pandas as pd
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Optional
import time
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv(Path(__file__).parent.parent / ".env")

# Try to import SourceAdapter from ontology framework, otherwise define a simple base class
try:
    # Add parent directory to path to import ontology framework
    sys.path.insert(0, str(Path(__file__).parent.parent.parent.parent))
    from hydration.adapters.base import SourceAdapter
except ImportError:
    # Define a simple base class if hydration module is not available
    class SourceAdapter:
        """Simple base adapter class."""
        def connect(self) -> None:
            """Establish connection."""
            pass
        
        def disconnect(self) -> None:
            """Close connection."""
            pass
        
        def read_rows(self, limit: Optional[int] = None) -> List[Dict[str, any]]:
            """Read rows from the source."""
            raise NotImplementedError("Subclasses must implement read_rows")


class CensusDataAPIAdapter(SourceAdapter):
    """Adapter for Census Data API (ACS and Decennial)."""
    
    BASE_URL = "https://api.census.gov/data"
    
    def __init__(self, api_key: Optional[str] = None):
        """
        Initialize Census Data API adapter.
        
        Args:
            api_key: Optional API key for higher rate limits.
                     If not provided, will try CENSUS_API_KEY from environment.
        """
        self.api_key = api_key or os.getenv("CENSUS_API_KEY")
        self.connected = False
        
    def connect(self) -> None:
        """Establish connection (no-op for REST API)."""
        self.connected = True
        
    def disconnect(self) -> None:
        """Close connection (no-op for REST API)."""
        self.connected = False
        
    def fetch_acs_tracts(
        self,
        year: int,
        state_fips: str,
        variables: List[str],
        dataset: str = "acs/acs5"
    ) -> List[Dict[str, any]]:
        """
        Fetch ACS data for all tracts in a state.
        
        Args:
            year: Census year
            state_fips: State FIPS code (e.g., "06" for California)
            variables: List of variable IDs (e.g., ["B19013_001E"])
            dataset: Dataset name (default: "acs/acs5")
            
        Returns:
            List of dictionaries, one per tract
        """
        if not self.connected:
            raise RuntimeError("Adapter not connected")
            
        url = f"{self.BASE_URL}/{year}/{dataset}"
        params = {
            "get": ",".join(["NAME"] + variables),
            "for": "tract:*",
            "in": f"state:{state_fips}",
        }
        
        if self.api_key:
            params["key"] = self.api_key
            
        try:
            response = requests.get(url, params=params, timeout=30)
            response.raise_for_status()
            data = response.json()
            
            # First row is headers
            headers = data[0]
            rows = data[1:]
            
            # Convert to list of dicts
            result = []
            for row in rows:
                tract_data = dict(zip(headers, row))
                
                # Parse GEOID from state, county, tract codes
                state = tract_data.get("state", "")
                county = tract_data.get("county", "")
                tract = tract_data.get("tract", "")
                geoid = f"14000US{state}{county}{tract}"
                
                # Create geoid_year key
                geoid_year = f"{geoid}_{year}"
                
                # Convert to ontology format
                tract_obj = {
                    "geoid_year": geoid_year,
                    "geoid": geoid,
                    "year": year,
                    "state_fips": state,
                    "county_fips": f"{state}{county}",
                    "tract_ce": tract,
                    "name": tract_data.get("NAME", ""),
                }
                
                # Add variable values
                for var in variables:
                    value = tract_data.get(var, None)
                    if value:
                        try:
                            # Map common variable IDs to property names
                            var_mapping = {
                                "B19013_001E": "median_household_income",
                                "B25064_001E": "median_rent",
                                "B01001_001E": "total_population",
                            }
                            prop_name = var_mapping.get(var, var.lower())
                            tract_obj[prop_name] = float(value)
                        except (ValueError, TypeError):
                            pass
                
                result.append(tract_obj)
                
            return result
            
        except requests.RequestException as e:
            print(f"Error fetching ACS data: {e}")
            raise
    
    def fetch_acs_states(
        self,
        year: int,
        variables: List[str],
        dataset: str = "acs/acs5"
    ) -> List[Dict[str, any]]:
        """
        Fetch ACS data for all states.
        
        Args:
            year: Census year
            variables: List of variable IDs
            dataset: Dataset name (default: "acs/acs5")
            
        Returns:
            List of dictionaries, one per state
        """
        if not self.connected:
            raise RuntimeError("Adapter not connected")
            
        url = f"{self.BASE_URL}/{year}/{dataset}"
        params = {
            "get": ",".join(["NAME"] + variables),
            "for": "state:*",
        }
        
        if self.api_key:
            params["key"] = self.api_key
            
        try:
            response = requests.get(url, params=params, timeout=30)
            response.raise_for_status()
            data = response.json()
            
            headers = data[0]
            rows = data[1:]
            
            result = []
            for row in rows:
                state_data = dict(zip(headers, row))
                state_fips = state_data.get("state", "")
                geoid = f"04000US{state_fips}"
                geoid_year = f"{geoid}_{year}"
                
                state_obj = {
                    "geoid_year": geoid_year,
                    "geoid": geoid,
                    "state_fips": state_fips,
                    "name": state_data.get("NAME", ""),
                    "year": year,
                }
                
                # Add variable values
                for var in variables:
                    value = state_data.get(var, None)
                    if value:
                        try:
                            var_mapping = {
                                "B19013_001E": "median_household_income",
                                "B01001_001E": "total_population",
                            }
                            prop_name = var_mapping.get(var, var.lower())
                            state_obj[prop_name] = float(value)
                        except (ValueError, TypeError):
                            pass
                
                result.append(state_obj)
                
            return result
            
        except requests.RequestException as e:
            print(f"Error fetching state data: {e}")
            raise
    
    def fetch_acs_counties(
        self,
        year: int,
        state_fips: str,
        variables: List[str],
        dataset: str = "acs/acs5"
    ) -> List[Dict[str, any]]:
        """
        Fetch ACS data for all counties in a state.
        
        Args:
            year: Census year
            state_fips: State FIPS code
            variables: List of variable IDs
            dataset: Dataset name (default: "acs/acs5")
            
        Returns:
            List of dictionaries, one per county
        """
        if not self.connected:
            raise RuntimeError("Adapter not connected")
            
        url = f"{self.BASE_URL}/{year}/{dataset}"
        params = {
            "get": ",".join(["NAME"] + variables),
            "for": "county:*",
            "in": f"state:{state_fips}",
        }
        
        if self.api_key:
            params["key"] = self.api_key
            
        try:
            response = requests.get(url, params=params, timeout=30)
            response.raise_for_status()
            data = response.json()
            
            headers = data[0]
            rows = data[1:]
            
            result = []
            for row in rows:
                county_data = dict(zip(headers, row))
                state = county_data.get("state", "")
                county = county_data.get("county", "")
                county_fips = f"{state}{county}"
                geoid = f"05000US{county_fips}"
                geoid_year = f"{geoid}_{year}"
                
                county_obj = {
                    "geoid_year": geoid_year,
                    "geoid": geoid,
                    "state_fips": state,
                    "county_fips": county_fips,
                    "name": county_data.get("NAME", ""),
                    "year": year,
                }
                
                # Add variable values
                for var in variables:
                    value = county_data.get(var, None)
                    if value:
                        try:
                            var_mapping = {
                                "B19013_001E": "median_household_income",
                                "B01001_001E": "total_population",
                            }
                            prop_name = var_mapping.get(var, var.lower())
                            county_obj[prop_name] = float(value)
                        except (ValueError, TypeError):
                            pass
                
                result.append(county_obj)
                
            return result
            
        except requests.RequestException as e:
            print(f"Error fetching county data: {e}")
            raise
    
    def fetch_acs_pumas(
        self,
        year: int,
        state_fips: str,
        variables: List[str],
        dataset: str = "acs/acs5"
    ) -> List[Dict[str, any]]:
        """
        Fetch ACS data for all PUMAs in a state.
        
        Args:
            year: Census year
            state_fips: State FIPS code
            variables: List of variable IDs
            dataset: Dataset name (default: "acs/acs5")
            
        Returns:
            List of dictionaries, one per PUMA
        """
        if not self.connected:
            raise RuntimeError("Adapter not connected")
            
        url = f"{self.BASE_URL}/{year}/{dataset}"
        params = {
            "get": ",".join(["NAME"] + variables),
            "for": "public use microdata area:*",
            "in": f"state:{state_fips}",
        }
        
        if self.api_key:
            params["key"] = self.api_key
            
        try:
            response = requests.get(url, params=params, timeout=30)
            response.raise_for_status()
            data = response.json()
            
            headers = data[0]
            rows = data[1:]
            
            result = []
            for row in rows:
                puma_data = dict(zip(headers, row))
                state = puma_data.get("state", "")
                puma = puma_data.get("public use microdata area", "")
                puma_id = f"{state}{puma}"
                puma_id_year = f"{puma_id}_{year}"
                
                puma_obj = {
                    "puma_id_year": puma_id_year,
                    "puma_id": puma_id,
                    "state_fips": state,
                    "year": year,
                    "name": puma_data.get("NAME", ""),
                    "associated_pums_file_id": f"pums_{year}_{puma}",
                }
                
                # Add variable values
                for var in variables:
                    value = puma_data.get(var, None)
                    if value:
                        try:
                            var_mapping = {
                                "B19013_001E": "median_household_income",
                                "B01001_001E": "total_population",
                            }
                            prop_name = var_mapping.get(var, var.lower())
                            puma_obj[prop_name] = float(value)
                        except (ValueError, TypeError):
                            pass
                
                result.append(puma_obj)
                
            return result
            
        except requests.RequestException as e:
            print(f"Error fetching PUMA data: {e}")
            raise
    
    def read_rows(self, limit: Optional[int] = None) -> List[Dict[str, any]]:
        """Not implemented - use fetch_acs_tracts instead."""
        raise NotImplementedError("Use fetch_acs_tracts method directly")
    
    def get_schema(self) -> Dict[str, str]:
        """Get schema for ACS tract data."""
        return {
            "geoid_year": "string",
            "geoid": "string",
            "year": "integer",
            "state_fips": "string",
            "county_fips": "string",
            "tract_ce": "string",
            "median_household_income": "double",
            "median_rent": "double",
            "total_population": "integer",
        }


def ingest_acs_tracts(
    year: int,
    state_fips: str,
    output_dir: str,
    api_key: Optional[str] = None,
    variables: Optional[List[str]] = None
) -> str:
    """
    Ingest ACS tract data and save to JSON.
    
    Args:
        year: Census year
        state_fips: State FIPS code
        output_dir: Output directory for JSON files
        api_key: Optional API key
        variables: List of variable IDs (default: common variables)
        
    Returns:
        Path to created JSON file
    """
    if variables is None:
        variables = [
            "B19013_001E",  # Median Household Income
            "B25064_001E",  # Median Rent
            "B01001_001E",  # Total Population
        ]
    
    adapter = CensusDataAPIAdapter(api_key)
    adapter.connect()
    
    try:
        print(f"Fetching ACS {year} tract data for state {state_fips}...")
        tracts = adapter.fetch_acs_tracts(year, state_fips, variables)
        
        print(f"Fetched {len(tracts)} tracts")
        
        # Save to JSON
        output_path = Path(output_dir) / "tracts.json"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Load existing data if it exists
        existing_data = []
        if output_path.exists():
            with open(output_path, "r") as f:
                existing_data = json.load(f)
        
        # Filter out existing entries for this year/state and add new ones
        existing_data = [t for t in existing_data 
                        if not (t.get("year") == year and t.get("state_fips") == state_fips)]
        existing_data.extend(tracts)
        
        with open(output_path, "w") as f:
            json.dump(existing_data, f, indent=2)
        
        print(f"Saved to {output_path}")
        return str(output_path)
        
    finally:
        adapter.disconnect()


def ingest_acs_states(
    year: int,
    output_dir: str,
    api_key: Optional[str] = None,
    variables: Optional[List[str]] = None
) -> str:
    """
    Ingest ACS state data and save to JSON.
    
    Args:
        year: Census year
        output_dir: Output directory for JSON files
        api_key: Optional API key
        variables: List of variable IDs (default: common variables)
        
    Returns:
        Path to created JSON file
    """
    if variables is None:
        variables = [
            "B19013_001E",  # Median Household Income
            "B01001_001E",  # Total Population
        ]
    
    adapter = CensusDataAPIAdapter(api_key)
    adapter.connect()
    
    try:
        print(f"Fetching ACS {year} state data...")
        states = adapter.fetch_acs_states(year, variables)
        
        print(f"Fetched {len(states)} states")
        
        # Save to JSON
        output_path = Path(output_dir) / "states.json"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Load existing data if it exists
        existing_data = []
        if output_path.exists():
            with open(output_path, "r") as f:
                existing_data = json.load(f)
        
        # Filter out existing entries for this year and add new ones
        existing_data = [s for s in existing_data if s.get("year") != year]
        existing_data.extend(states)
        
        with open(output_path, "w") as f:
            json.dump(existing_data, f, indent=2)
        
        print(f"Saved to {output_path}")
        return str(output_path)
        
    finally:
        adapter.disconnect()


def ingest_acs_counties(
    year: int,
    state_fips: str,
    output_dir: str,
    api_key: Optional[str] = None,
    variables: Optional[List[str]] = None
) -> str:
    """
    Ingest ACS county data and save to JSON.
    
    Args:
        year: Census year
        state_fips: State FIPS code
        output_dir: Output directory for JSON files
        api_key: Optional API key
        variables: List of variable IDs (default: common variables)
        
    Returns:
        Path to created JSON file
    """
    if variables is None:
        variables = [
            "B19013_001E",  # Median Household Income
            "B01001_001E",  # Total Population
        ]
    
    adapter = CensusDataAPIAdapter(api_key)
    adapter.connect()
    
    try:
        print(f"Fetching ACS {year} county data for state {state_fips}...")
        counties = adapter.fetch_acs_counties(year, state_fips, variables)
        
        print(f"Fetched {len(counties)} counties")
        
        # Save to JSON
        output_path = Path(output_dir) / "counties.json"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Load existing data if it exists
        existing_data = []
        if output_path.exists():
            with open(output_path, "r") as f:
                existing_data = json.load(f)
        
        # Filter out existing entries for this year/state and add new ones
        existing_data = [c for c in existing_data 
                        if not (c.get("year") == year and c.get("state_fips") == state_fips)]
        existing_data.extend(counties)
        
        with open(output_path, "w") as f:
            json.dump(existing_data, f, indent=2)
        
        print(f"Saved to {output_path}")
        return str(output_path)
        
    finally:
        adapter.disconnect()


def ingest_acs_pumas(
    year: int,
    state_fips: str,
    output_dir: str,
    api_key: Optional[str] = None,
    variables: Optional[List[str]] = None
) -> str:
    """
    Ingest ACS PUMA data and save to JSON.
    
    Args:
        year: Census year
        state_fips: State FIPS code
        output_dir: Output directory for JSON files
        api_key: Optional API key
        variables: List of variable IDs (default: common variables)
        
    Returns:
        Path to created JSON file
    """
    if variables is None:
        variables = [
            "B19013_001E",  # Median Household Income
            "B01001_001E",  # Total Population
        ]
    
    adapter = CensusDataAPIAdapter(api_key)
    adapter.connect()
    
    try:
        print(f"Fetching ACS {year} PUMA data for state {state_fips}...")
        pumas = adapter.fetch_acs_pumas(year, state_fips, variables)
        
        print(f"Fetched {len(pumas)} PUMAs")
        
        # Save to JSON
        output_path = Path(output_dir) / "pumas.json"
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Load existing data if it exists
        existing_data = []
        if output_path.exists():
            with open(output_path, "r") as f:
                existing_data = json.load(f)
        
        # Filter out existing entries for this year/state and add new ones
        existing_data = [p for p in existing_data 
                         if not (p.get("year") == year and p.get("state_fips") == state_fips)]
        existing_data.extend(pumas)
        
        with open(output_path, "w") as f:
            json.dump(existing_data, f, indent=2)
        
        print(f"Saved to {output_path}")
        return str(output_path)
        
    finally:
        adapter.disconnect()


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Ingest Census ACS data")
    parser.add_argument("--year", type=int, required=True, help="Census year")
    parser.add_argument("--state", type=str, help="State FIPS code (required for tracts, counties, PUMAs)")
    parser.add_argument("--output", type=str, default="../data", help="Output directory")
    parser.add_argument("--api-key", type=str, help="Census API key")
    parser.add_argument("--variables", nargs="+", help="Variable IDs to fetch")
    parser.add_argument("--geography", type=str, choices=["states", "counties", "tracts", "pumas", "all"],
                       default="all", help="Geography level to fetch")
    
    args = parser.parse_args()
    
    output_dir = Path(__file__).parent.parent / "data" if args.output == "../data" else Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    if args.geography in ["states", "all"]:
        ingest_acs_states(
            year=args.year,
            output_dir=str(output_dir),
            api_key=args.api_key,
            variables=args.variables
        )
    
    if args.geography in ["counties", "tracts", "pumas", "all"]:
        if not args.state:
            print("Error: --state is required for counties, tracts, and PUMAs")
            parser.print_help()
            sys.exit(1)
        
        if args.geography in ["counties", "all"]:
            ingest_acs_counties(
                year=args.year,
                state_fips=args.state,
                output_dir=str(output_dir),
                api_key=args.api_key,
                variables=args.variables
            )
        
        if args.geography in ["tracts", "all"]:
            ingest_acs_tracts(
                year=args.year,
                state_fips=args.state,
                output_dir=str(output_dir),
                api_key=args.api_key,
                variables=args.variables
            )
        
        if args.geography in ["pumas", "all"]:
            ingest_acs_pumas(
                year=args.year,
                state_fips=args.state,
                output_dir=str(output_dir),
                api_key=args.api_key,
                variables=args.variables
            )

