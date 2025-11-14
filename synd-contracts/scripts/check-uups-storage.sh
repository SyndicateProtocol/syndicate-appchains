#!/bin/bash
# Script to check that UUPS upgradeable contracts have empty storage layouts
# This ensures all storage is in ERC-7201 namespaced storage slots

set -e

# UUPS Upgradeable contracts that should have empty storage
UUPS_CONTRACTS=(
    "SyndicateSequencingChain"
    "GasArchive"
)

echo "================================================"
echo "UUPS Storage Layout Validation"
echo "================================================"
echo ""
echo "Checking that UUPS contracts use only namespaced storage..."
echo ""

FAILED=0
PASSED=0

for contract in "${UUPS_CONTRACTS[@]}"; do
    echo "Checking $contract..."

    # Get storage layout
    STORAGE_OUTPUT=$(forge inspect "$contract" storageLayout 2>&1 || true)

    if echo "$STORAGE_OUTPUT" | grep -q "Error"; then
        echo "✗ FAILED: Could not inspect $contract"
        echo "$STORAGE_OUTPUT"
        FAILED=$((FAILED + 1))
        continue
    fi

    # Check if storage layout is empty (should only have the table header)
    # An empty storage layout will have a table with no data rows
    STORAGE_COUNT=$(echo "$STORAGE_OUTPUT" | grep -E "^\|" | grep -v "^\+=" | grep -v "Name.*Type.*Slot" | grep -v "^+-" | wc -l | tr -d ' ')

    if [ "$STORAGE_COUNT" -eq "0" ]; then
        echo "✓ PASSED: $contract has empty storage (uses namespaced storage)"
        PASSED=$((PASSED + 1))
    else
        echo "✗ FAILED: $contract has non-empty storage layout"
        echo ""
        echo "Storage layout:"
        echo "$STORAGE_OUTPUT"
        echo ""
        echo "UUPS upgradeable contracts MUST use ERC-7201 namespaced storage."
        echo "See SyndicateSequencingChain.sol for an example of correct implementation."
        echo ""
        FAILED=$((FAILED + 1))
    fi
    echo ""
done

echo "================================================"
echo "Results:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo "================================================"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Storage validation FAILED!"
    echo ""
    echo "All UUPS upgradeable contracts must use ERC-7201 namespaced storage."
    echo "This ensures storage slots don't conflict during upgrades."
    echo ""
    echo "To fix:"
    echo "1. Move all storage variables into a storage struct"
    echo "2. Use @custom:storage-location erc7201:syndicate.storage.YourContract"
    echo "3. Access storage via a private function using assembly"
    echo "4. Generate the storage slot with: cast index-erc7201 syndicate.storage.YourContract"
    echo ""
    echo "Example: see SyndicateSequencingChain.sol or GasCounter.sol"
    echo ""
    exit 1
else
    echo ""
    echo "All UUPS contracts use namespaced storage correctly!"
    echo ""
    exit 0
fi
