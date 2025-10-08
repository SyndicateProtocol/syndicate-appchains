# ⚡ Quick Start Guide

Get up and running in 5 minutes!

## Step 1: Setup (2 minutes)

```bash
# Copy environment template
cp .env.example .env

# Edit .env and fill in:
# - RPC_URL
# - ACCOUNT (your foundry account name)
# - DEV_PUB_ADDRESS
# - ADMIN_ADDRESS
```

## Step 2: Deploy (1 minute)

```bash
# Preview addresses (optional)
make preview-factory

# Deploy everything
make deploy-factory
```

**Save the Factory address to `.env`:**
```bash
FACTORY_ADDRESS=0x... # From output
```

## Step 3: Get GasAggregator Address (30 seconds)

```bash
# Get GasAggregator address
cast call $FACTORY_ADDRESS "gasAggregator()(address)" --rpc-url $RPC_URL

# Add to .env
echo "GAS_AGGREGATOR_ADDRESS=0x..." >> .env
```

## Step 4: Create Your First Chain (1 minute)

```bash
APPCHAIN_ID=1 make create-sequencing-chain
```

**Save the chain address to `.env`:**
```bash
CHAIN_ADDRESS=0x... # From output
```

## Done! 🎉

You now have:
- ✅ SyndicateFactory deployed
- ✅ GasAggregator deployed (automatic)
- ✅ Your first sequencing chain created

---

## What's Next?

### Create More Chains
```bash
APPCHAIN_ID=2 make create-sequencing-chain
APPCHAIN_ID=3 make create-sequencing-chain
```

### When You Need to Upgrade
```bash
# Always check storage first!
make storage-layout-check

# Then upgrade
make upgrade-factory
make upgrade-gas-aggregator
make upgrade-sequencing-chain
```

---

## Quick Reference

```bash
# Deployment
make preview-factory              # Preview addresses
make deploy-factory               # Deploy everything
make deploy-arb-config           # Deploy Arb config (optional)

# Create chains
make create-sequencing-chain     # Create new chain

# Upgrades
make storage-layout-check        # ⚠️ ALWAYS run first!
make upgrade-factory             # Upgrade factory
make upgrade-gas-aggregator      # Upgrade aggregator
make upgrade-sequencing-chain    # Upgrade chain
```

---

## Need Help?

📖 **Detailed guides:**
- [README.md](./README.md) - Complete guide with examples
- [ENVIRONMENT.md](./ENVIRONMENT.md) - Environment variables
- [UPGRADE_SAFETY.md](./UPGRADE_SAFETY.md) - Safety best practices

🐛 **Troubleshooting:** See [README.md](./README.md#-troubleshooting)

---

**Pro tip:** Test everything on a local fork first!
```bash
# Terminal 1
anvil --fork-url $RPC_URL

# Terminal 2
RPC_URL=http://localhost:8545 make deploy-factory
```
