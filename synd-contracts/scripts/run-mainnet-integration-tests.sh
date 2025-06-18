#!/bin/bash

# Mainnet Integration Test Runner
# This script runs the mainnet integration tests using forge with mainnet forking

set -e

echo "🔧 Running Mainnet Integration Tests..."
echo "📡 Using Ethereum Mainnet fork for realistic bridge testing"

# Check if MAINNET_RPC_URL is set, otherwise use default
if [ -z "$MAINNET_RPC_URL" ]; then
    echo "⚠️  MAINNET_RPC_URL not set, using default public RPC"
    export MAINNET_RPC_URL="https://eth.llamarpc.com"
fi

echo "🌐 RPC URL: $MAINNET_RPC_URL"

# Run the mainnet integration tests with forking
echo "🧪 Running MainnetIntegrationTest..."

forge test \
    --match-contract MainnetIntegrationTest \
    --fork-url $MAINNET_RPC_URL \
    --fork-block-number 19000000 \
    -vv \
    --gas-report

echo "✅ Mainnet integration tests completed!"

# Optional: Run specific test functions
echo ""
echo "🎯 Running specific integration scenarios..."

echo "🟦 Testing Arbitrum Bridge Integration..."
forge test \
    --match-test "test_Integration_ArbitrumBridge" \
    --fork-url $MAINNET_RPC_URL \
    --fork-block-number 19000000 \
    -vv

echo "🔴 Testing Optimism Bridge Integration..."
forge test \
    --match-test "test_Integration_OptimismBridge" \
    --fork-url $MAINNET_RPC_URL \
    --fork-block-number 19000000 \
    -vv

echo "⛽ Testing Gas Estimation..."
forge test \
    --match-test "test_Integration_GasEstimation" \
    --fork-url $MAINNET_RPC_URL \
    --fork-block-number 19000000 \
    -vv

echo "🚨 Testing Edge Cases..."
forge test \
    --match-test "test_Integration_MaximumEmissionAmount\|test_Integration_DailyLimitEnforcement" \
    --fork-url $MAINNET_RPC_URL \
    --fork-block-number 19000000 \
    -vv

echo ""
echo "🎉 All mainnet integration tests completed successfully!"
echo "📊 Check the gas reports above for bridge operation costs"