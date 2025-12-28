#!/bin/bash

# Script to create a new ontology application from template
# Usage: ./scripts/create-app.sh <app-name> [display-name] [description]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if app name is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: App name is required${NC}"
    echo "Usage: ./scripts/create-app.sh <app-name> [display-name] [description]"
    echo "Example: ./scripts/create-app.sh my-app \"My App\" \"A description of my app\""
    exit 1
fi

APP_NAME="$1"
DISPLAY_NAME="${2:-$APP_NAME}"
DESCRIPTION="${3:-A new ontology application}"

# Validate app name (kebab-case)
if [[ ! "$APP_NAME" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
    echo -e "${RED}Error: App name must be in kebab-case (lowercase letters, numbers, and hyphens only)${NC}"
    echo "Example: my-app, financial-portfolio, supply-chain"
    exit 1
fi

# Get the script directory and project root
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEMPLATE_DIR="$PROJECT_ROOT/ui-framework/templates/app-template"
APP_DIR="$PROJECT_ROOT/ui-framework/apps/$APP_NAME"

# Check if app already exists
if [ -d "$APP_DIR" ]; then
    echo -e "${RED}Error: App '$APP_NAME' already exists at $APP_DIR${NC}"
    exit 1
fi

# Check if template exists
if [ ! -d "$TEMPLATE_DIR" ]; then
    echo -e "${RED}Error: Template directory not found at $TEMPLATE_DIR${NC}"
    exit 1
fi

echo -e "${GREEN}Creating new app: $APP_NAME${NC}"
echo "  Display Name: $DISPLAY_NAME"
echo "  Description: $DESCRIPTION"
echo "  Directory: $APP_DIR"

# Create app directory
mkdir -p "$APP_DIR"

# Copy template files
echo -e "${YELLOW}Copying template files...${NC}"
cp -r "$TEMPLATE_DIR"/* "$APP_DIR/"
cp -r "$TEMPLATE_DIR"/. "$APP_DIR/" 2>/dev/null || true

# Replace placeholders in files
echo -e "${YELLOW}Replacing placeholders...${NC}"

# Replace placeholders in files
# Use sed with different commands for macOS vs Linux
for file in $(find "$APP_DIR" -type f \( -name "*.json" -o -name "*.ts" -o -name "*.tsx" -o -name "*.html" -o -name "*.md" \)); do
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/{{APP_NAME}}/$APP_NAME/g" "$file"
        sed -i '' "s/{{APP_DISPLAY_NAME}}/$DISPLAY_NAME/g" "$file"
        sed -i '' "s/{{APP_DESCRIPTION}}/$DESCRIPTION/g" "$file"
    else
        sed -i "s/{{APP_NAME}}/$APP_NAME/g" "$file"
        sed -i "s/{{APP_DISPLAY_NAME}}/$DISPLAY_NAME/g" "$file"
        sed -i "s/{{APP_DESCRIPTION}}/$DESCRIPTION/g" "$file"
    fi
done

echo -e "${GREEN}✓ App created successfully!${NC}"
echo ""
echo "Next steps:"
echo "  1. cd ui-framework/apps/$APP_NAME"
echo "  2. npm install"
echo "  3. (Optional) Create .env file with VITE_GRAPHQL_URL=http://localhost:8080/graphql"
echo "  4. npm run dev"
echo ""
echo "Don't forget to:"
echo "  - Update object types in src/pages/Search.tsx and src/pages/Browse.tsx"
echo "  - Customize pages in src/pages/"
echo "  - Add new navigation items in src/App.tsx if needed"
echo ""
echo "See ui-framework/APPLICATION_GUIDE.md for detailed documentation."
