#!/bin/bash

# Storage layout baseline update script
# This script checks if storage layouts need updating and updates them automatically

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTRACT_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"

cd "$CONTRACT_DIR"

echo "🔍 Checking if storage layout baseline needs updating..."

# Check if any Solidity files changed
if git diff --cached --name-only | grep -q "\.sol$"; then
    echo "📝 Solidity files detected in commit, updating storage layout baseline..."

    # Only rebuild if we don't have current build artifacts
    if [ ! -f ".storage-layouts/current.json" ]; then
        echo "🏗️ Building contracts..."
        forge build --build-info
        mkdir -p .storage-layouts
        cp out/build-info/*.json .storage-layouts/current.json 2>/dev/null || echo "No build-info files found"
    fi

    # Extract current layout as the new baseline
    if [ -f ".storage-layouts/current.json" ]; then
        echo "📊 Extracting storage layouts..."
        node scripts/storage-checks/simple-extract.js .storage-layouts/current.json > .storage-layouts/main-layout.json.tmp 2>/dev/null || echo "{}" > .storage-layouts/main-layout.json.tmp

        # Only update if extraction succeeded and produced meaningful output
        if [ -s .storage-layouts/main-layout.json.tmp ] && [ "$(cat .storage-layouts/main-layout.json.tmp)" != "{}" ]; then
            mv .storage-layouts/main-layout.json.tmp .storage-layouts/main-layout.json

            # Add the updated baseline to the commit
            git add .storage-layouts/main-layout.json
            echo "✅ Storage layout baseline updated and added to commit"
        else
            rm -f .storage-layouts/main-layout.json.tmp
            echo "⚠️ Storage extraction failed, keeping existing baseline"
        fi
    else
        echo "⚠️ No build artifacts found, skipping baseline update"
    fi

    # Clean up temporary files
    rm -f .storage-layouts/current*.json .storage-layouts/temp*
else
    echo "ℹ️ No Solidity files changed, skipping storage layout update"
fi

echo "🎯 Storage baseline check complete"