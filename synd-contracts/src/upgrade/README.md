# 🚀 Deployment & Upgrade Guide

Quick and easy guide for deploying and upgrading Syndicate Appchains contracts.

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Deployment](#-deployment)
- [Creating Chains](#-creating-chains)
- [Upgrades](#-upgrades)
- [Verification](#-verification)
- [Troubleshooting](#-troubleshooting)

---

## ⚡ Quick Start

### First Time Setup

```bash
# 1. Copy environment template
cp .env.example .env

# 2. Edit .env with your values
# RPC_URL, ACCOUNT, DEV_PUB_ADDRESS, ADMIN_ADDRESS

# 3. Preview deployment addresses
make preview-factory

# 4. Deploy (this also deploys GasAggregator!)
make deploy-factory
```

### After Deployment

```bash
# Create your first sequencing chain
NONCE=1 make create-sequencing-chain

# When you need to upgrade
make storage-layout-check
make upgrade-factory
```

---

## 🌍 Deployment

### What Gets Deployed?

Running `make deploy-factory` deploys:

```
✅ SyndicateFactory (proxy + implementation)
  ├── ✅ GasAggregator (non-upgradeable contract) - AUTOMATIC!
  ├── ✅ SyndicateSequencingChain implementation - AUTOMATIC!
  └── ✅ MinimalUUPSStub - AUTOMATIC!
```

You only manually deploy:
- **SyndicateFactory** (everything else is automatic)
- **ArbConfigManager** (optional, only if using Arbitrum)

**Note:** GasAggregator is no longer upgradeable (changed from UUPS proxy pattern to simple contract deployment)

### Step 1: Configure Environment

Create `.env` file:

```bash
# Network
RPC_URL=https://your-rpc-url
ACCOUNT=deployer                    # Your foundry account name
DEV_PUB_ADDRESS=0xYourAddress

# Admin (will own all contracts)
ADMIN_ADDRESS=0xAdminAddress
```

### Step 2: Preview Addresses

```bash
make preview-factory
```

Shows you what addresses will be used (same on all chains due to CREATE2).

### Step 3: Deploy Factory

```bash
make deploy-factory
```

**Output:**
```
Implementation deployed to: 0x1234...
SyndicateFactory proxy deployed to: 0x5678...
✅ Deterministic deployment successful!
```

### Step 4: Save Addresses

Add to your `.env`:

```bash
FACTORY_ADDRESS=0x5678...  # From output above
GAS_AGGREGATOR_ADDRESS=0xabcd...  # From deployment output
```

### Optional: Deploy ArbConfigManager

Only if using Arbitrum as settlement chain:

```bash
OWNER_ADDRESS=0xYourOwner make deploy-arb-config
```

---

## 🔗 Creating Chains

### Create First Chain

```bash
# Set nonce and create (factory will compute deterministic chain ID)
NONCE=1 make create-sequencing-chain
```

**What you'll see:**
```
=== Creating Sequencing Chain ===
Factory: 0x5678...
Nonce: 1
Admin: 0xAdmin...

No PERMISSION_MODULE set, deploying AlwaysAllowedModule...
AlwaysAllowedModule deployed: 0xabcd...

=== Sequencing Chain Created ===
Chain Address: 0x9abc...
Chain ID: 1

Save this address to your .env file:
CHAIN_ADDRESS=0x9abc...
```

### Create More Chains

```bash
# Create multiple chains with different nonces
NONCE=2 make create-sequencing-chain
NONCE=3 make create-sequencing-chain
```

### Using Custom Permission Module

```bash
# Use your own permission module
NONCE=4 PERMISSION_MODULE=0xYourModule make create-sequencing-chain
```

---

## 🔄 Upgrades

### ⚠️ Before Every Upgrade

**ALWAYS run storage layout validation:**

```bash
make storage-layout-check
```

**Expected output:**
```
✅ SyndicateFactory: Storage layout unchanged
✅ SyndicateSequencingChain: Storage layout unchanged
✅ All storage layouts are safe for upgrades
```

**Note:** GasAggregator is no longer checked as it's not upgradeable.

If you see `❌ STORAGE LAYOUT CHANGED`, **STOP and review changes!**

### Upgrade SyndicateFactory

```bash
make upgrade-factory
```

**What happens:**
1. Deploys new implementation
2. Upgrades proxy to new implementation
3. Verifies upgrade succeeded

**Output:**
```
=== Upgrading SyndicateFactory ===
Deploying new implementation...
New implementation: 0xdef0...

Upgrading proxy to new implementation...

=== Upgrade Complete ===
Proxy: 0x5678...
Implementation: 0xdef0...
Version: 1.1.0
```

### ~~Upgrade GasAggregator~~ (No Longer Supported)

**GasAggregator is non-upgradeable.** If you need to upgrade it, you must:
1. Deploy a new GasAggregator contract
2. Update all references to point to the new contract
3. Migrate tracked chains to the new instance

This is intentional to keep the gas aggregation logic simple and immutable.

### Upgrade Sequencing Chain

```bash
# Set which chain to upgrade
CHAIN_ADDRESS=0x9abc... make upgrade-sequencing-chain
```

**What happens:**
1. Deploys new implementation
2. Sets it as default in factory (for new chains)
3. Upgrades the specific chain

**Output:**
```
=== Upgrading SyndicateSequencingChain ===
Deploying new implementation...
Setting as default implementation in factory...

Upgrading chain proxy...

=== Upgrade Complete ===
Chain proxy: 0x9abc...
Implementation: 0x5678...
Version: 1.1.0

New chains created via factory will use this implementation
```

### Upgrade All Chains

```bash
# Get all chain IDs
cast call $FACTORY_ADDRESS "chainIDs()(uint256[])" --rpc-url $RPC_URL

# Upgrade each chain
for id in 1 2 3; do
  CHAIN=$(cast call $FACTORY_ADDRESS "appchainContracts(uint256)(address)" $id --rpc-url $RPC_URL)
  CHAIN_ADDRESS=$CHAIN make upgrade-sequencing-chain
done
```

---

## ✅ Verification

### Check Contract Versions

```bash
# Factory version
cast call $FACTORY_ADDRESS "version()(string)" --rpc-url $RPC_URL

# GasAggregator version (constant)
cast call $GAS_AGGREGATOR_ADDRESS "VERSION()(uint256)" --rpc-url $RPC_URL

# Chain version
cast call $CHAIN_ADDRESS "version()(string)" --rpc-url $RPC_URL
```

### Check Admin Roles

```bash
# Check if address has admin role
cast call $FACTORY_ADDRESS \
  "hasRole(bytes32,address)(bool)" \
  $(cast keccak "DEFAULT_ADMIN_ROLE()") \
  $ADMIN_ADDRESS \
  --rpc-url $RPC_URL
```

### Get Contract Addresses

```bash
# Current sequencing chain implementation
cast call $FACTORY_ADDRESS "syndicateChainImpl()(address)" --rpc-url $RPC_URL

# Check if chain ID is used
cast call $FACTORY_ADDRESS "isChainIdUsed(uint256)(bool)" 1 --rpc-url $RPC_URL

# Compute sequencing chain address
cast call $FACTORY_ADDRESS "computeSequencingChainAddress(uint256)(address)" 1 --rpc-url $RPC_URL
```

---

## 🐛 Troubleshooting

### "Storage layout changed" Error

**Cause:** Storage layout has changed

**Fix:**
1. Review what changed
2. If safe (adding at end), update baseline:
   ```bash
   forge inspect SyndicateFactory storageLayout > .storage-layouts/SyndicateFactory.json
   ```
3. If unsafe, revert code changes

### "Unauthorized" Error

**Cause:** You don't have required role

**Fix:** Use the admin account that deployed the contracts

### "Contract already deployed"

**Cause:** Using deterministic deployment, address already exists

**Fix:** This is normal! The script will use the existing deployment

### Upgrade Fails Silently

**Cause:** Need more verbose output

**Fix:** Add `-vvvv` flag:
```bash
forge script script/upgrade/UpgradeContracts.s.sol:UpgradeSyndicateFactory \
  --rpc-url $RPC_URL \
  --broadcast \
  -vvvv
```

---

## 🧪 Testing Locally

Test everything on a local fork before mainnet:

```bash
# Terminal 1: Start local fork
anvil --fork-url $RPC_URL

# Terminal 2: Deploy and test
export RPC_URL=http://localhost:8545

make deploy-factory
APPCHAIN_ID=1 make create-sequencing-chain
make upgrade-factory
```

---

## 📚 Available Commands

```bash
# Deployment
make preview-factory              # Preview deployment addresses
make deploy-factory               # Deploy SyndicateFactory
make deploy-arb-config           # Deploy ArbConfigManager

# Create Chains
make create-sequencing-chain     # Create new sequencing chain

# Upgrades
make upgrade-factory             # Upgrade SyndicateFactory
make upgrade-sequencing-chain    # Upgrade sequencing chain

# Validation
make storage-layout-check        # Validate storage layouts
```

---

## 🔐 Security Best Practices

✅ Always run `make storage-layout-check` before upgrades
✅ Test on local fork with Anvil first
✅ Deploy to testnet before mainnet
✅ Use multi-sig for admin operations on mainnet
✅ Monitor contracts after upgrades
✅ Have rollback plan ready

---

## 📖 Additional Resources

- [Quick Start Guide](./QUICK_START.md) - Get started in 5 minutes
- [Environment Variables](./ENVIRONMENT.md) - All configuration options
- [Upgrade Safety](./UPGRADE_SAFETY.md) - Storage layout and safety checks
- [Upgrade Checklist](./UPGRADE_CHECKLIST.md) - Pre-upgrade safety checklist
- [Risa Testnet Deployment](./RISA_DEPLOYMENT.md) - Risa-specific deployment guide

---

Need help? Check the detailed guides in `docs/upgrade/` folder!
