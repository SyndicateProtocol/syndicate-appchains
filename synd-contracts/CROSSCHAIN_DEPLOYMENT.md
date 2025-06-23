# Crosschain Token Deployment Guide

This guide explains how to deploy Syndicate crosschain tokens using the new factory-based deployment system with deterministic addressing.

## Overview

The crosschain token system includes:

1. **SyndicateTokenCrosschain** - Production crosschain token extending the main SyndicateToken
2. **TestnetSyndTokenCrosschain** - Testnet crosschain token extending TestnetSyndToken
3. **Factory contracts** - Enable deterministic deployment using Solmate CREATE3
4. **Deployment scripts** - Automate the deployment process across chains

## Key Benefits

- **Same contract address across ALL chains** (Mainnet, Arbitrum, Optimism, Base, etc.)
- **Seamless cross-chain bridging** without address confusion
- **Rate limiting per bridge** for security
- **ERC7802 compatibility** for SuperChain and modern bridges
- **Full backward compatibility** with existing token functionality

## Prerequisites

Ensure these environment variables are set in your `.env` file:

```bash
# Required for all deployments
ADMIN_ADDR=0x...           # Admin address (receives DEFAULT_ADMIN_ROLE and BRIDGE_MANAGER_ROLE)
DEV_PUB_ADDRESS=0x...      # Deployer public address

# For mainnet deployments
MANAGER_ADDR=0x...         # Treasury address (receives initial token mint)
ETH_HOLESKY_RPC_URL=...    # Mainnet RPC URL

# For testnet deployments
MINTER_ADDR=0x...          # Minter address (receives MINTER_ROLE)

# Optional
ETHERSCAN_API_KEY=...      # For contract verification
```

## Deployment Steps

### 1. Deploy Factories

First, deploy the factory contracts:

```bash
# Deploy mainnet factory (Holesky/Mainnet)
make deploy-syndicate-crosschain-factory

# Deploy testnet factory (Sepolia/testnets)
make deploy-testnet-crosschain-factory
```

### 2. Update Factory Addresses

After deployment, update the factory addresses in the deployment script:

Edit `script/DeployCrosschainTokenFactories.s.sol`:
- Update `FACTORY_ADDRESS` constants with deployed factory addresses

### 3. Deploy Tokens via Factories

Deploy the actual token contracts:

```bash
# Deploy mainnet crosschain token
make deploy-syndicate-crosschain-token

# Deploy testnet crosschain token
make deploy-testnet-crosschain-token
```

### 4. Configure Bridges (Optional)

Configure bridge addresses and limits:

```bash
# Update bridge addresses in the script first
make configure-crosschain-bridges
```

## Verification Commands

### Test Address Consistency

Verify that the same salt produces the same address:

```bash
make verify-crosschain-consistency
```

### Deploy Both Types for Testing

```bash
# Deploy both factories
make deploy-both-crosschain-factories

# Deploy both tokens
make deploy-both-crosschain-tokens
```

## Important Notes

### Address Prediction

The CREATE3 pattern ensures:
- Same salt = same address across ALL chains
- Independent of constructor arguments
- Independent of deployer nonce
- Enables true cross-chain address consistency

### Security Features

Each crosschain token includes:
- **Bridge rate limiting** - Daily mint/burn limits per bridge
- **Access controls** - BRIDGE_MANAGER_ROLE for bridge configuration
- **Emergency controls** - Bridge activation/deactivation
- **ERC165 interface detection** - For bridge compatibility

### Bridge Configuration

After deployment, configure authorized bridges:

```solidity
// Example bridge configuration
token.setBridgeLimits(
    bridgeAddress,
    1_000_000 * 10**18,  // 1M daily mint limit
    1_000_000 * 10**18   // 1M daily burn limit
);
```

## Chain-Specific Deployment

The same factory and salt will produce identical addresses on:

- **Mainnet**: Ethereum, Arbitrum, Optimism, Base, Polygon
- **Testnets**: Sepolia, Arbitrum Sepolia, Optimism Sepolia, Base Sepolia

## Example Deployment Workflow

1. **Deploy factories on all target chains**
2. **Generate universal salt** for consistent addressing
3. **Deploy tokens using same salt** on each chain
4. **Verify addresses match** across all chains
5. **Configure bridges** for cross-chain functionality
6. **Test cross-chain transfers** to ensure proper operation

## Troubleshooting

### Address Mismatch

If addresses don't match across chains:
- Verify same factory address is used
- Verify same salt is used
- Verify constructor arguments are identical

### Bridge Issues

If bridge operations fail:
- Check bridge authorization: `token.isBridgeAuthorized(bridge)`
- Check rate limits: `token.getAvailableMintLimit(bridge)`
- Verify bridge has sufficient allowance for burns

### Factory Not Found

If factory deployment fails:
- Check deployer has sufficient ETH for gas
- Verify network configuration in Makefile
- Check if factory already exists at predicted address