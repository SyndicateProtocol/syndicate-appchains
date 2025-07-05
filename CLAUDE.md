# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a monorepo for Syndicate Appchains - a blockchain/rollup infrastructure project. The codebase consists of multiple Rust services, Solidity smart contracts, and Go components for TEE (Trusted Execution Environment) functionality.

## Architecture Components

### Core Rust Services
- **synd-translator**: Translates and signs transactions from layer 2 to layer 1
- **synd-maestro**: Orchestrates batcher and RPC node via RPC calls
- **synd-batch-sequencer**: Sequences and submits batches to contracts
- **synd-chain-ingestor**: Ingests chain data via RPC
- **synd-mchain**: RPC node implementation
- **test-framework**: Integration testing framework

### Smart Contracts
- **synd-contracts**: Core system contracts (SystemConfig, AppchainFactory, Inbox/Outbox)
- **metabased-contracts**: Additional smart contracts

### Withdrawal System
- **synd-withdrawals**: Main withdrawal service (Rust)
- **synd-withdrawals-go**: Go server for withdrawal operations
- **synd-withdrawals-tee**: TEE (Trusted Execution Environment) components

## Development Setup

### Prerequisites
- Rust (with nightly toolchain)
- Docker and Docker Compose
- Foundry (for smart contracts)
- GitHub personal access token for downloading packages

### Key Commands

```bash
# Build all Rust services
cargo build

# Build specific service
cargo build -p synd-translator

# Run tests
cargo test --workspace

# Linting and formatting
cargo +nightly fmt --all -- --unstable-features
cargo clippy --workspace --all-targets --all-features
cargo machete
taplo fmt "**/Cargo.toml"

# Run with sample command
cargo make print-sample-command

# Build Docker images
docker build --target synd-translator -f Dockerfile . --tag synd-translator
# Or build all targets
./build_all_docker_targets.sh [release|debug]
```

## Local Development

The project uses Docker Compose for local development with different chain configurations:

```bash
# Anvil chain (local testing)
docker compose -f docker-compose-anvil.yml up --build

# Arbitrum Sepolia
docker compose -f docker-compose-arbsepolia.yml up --build

# Base Sepolia
docker compose -f docker-compose-basesepolia.yml up --build

# Optimism Sepolia
docker compose -f docker-compose-opsepolia.yml up --build
```

## Docker Build

The Dockerfile supports multiple targets for different services:

```bash
# Available targets: synd-translator, synd-maestro, synd-batch-sequencer, 
# synd-chain-ingestor, synd-mchain, synd-withdrawals
docker build --target <service-name> -f Dockerfile . --tag <service-name>
```

## Smart Contract Development

```bash
cd synd-contracts

# Install dependencies
forge install

# Build contracts
forge build

# Run tests
forge test

# Deploy contracts (requires environment setup)
./deploy-anvil.sh
./deploy-sepolia.sh
./deploy-basesepolia.sh

# Verify contracts on Etherscan
forge verify-contract <address> <contract> --chain-id <id> --etherscan-api-key $API_KEY
```

## Testnet Deployment

For testnet RPC node setup:
```bash
cd synd-testnet
# Components: testnet-data-poster, testnet-fraud-prover, testnet-orchestrator
cargo build -p <component>
```

## Key Configuration Files

- `Cargo.toml`: Workspace configuration
- `foundry.toml`: Solidity contracts configuration
- `rust-toolchain.toml`: Rust version specification
- `clippy.toml`: Linting rules
- `taplo.toml`: TOML formatter config
- `config/*.toml`: Environment-specific configurations (dev, testnet, mainnet)

## Environment Profiles

- **dev**: Local development with minimal verification
- **testnet**: Testnet deployment with production defaults  
- **mainnet**: Production deployment
- **no-logs**: Special profile for log aggregation

## Integration Test Setup

For running integration tests with GitHub packages:

```bash
# Set GitHub token for package access
export DOCKER_BUILDKIT=1
echo "$GITHUB_TOKEN" | docker login ghcr.io -u USERNAME --password-stdin

# Run integration tests
cd test-framework
cargo test
```

## Debugging

Enable debug logs:
```bash
export RUST_LOG=debug
# Or for specific modules
export RUST_LOG=synd_translator=debug
```

## Contract Addresses

Key deployed contracts on various networks:
- Base Sepolia: `0x5d0E6CE4fd951c8B35c7AC67C7783C0e319CbE91`
- Optimism Sepolia: `0xA2eD7e5442D61FBbC3a1eBA6F3cBC44aaB82A61d`
- Arbitrum Sepolia: `0xba3E65dF7f433C84A23fE26533aE2B5dB1867479`

## Important Notes

1. Always run linting before committing code
2. Use the provided Docker Compose files for local testing
3. The project uses workspace inheritance for dependencies
4. TEE components require special build environments
5. Contract deployment requires proper environment variables set
6. Integration tests require GitHub package authentication