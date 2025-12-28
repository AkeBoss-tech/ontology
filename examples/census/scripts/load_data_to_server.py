#!/usr/bin/env python3
"""
Load census data into the GraphQL server via HTTP requests.
"""
import json
import requests
import sys
from pathlib import Path
from typing import List, Dict, Any

GRAPHQL_ENDPOINT = "http://localhost:8080/graphql"

def execute_graphql(query: str, variables: Dict[str, Any] = None) -> Dict[str, Any]:
    """Execute a GraphQL query."""
    response = requests.post(
        GRAPHQL_ENDPOINT,
        json={"query": query, "variables": variables or {}},
        headers={"Content-Type": "application/json"},
    )
    if response.status_code != 200:
        raise Exception(f"GraphQL request failed: {response.status_code} - {response.text}")
    data = response.json()
    if "errors" in data:
        raise Exception(f"GraphQL errors: {data['errors']}")
    return data.get("data", {})

def create_object_mutation(object_type: str, properties: Dict[str, Any]) -> str:
    """Create a mutation to add an object."""
    # Convert properties to GraphQL input format
    props_str = json.dumps(properties, ensure_ascii=False)
    
    mutation = f"""
    mutation CreateObject {{
      createObject(
        objectType: "{object_type}"
        properties: {props_str}
      ) {{
        objectId
        success
      }}
    }}
    """
    return mutation

def load_objects_from_file(file_path: Path, object_type: str, batch_size: int = 50):
    """Load objects from a JSON file."""
    print(f"Loading {object_type} from {file_path.name}...")
    
    with open(file_path) as f:
        objects = json.load(f)
    
    total = len(objects)
    print(f"  Found {total} objects")
    
    # Since we don't have a createObject mutation yet, we'll use a simpler approach:
    # Store data in a format the server can read, or use a direct API
    
    # For now, let's create a simple HTTP endpoint approach
    # We'll create objects via a bulk import endpoint if it exists
    
    # Alternative: Write to a file that the server can read
    output_file = Path(__file__).parent.parent / "data" / f"{object_type}_import.json"
    with open(output_file, "w") as f:
        json.dump(objects, f, indent=2)
    
    print(f"  Wrote {total} objects to {output_file.name}")
    return total

def main():
    """Load all census data."""
    data_dir = Path(__file__).parent.parent / "data"
    
    if not data_dir.exists():
        print(f"Error: Data directory not found: {data_dir}")
        sys.exit(1)
    
    print("Loading census data into server...")
    print(f"GraphQL endpoint: {GRAPHQL_ENDPOINT}")
    print()
    
    # Check if server is running
    try:
        response = requests.get("http://localhost:8080", timeout=2)
        print("✓ GraphQL server is running")
    except requests.exceptions.RequestException:
        print("⚠ Warning: GraphQL server may not be running at http://localhost:8080")
        print("  Start it with: cd rust-core/graphql-api && cargo run --bin server")
        print()
    
    total_loaded = 0
    
    # Load states
    states_file = data_dir / "states.json"
    if states_file.exists():
        total_loaded += load_objects_from_file(states_file, "state_vintage")
    
    # Load counties
    counties_file = data_dir / "counties.json"
    if counties_file.exists():
        total_loaded += load_objects_from_file(counties_file, "county_vintage")
    
    # Load tracts
    tracts_file = data_dir / "tracts.json"
    if tracts_file.exists():
        total_loaded += load_objects_from_file(tracts_file, "census_tract_vintage")
    
    # Load households
    households_file = data_dir / "households.json"
    if households_file.exists():
        total_loaded += load_objects_from_file(households_file, "pums_household")
    
    # Load persons
    persons_file = data_dir / "persons.json"
    if persons_file.exists():
        total_loaded += load_objects_from_file(persons_file, "pums_person")
    
    print()
    print(f"✓ Prepared {total_loaded} objects for import")
    print()
    print("Note: The GraphQL server needs to be configured to read from these import files.")
    print("For now, data files are ready at:")
    print(f"  {data_dir}")
    print()
    print("To make data available, the server needs to:")
    print("  1. Read from the import JSON files on startup, OR")
    print("  2. Implement a createObject mutation, OR")
    print("  3. Load data directly into the event log/store backends")

if __name__ == "__main__":
    main()




