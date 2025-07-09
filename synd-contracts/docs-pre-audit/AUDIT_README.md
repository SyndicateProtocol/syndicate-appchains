# Syndicate Sequencer Chain Smart Contracts - Pre Audit

The Smart contracts we intend to have the audit are the following:

- [SyndicateFactory](src/src/factory/SyndicateFactory.sol/contract.SyndicateFactory.md)
- [PermissionModuleFactories](src/src/factory/PermissionModuleFactories.sol/contract.PermissionModuleFactories.md)
- [SyndicateFactoryWrapper](src/src/factory/SyndicateFactoryWrapper.sol/contract.SyndicateFactoryWrapper.md)
- [SyndicateSequencingChain](src/src/SyndicateSequencingChain.sol/contract.SyndicateSequencingChain.md)
- [SequencingModuleChecker](src/src/SequencingModuleChecker.sol/abstract.SequencingModuleChecker.md)
- [AllowlistSequencingModule](src/src/sequencing-modules/AllowlistSequencingModule.sol/contract.AllowlistSequencingModule.md)
- [WalletPoolWrapperModule](src/src/sequencing-modules/WalletPoolWrapperModule.sol/contract.WalletPoolWrapperModule.md)
- [AtomicSequencer](src/src/atomic-sequencer/AtomicSequencer.sol/contract.AtomicSequencer.md)
- [AtomicSequencerImplementation](src/src/atomic-sequencer/AtomicSequencerImplementation.sol/contract.AtomicSequencerImplementation.md)
- [RequireAndModule](src/src/requirement-modules/RequireAndModule.sol/contract.RequireAndModule.md)
- [RequireOrModule](src/src/requirement-modules/RequireOrModule.sol/contract.RequireOrModule.md)
- [RequireCompositeModule](src/src/requirement-modules/RequireCompositeModule.sol/contract.RequireCompositeModule.md)
- [IRequirementModule](src/src/interfaces/IRequirementModule.sol/interface.IRequirementModule.md)
- [PermissionModule](src/src/interfaces/IPermissionModule.sol/interface.IPermissionModule.md)
- [AddressStructuredLinkedList](src/src/LinkedList/AddressStructuredLinkedList.sol/library.AddressStructuredLinkedList.md)
- [ArbConfigManager](src/src/config/ArbConfigManager.sol/contract.ArbConfigManager.md)
- [ArbChainConfig](src/src/config/ArbChainConfig.sol/contract.ArbChainConfig.md)
- [ArbConfigManagerFactory](src/src/config/ArbConfigManagerFactory.sol/contract.ArbConfigManagerFactory.md)
- [AssertionPoster](src/src/withdrawal/AssertionPoster.sol/contract.AssertionPoster.md)
- [AttestationDocVerifier](src/src/withdrawal/AttestationDocVerifier.sol/contract.AttestationDocVerifier.md)
- [TeeKeyManager](src/src/withdrawal/TeeKeyManager.sol/contract.TeeKeyManager.md)
- [TeeModule](src/src/withdrawal/TeeModule.sol/contract.TeeModule.md)

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

### Assertion Poster

The `AssertionPoster` contract is a generic assertion posting utility for Arbitrum rollups

Core functionalities include:

1. **Generic Assertion Management**:

   - Posts assertions to Arbitrum rollups with configurable block hashes and send roots
   - Supports both legacy (v2) and new (v3) Arbitrum Nitro rollup protocols
   - Manages rollup state progression through assertion posting

2. **Permission Control**:

   - Restricts assertion posting to authorized addresses (contract owner)
   - Configures rollup permissions to prevent unauthorized assertion creation
   - Sets up proper validator and batch poster permissions

3. **Protocol Compatibility**:
   - Auto-detects rollup version and adapts behavior accordingly
   - Handles version-specific assertion formats and function calls
   - Manages rollup configuration updates and state synchronization
