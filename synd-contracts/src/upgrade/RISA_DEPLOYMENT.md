# 🚀 Risa Testnet Deployment Guide

Step-by-step guide to deploy Syndicate Appchains contracts to Risa testnet.

## Prerequisites

- [ ] Foundry installed and updated (`foundryup`)
- [ ] Access to Risa testnet RPC
- [ ] Deployer wallet with testnet funds
- [ ] Admin wallet address decided

---

## Step 1: Environment Setup (5 minutes)

### 1.1 Create/Update .env File

```bash
# Copy template if you don't have .env
cp .env.example .env

# Edit .env
nano .env  # or use your preferred editor
```

### 1.2 Configure .env for Risa

```bash
# Network Configuration
RPC_URL=risa_devnet                    # Use the foundry.toml alias
RISA_DEVNET_RPC_URL=https://your-risa-rpc-url  # Your actual Risa RPC endpoint

# Account Configuration
ACCOUNT=deployer                       # Your foundry account name
DEV_PUB_ADDRESS=0xYourDeployerAddress  # Your deployer public address

# Admin Configuration
ADMIN_ADDRESS=0xYourAdminAddress       # Who will own the contracts

# These will be filled in after deployment
FACTORY_ADDRESS=
GAS_AGGREGATOR_ADDRESS=
```

### 1.3 Set Up Deployer Account

If you haven't created a foundry account yet:

```bash
# Import your private key (interactive)
cast wallet import deployer --interactive

# Verify it's imported
cast wallet list

# Check your balance on Risa
cast balance $DEV_PUB_ADDRESS --rpc-url risa_devnet
```

**⚠️ Make sure you have enough testnet tokens for deployment!**

---

## Step 2: Preview Deployment (1 minute)

Preview what addresses will be deployed (deterministic):

```bash
make preview-factory
```

**Output:**
```
=== Address Preview ===
Implementation will deploy to: 0x1234...
Proxy will deploy to: 0x5678...
=====================
```

**✅ These addresses will be the SAME on all chains!**

---

## Step 3: Deploy SyndicateFactory (2 minutes)

Deploy the factory (this also auto-deploys GasAggregator):

```bash
make deploy-factory
```

**What happens:**
1. ✅ Deploys SyndicateFactory implementation
2. ✅ Deploys SyndicateFactory proxy
3. ✅ Initializes factory with your admin
4. ✅ **Auto-deploys GasAggregator** (proxy + implementation)
5. ✅ **Auto-deploys SyndicateSequencingChain** implementation
6. ✅ **Auto-deploys MinimalUUPSStub**

**Expected Output:**
```
Implementation deployed to: 0x1234567890...
SyndicateFactory proxy deployed to: 0xabcdef1234...
Deterministic deployment successful!
SyndicateFactory address (consistent across all chains): 0xabcdef1234...
```

**⚠️ Save the Factory proxy address!**

---

## Step 4: Save Deployed Addresses (1 minute)

### 4.1 Add Factory Address to .env

```bash
# Add to .env file
echo "FACTORY_ADDRESS=0xYourFactoryProxyAddress" >> .env
```

### 4.2 Get GasAggregator Address

```bash
# Query factory for GasAggregator address
cast call $FACTORY_ADDRESS "gasAggregator()(address)" --rpc-url risa_devnet
```

**Output:** `0x...` (the GasAggregator proxy address)

```bash
# Add to .env file
echo "GAS_AGGREGATOR_ADDRESS=0xYourGasAggregatorAddress" >> .env
```

### 4.3 Verify Your .env

Your `.env` should now have:
```bash
RPC_URL=risa_devnet
RISA_DEVNET_RPC_URL=https://...
ACCOUNT=deployer
DEV_PUB_ADDRESS=0x...
ADMIN_ADDRESS=0x...
FACTORY_ADDRESS=0x...          # ✅ Added
GAS_AGGREGATOR_ADDRESS=0x...   # ✅ Added
```

---

## Step 5: Verify Deployment (2 minutes)

### 5.1 Check Factory Version

```bash
cast call $FACTORY_ADDRESS "version()(string)" --rpc-url risa_devnet
```

**Expected:** `1.0.0` (or current version)

### 5.2 Check GasAggregator Version

```bash
cast call $GAS_AGGREGATOR_ADDRESS "version()(string)" --rpc-url risa_devnet
```

**Expected:** `1.0.0` (or current version)

### 5.3 Check Admin Role

```bash
# Get DEFAULT_ADMIN_ROLE hash
DEFAULT_ADMIN_ROLE=$(cast keccak "DEFAULT_ADMIN_ROLE()")

# Check if your admin has the role
cast call $FACTORY_ADDRESS \
  "hasRole(bytes32,address)(bool)" \
  $DEFAULT_ADMIN_ROLE \
  $ADMIN_ADDRESS \
  --rpc-url risa_devnet
```

**Expected:** `true`

### 5.4 Check Factory State

```bash
# Get current implementation
cast call $FACTORY_ADDRESS "syndicateChainImpl()(address)" --rpc-url risa_devnet

# Check factory is not paused
cast call $FACTORY_ADDRESS "paused()(bool)" --rpc-url risa_devnet
```

**Expected:** Address and `false`

---

## Step 6: Create Your First Sequencing Chain (3 minutes)

### 6.1 Create Chain

```bash
# Create chain with nonce 1
NONCE=1 make create-sequencing-chain
```

**What happens:**
1. Checks if PERMISSION_MODULE is set
2. If not, deploys new AlwaysAllowedModule
3. Calls factory.createSyndicateSequencingChain(nonce, admin, module)
4. Returns chain address and computed chain ID

**Expected Output:**
```
=== Creating Sequencing Chain ===
Factory: 0xabcdef...
Nonce: 1
Admin: 0xAdmin...

No PERMISSION_MODULE set, deploying AlwaysAllowedModule...
AlwaysAllowedModule deployed: 0x1234...

=== Sequencing Chain Created ===
Chain Address: 0x9abc...
Chain ID: 123456789

Save this address to your .env file:
CHAIN_ADDRESS=0x9abc...
```

### 6.2 Save Chain Address

```bash
# Add to .env
echo "CHAIN_ADDRESS=0xYourChainAddress" >> .env
```

### 6.3 Verify Chain

```bash
# Check chain version
cast call $CHAIN_ADDRESS "version()(string)" --rpc-url risa_devnet

# Check chain appchainId
cast call $CHAIN_ADDRESS "appchainId()(uint256)" --rpc-url risa_devnet

# Check chain owner
cast call $CHAIN_ADDRESS "owner()(address)" --rpc-url risa_devnet
```

---

## Step 7: Create More Chains (Optional)

```bash
# Create additional chains with different nonces
NONCE=2 make create-sequencing-chain
NONCE=3 make create-sequencing-chain
NONCE=4 make create-sequencing-chain
```

**Note:** Each nonce produces a different chain ID deterministically!

---

## Step 8: Document Your Deployment

Create a deployment record:

```bash
cat > deployment-risa.txt << EOF
=== Risa Testnet Deployment ===
Date: $(date)
Network: Risa Devnet
Deployer: $DEV_PUB_ADDRESS
Admin: $ADMIN_ADDRESS

Deployed Contracts:
- SyndicateFactory: $FACTORY_ADDRESS
- GasAggregator: $GAS_AGGREGATOR_ADDRESS
- Chain #1: $CHAIN_ADDRESS

Block Explorer:
- Factory: https://risa-explorer.io/address/$FACTORY_ADDRESS
- GasAggregator: https://risa-explorer.io/address/$GAS_AGGREGATOR_ADDRESS
- Chain #1: https://risa-explorer.io/address/$CHAIN_ADDRESS
EOF

cat deployment-risa.txt
```

---

## Verification Checklist

After deployment, verify:

- [ ] Factory deployed and initialized
- [ ] GasAggregator deployed and linked to factory
- [ ] Admin role assigned correctly
- [ ] Factory not paused
- [ ] Can create sequencing chains
- [ ] Chains have correct admin
- [ ] All addresses saved to `.env`
- [ ] Deployment documented

---

## Quick Reference - All Commands

```bash
# 1. Setup
cp .env.example .env
# Edit .env with your values

# 2. Import account (if needed)
cast wallet import deployer --interactive

# 3. Check balance
cast balance $DEV_PUB_ADDRESS --rpc-url risa_devnet

# 4. Preview
make preview-factory

# 5. Deploy
make deploy-factory

# 6. Get addresses
cast call $FACTORY_ADDRESS "gasAggregator()(address)" --rpc-url risa_devnet

# 7. Create chain
NONCE=1 make create-sequencing-chain

# 8. Verify
cast call $FACTORY_ADDRESS "version()(string)" --rpc-url risa_devnet
```

---

## Troubleshooting

### "Insufficient funds for gas"
**Solution:** Get more testnet tokens for your deployer address

### "Deterministic deployment proxy not found"
**Solution:** Deploy the proxy first:
```bash
# Check if it exists
cast code 0x4e59b44847b379578588920cA78FbF26c0B4956C --rpc-url risa_devnet

# If empty, you need to deploy it (contact team for instructions)
```

### "Transaction reverted"
**Solution:** Check with verbose output:
```bash
make deploy-factory -vvvv
```

### "Contract already deployed"
**Solution:** This is OK! Deterministic deployment found existing contract

### "RPC_URL not set"
**Solution:** Make sure `.env` has `RPC_URL=risa_devnet` and `RISA_DEVNET_RPC_URL=https://...`

---

## Next Steps

After successful deployment:

1. **Share addresses** with your team
2. **Add chains to GasAggregator** tracking (if needed)
3. **Test transaction processing** on your chains
4. **Set up monitoring** for your contracts
5. **Document any custom configurations**

---

## Need Help?

- 📖 [Main Guide](./README.md) - Detailed documentation
- 🔧 [Environment Variables](./ENVIRONMENT.md) - All config options
- 🛡️ [Upgrade Safety](./UPGRADE_SAFETY.md) - When you need to upgrade

---

**🎉 Congratulations! You've deployed to Risa testnet!**
