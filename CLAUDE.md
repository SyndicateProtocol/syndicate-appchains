# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a monorepo for Syndicate Appchains - a blockchain/rollup infrastructure project. The codebase consists of multiple Rust services, Solidity smart contracts, and Go components for TEE (Trusted Execution Environment) functionality.

Syndicate appchains are rollups that use two distinct `source chains` for deriving their state:

- **Sequencing chain**: Where the appchain transaction data is stored
- **Settlement chain**: Where funds can be bridged to/from

This architecture enables cost-effective and customizable sequencing while leveraging the liquidity network of a larger chain.

To achieve this in a non-breaking way, data from the `source chains` gets stitched together into an intermediate format (colloquially called the `mchain`), looks like a standard L1 node from the perspective of the rollup node (nitro/op-node).

## Architecture Components

### Core Rust Services

- **synd-translator**: Reads block data from each `source chain`'s `ingestor`, uses the `slotter` logic to fit them into slots and transforms their data into `mchain` blocks
- **synd-maestro**: Receives user appchain transactions and adds them to Redis streams (one stream per chain), where sequencers (`synd-batch-sequencer`) can read from. Maestro also handles transaction re-submission in case of re-orgs or sequencer failures
- **synd-batch-sequencer**: Reads from Redis transaction streams, batches/compresses transactions together, then signs and publishes batches to the sequencing chain
- **synd-chain-ingestor**: Connects to a `source chain` and builds a cache of (`block_hash`, `timestamp`) pairs. This cache enables fast initial translator cold-sync. Also reads new data from upstream RPC and forwards it to the `translator`. Each translator must connect to exactly 2 ingestors: one for the sequencing chain and one for the settlement chain. This architecture saves bandwidth by avoiding redundant requests to external servers when multiple translators (for multiple appchains) connect to the same ingestors
- **synd-mchain**: Stores `mchain` block data in a RocksDB database. Implements the RPC API that any rollup node expects when reading from Ethereum
- **test-framework**: Integration testing framework

### Smart Contracts

- **synd-contracts**: Core system contracts (SystemConfig, AppchainFactory, Inbox/Outbox)

### Withdrawal System

- **synd-enclave**: TEE (Trusted Execution Environment) component that receives block data from `Ethereum L1`, the `sequencing chain`, and `settlement chain`, applies the rollup STF (state transition function) to derive the rollup state, and signs a state commitment that enables withdrawals from the settlement chain's bridge (in Arbitrum this is called the `sendRoot`)
- **synd-proposer**: Acts as the connector between the TEE and the settlement chain bridge. This service feeds chain data to the TEE enclave at configurable intervals, obtains the signed state commitment (`sendRoot`), wraps it into an Ethereum transaction, and submits it to the [`TEEModule` smart contract](synd-contracts/src/withdrawal/TeeModule.sol)
- **synd-tee-attestation-zk-proofs**: Split into multiple sub-crates:
  - **aws-nitro**: Contains vanilla Rust code for validating TEE attestation documents signed by Amazon's x509 root certificate. It outputs the document validity window, relevant PCRs, and the TEE signing key. Designed for reuse across different ZK proof systems
  - **sp1**: SP1-specific implementation for generating and verifying ZK proofs of AWS Nitro attestation documents. On-chain proof verification is handled by the [`AttestationDocVerifier` smart contract](synd-contracts/src/withdrawal/AttestationDocVerifier.sol)
  - **proof-submitter**: CLI tool for fetching TEE attestation from an enclave, generating SP1-based ZK proofs, and submitting them on-chain

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
cargo nextest run --workspace

# Linting and formatting
cargo +nightly fmt --all -- --unstable-features
cargo clippy --workspace --all-targets --all-features
cargo machete
taplo fmt /path-to-a-cargo-file/Cargo.toml # NOTE: DO NOT RUN WITH `"**/Cargo.toml"` - this will produce unintended changes to this repo's submodules

# Build Docker images
docker build --target synd-translator -f Dockerfile . --tag synd-translator
# Or build all targets
./build_all_docker_targets.sh [release|debug]
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
- `synd-contracts/foundry.toml`: Solidity contracts configuration
- `rust-toolchain.toml`: Rust version specification
- `clippy.toml`: Linting rules
- `taplo.toml`: TOML formatter config

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

You can find all the `synd-contract` deployments in this [README file](synd-contracts/README.md).

## Important Notes

1. Always run linting before committing code
2. Use the provided Docker Compose files for local testing
3. The project uses workspace inheritance for dependencies
4. Specify only the required features for each dependency at crate level
5. Integration tests require GitHub package authentication
6. Prefer writing integration tests over manually spawning Docker containers for testing/experimentation

## Pre-commit Hooks and Formatting

The repository uses pre-commit hooks that run automatically before commits. Key points:

1. **pre-commit** tool and hook must be installed:

   ```shell
   brew install pre-commit
   pre-commit install
   ```

2. **Nightly Rust Required**: The formatter uses nightly features, so ensure you have nightly installed:

   ```bash
   rustup toolchain install nightly
   ```

3. **Auto-formatting**: The pre-commit hook should automatically format Rust files. If commits fail due to formatting:

   ```bash
   cargo +nightly fmt --all -- --unstable-features
   git add -u
   git commit
   ```

4. **Bypassing Hooks**: For documentation-only changes, you can bypass hooks:

   ```bash
   git commit --no-verify -m "your message"
   ```

5. **Hook Location**: The actual hook is at `.git/hooks/pre-commit` (not the `.pre-commit-config.yaml`)
