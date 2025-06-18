# Mainnet Bridge Flow Testing

This directory contains scripts to test the complete emission-to-bridge flow on Ethereum mainnet using real Arbitrum and Optimism bridge contracts.

## Scripts Overview

### 🔗 `TestArbitrumBridgeFlow.s.sol`
Tests the complete flow from token emission to Arbitrum L2 bridging:
- Deploys SyndicateToken, EmissionScheduler, and ArbitrumBridgeProxy
- Configures the emission scheduler with Arbitrum bridge
- Attempts to execute emission and bridge flow
- Uses real Arbitrum L1GatewayRouter at `0x72Ce9c846789fdB6fC1f34aC4AD25Dd9ef7031ef`

### 🔗 `TestOptimismBridgeFlow.s.sol`  
Tests the complete flow from token emission to Optimism L2 bridging:
- Deploys SyndicateToken, EmissionScheduler, and OptimismBridgeProxy
- Configures the emission scheduler with Optimism bridge
- Attempts to execute emission and bridge flow
- Uses real Optimism L1StandardBridge at `0x99C9fc46f92E8a1c0deC1b1747d010903E884bE1`

## Configuration

### Environment Variables Required

```bash
# Required
PRIVATE_KEY=<your_private_key>
ETH_MAINNET_RPC_URL=<mainnet_rpc_url>
DEV_PUB_ADDRESS=<your_public_address>

# Optional (defaults to deployer address)
ADMIN_ADDR=<admin_address>
SYND_FOUNDATION_ADDR=<syndicate_foundation_address>
EMISSIONS_MANAGER_ADDR=<emissions_manager_address>
PAUSER_ADDR=<pauser_address>
L2_RECIPIENT_ADDR=<l2_recipient_address>
OP_L2_TOKEN_ADDR=<optimism_l2_token_address>
```

### Foundry Account Setup

Ensure you have a deployer account configured:
```bash
cast wallet import deployer --interactive
```

## Usage

### 🚀 Run Complete Tests (Deploy + Execute)

Test Arbitrum bridge flow:
```bash
make test-arbitrum-bridge-flow
```

Test Optimism bridge flow:
```bash
make test-optimism-bridge-flow
```

### 🧪 Simulation Only (No Deployment)

Simulate Arbitrum bridge flow:
```bash
make simulate-arbitrum-bridge-flow
```

Simulate Optimism bridge flow:
```bash
make simulate-optimism-bridge-flow
```

## Expected Results

### ✅ Successful Deployment
Both scripts will successfully:
1. Deploy all contracts (SyndicateToken, EmissionScheduler, BridgeProxy)
2. Configure proper roles and permissions
3. Set up bridge configuration
4. Start emission schedule

### ❌ Expected Bridge Failures
The actual bridge operations will likely fail with:

#### Arbitrum Bridge
- **Error**: `EvmError: Revert` during bridge call
- **Reason**: Token not registered with Arbitrum gateway system
- **Solution**: Deploy L2 token on Arbitrum and register with gateway

#### Optimism Bridge  
- **Error**: `EvmError: Revert` during bridge call
- **Reason**: L2 token doesn't implement `IOptimismMintableERC20` interface
- **Solution**: Deploy proper L2 token implementing required interface

## Gas and ETH Requirements

### Arbitrum Bridge
- **ETH Needed**: `maxGas * gasPriceBid = 1,000,000 * 10e9 = 0.01 ETH`
- **Script automatically funds bridge proxy with 2x buffer**

### Optimism Bridge
- **ETH Needed**: Minimal (no L2 gas payment required for L1 call)
- **Bridge proxy doesn't need ETH funding**

## Contract Addresses

The scripts will output deployed contract addresses:

```
=== DEPLOYMENT SUMMARY ===
SyndicateToken: 0x...
EmissionScheduler: 0x...
ArbitrumBridgeProxy: 0x...  (or OptimismBridgeProxy)
```

## Timing Considerations

### Emission Schedule
- **First epoch**: Available 30 days after `startEmissions()` call
- **If testing immediately**: Scripts will show "EMISSIONS NOT YET READY"
- **To test emission**: Run script 30+ days after deployment

### Testing Approach
1. **Deploy contracts**: Use the scripts to deploy and configure
2. **Wait for epoch**: Wait 30 days or modify timestamp in tests
3. **Attempt emission**: Run the emission call (expected to fail at bridge)
4. **Deploy L2 tokens**: Deploy proper L2 token contracts
5. **Reconfigure bridges**: Update bridge configuration with real L2 addresses
6. **Retry emission**: Should succeed with proper L2 token setup

## Integration with Tests

These scripts mirror the logic from:
- `test/token/MainnetIntegrationTest.t.sol::test_Integration_ArbitrumBridge_EndToEnd`
- `test/token/MainnetIntegrationTest.t.sol::test_Integration_OptimismBridge_EndToEnd`

The key difference is real deployment vs. test environment.

## Security Notes

⚠️ **These scripts deploy to mainnet** - ensure:
1. Sufficient ETH for deployment costs
2. Correct environment variable configuration  
3. Private key security
4. Understanding that this creates real, funded contracts

## Troubleshooting

### Common Issues

1. **Insufficient ETH**: Ensure deployer has ETH for gas costs
2. **RPC Rate Limits**: Use reliable RPC endpoint
3. **Account Setup**: Verify `cast wallet` configuration
4. **Environment Variables**: Double-check all required variables

### Expected Behavior
✅ Contract deployment and configuration should succeed
✅ Bridge flow setup should complete
❌ Actual bridging will fail (expected without L2 token setup)

This is the expected behavior and proves the integration architecture works correctly!