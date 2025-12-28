#!/usr/bin/env python3
"""
Load real census data from Census Data API and IPUMS for testing the ontology.

This script fetches real data from APIs instead of generating sample data.
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
    ingest_acs_states,
    ingest_acs_counties,
    ingest_acs_tracts,
    ingest_acs_pumas
)


def main():
    """Load real census data from APIs."""
    import argparse
    
    parser = argparse.ArgumentParser(description="Load real census data from APIs")
    parser.add_argument("--year", type=int, default=2020, help="Census year (default: 2020)")
    parser.add_argument("--state", type=str, default="06", help="State FIPS code (default: 06 for California)")
    parser.add_argument("--api-key", type=str, help="Census API key (uses CENSUS_API_KEY from .env if not provided)")
    parser.add_argument("--output", type=str, default="../data", help="Output directory")
    
    args = parser.parse_args()
    
    data_dir = Path(__file__).parent.parent / "data" if args.output == "../data" else Path(args.output)
    data_dir.mkdir(parents=True, exist_ok=True)
    
    api_key = args.api_key or os.getenv("CENSUS_API_KEY")
    
    print("=" * 60)
    print("Loading real census data from APIs")
    print("=" * 60)
    print(f"Year: {args.year}")
    print(f"State: {args.state}")
    print(f"Output directory: {data_dir}")
    print()
    
    # Fetch states
    print("1. Fetching state-level data...")
    try:
        ingest_acs_states(
            year=args.year,
            output_dir=str(data_dir),
            api_key=api_key
        )
    except Exception as e:
        print(f"   Error: {e}")
    
    # Fetch counties
    print(f"\n2. Fetching county-level data for state {args.state}...")
    try:
        ingest_acs_counties(
            year=args.year,
            state_fips=args.state,
            output_dir=str(data_dir),
            api_key=api_key
        )
    except Exception as e:
        print(f"   Error: {e}")
    
    # Fetch tracts
    print(f"\n3. Fetching tract-level data for state {args.state}...")
    try:
        ingest_acs_tracts(
            year=args.year,
            state_fips=args.state,
            output_dir=str(data_dir),
            api_key=api_key
        )
    except Exception as e:
        print(f"   Error: {e}")
    
    # Fetch PUMAs
    print(f"\n4. Fetching PUMA-level data for state {args.state}...")
    try:
        ingest_acs_pumas(
            year=args.year,
            state_fips=args.state,
            output_dir=str(data_dir),
            api_key=api_key
        )
    except Exception as e:
        print(f"   Error: {e}")
    
    # Create summary
    print("\n5. Creating summary...")
    summary = {
        "generated_at": datetime.now().isoformat(),
        "year": args.year,
        "state_fips": args.state,
        "counts": {}
    }
    
    for file_name in ["states", "counties", "tracts", "pumas", "households", "persons"]:
        file_path = data_dir / f"{file_name}.json"
        if file_path.exists():
            with open(file_path, "r") as f:
                data = json.load(f)
                summary["counts"][file_name] = len([d for d in data if d.get("year") == args.year])
        else:
            summary["counts"][file_name] = 0
    
    (data_dir / "summary.json").write_text(json.dumps(summary, indent=2))
    
    print("\n" + "=" * 60)
    print("Data loading complete!")
    print("=" * 60)
    for name, count in summary["counts"].items():
        print(f"  {name}: {count} records")
    
    print(f"\nData files written to {data_dir}")
    print("\nNote: To add IPUMS microdata (households/persons), run:")
    print("  python ingest_ipums_data.py --data-file <ipums_file.csv> --year <year>")

if __name__ == "__main__":
    main()
