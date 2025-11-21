# Syndicate Appchain CLI

A TypeScript-based CLI tool for creating and managing Syndicate Appchains. It handles the configuration and deployment of required contracts to both the sequencing chain and the settlement chain.

## Installation

```bash
bun install
```

## Usage

The CLI is invoked using:

```bash
bun run synd-cli [command] [options]
```

### Available Commands

- `appchain` - Manage appchains
  - `create foundation` - Deploy foundational contracts
  - `create features` - Deploy additional features (token bridge, withdrawals, etc.)
  - `create sequencing` - Deploy sequencing-related contracts
  - `create withdrawals` - Deploy withdrawal-related contracts
  - `create assertion-poster` - Deploy AssertionPoster contract
  - `create tee-module` - Deploy TeeModule contract
  - `handoff` - Transfer ownership of contracts
  - `arb-owner` - Manage Arbitrum owner operations
  - `check-token-bridge` - Verify token bridge setup
  - `e2e` - Run end-to-end tests
- `alias` - Calculate aliased address for L1->L2 messages

## Using Config Files

All commands support loading configuration from a JSON file using the `--config` flag. This is more convenient than passing many CLI arguments.

### Example: Using a config file

```bash
# Create a config file
cp foundation.config.json.example my-chain.config.json
# Edit the config file with your values
# Run with config file
bun run synd-cli appchain create foundation --config my-chain.config.json
```

**Config file format:**

Config files use kebab-case keys (matching the CLI flag names):

```json
{
  "settlement-rpc": "https://...",
  "sequencing-rpc": "https://...",
  "ethereum-rpc": "https://...",
  "id": 123456,
  "name": "my-appchain",
  "deployer-private-key": "0x...",
  "owner-private-key": "0x..."
}
```

**Overriding config values:**

CLI flags take precedence over config file values:

```bash
# Use config file but override the chain name
bun run synd-cli appchain create foundation \
  --config my-chain.config.json \
  --name different-chain-name
```

**File path support:**

For complex JSON values like `--core-contracts` and `--synd`, you can provide a file path instead of a JSON string:

```bash
bun run synd-cli appchain create features \
  --config features.config.json \
  --core-contracts ./appchains/my-chain/core-contracts.json
```

See the `*.config.json.example` files for complete examples.

## Creating a New Appchain

### Step 1: Deploy Foundation Contracts

Deploy foundational dependencies onto the settlement & sequencing chains required by the Appchain node to run.

**What gets deployed:**

Settlement Chain:
- [Nitro Core Contracts](https://github.com/OffchainLabs/nitro-contracts)
- `ArbChainConfig`

Sequencing Chain:
- `RequireAndModule`
- `AllowlistSequencingModule`
- `SyndicateSequencingChain`

**Command (with config file):**

```bash
bun run synd-cli appchain create foundation --config foundation.config.json
```

**Command (with CLI flags):**

```bash
bun run synd-cli appchain create foundation \
  --settlement-rpc <SETTLEMENT_RPC_URL> \
  --sequencing-rpc <SEQUENCING_RPC_URL> \
  --ethereum-rpc <ETHEREUM_RPC_URL> \
  --appchain-rpc <APPCHAIN_RPC_URL> \
  --appchain-explorer <APPCHAIN_EXPLORER_URL> \
  --id <CHAIN_ID> \
  --name <CHAIN_NAME> \
  --deployer-private-key <DEPLOYER_PRIVATE_KEY> \
  --owner-private-key <OWNER_PRIVATE_KEY> \
  [--native-token <TOKEN_ADDRESS>] \
  [--core-contracts-created-at-hash <HASH>]
```

**Optional flags:**
- `--native-token` - Native token address (defaults to ETH if not provided)
- `--core-contracts-created-at-hash` - Skip deploying nitro core contracts if already deployed

**Output:** Contract addresses will be saved to `appchains/<chain_name>/*.json`

> [!NOTE]
> An EOA is created for the batch sequencer during this process. Save the interim-owner and sequencer private keys securely.

### Step 2: Deploy Features

Deploy additional contracts to the settlement chain & appchain that depend on the Appchain node.

> [!IMPORTANT]
> The Appchain RPC URL must be available for this step! DO NOT CONTINUE unless the appchain RPC is working!

**What gets deployed:**

Settlement Chain:
- [Nitro Token Bridge](https://github.com/OffchainLabs/token-bridge-contracts): Allows users to bridge non-native tokens
- `TeeKeyManager`: Required for withdrawals
- `AssertionPoster`: Required for withdrawals
- `TeeModule`: Required for withdrawals

Appchain:
- [`Multicall3`](https://github.com/mds1/multicall3/blob/main/src/Multicall3.sol): Utility contract for aggregating function calls

**Command (with config file):**

```bash
bun run synd-cli appchain create features --config features.config.json
```

**Command (with CLI flags):**

```bash
bun run synd-cli appchain create features \
  --settlement-rpc <SETTLEMENT_RPC_URL> \
  --sequencing-rpc <SEQUENCING_RPC_URL> \
  --synd-fork-sequencing-rpc <SYND_FORK_SEQUENCING_RPC_URL> \
  --ethereum-rpc <ETHEREUM_RPC_URL> \
  --appchain-rpc <APPCHAIN_RPC_URL> \
  --appchain-explorer <APPCHAIN_EXPLORER_URL> \
  --owner-private-key <OWNER_PRIVATE_KEY> \
  --deployer-private-key <DEPLOYER_PRIVATE_KEY> \
  --chain-name <CHAIN_NAME> \
  --sequencing-contract <SEQUENCING_CONTRACT_ADDRESS> \
  --core-contracts <CORE_CONTRACTS_JSON_OR_FILE_PATH>
```

**Output:** Contract addresses will be saved to `appchains/<chain_name>/*.json`

> [!NOTE]
> This process can take 5-10 minutes as it waits for retryable tickets between settlement and appchain to succeed. An EOA is created for the proposer during this process - save the private key securely.

## Additional Commands

### Transfer Ownership (Handoff)

Transfer contract ownership to a new address:

```bash
bun run synd-cli appchain handoff [options]
```

### Check Token Bridge

Verify token bridge setup and configuration:

```bash
bun run synd-cli appchain check-token-bridge [options]
```

### Calculate L1->L2 Alias

Calculate the aliased address for L1->L2 messages:

```bash
bun run synd-cli alias <address>
```

### E2E Testing

Run end-to-end tests using a config file:

```bash
bun run synd-cli appchain e2e --config e2e.config.json
```

See [e2e.config.json.example](./e2e.config.json.example) for the required configuration format.

## Development

### Format Code

```bash
bun run format
```

### Lint

```bash
bun run lint
```

### Type Check

```bash
bun run typecheck
```

### Generate Contract ABIs

Contract ABIs are generated from the `synd-contracts` directory:

```bash
make generate-contract-abis
```
