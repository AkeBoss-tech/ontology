#!/usr/bin/env python3
"""
Generate comprehensive census data using real Census Data API and IPUMS.

This script fetches real data from the Census Data API for aggregate statistics
and optionally processes IPUMS microdata files.
"""
import json
import sys
import os
from pathlib import Path
from datetime import datetime
from dotenv import load_dotenv

# Load environment variables
load_dotenv(Path(__file__).parent.parent / ".env")

# Import the ingestion modules
sys.path.insert(0, str(Path(__file__).parent))
from ingest_census_data import (
    CensusDataAPIAdapter,
    ingest_acs_states,
    ingest_acs_counties,
    ingest_acs_tracts,
    ingest_acs_pumas
)
from ingest_geography import TIGERwebAdapter


def fetch_geography_for_tracts(tracts_data: list, year: int) -> None:
    """Fetch real geography shapes for tracts from TIGERweb."""
    print("\nFetching geography shapes from TIGERweb...")
    adapter = TIGERwebAdapter()
    
    for i, tract in enumerate(tracts_data):
        geoid = tract.get("geoid", "").replace("14000US", "")
        if geoid:
            feature = adapter.fetch_tract_geojson(geoid, year)
            if feature and "geometry" in feature:
                tract["geoshape"] = json.dumps(feature["geometry"])
        
        if (i + 1) % 50 == 0:
            print(f"  Processed {i + 1}/{len(tracts_data)} tracts...")
    
    print(f"✓ Fetched geography for {len(tracts_data)} tracts")


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Generate comprehensive census data from real APIs")
    parser.add_argument("--year", type=int, default=2020, help="Census year (default: 2020)")
    parser.add_argument("--states", nargs="+", help="State FIPS codes to fetch (default: all states)")
    parser.add_argument("--api-key", type=str, help="Census API key (uses CENSUS_API_KEY from .env if not provided)")
    parser.add_argument("--fetch-geography", action="store_true", help="Fetch geography shapes from TIGERweb")
    parser.add_argument("--output", type=str, default="../data", help="Output directory")
    
    args = parser.parse_args()
    
    data_dir = Path(__file__).parent.parent / "data" if args.output == "../data" else Path(args.output)
    data_dir.mkdir(parents=True, exist_ok=True)
    
    api_key = args.api_key or os.getenv("CENSUS_API_KEY")
    
    print("=" * 60)
    print("Generating comprehensive census data from real APIs")
    print("=" * 60)
    print(f"Year: {args.year}")
    print(f"Output directory: {data_dir}")
    print()
    
    # Default states if not specified (use a few representative states for demo)
    default_states = ["06", "36", "48", "12", "17"]  # CA, NY, TX, FL, IL
    states_to_fetch = args.states if args.states else default_states
    
    # Fetch states
    print("1. Fetching state-level data...")
    try:
        ingest_acs_states(
            year=args.year,
            output_dir=str(data_dir),
            api_key=api_key
        )
    except Exception as e:
        print(f"   Error fetching states: {e}")
    
    # Fetch counties, tracts, and PUMAs for each state
    all_tracts = []
    all_counties = []
    all_pumas = []
    
    for state_fips in states_to_fetch:
        print(f"\n2. Fetching data for state {state_fips}...")
        
        # Fetch counties
        try:
            ingest_acs_counties(
                year=args.year,
                state_fips=state_fips,
                output_dir=str(data_dir),
                api_key=api_key
            )
        except Exception as e:
            print(f"   Error fetching counties for state {state_fips}: {e}")
        
        # Fetch tracts
        try:
            ingest_acs_tracts(
                year=args.year,
                state_fips=state_fips,
                output_dir=str(data_dir),
                api_key=api_key
            )
        except Exception as e:
            print(f"   Error fetching tracts for state {state_fips}: {e}")
        
        # Fetch PUMAs
        try:
            ingest_acs_pumas(
                year=args.year,
                state_fips=state_fips,
                output_dir=str(data_dir),
                api_key=api_key
            )
        except Exception as e:
            print(f"   Error fetching PUMAs for state {state_fips}: {e}")
    
    # Optionally fetch geography shapes
    if args.fetch_geography:
        print("\n3. Fetching geography shapes...")
        tracts_file = data_dir / "tracts.json"
        if tracts_file.exists():
            with open(tracts_file, "r") as f:
                tracts_data = json.load(f)
            fetch_geography_for_tracts(tracts_data, args.year)
            # Save updated tracts with geography
            with open(tracts_file, "w") as f:
                json.dump(tracts_data, f, indent=2)
    
    # Generate links
    print("\n4. Generating relationship links...")
    links = []
    
    # Load data to generate links
    states_file = data_dir / "states.json"
    counties_file = data_dir / "counties.json"
    tracts_file = data_dir / "tracts.json"
    households_file = data_dir / "households.json"
    
    if states_file.exists() and counties_file.exists():
        with open(states_file, "r") as f:
            states = json.load(f)
        with open(counties_file, "r") as f:
            counties = json.load(f)
    
    # State to County links
    for county in counties:
            if county.get("year") == args.year:
                state_geoid = f"04000US{county.get('state_fips', '')}"
        links.append({
            "link_type": "state_to_county",
                    "source_id": f"{state_geoid}_{args.year}",
                    "target_id": county.get("geoid_year", ""),
            "properties": {}
        })
    
    if counties_file.exists() and tracts_file.exists():
        with open(counties_file, "r") as f:
            counties = json.load(f)
        with open(tracts_file, "r") as f:
            tracts = json.load(f)
    
    # County to Tract links
    for tract in tracts:
            if tract.get("year") == args.year:
                county_geoid = f"05000US{tract.get('county_fips', '')}"
        links.append({
            "link_type": "county_to_tract",
                    "source_id": f"{county_geoid}_{args.year}",
                    "target_id": tract.get("geoid_year", ""),
            "properties": {}
        })
    
    if households_file.exists():
        with open(households_file, "r") as f:
            households = json.load(f)
    
    # Tract to Household links
    for household in households:
            if household.get("year") == args.year:
                tract_geoid = household.get("tract_geoid", "")
                if tract_geoid:
        links.append({
            "link_type": "tract_to_household",
                        "source_id": f"{tract_geoid}_{args.year}",
                        "target_id": household.get("household_id", ""),
            "properties": {}
        })
    
    # Save links
    links_file = data_dir / "links.json"
    existing_links = []
    if links_file.exists():
        with open(links_file, "r") as f:
            existing_links = json.load(f)
    
    existing_links.extend(links)
    with open(links_file, "w") as f:
        json.dump(existing_links, f, indent=2)
    
    print(f"✓ Created {len(links)} links")
    
    # Summary
    print("\n" + "=" * 60)
    print("Data generation complete!")
    print("=" * 60)
    
    # Count records
    counts = {}
    for file_name in ["states", "counties", "tracts", "pumas", "households", "persons", "links"]:
        file_path = data_dir / f"{file_name}.json"
        if file_path.exists():
            with open(file_path, "r") as f:
                data = json.load(f)
                counts[file_name] = len([d for d in data if d.get("year") == args.year])
    
    for name, count in counts.items():
        print(f"  {name}: {count} records")
    
    print(f"\nData files written to {data_dir}")
    print("\nNote: To add IPUMS microdata (households/persons), run:")
    print("  python ingest_ipums_data.py --data-file <ipums_file.csv> --year <year>")

if __name__ == "__main__":
    main()

