#!/usr/bin/env python3
"""
Verify that the census example is set up correctly.
"""
import json
import sys
from pathlib import Path

def check_data_files():
    """Check that sample data files exist."""
    data_dir = Path(__file__).parent.parent / "data"
    required_files = [
        "tracts.json",
        "counties.json",
        "pumas.json",
        "crosswalks.json",
        "households.json",
        "persons.json"
    ]
    
    missing = []
    for file in required_files:
        if not (data_dir / file).exists():
            missing.append(file)
    
    if missing:
        print(f"❌ Missing data files: {', '.join(missing)}")
        print("   Run: python3 scripts/load_sample_data.py")
        return False
    
    print("✓ All data files exist")
    
    # Check data counts
    for file in required_files:
        with open(data_dir / file) as f:
            data = json.load(f)
            print(f"  - {file}: {len(data)} records")
    
    return True

def check_ontology():
    """Check that ontology file exists and is valid."""
    ontology_path = Path(__file__).parent.parent / "config" / "census_ontology.yaml"
    
    if not ontology_path.exists():
        print(f"❌ Ontology file not found: {ontology_path}")
        return False
    
    print(f"✓ Ontology file exists: {ontology_path}")
    
    # Basic YAML validation
    try:
        import yaml
        with open(ontology_path) as f:
            data = yaml.safe_load(f)
            if "ontology" not in data:
                print("❌ Invalid ontology structure")
                return False
            
            obj_types = len(data["ontology"].get("objectTypes", []))
            link_types = len(data["ontology"].get("linkTypes", []))
            print(f"  - {obj_types} object types")
            print(f"  - {link_types} link types")
    except Exception as e:
        print(f"❌ Error reading ontology: {e}")
        return False
    
    return True

def main():
    """Run all checks."""
    print("=== Verifying Census Example Setup ===\n")
    
    checks = [
        ("Data Files", check_data_files),
        ("Ontology", check_ontology),
    ]
    
    all_passed = True
    for name, check_func in checks:
        print(f"\n{name}:")
        if not check_func():
            all_passed = False
    
    print("\n" + "=" * 40)
    if all_passed:
        print("✓ All checks passed!")
        print("\nNext steps:")
        print("1. Start GraphQL server: cd ../../rust-core/graphql-api && cargo run --bin server")
        print("2. Start frontend: cd ../../ui-framework/apps/census-example && npm install && npm run dev")
        return 0
    else:
        print("❌ Some checks failed. Please fix the issues above.")
        return 1

if __name__ == "__main__":
    sys.exit(main())







