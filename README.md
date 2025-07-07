# Syndicate Appchains

[![Rust](https://img.shields.io/badge/rust-1.88.0+-blue.svg)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/go-1.24.3+-blue.svg)](https://golang.org/)
[![Foundry](https://img.shields.io/badge/foundry-1.2.3--nightly+-orange.svg)](https://getfoundry.sh/)
[![Docker](https://img.shields.io/badge/docker-28.2.2+-blue.svg)](https://www.docker.com/)
<!-- [![License](https://img.shields.io/badge/license-XXX-green.svg)](LICENSE) -->

A comprehensive monorepo containing the complete Syndicate Appchains infrastructure stack.
This repository includes the core components for building, deploying, and managing syndicate appchains.

## 📖 Documentation

For detailed documentation, architecture guides, and tutorials, visit [docs.syndicate.io](https://docs.syndicate.io/)

## 🏗️ Architecture Overview

Syndicate Appchains enables the creation of custom rollups that utilize a two-source chain architecture:

- **Sequencing Chain**: Stores transaction data and handles appchain sequencing
- **Settlement Chain**: Manages fund bridging from/to the rollup

This design provides cost-effective, customizable sequencing while leveraging the liquidity and security of established blockchain networks.

## 🚀 Quick Start

### Prerequisites

Ensure you have the following dependencies installed:

| Dependency         | Version           | Installation                                                   |
| ------------------ | ----------------- | -------------------------------------------------------------- |
| **Rust Toolchain** | `1.88.0+`         | [rustup.rs](https://rustup.rs/)                                |
| **Go**             | `1.24.3+`         | [golang.org](https://golang.org/dl/)                           |
| **Foundry/Forge**  | `1.2.3-nightly+`  | [getfoundry.sh](https://getfoundry.sh/)                        |
| **Docker**         | `28.2.2+`         | [docker.com](https://www.docker.com/products/docker-desktop/)  |
| **abigen**         | `1.15.11-stable+` | `go install github.com/ethereum/go-ethereum/cmd/abigen@latest` |
| **SP1 Toolchain**  | `v5.0.5+`         | [SP1 Documentation](https://docs.supranational.net/)           |

### Development Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/syndicateio/syndicate-appchains.git
   cd syndicate-appchains
   ```

2. **Install development tools**

   ```bash
   # Install pre-commit hooks
   pip install pre-commit
   pre-commit install
   
   # Install Rust utilities
   cargo install cargo-machete
   cargo install taplo-cli
   ```

3. **Build the project**

   ```bash
   # Build all Rust components
   cargo build
   
   # Build smart contracts
   cd synd-contracts
   forge build
   ```

## 🧪 Testing

```bash
# Run all tests
cargo nextest run --workspace

# Run specific component tests
cargo test -p synd-translator
cargo test -p synd-maestro

# Run smart contract tests
cd synd-contracts
forge test
```

## 🔧 Code Quality

### Formatting and Linting

```bash
# Format Rust code
cargo +nightly fmt --all -- --unstable-features

# Run Clippy linter
cargo clippy --workspace --all-targets --all-features

# Format TOML files
taplo fmt "path/to/Cargo.toml"

# Check for unused dependencies
cargo machete
```

## 📁 Project Structure

```none
syndicate-appchains/
├── synd-translator/         # Source chains data translation and slotting into mchain blocks
├── synd-maestro/            # Transaction management and Redis streams
├── synd-batch-sequencer/    # Transaction batching and submission
├── synd-chain-ingestor/     # Chain data ingestion and caching
├── synd-mchain/             # Mchain storage and RPC API
├── synd-contracts/          # Smart contracts
├── synd-withdrawals/        # TEE-based withdrawal system
│   ├── synd-enclave/        # Trusted Execution Environment enclave code
│   ├── synd-proposer/       # Proposer service that publishes signed state commitments on-chain
│   └── synd-tee-attestation-zk-proofs/  # ZK proof system for TEE attestion docs
├── test-framework/          # Integration testing
└── documentation/           # Deployment guides and configs
```

### Core Components

#### Rust Services

- **synd-translator**: Reads block data from each source chain's ingestor, uses slotter logic to fit them into slots and transforms data into mchain blocks
- **synd-maestro**: Receives user appchain transactions and adds them to Redis streams (one stream per chain), where sequencers can read from. Handles transaction re-submission for re-orgs or sequencer failures
- **synd-batch-sequencer**: Reads from Redis transaction streams, batches/compresses transactions, then signs and publishes batches to the sequencing chain
- **synd-chain-ingestor**: Connects to source chains and builds a cache of (block_hash, timestamp) pairs for fast translator cold-sync. Reads new data from upstream RPC and forwards to translators
- **synd-mchain**: Stores mchain block data in RocksDB database. Implements the RPC API that rollup nodes expect when reading from Ethereum
- **test-framework**: Integration testing framework

#### Smart Contracts

- **synd-contracts**: Core system contracts including SystemConfig, AppchainFactory, Inbox/Outbox, and withdrawal system contracts

#### Withdrawal System

- **synd-enclave**: TEE (Trusted Execution Environment) component that receives block data from Ethereum L1, sequencing chain, and settlement chain, applies the rollup STF to derive rollup state, and signs state commitments enabling withdrawals
- **synd-proposer**: Connects TEE to settlement chain bridge. Feeds chain data to TEE enclave, obtains signed state commitments (sendRoot), and submits them to the TEEModule smart contract
- **synd-tee-attestation-zk-proofs**: Multi-component system for TEE attestation:
  - **aws-nitro**: Validates TEE attestation documents signed by Amazon's x509 root certificate
  - **sp1**: SP1-specific implementation for generating and verifying ZK proofs of AWS Nitro attestation
  - **proof-submitter**: CLI tool for fetching TEE attestation, generating SP1-based ZK proofs, and submitting on-chain

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting (`cargo test && cargo clippy`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Follow Rust coding standards and use `cargo fmt` and `cargo clippy`
- Write comprehensive tests for new features
- Update documentation for any API changes
- Ensure all tests and pre-commit hooks pass before submitting PRs

<!-- ## 📄 License -->

<!-- This project is licensed under the XXX License - see the [LICENSE](LICENSE) file for details. -->

## 🆘 Support

- **Documentation**: [docs.syndicate.io](https://docs.syndicate.io/)
- **Issues**: [GitHub Issues](https://github.com/syndicateio/syndicate-appchains/issues)
- **Discussions**: [GitHub Discussions](https://github.com/syndicateio/syndicate-appchains/discussions)
