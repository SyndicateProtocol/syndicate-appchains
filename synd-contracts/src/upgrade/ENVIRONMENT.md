# 🔧 Environment Variables Reference

Complete reference for all environment variables used in deployment and upgrades.

## Required Variables

### Network Configuration

```bash
# RPC endpoint for the target network
RPC_URL=https://your-rpc-endpoint

# Your foundry account name (created with `cast wallet import`)
ACCOUNT=deployer

# Your deployer address
DEV_PUB_ADDRESS=0xYourDeployerAddress
```

### Admin Configuration

```bash
# Address that will own all contracts (receive DEFAULT_ADMIN_ROLE)
ADMIN_ADDRESS=0xYourAdminAddress
```

## Post-Deployment Variables

After deploying, add these to your `.env`:

```bash
# SyndicateFactory proxy address
FACTORY_ADDRESS=0xFactoryProxyAddress

# GasAggregator proxy address (get with: cast call $FACTORY_ADDRESS "gasAggregator()(address)")
GAS_AGGREGATOR_ADDRESS=0xGasAggregatorProxyAddress
```

## Chain-Specific Variables

### Creating Chains

```bash
# User-provided nonce for deterministic chain ID generation (1, 2, 3, ...)
# The factory will compute the actual chain ID from this nonce
NONCE=1

# Optional: Custom permission module address
# If not set, deploys new AlwaysAllowedModule
PERMISSION_MODULE=0xYourPermissionModuleAddress
```

### Upgrading Chains

```bash
# Address of the chain to upgrade
CHAIN_ADDRESS=0xSequencingChainProxyAddress
```

## Optional Variables

### ArbConfigManager Deployment

```bash
# Owner address for ArbConfigManager
OWNER_ADDRESS=0xOwnerAddress
```

---

## Example .env File

```bash
# Network
RPC_URL=https://sepolia.base.org
ACCOUNT=deployer
DEV_PUB_ADDRESS=0x1234567890123456789012345678901234567890

# Admin
ADMIN_ADDRESS=0xabcdefabcdefabcdefabcdefabcdefabcdefabcd

# Deployed Contracts (fill in after deployment)
FACTORY_ADDRESS=
GAS_AGGREGATOR_ADDRESS=

# For operations
NONCE=1
CHAIN_ADDRESS=

# Optional
PERMISSION_MODULE=
OWNER_ADDRESS=
```

---

## Getting Addresses

### After Factory Deployment

```bash
# Get GasAggregator address
cast call $FACTORY_ADDRESS "gasAggregator()(address)" --rpc-url $RPC_URL

# Get all chain IDs
cast call $FACTORY_ADDRESS "chainIDs()(uint256[])" --rpc-url $RPC_URL

# Get specific chain address
cast call $FACTORY_ADDRESS "appchainContracts(uint256)(address)" 1 --rpc-url $RPC_URL

# Get current implementation
cast call $FACTORY_ADDRESS "syndicateChainImpl()(address)" --rpc-url $RPC_URL
```

---

## Network-Specific Examples

### Base Sepolia

```bash
RPC_URL=https://sepolia.base.org
```

### Arbitrum Sepolia

```bash
RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
```

### Local Fork

```bash
# Start anvil: anvil --fork-url https://sepolia.base.org
RPC_URL=http://localhost:8545
```

---

## Security Notes

🔒 **Never commit `.env` to git!**

Add to `.gitignore`:
```
.env
.env.local
.env.*.local
```

🔐 **Use different accounts for different networks:**
- Testnet: Can use less secure key
- Mainnet: Use hardware wallet or secure keystore

🛡️ **For production, use multi-sig for ADMIN_ADDRESS**
