# Syndicate Metabased Sequencer Chain Smart Contracts - Pre Audit

The Smart contracts we intend to have the audit are the following:

- [MetabasedFactory](contract.MetabasedFactory.md)
- [MetabasedSequencerChain](contract.MetabasedSequencerChain.md)
- [SequencingModuleChecker](abstract.SequencingModuleChecker.md)
- [AllowlistSequencingModule](contract.AllowlistSequencingModule.md)
- [AtomicSequencer](contract.AtomicSequencer.md)
- [AtomicSequencerImplementation](contract.AtomicSequencerImplementation.md)
- [RequireAllModule](contract.RequireAllModule.md)
- [RequireAnyModule](contract.RequireAnyModule.md)
- [SyndicateToken](contract.SyndicateToken.md)
- [IRequirementModule](interface.IRequirementModule.md)
- [PermissionModule](interface.PermissionModule.md)

## General Overview of Syndicate Metabased Sequencer Chain and Its Smart Contracts

## What is the Metabased Sequencer Chain?

The Metabased Sequencer Chain is a specialized blockchain designed to serve as a shared sequencing layer for rollups and appchains. It abstracts the sequencing functionality from existing scaling solutions like Arbitrum Orbit and Optimism Stack, allowing developers to create customized rollups with community-driven sequencing rules.

## Understanding Sequencers in Rollups

In a rollup architecture, the sequencer plays a crucial role:

1. **Transaction Collection**: Sequencers gather user transactions that are intended for the rollup.

2. **Ordering**: They determine the order in which transactions will be processed, which is critical for establishing a canonical state.

3. **Batch Creation**: Sequencers bundle transactions into batches that are submitted to the settlement chain. In the case of Optimism mainnet chain, for instance, it is Ethereum mainnet the chain used for final settlement.

Traditionally, sequencing has been centralized, with projects like Optimism and Base maintaining control over their sequencers. This creates potential points of centralization and limitations on customization.

## The Metabased Sequencer Architecture

The Metabased Sequencer Chain consists of several interconnected contracts:

### Core Components:

1. **MetabasedSequencerChain**:

   - Central contract that processes and orders transactions
   - Interfaces with various requirement modules
   - Maintains the canonical ordering of transactions

2. **SequencingModuleChecker**:

   - Validates that transactions meet criteria specified by requirement modules
   - Acts as a registry and orchestrator for modular sequencing rules

3. **MetabasedFactory**:
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

2. They use the MetabasedFactory to deploy their configuration, selecting which requirement modules to include.

3. The community can define who has permission to sequence transactions through modules.

4. When users submit transactions to the rollup, it will be picked up by the sequencer chain via the execution pipeline:
   Then the following steps are executed:

   - validators called sequencers will submit transactions to the Metabase sequencer chain
   - These transactions are received by the MetabasedSequencerChain
   - The SequencingModuleChecker validates them against all requirement modules
   - If approved, transactions are sequenced in a deterministic order
   - Batches are created and submitted to the Layer 1 for settlement

5. The community can update sequencing rules by adding, removing, or configuring requirement modules.

## Why This Matters

The Metabased Sequencer Chain democratizes the sequencing process by:

1. **Removing Centralization**: No single entity controls the sequencing
2. **Enabling Customization**: Communities can create rollups with rules that fit their specific needs
3. **Supporting Innovation**: New sequencing algorithms and rules can be implemented as modules
4. **Abstracting Complexity**: Developers don't need to build sequencing from scratch for new rollups

By abstracting sequencing functionality from stacks like Arbitrum Orbit and Optimism Stack, the Metabased Sequencer Chain enables greater experimentation with appchains and rollups while maintaining the security and efficiency benefits of these scaling solutions.

This approach moves the ecosystem toward a more open, flexible, and democratic model for transaction sequencing, which is a critical component of the decentralized future of blockchain scaling.
