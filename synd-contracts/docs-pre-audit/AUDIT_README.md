# Syndicate Sequencer Chain Smart Contracts - Pre Audit

The Smart contracts we intend to have the audit are the following:

## Core Sequencing Contracts

- [SyndicateFactory](src/src/factory/SyndicateFactory.sol) - Deploys new rollup configurations with customized sequencing rules
- [RequireAndModuleFactory](src/src/factory/PermissionModuleFactories.sol) - Factory for AND logic requirement modules
- [RequireCompositeModuleFactory](src/src/factory/PermissionModuleFactories.sol) - Factory for composite requirement modules
- [RequireOrModuleFactory](src/src/factory/PermissionModuleFactories.sol) - Factory for OR logic requirement modules
- [SyndicateFactoryWrapper](src/src/factory/SyndicateFactoryWrapper.sol) - Wrapper for enhanced factory functionality
- [SyndicateSequencingChain](src/src/SyndicateSequencingChain.sol) - Central contract that processes and orders transactions
- [SequencingModuleChecker](src/src/SequencingModuleChecker.sol) - Validates transactions against requirement modules
- [AllowlistSequencingModule](src/src/sequencing-modules/AllowlistSequencingModule.sol) - Allowlist-based sequencing control
- [WalletPoolWrapperModule](src/src/sequencing-modules/WalletPoolWrapperModule.sol) - Wallet pool wrapper for sequencing
- [AtomicSequencer](src/src/atomic-sequencer/AtomicSequencer.sol) - Low-level atomic sequencing components
- [AtomicSequencerImplementation](src/src/atomic-sequencer/AtomicSequencerImplementation.sol) - Implementation for atomic sequencer

## Permission and Requirement Modules

- [RequireAndModule](src/src/requirement-modules/RequireAndModule.sol) - Requires ALL permission checks to pass (AND logic)
- [RequireOrModule](src/src/requirement-modules/RequireOrModule.sol) - Requires ANY permission check to pass (OR logic)
- [RequireCompositeModule](src/src/requirement-modules/RequireCompositeModule.sol) - Composite permission logic combining AND/OR

## Interfaces and Libraries

- [IRequirementModule](src/src/interfaces/IRequirementModule.sol) - Interface for requirement modules
- [IPermissionModule](src/src/interfaces/IPermissionModule.sol) - Interface for permission modules
- [AddressStructuredLinkedList](src/src/LinkedList/AddressStructuredLinkedList.sol) - Library for managing linked lists

## Configuration Management

- [ArbConfigManager](src/src/config/ArbConfigManager.sol) - Manages deployment and upgrading of chain configurations
- [ArbChainConfig](src/src/config/ArbChainConfig.sol) - Configuration store for Arbitrum rollup parameters
- [ArbConfigManagerFactory](src/src/config/ArbConfigManagerFactory.sol) - Factory for deploying configuration managers

### Bridge Proxy Contracts

- [ArbitrumBridgeProxy](src/src/token/bridges/ArbitrumBridgeProxy.sol) - Arbitrum L1-L2 bridge integration
- [BaseBridgeProxy](src/src/token/bridges/BaseBridgeProxy.sol) - Base L1-L2 bridge integration
- [OptimismBridgeProxy](src/src/token/bridges/OptimismBridgeProxy.sol) - Optimism L1-L2 bridge integration

### Token Interfaces and Libraries

- [IBridgeProxy](src/src/token/interfaces/IBridgeProxy.sol) - Generic bridge proxy interface
- [IERC7802](src/src/token/crosschain/interfaces/IERC7802.sol) - ERC-7802 Crosschain Fungibility Extension
- [IBridgeRateLimiter](src/src/token/crosschain/interfaces/IBridgeRateLimiter.sol) - Bridge rate limiting interface
- [CREATE3](src/src/token/crosschain/libraries/CREATE3.sol) - Deterministic deployment library
- [Bytes32AddressLib](src/src/token/crosschain/libraries/Bytes32AddressLib.sol) - Address utilities

## Withdrawal System (TEE-based)

- [AssertionPoster](src/src/withdrawal/AssertionPoster.sol) - Posts assertions to Arbitrum rollups (supports v2 and v3)
- [AttestationDocVerifier](src/src/withdrawal/AttestationDocVerifier.sol) - Verifies TEE attestation documents using SP1 proofs
- [TeeKeyManager](src/src/withdrawal/TeeKeyManager.sol) - Manages TEE program hashes and public keys
- [TeeModule](src/src/withdrawal/TeeModule.sol) - Main TEE orchestrator for withdrawal process with challenge mechanism

### Withdrawal Interfaces

- [IAssertionPoster](src/src/withdrawal/IAssertionPoster.sol) - Interface for posting assertions
- [IAttestationDocVerifier](src/src/withdrawal/IAttestationDocVerifier.sol) - Interface for verifying attestation documents
- [ITeeKeyManager](src/src/withdrawal/ITeeKeyManager.sol) - Interface for TEE key management

## General Overview of Syndicate Sequencer Chain and Its Smart Contracts

## What is the Syndicate Sequencer Chain?

The Syndicate Sequencer Chain is a specialized blockchain designed to serve as a shared sequencing layer for rollups and appchains. It abstracts the sequencing functionality from existing scaling solutions like Arbitrum Orbit and Optimism Stack, allowing developers to create customized rollups with community-driven sequencing rules.

In this audit, we want to focus on the components of Syndicate Sequencer Chain that allows for transaction sequencing for Arbitrum based rollups.

## Understanding Sequencers in Rollups

In a rollup architecture, the sequencer plays a crucial role:

1. **Transaction Collection**: Sequencers gather user transactions that are intended for the rollup.

2. **Ordering**: They determine the order in which transactions will be processed, which is critical for establishing a canonical state.

3. **Batch Creation**: Sequencers bundle transactions into batches that are submitted to the settlement chain. In the case of Optimism mainnet chain, for instance, it is Ethereum mainnet the chain used for final settlement.

Traditionally, sequencing has been centralized, with projects like Optimism and Base maintaining control over their sequencers. This creates potential points of centralization and limitations on customization.

## The Syndicate Sequencer Architecture

The Syndicate Sequencer Chain consists of several interconnected contracts:

### Core Components:

1. **SyndicateSequencingChain**:

   - Central contract that processes and orders transactions
   - Interfaces with various requirement modules
   - Maintains the canonical ordering of transactions

2. **SequencingModuleChecker**:

   - Validates that transactions meet criteria specified by requirement modules
   - Acts as a registry and orchestrator for modular sequencing rules

3. **SyndicateFactory**:
   - Deploys new rollup configurations with customized sequencing rules
   - Allows for template-based creation of new chains

### Modular Components:

1. **Requirement Modules**:

   - Pluggable components that define specific rules for transaction acceptance
   - Can include rules for permissions, fees, spam prevention, MEV protection, etc.

2. **Atomic Sequencer Components**:
   - Low-level execution components for the sequencing process
   - Provide basic building blocks for the sequencing pipeline, for example it can sequence transactions from multiple chains in a single batch

## How It Works Together

1. A community or project decides to create a rollup with custom sequencing rules.

2. They use the SyndicateFactory to deploy their configuration, selecting which requirement modules to include.

3. The community can define who has permission to sequence transactions through modules.

4. When users submit transactions to the rollup, it will be picked up by the sequencer chain via the execution pipeline:
   Then the following steps are executed:

   - validators called sequencers will submit transactions to the Metabase sequencer chain
   - These transactions are received by the SyndicateSequencingChain
   - The SequencingModuleChecker validates them against all requirement modules
   - If approved, transactions are sequenced in a deterministic order
   - Batches are created and submitted to the Layer 1 for settlement

5. The community can update sequencing rules by adding, removing, or configuring requirement modules.

## Why This Matters

The Syndicate Sequencer Chain democratizes the sequencing process by:

1. **Removing Centralization**: No single entity controls the sequencing
2. **Enabling Customization**: Communities can create rollups with rules that fit their specific needs
3. **Supporting Innovation**: New sequencing algorithms and rules can be implemented as modules
4. **Abstracting Complexity**: Developers don't need to build sequencing from scratch for new rollups

By abstracting sequencing functionality from stacks like Arbitrum Orbit and Optimism Stack, the Syndicate Sequencer Chain enables greater experimentation with appchains and rollups while maintaining the security and efficiency benefits of these scaling solutions.

This approach moves the ecosystem toward a more open, flexible, and democratic model for transaction sequencing, which is a critical component of the decentralized future of blockchain scaling.

## Configuration Management System

The Syndicate Sequencer Chain includes a robust configuration management system implemented through three key components in the `config` folder:

### ArbChainConfig

The `ArbChainConfig` contract serves as a configuration store for Arbitrum rollup parameters. It provides:

- **Immutable Configuration Storage**: Stores critical chain parameters that cannot be changed after initialization, including:

  - Chain IDs (both sequencing and appchain)
  - Arbitrum bridge and inbox addresses
  - Settlement and sequencing parameters
  - Allowed settlement addresses

- **Mutable Configuration Parameters**: Maintains updatable parameters like:

  - RPC URLs for the sequencing chain
  - Block explorer URLs
  - Other non-critical configuration values

- **Ownership Control**: Implements secure ownership mechanisms for parameter updates

- **Proxy Pattern Support**: Designed to work with OpenZeppelin's proxy pattern for upgradeability

### ArbConfigManager

The `ArbConfigManager` contract manages the deployment and upgrading of `ArbChainConfig` instances:

- **Deterministic Deployment**: Uses CREATE2 for deploying `ArbChainConfig` proxies with predictable addresses

- **Beacon Proxy Pattern**: Implements the Beacon Proxy pattern to enable synchronized upgrades of all deployed configurations

- **Centralized Upgrade Management**: Provides a single point for upgrading the implementation for all proxies

- **Address Registry**: Maintains a mapping of chain IDs to deployed proxy addresses

- **Chain-Specific Configuration**: Enables creation of custom configurations for each chain while sharing the same implementation

### ArbConfigManagerFactory

The `ArbConfigManagerFactory` enables consistent deployment of `ArbConfigManager` instances across different environments:

- **Cross-Chain Consistency**: Ensures that `ArbConfigManager` can be deployed with the same address on different chains

- **Deterministic Deployment**: Uses CREATE2 to generate predictable addresses based on a provided salt

- **Address Prediction**: Allows calculation of deployment addresses before actual deployment

- **Factory Pattern**: Implements a clean factory pattern for deploying configuration managers

## How the Configuration System Integrates with Syndicate Sequencer Chain

The configuration management system is a critical infrastructure component that:

1. **Standardizes Chain Configuration**: Provides a consistent way to store and access chain-specific parameters needed by the sequencer

2. **Enables Multi-Chain Support**: Allows the Syndicate Sequencer Chain to interact with multiple Arbitrum rollups, each with unique configuration

3. **Supports Governance**: The ownership model enables decentralized governance of configuration parameters

4. **Facilitates Upgrades**: The upgradeability pattern allows for improvements to the configuration contracts without disrupting existing deployments

5. **Ensures Deterministic Addressing**: CREATE2-based deployment ensures that configurations can be found at predictable addresses, simplifying cross-chain interactions

This configuration system acts as the backbone for the Syndicate Sequencer Chain's multi-chain capabilities, allowing it to correctly route transactions and interact with different Arbitrum-based rollups while maintaining separation and security between chains.

## Withdrawal System Architecture

The withdrawal system represents a critical component of the Syndicate Sequencer Chain, implementing a TEE (Trusted Execution Environment) based approach to secure and verifiable withdrawal operations. This system enables secure withdrawal of funds from the sequencer chain while maintaining decentralization and trustlessness.

### Key Components Overview

The withdrawal system consists of four main contracts working together:

1. **TeeModule**: The main orchestrator contract that manages the withdrawal process
2. **AssertionPoster**: Handles posting assertions to Arbitrum rollups
3. **AttestationDocVerifier**: Verifies TEE attestation documents using zero-knowledge proofs
4. **TeeKeyManager**: Manages valid TEE program hashes and public keys

### TeeModule - Core Withdrawal Logic

The `TeeModule` contract serves as the central coordinator for the withdrawal process, implementing a sophisticated challenge-response mechanism:

#### Core Functionality:

1. **Assertion Management**:

   - Accepts assertions signed by verified TEE instances
   - Manages pending assertions with configurable challenge windows
   - Tracks assertion state through multiple phases (pending, challenged, finalized)

2. **Challenge System**:

   - Implements a challenge period where disputes can be raised
   - Handles multiple competing assertions through a "hack" detection mechanism
   - Rewards honest participants and penalizes malicious behavior

3. **TEE Integration**:

   - Validates that assertions come from authorized TEE instances
   - Ensures assertions are properly signed by verified TEE keys
   - Maintains separation between different TEE program versions

4. **Input Validation**:
   - Manages trusted inputs that TEE instances can reference
   - Provides a secure way to feed external data into the TEE environment
   - Prevents unauthorized manipulation of input data

### AssertionPoster - Rollup Integration

The `AssertionPoster` contract provides a generic interface for posting assertions to Arbitrum rollups:

#### Key Features:

1. **Multi-Protocol Support**:

   - Supports both legacy (v2) and new (v3) Arbitrum Nitro rollup protocols
   - Auto-detects rollup version and adapts behavior accordingly
   - Handles version-specific assertion formats and function calls

2. **Permission Management**:

   - Restricts assertion posting to authorized addresses (contract owner)
   - Configures rollup permissions to prevent unauthorized assertion creation
   - Sets up proper validator and batch poster permissions

3. **State Management**:
   - Manages rollup state progression through assertion posting
   - Handles rollup configuration updates and state synchronization
   - Ensures assertions are posted in correct sequence

### AttestationDocVerifier - TEE Validation

The `AttestationDocVerifier` contract implements cryptographic verification of TEE attestation documents:

#### Verification Process:

1. **Document Validation**:

   - Verifies AWS Nitro Enclave attestation documents using SP1 zero-knowledge proofs
   - Validates root certificate hashes against trusted roots
   - Checks document validity windows and timestamps

2. **PCR (Platform Configuration Register) Validation**:

   - Validates PCR values to ensure TEE instances are running expected code
   - Prevents tampering with TEE execution environment
   - Ensures reproducible and verifiable TEE deployments

3. **Key Extraction**:
   - Extracts TEE signing keys from verified attestation documents
   - Provides cryptographic proof that keys belong to legitimate TEE instances
   - Enables downstream verification of TEE-signed assertions

### TeeKeyManager - Key Lifecycle Management

The `TeeKeyManager` contract manages the lifecycle of TEE keys and program hashes:

#### Key Management Features:

1. **Permissionless Key Addition**:

   - Allows anyone to add a key by providing a valid attestation document proof
   - Verifies attestation documents before accepting keys
   - Maintains registry of all valid TEE keys

2. **Program Hash Management**:

   - Associates keys with specific TEE program hashes
   - Ensures keys can only be used by intended TEE programs
   - Prevents key reuse across different program versions

3. **Emergency Controls**:
   - Owner can revoke all keys in case of security incidents
   - Ability to update attestation document verifier
   - Provides escape hatch for system recovery
