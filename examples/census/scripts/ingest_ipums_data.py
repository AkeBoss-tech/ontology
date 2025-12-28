#!/usr/bin/env python3
"""
IPUMS Data Ingestion Script

Ingests microdata from IPUMS (households and persons) and stores them
in a format ready for the ontology framework.

Note: IPUMS requires creating data extracts via their web interface or API.
This script can process downloaded IPUMS extract files (CSV format).
"""
import pandas as pd
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Optional
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


class IPUMSAdapter(SourceAdapter):
    """Adapter for IPUMS microdata files."""
    
    def __init__(self, data_file: Optional[str] = None):
        """
        Initialize IPUMS adapter.
        
        Args:
            data_file: Path to IPUMS extract CSV file
        """
        self.data_file = data_file
        self.connected = False
        self.df = None
        
    def connect(self) -> None:
        """Load IPUMS data file."""
        if not self.data_file or not Path(self.data_file).exists():
            raise FileNotFoundError(f"IPUMS data file not found: {self.data_file}")
        
        print(f"Loading IPUMS data from {self.data_file}...")
        self.df = pd.read_csv(self.data_file, low_memory=False)
        self.connected = True
        print(f"Loaded {len(self.df)} records")
        
    def disconnect(self) -> None:
        """Close connection."""
        self.df = None
        self.connected = False
        
    def read_rows(self, limit: Optional[int] = None) -> List[Dict[str, any]]:
        """Read rows from IPUMS data."""
        if not self.connected:
            raise RuntimeError("Adapter not connected")
        
        df_subset = self.df.head(limit) if limit else self.df
        return df_subset.to_dict('records')
    
    def get_schema(self) -> Dict[str, str]:
        """Get schema for IPUMS data."""
        if not self.connected:
            raise RuntimeError("Adapter not connected")
        return {col: str(dtype) for col, dtype in self.df.dtypes.items()}


def process_ipums_households(
    data_file: str,
    output_dir: str,
    year: int,
    tract_geoid_col: str = "GEOID",
    serialno_col: str = "SERIAL",
    hhincome_col: str = "HHINCOME",
    hhwt_col: str = "HHWT"
) -> str:
    """
    Process IPUMS household data and save to JSON.
    
    Args:
        data_file: Path to IPUMS extract CSV file
        output_dir: Output directory for JSON files
        tract_geoid_col: Column name for tract GEOID
        serialno_col: Column name for household serial number
        hhincome_col: Column name for household income
        hhwt_col: Column name for household weight
        
    Returns:
        Path to created JSON file
    """
    print(f"Processing IPUMS household data from {data_file}...")
    
    df = pd.read_csv(data_file, low_memory=False)
    
    # Group by household (SERIAL) to get household-level data
    households = []
    household_ids = set()
    
    for _, row in df.iterrows():
        serialno = str(row.get(serialno_col, ""))
        if not serialno or serialno in household_ids:
            continue
        
        household_ids.add(serialno)
        
        # Get tract GEOID - IPUMS format may vary
        geoid_raw = str(row.get(tract_geoid_col, ""))
        # Convert to standard format if needed
        if geoid_raw and not geoid_raw.startswith("14000US"):
            # Assume it's just the numeric GEOID
            geoid = f"14000US{geoid_raw.zfill(11)}"
        else:
            geoid = geoid_raw if geoid_raw else ""
        
        household_data = {
            "household_id": f"HH{serialno.zfill(6)}",
            "tract_geoid": geoid,
            "year": year,
            "household_income": float(row.get(hhincome_col, 0)) if pd.notna(row.get(hhincome_col)) else 0,
            "original_weight": float(row.get(hhwt_col, 1.0)) if pd.notna(row.get(hhwt_col)) else 1.0,
        }
        
        # Add other household variables if available
        if "HHTYPE" in row:
            household_data["household_type"] = str(row["HHTYPE"])
        if "NUMPREC" in row:
            household_data["household_size"] = int(row["NUMPREC"]) if pd.notna(row["NUMPREC"]) else 1
        if "OWNERSHP" in row:
            household_data["tenure"] = "Owned" if row["OWNERSHP"] == 1 else "Rented"
        
        households.append(household_data)
    
    print(f"Processed {len(households)} households")
    
    # Save to JSON
    output_path = Path(output_dir) / "households.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Load existing data if it exists
    existing_data = []
    if output_path.exists():
        with open(output_path, "r") as f:
            existing_data = json.load(f)
    
    # Filter out existing entries for this year and add new ones
    existing_data = [h for h in existing_data if h.get("year") != year]
    existing_data.extend(households)
    
    with open(output_path, "w") as f:
        json.dump(existing_data, f, indent=2)
    
    print(f"Saved to {output_path}")
    return str(output_path)


def process_ipums_persons(
    data_file: str,
    output_dir: str,
    year: int,
    serialno_col: str = "SERIAL",
    pernum_col: str = "PERNUM",
    age_col: str = "AGE",
    sex_col: str = "SEX",
    race_col: str = "RACE",
    educ_col: str = "EDUC",
    occ_col: str = "OCC",
    ind_col: str = "IND",
    incwage_col: str = "INCWAGE",
    uhrswork_col: str = "UHRSWORK"
) -> str:
    """
    Process IPUMS person data and save to JSON.
    
    Args:
        data_file: Path to IPUMS extract CSV file
        output_dir: Output directory for JSON files
        year: Census year
        serialno_col: Column name for household serial number
        pernum_col: Column name for person number
        age_col: Column name for age
        sex_col: Column name for sex
        race_col: Column name for race
        educ_col: Column name for education
        occ_col: Column name for occupation
        ind_col: Column name for industry
        incwage_col: Column name for wage income
        uhrswork_col: Column name for hours worked
        
    Returns:
        Path to created JSON file
    """
    print(f"Processing IPUMS person data from {data_file}...")
    
    df = pd.read_csv(data_file, low_memory=False)
    
    persons = []
    
    for _, row in df.iterrows():
        serialno = str(row.get(serialno_col, ""))
        pernum = str(row.get(pernum_col, "1"))
        person_id = f"P{serialno.zfill(6)}{pernum.zfill(2)}"
        
        person_data = {
            "person_id": person_id,
            "household_id": f"HH{serialno.zfill(6)}",
            "year": year,
            "age": int(row.get(age_col, 0)) if pd.notna(row.get(age_col)) else 0,
        }
        
        # Map sex
        sex_val = row.get(sex_col)
        if pd.notna(sex_val):
            person_data["sex"] = "Male" if sex_val == 1 else "Female"
        
        # Map race (IPUMS race codes)
        race_val = row.get(race_col)
        if pd.notna(race_val):
            race_map = {
                1: "White",
                2: "Black or African American",
                3: "American Indian or Alaska Native",
                4: "Chinese",
                5: "Japanese",
                6: "Other Asian or Pacific Islander",
                7: "Other race, nec",
                8: "Two major races",
                9: "Three or more major races"
            }
            person_data["race_code"] = race_map.get(int(race_val), "Other")
        
        # Map education
        educ_val = row.get(educ_col)
        if pd.notna(educ_val):
            educ_map = {
                0: "N/A",
                1: "N/A or no schooling",
                2: "Nursery school to grade 4",
                3: "Grade 5, 6, 7, or 8",
                4: "Grade 9",
                5: "Grade 10",
                6: "Grade 11",
                7: "Grade 12",
                8: "1 year of college",
                9: "2 years of college",
                10: "3 years of college",
                11: "4 years of college",
                12: "5+ years of college"
            }
            person_data["education_attainment"] = educ_map.get(int(educ_val), "Unknown")
        
        # Occupation and industry
        occ_val = row.get(occ_col)
        if pd.notna(occ_val) and occ_val != 0:
            person_data["occupation_code"] = str(int(occ_val)).zfill(4)
        else:
            person_data["occupation_code"] = "Not in labor force"
        
        ind_val = row.get(ind_col)
        if pd.notna(ind_val) and ind_val != 0:
            person_data["industry_code"] = str(int(ind_val)).zfill(4)
        else:
            person_data["industry_code"] = "Not applicable"
        
        # Wages and hours
        wage_val = row.get(incwage_col)
        person_data["wages"] = float(wage_val) if pd.notna(wage_val) and wage_val > 0 else 0
        
        hours_val = row.get(uhrswork_col)
        person_data["hours_worked"] = int(hours_val) if pd.notna(hours_val) and hours_val > 0 else 0
        
        persons.append(person_data)
    
    print(f"Processed {len(persons)} persons")
    
    # Save to JSON
    output_path = Path(output_dir) / "persons.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Load existing data if it exists
    existing_data = []
    if output_path.exists():
        with open(output_path, "r") as f:
            existing_data = json.load(f)
    
    # Filter out existing entries for this year and add new ones
    existing_data = [p for p in existing_data if p.get("year") != year]
    existing_data.extend(persons)
    
    with open(output_path, "w") as f:
        json.dump(existing_data, f, indent=2)
    
    print(f"Saved to {output_path}")
    return str(output_path)


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Ingest IPUMS microdata")
    parser.add_argument("--data-file", type=str, required=True, help="Path to IPUMS extract CSV file")
    parser.add_argument("--year", type=int, required=True, help="Census year")
    parser.add_argument("--output", type=str, default="../data", help="Output directory")
    parser.add_argument("--type", type=str, choices=["households", "persons", "both"], default="both",
                       help="Type of data to process")
    
    args = parser.parse_args()
    
    output_dir = Path(__file__).parent.parent / "data" if args.output == "../data" else Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    if args.type in ["households", "both"]:
        process_ipums_households(
            data_file=args.data_file,
            output_dir=str(output_dir),
            year=args.year
        )
    
    if args.type in ["persons", "both"]:
        process_ipums_persons(
            data_file=args.data_file,
            output_dir=str(output_dir),
            year=args.year
        )
    
    print("\nNote: To get IPUMS data:")
    print("1. Register at https://usa.ipums.org")
    print("2. Create a data extract with desired variables")
    print("3. Download the extract as CSV")
    print("4. Run this script with --data-file pointing to the CSV")

