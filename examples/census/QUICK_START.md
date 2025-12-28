# Quick Start Guide

## Complete Setup in 3 Steps

### Step 1: Generate Data
```bash
cd examples/census
python3 scripts/load_sample_data.py
```

### Step 2: Start Server (Terminal 1)
```bash
cd ../../rust-core/graphql-api
cargo run --bin server
```

Wait for: `Starting GraphQL server on http://localhost:8080`

### Step 3: Start Frontend (Terminal 2)
```bash
cd ../../ui-framework/apps/census-example
npm install  # First time only
npm run dev
```

Wait for: `Local: http://localhost:3000`

## Open in Browser

Navigate to: **http://localhost:3000**

You should see:
- Navigation bar with 5 pages
- Tract Map page by default
- Time slider showing years 1990-2020

## Test Each Feature

1. **Tract Map**: Use time slider to filter by year
2. **PUMS Analysis**: Search for a tract, then traverse graph
3. **Crosswalk**: Select source tract and target year
4. **Cohort Builder**: Add filters and search
5. **Data Sources**: View data source list

## Verify It's Working

Check browser console (F12) - should see:
- GraphQL queries being made
- Responses from server
- No connection errors

## If Something Doesn't Work

1. Check server is running: `curl http://localhost:8080`
2. Check data exists: `ls examples/census/data/`
3. Check browser console for errors
4. See `TESTING.md` for detailed troubleshooting



