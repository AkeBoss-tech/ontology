# Census Data Ingestion Scripts

## Environment Setup

1. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```

2. Add your API keys to `.env`:
   ```bash
   IPUMS_API_KEY=your_key_here
   FRED_API_KEY=your_key_here
   CENSUS_API_KEY=your_key_here  # Optional but recommended
   ```

## Available Scripts

### `ingest_census_data.py`
Ingests aggregate statistics from the Census Data API (ACS and Decennial) for states, counties, tracts, and PUMAs.

**Usage:**
```bash
# Fetch all geographies for a state
python ingest_census_data.py \
    --year 2020 \
    --state 06 \
    --geography all

# Fetch only states (no --state required)
python ingest_census_data.py \
    --year 2020 \
    --geography states

# Fetch only tracts for a state
python ingest_census_data.py \
    --year 2020 \
    --state 06 \
    --geography tracts
```

**Options:**
- `--year`: Census year (required)
- `--state`: State FIPS code (required for counties, tracts, PUMAs; e.g., "06" for California)
- `--geography`: Geography level to fetch - `states`, `counties`, `tracts`, `pumas`, or `all` (default: `all`)
- `--output`: Output directory for JSON files (default: `../data`)
- `--api-key`: Census API key (optional, uses CENSUS_API_KEY from .env if not provided)
- `--variables`: Variable IDs to fetch (optional, defaults to common variables like B19013_001E, B01001_001E)

### `ingest_geography.py`
Fetches TIGER/Line shapefiles from TIGERweb REST API.

**Usage:**
```bash
# Fetch single tract
python ingest_geography.py \
    --geoid 06037001000 \
    --year 2020 \
    --output ../data/geography

# Fetch all tracts in a county
python ingest_geography.py \
    --state 06 \
    --county 037 \
    --year 2020 \
    --output ../data/geography
```

### `simple_map_view.py`
Generates an interactive HTML map visualization.

**Usage:**
```bash
python simple_map_view.py \
    --data ../data/tracts.json \
    --output map.html \
    --center-lat 40.7128 \
    --center-lon -74.0060 \
    --zoom 10
```

### `ingest_ipums_data.py`
Processes IPUMS microdata extracts (households and persons) from downloaded CSV files.

**Usage:**
```bash
# Process both households and persons
python ingest_ipums_data.py \
    --data-file /path/to/ipums_extract.csv \
    --year 2020 \
    --type both

# Process only households
python ingest_ipums_data.py \
    --data-file /path/to/ipums_extract.csv \
    --year 2020 \
    --type households
```

**Options:**
- `--data-file`: Path to IPUMS extract CSV file (required)
- `--year`: Census year (required)
- `--type`: Type of data to process - `households`, `persons`, or `both` (default: `both`)
- `--output`: Output directory for JSON files (default: `../data`)

**Note:** IPUMS requires creating data extracts via their web interface. After downloading the extract as CSV, use this script to process it.

### `generate_comprehensive_data.py`
Fetches comprehensive census data from real APIs for multiple states and years.

**Usage:**
```bash
# Fetch data for default states (CA, NY, TX, FL, IL)
python generate_comprehensive_data.py --year 2020

# Fetch data for specific states
python generate_comprehensive_data.py \
    --year 2020 \
    --states 06 36 48

# Fetch data with geography shapes
python generate_comprehensive_data.py \
    --year 2020 \
    --fetch-geography
```

**Options:**
- `--year`: Census year (default: 2020)
- `--states`: State FIPS codes to fetch (default: 06, 36, 48, 12, 17)
- `--api-key`: Census API key (optional)
- `--fetch-geography`: Fetch geography shapes from TIGERweb (slower but adds real boundaries)
- `--output`: Output directory (default: `../data`)

### `load_sample_data.py`
Loads real census data from APIs for a single state (replaces old sample data generation).

**Usage:**
```bash
# Load data for California (default)
python load_sample_data.py --year 2020

# Load data for New York
python load_sample_data.py \
    --year 2020 \
    --state 36
```

**Options:**
- `--year`: Census year (default: 2020)
- `--state`: State FIPS code (default: 06 for California)
- `--api-key`: Census API key (optional)
- `--output`: Output directory (default: `../data`)

### `test_ontology_loading.py`
Validates the census ontology YAML file.

**Usage:**
```bash
python test_ontology_loading.py
```

## API Keys

- **Census Data API**: Get at https://api.census.gov/data/key_signup.html
- **IPUMS**: Register at https://usa.ipums.org
- **FRED**: Get at https://fred.stlouisfed.org/docs/api/api_key.html

## Notes

- The `.env` file is gitignored to protect API keys
- Always use `.env.example` as a template
- API keys are loaded automatically by scripts using `python-dotenv`


