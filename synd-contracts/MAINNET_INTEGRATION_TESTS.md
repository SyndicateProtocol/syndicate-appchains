# Mainnet Integration Tests

## Overview

This directory contains comprehensive integration tests that verify the end-to-end flow from token minting to L2 bridging using **real Ethereum mainnet state**. These tests fork mainnet and interact with actual Arbitrum and Optimism bridge contracts to ensure the complete emission and bridging pipeline works correctly.

## Test Coverage

### 🎯 Core Integration Tests

- **End-to-End Arbitrum Bridging**: Complete flow from emission minting through Arbitrum L1GatewayRouter
- **End-to-End Optimism Bridging**: Complete flow from emission minting through Optimism L1StandardBridge  
- **Multi-Epoch Processing**: Verifies multiple emission epochs can be processed and bridged
- **Gas Estimation**: Measures actual gas costs for bridge operations on mainnet

### 🚨 Edge Cases & Failure Scenarios

- **Bridge Failure Handling**: What happens when bridge contracts fail
- **Daily Limit Enforcement**: Verifies rate limiting works with real bridge amounts
- **Maximum Emission Processing**: Tests processing all 48 epochs at once
- **Configuration Validation**: Ensures bridge configurations are correct

## Bridge Contracts Used

### Arbitrum
- **L1GatewayRouter**: `0x72Ce9c846789fdB6fC1f34aC4AD25Dd9ef7031ef`
- **Integration**: Tests `outboundTransferCustomRefund()` with ETH gas payments

### Optimism  
- **L1StandardBridge**: `0x99C9fc46f92E8a1c0deC1b1747d010903E884bE1`
- **Integration**: Tests `depositERC20To()` with L2 gas configuration

## Running the Tests

### Prerequisites

```bash
# Ensure you have forge installed
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### Quick Start

```bash
# Run all mainnet integration tests
./scripts/run-mainnet-integration-tests.sh
```

### Manual Execution

```bash
# Set RPC URL (optional - defaults to public RPC)
export MAINNET_RPC_URL="https://your-mainnet-rpc-url"

# Run all integration tests
forge test --match-contract MainnetIntegrationTest --fork-url $MAINNET_RPC_URL -vv

# Run specific test categories
forge test --match-test "test_Integration_ArbitrumBridge" --fork-url $MAINNET_RPC_URL -vv
forge test --match-test "test_Integration_OptimismBridge" --fork-url $MAINNET_RPC_URL -vv
forge test --match-test "test_Integration_GasEstimation" --fork-url $MAINNET_RPC_URL -vv
```

### With Custom RPC

```bash
# Using Alchemy/Infura (recommended for stability)
export MAINNET_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY"
forge test --match-contract MainnetIntegrationTest --fork-url $MAINNET_RPC_URL -vv

# Using built-in RPC endpoint
forge test --match-contract MainnetIntegrationTest --rpc-url mainnet -vv
```

## Test Structure

### MainnetIntegrationTest.t.sol

```solidity
contract MainnetIntegrationTest is Test {
    // Real mainnet bridge addresses
    address constant ARBITRUM_L1_GATEWAY_ROUTER = 0x72Ce9c846789fdB6fC1f34aC4AD25Dd9ef7031ef;
    address constant OPTIMISM_L1_STANDARD_BRIDGE = 0x99C9fc46f92E8a1c0deC1b1747d010903E884bE1;
    
    // Test scenarios:
    // ✅ End-to-end minting → bridging flow
    // ✅ Multi-epoch emission processing  
    // ✅ Bridge failure scenarios
    // ✅ Gas cost analysis
    // ✅ Rate limiting validation
}
```

## What These Tests Verify

### ✅ **Successful Cases**
1. **Token Minting**: EmissionScheduler mints correct amounts per epoch
2. **Bridge Approval**: Tokens are properly approved for bridge contracts
3. **Bridge Execution**: Real bridge contracts accept and process tokens
4. **State Updates**: Epoch counters and emission tracking work correctly
5. **Gas Costs**: Bridge operations complete within reasonable gas limits

### ❌ **Failure Cases**  
1. **Invalid Bridge Targets**: Emissions fail when bridges are misconfigured
2. **Insufficient ETH**: Arbitrum bridging fails without ETH for L2 gas
3. **Rate Limits**: Daily limits prevent oversized transfers
4. **Unauthorized Access**: Only authorized roles can execute emissions

## Expected Outcomes

### Successful Test Results
- ✅ Tokens minted to EmissionScheduler
- ✅ Tokens transferred to bridge contracts  
- ✅ Bridge contracts called with correct parameters
- ✅ EmissionScheduler balance becomes zero after bridging
- ✅ Epoch counters increment correctly
- ✅ Gas usage under 500k per emission+bridge operation

### Bridge Integration Verification
- ✅ **Arbitrum**: Calls `outboundTransferCustomRefund()` with ETH value
- ✅ **Optimism**: Calls `depositERC20To()` with L2 token mapping
- ✅ **Token Flow**: Tokens leave EmissionScheduler and enter bridge contracts
- ✅ **Error Handling**: Failed bridges revert entire emission transaction

## Debugging

### Common Issues

**Fork Block Too Old**: Use recent block numbers
```bash
forge test --fork-block-number 19000000 --fork-url $MAINNET_RPC_URL
```

**RPC Rate Limits**: Use private RPC endpoints for better reliability
```bash
export MAINNET_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY"
```

**Gas Estimation Failures**: Increase gas limits in test configuration
```bash
forge test --gas-limit 10000000 --fork-url $MAINNET_RPC_URL
```

### Verbose Output

```bash
# Maximum verbosity for debugging
forge test --match-contract MainnetIntegrationTest -vvvv --fork-url $MAINNET_RPC_URL

# Gas reporting
forge test --match-contract MainnetIntegrationTest --gas-report --fork-url $MAINNET_RPC_URL
```

## Security Considerations

### ⚠️ **Limitations**
- Tests verify tokens reach bridge contracts but **cannot verify L2 delivery**
- Mainnet state may change between test runs affecting bridge behavior  
- Real bridge contracts may have different behavior than test mocks
- Gas costs measured on mainnet fork may differ from actual mainnet execution

### 🔒 **Safety**
- Tests use forked mainnet state - **no real transactions are submitted**
- All test addresses are isolated and don't affect real balances
- Bridge operations are simulated without actual L2 token minting

## Integration with CI/CD

```yaml
# Example GitHub Actions workflow
- name: Run Mainnet Integration Tests
  run: |
    export MAINNET_RPC_URL=${{ secrets.MAINNET_RPC_URL }}
    ./scripts/run-mainnet-integration-tests.sh
  env:
    MAINNET_RPC_URL: ${{ secrets.MAINNET_RPC_URL }}
```

## Maintenance

### Updating Bridge Addresses
If bridge contracts are upgraded, update the constants in `MainnetIntegrationTest.t.sol`:

```solidity
address constant ARBITRUM_L1_GATEWAY_ROUTER = 0x...;  // New address
address constant OPTIMISM_L1_STANDARD_BRIDGE = 0x...; // New address
```

### Fork Block Updates
Update fork block numbers periodically to ensure recent mainnet state:

```bash
# Get latest block
cast block-number --rpc-url https://eth.llamarpc.com

# Update in script
--fork-block-number LATEST_BLOCK_NUMBER
```