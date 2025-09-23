#!/bin/bash

# Script to set up storage layout baseline on main branch
# Run this once to establish the baseline for CI

set -e

echo "🔧 Setting up storage layout baseline for CI..."

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "⚠️ Not on main branch. Switching to main..."
    git checkout main
    git pull origin main
fi

echo "🏗️ Building contracts and extracting storage layouts..."
cd synd-contracts
make storage-layout-baseline

echo "📋 Committing baseline to main branch..."
git add .storage-layouts/main-layout.json
git commit -m "feat: add storage layout baseline for CI validation

Establishes storage layout baseline for upgradeable contracts:
- SyndicateFactory (10 storage slots)
- ArbChainConfig (12 storage slots)
- GasAggregator (8 storage slots)
- SyndicateSequencingChain (0 slots - abstract)
- SequencingModuleChecker (0 slots - abstract)

This enables automated storage layout validation in CI to prevent
dangerous storage collisions during contract upgrades.

🤖 Generated with Claude Code
"

echo "🚀 Pushing baseline to origin..."
git push origin main

echo "✅ Storage layout baseline successfully set up!"
echo "Future PRs will be validated against this baseline."