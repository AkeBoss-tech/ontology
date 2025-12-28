#!/bin/bash
# Test script for the census example frontend

set -e

echo "=== Testing Census Example ==="

# Check if data exists
if [ ! -f "data/tracts.json" ]; then
    echo "Generating sample data..."
    python3 scripts/load_sample_data.py
fi

echo ""
echo "=== Sample Data Generated ==="
echo "✓ Tracts: $(jq length data/tracts.json)"
echo "✓ Counties: $(jq length data/counties.json)"
echo "✓ PUMAs: $(jq length data/pumas.json)"
echo "✓ Crosswalks: $(jq length data/crosswalks.json)"
echo "✓ Households: $(jq length data/households.json)"
echo "✓ Persons: $(jq length data/persons.json)"

echo ""
echo "=== Next Steps ==="
echo "1. Start the GraphQL server:"
echo "   cd ../../rust-core/graphql-api"
echo "   cargo run --bin server"
echo ""
echo "2. In another terminal, start the frontend:"
echo "   cd ../../ui-framework/apps/census-example"
echo "   npm install"
echo "   npm run dev"
echo ""
echo "3. Open http://localhost:3000 in your browser"



