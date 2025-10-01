# Staking Infrastructure Off-Chain Actions

This document outlines all the off-chain actions that must be triggered for the staking infrastructure to function.

All the actions should be executed using `synd-stake-cli`

The process consists of two main parts:

1. **Mint Epoch Emissions** - Mint tokens for the epoch and get them onto the Commons chain
2. **Gas Aggregation** - Funnel all appchain gas usage data to the Commons chain

## 1. Mint Epoch Emissions

### Mint Command

Mints emissions to the staking system based on epoch rewards calculations.

```bash
synd-stake-cli mint \\
  --private-key <PRIVATE_KEY> \\
  --emissions-address <EMISSIONS_SCHEDULER_ADDRESS> \\
  --rpc-url <RPC_URL> \\
  [--sim]  # Optional: run in simulation mode
```

**Environment Variables:**

- `PRIVATE_KEY` - The private key of the account to mint the emissions
- `EMISSIONS_ADDRESS` - The address of the emissions scheduler contract
- `RPC_URL` - The RPC URL (defaults to "https://eth.drpc.org")

**What this does:**

- Calls `EmissionsScheduler.mintEmission()` on Ethereum mainnet
- `EmissionsScheduler` calls the `EmissionsCalculator` to mint the correct amount of tokens for the next epoch to the `L1Relayer` address
- `EmissionsScheduler` then kicks off the `L1Relayer` which sends two L1 to L2 transactions to Base:
  1. To send the funds to the `L2Relayer`
  2. To kick off the `L2Relayer` logic
- Once these transactions get picked up on Base, the `L2Relayer` sends a retryable transaction to the destination address (pool or splitter) on Commons Chain with the funds

### Refund Gas Command

Refunds excess gas from the bridging of emissions to Commons Chain.

```bash
synd-stake-cli refund-gas \\
  --private-key <PRIVATE_KEY> \\
  --refunder-address <REFUNDER_CONTRACT_ADDRESS> \\
  --rpc-url <RPC_URL> \\
  [--sim]  # Optional: run in simulation mode
```

**Environment Variables:**

- `PRIVATE_KEY` - The private key for signing transactions
- `REFUNDER_ADDRESS` - The address of the refunder contract
- `RPC_URL` - The RPC URL (defaults to "https://commons.rpc.syndicate.io")

**What this does:**

- Checks the balance of the Refunder contract on Commons Chain
- If excess gas exists (balance > 0), calls `Refunder.recover()`
- Returns the excess gas to the appropriate recipient
- If no excess gas exists, exits without submitting a transaction

## 2. Gas Aggregation

### Overview

The gas aggregation workflow consists of four main steps that must be executed in order:

1. **Aggregate gas data** on the sequencing chain
2. Wait for the Sequencing chain block to be settled to Ethereum (~ 7 days)
3. **Update block hashes** on the staking appchain
4. **Submit proofs** to confirm and finalize the data on the staking appchain `GasArchive`

### Step 1: Aggregate Gas Data

First, aggregate the gas usage data from all appchains on the sequencing chain.

```bash
synd-stake-cli gas-agg \\
  --private-key <PRIVATE_KEY> \\
  --gas-aggregator-address <GAS_AGGREGATOR_ADDRESS> \\
  --rpc-url <SEQUENCING_CHAIN_RPC_URL> \\
  [--sim]  # Optional: run in simulation mode
```

**Environment Variables:**

- `PRIVATE_KEY` - The private key for signing transactions
- `GAS_AGGREGATOR_ADDRESS` - The address of the gas aggregator contract
- `RPC_URL` - The sequencing chain RPC URL

**What this does:**

- Calls `GasAggregator.aggregateTokensUsed()` on the sequencing chain
- Aggregates tokens used by all appchains for the current epoch
- Sets `aggregatedEpochDataHash = keccak256(abi.encode(appchains, tokens, emissionsReceivers))` in storage
- Checks if the epoch is over before proceeding

### Step 2: Wait for Settlement

After the settlement period has elapsed (~7 days on mainnet & ~1h on testnet), we can rely on any Sequencing chain block produced after the gas aggregation transaction has been included, provided it has been confirmed through the chain's bridge to Ethereum.

**NOTE:** Look for the following event on the bridge `Outbox`:

```
event SendRootUpdated(bytes32 indexed outputRoot, bytes32 indexed l2BlockHash);
```

### Step 3: Update Block Hashes

After the settlement period, update the known Ethereum and Base block hashes on the staking appchain.

```bash
synd-stake-cli update-base-and-ethereum-block-hashes \\
  --base-rpc-url <BASE_RPC_URL> \\
  --private-key <PRIVATE_KEY> \\
  --relayer-address <BLOCK_HASH_RELAYER_ADDRESS> \\
  --gas-archive-address <GAS_ARCHIVE_ADDRESS> \\
  [--appchain-rpc-url <STAKING_APPCHAIN_RPC_URL>]  # Optional: wait for confirmation
```

**Environment Variables:**

- `BASE_RPC_URL` - Base chain RPC URL
- `PRIVATE_KEY` - Private key for signing transactions
- `STAKING_APPCHAIN_RPC_URL` - Staking appchain RPC URL (optional)

**What this does:**

- Calls `BlockHashRelayer.sendBlockHashes(gasArchiveAddress)` on Base
- Sends an L1→L2 message containing:
  - Ethereum block hash
  - Base block hash
- Updates `GasArchive` with the latest known block hashes
- If `--appchain-rpc-url` is provided, waits until the block hashes are confirmed on the staking appchain

### Step 4: Submit Gas Proofs

Finally, submit Merkle-Patricia proofs to validate the gas aggregation data.

```bash
synd-stake-cli submit-gas-proofs \\
  --seq-chain-rpc-url <SEQUENCING_CHAIN_RPC_URL> \\
  --ethereum-rpc-url <ETHEREUM_RPC_URL> \\
  --staking-appchain-rpc-url <STAKING_APPCHAIN_RPC_URL> \\
  --private-key <PRIVATE_KEY> \\
  --gas-archive-address <GAS_ARCHIVE_ADDRESS> \\
  [--epoch <EPOCH_NUMBER>]  # Optional: defaults to latest finalized epoch
```

**Environment Variables:**

- `SEQ_CHAIN_RPC_URL` - Sequencing chain RPC URL
- `ETHEREUM_RPC_URL` - Ethereum RPC URL
- `STAKING_APPCHAIN_RPC_URL` - Staking appchain RPC URL
- `PRIVATE_KEY` - Private key for signing transactions

**What this does:**

This command performs three sub-operations:

#### 1. Confirm Sequencing Chain Block Hash

- Retrieves the last known Ethereum block hash from `GasArchive`
- Searches for the `SendRootUpdated` event in the Outbox contract on Ethereum
- Calls `eth_getProof` to generate a Merkle-Patricia proof
- Calls `GasArchive.confirmEpochDataHash_0()` with:
  - RLP-encoded Ethereum block header
  - Account proof
  - Storage proof for the sequencing chain block hash

This establishes a trustless, cryptographically-verified sequencing chain block hash.

#### 2. Confirm Epoch Data Hash

- Calls `eth_getProof` on the sequencing chain at the confirmed block hash
- Generates a proof for the `aggregatedEpochDataHash` storage slot
- Calls `GasArchive.confirmEpochDataHash_0()` with:
  - RLP-encoded sequencing chain block header
  - Account proof
  - Storage proof for the epoch data hash

This verifies the gas aggregation data hash on the staking appchain.

#### 3. Submit Epoch Pre-Image Data

- Fetches the actual pre-image data (appchains, tokens, emissions receivers)
- Verifies `keccak256(abi.encode(appchains, tokens, emissionsReceivers)) === confirmedEpochDataHash`
- Calls `GasArchive.submitEpochPreImageData()` with the pre-image data

Once complete, the gas usage data is available for staking pools to calculate token emissions.

### Additional Command: Aggregate Gas Data Top N Chains

For cases where off-chain aggregation is enabled, you can submit gas data for the top N chains:

```bash
synd-stake-cli aggregate-gas-data-top-n-chains \\
  --seq-chain-rpc-url <SEQUENCING_CHAIN_RPC_URL> \\
  --private-key <PRIVATE_KEY> \\
  --gas-aggregator-address <GAS_AGGREGATOR_ADDRESS> \\
  [--epoch <EPOCH_NUMBER>]  # Optional: defaults to latest finalized epoch
```

**Environment Variables:**

- `SEQ_CHAIN_RPC_URL` - Sequencing chain RPC URL
- `PRIVATE_KEY` - Private key for signing transactions

**What this does:**

- Aggregates gas data from the top N chains based on gas usage
- Calls `GasAggregator.submitOffchainTopChains()` with the selected chains
- Used when off-chain aggregation is enabled and the number of chains exceeds the threshold

## Complete Example

Here's a complete workflow using environment variables:

```bash
# Set common environment variables
export SEQ_CHAIN_RPC_URL="https://syndicate-chain.example.com"
export ETHEREUM_RPC_URL="https://eth-mainnet.example.com"
export BASE_RPC_URL="https://base-mainnet.example.com"
export STAKING_APPCHAIN_RPC_URL="https://commons-chain.example.com"
export PRIVATE_KEY="0x..."

# Step 1: Mint emissions (on Ethereum)
synd-stake-cli mint \\
  --emissions-address 0x1234... \\
  --rpc-url https://eth-mainnet.example.com

# Step 2: Refund excess gas (on Commons Chain)
synd-stake-cli refund-gas \\
  --refunder-address 0x5678... \\
  --rpc-url https://commons-chain.example.com

# Step 3: Aggregate gas data (on sequencing chain)
synd-stake-cli gas-agg \\
  --gas-aggregator-address 0xabcd... \\
  --rpc-url https://syndicate-chain.example.com

# Wait for settlement period (1h testnet / 7 days mainnet)

# Step 4: Update block hashes (on staking appchain)
synd-stake-cli update-base-and-ethereum-block-hashes \\
  --base-rpc-url https://base-mainnet.example.com \\
  --relayer-address 0xefgh... \\
  --gas-archive-address 0xijkl... \\
  --appchain-rpc-url https://commons-chain.example.com

# Step 5: Submit gas proofs (validates everything on staking appchain)
synd-stake-cli submit-gas-proofs \\
  --gas-archive-address 0xijkl...
```

## Important Notes

- The `--epoch` parameter is optional for all commands. If not provided, it defaults to the latest finalized epoch (current epoch - 1)
- The private key must have sufficient balance on all relevant chains to pay for transaction gas
- Use the `--sim` flag to run commands in simulation mode without actually submitting transactions
- All commands support environment variables for configuration
- The settlement period is approximately 7 days on mainnet and 1 hour on testnet
- Commands will automatically check if conditions are met (e.g., epoch is over, excess gas exists) before proceeding

## Command Reference

| Command                                 | Purpose                  | Chain                |
| --------------------------------------- | ------------------------ | -------------------- |
| `mint`                                  | Mint epoch emissions     | Ethereum             |
| `refund-gas`                            | Refund excess gas        | Commons Chain        |
| `gas-agg`                               | Aggregate gas data       | Sequencing Chain     |
| `update-base-and-ethereum-block-hashes` | Update block hashes      | Base → Commons Chain |
| `submit-gas-proofs`                     | Submit Merkle proofs     | Commons Chain        |
| `aggregate-gas-data-top-n-chains`       | Submit top N chains data | Sequencing Chain     |
