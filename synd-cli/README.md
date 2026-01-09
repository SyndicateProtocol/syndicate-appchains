# Syndicate Appchain CLI

A TypeScript-based CLI tool for creating and managing Syndicate Appchains. It handles the configuration and deployment of required contracts to both the sequencing chain and the settlement chain.

## Installation

### Pre-built Binaries (Recommended)

Install the latest version:

```bash
curl -L https://raw.githubusercontent.com/SyndicateProtocol/syndicate-appchains/main/synd-cli/install.sh | bash
```

Install a specific version:

```bash
curl -L https://raw.githubusercontent.com/SyndicateProtocol/syndicate-appchains/main/synd-cli/install.sh | SYND_VERSION=synd-cli-v1.0.0 bash
```

Custom install directory:

```bash
curl -L https://raw.githubusercontent.com/SyndicateProtocol/syndicate-appchains/main/synd-cli/install.sh | SYND_INSTALL_DIR=/usr/local/bin bash
```

After installation, add `~/.synd/bin` to your PATH (the installer will provide instructions).

### From Source

```bash
bun install
```

## Quick Start

> **Note:** If running from source, replace `synd-cli` with `bun run synd-cli` in all commands below.

1. **Generate example config files:**
   ```bash
   synd-cli appchain create foundation init
   synd-cli appchain create features init
   ```

2. **Edit the generated config files** in `options/` with your values

3. **Deploy contracts:**
   ```bash
   synd-cli appchain create foundation --config options/foundation.json
   synd-cli appchain create features --config options/features.json
   ```

4. **Save the private keys** displayed during deployment (sequencer, proposer, etc.)

## Usage

The CLI supports two ways to provide configuration:

### Config Files (Recommended)

Most commands support an `init` subcommand that generates an example config file with only the required fields:

```bash
# Generate example config
synd-cli appchain create foundation init

# Use the config file
synd-cli appchain create foundation --config options/foundation.json
```

Config files use kebab-case keys matching CLI flag names. CLI flags can override config file values.

### CLI Flags

All options can also be passed as CLI flags. Run any command with `--help` to see available options:

```bash
synd-cli appchain create foundation --help
```

## Creating a New Appchain

The typical workflow involves two main steps:

### 1. Deploy Foundation Contracts

Deploys foundational contracts required by the Appchain node to run:
- Nitro core contracts on the settlement chain
- Sequencing contracts on the sequencing chain
- ArbChainConfig

> **Important:** Save the sequencer and interim-owner private keys displayed during this step.

### 2. Deploy Features

Deploys additional contracts that depend on the Appchain node:
- Token bridge contracts
- Withdrawal-related contracts (TeeModule, AssertionPoster, etc.)
- Utility contracts (Multicall3)

> **Important:** The Appchain RPC must be available before running this step. This process can take 5-10 minutes as it waits for retryable tickets to succeed.

## Available Commands

Run `synd-cli` to see all available commands. Main command categories:

- `appchain create` - Deploy various contract sets (foundation, features, sequencing, withdrawals, etc.)
- `appchain handoff` - Transfer contract ownership
- `appchain arb-owner` - Manage Arbitrum owner operations
- `appchain check-token-bridge` - Verify token bridge setup
- `appchain e2e` - Run end-to-end tests
- `alias` - Calculate L1->L2 aliased addresses

Each command supports:
- `init` subcommand to generate example config files
- `--config <path>` to load options from a JSON file
- `--help` to see all available options

## Development

### Prerequisites

- [Bun](https://bun.sh) - JavaScript runtime and package manager
- Node.js (for pre-commit hooks)

### Scripts

```bash
bun run format          # Format code with Biome
bun run lint            # Lint code with Biome
bun run typecheck       # Type check with TypeScript
bun run biome:check     # Run all Biome checks
```

### Generate Contract ABIs

```bash
make generate-contract-abis
```

## Project Structure

```
synd-cli/
├── src/
│   ├── abi/              # Contract ABIs
│   ├── cli/
│   │   ├── commands/     # CLI command implementations
│   │   ├── schema.ts     # Zod schemas for option validation
│   │   └── index.ts      # Main CLI entry point
│   └── utils/            # Utility functions
├── options/              # Config file examples and generated configs
│   └── examples/         # Example config files
└── biome.json            # Biome formatter/linter configuration
```
