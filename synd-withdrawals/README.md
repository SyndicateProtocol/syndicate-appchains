# Syndicate Withdrawals

This system handles the validation and submission of withdrawal proofs via a Trusted Execution Environment (TEE). It orchestrates input data from multiple chains and nodes, performs various verification steps inside a secure enclave, and outputs proofs to on-chain smart contracts.

## Components

### Synd-Proposer

The Proposer is the orchestrator of the withdrawal flow. It collects relevant inputs and generates assertions for TEE verification. Responsibilities include:

- Aggregating data from:

  - L1 node (eth_getProof)

  - Settlement node (eth_getLogs)

  - Sequencing node (eth_getBlockByNumber, eth_getLogs, eth_getProof, arbdebug_validationInputsAt)

  - Appchain node (arbdebug_validationInputsAt)

- Constructing the PendingAssertion and invoking the TEE module

- Submitting the verified assertion to smart contract:

  - submitPendingAssertion (calls into TEEModule.sol)

### Synd-Sequencing-Verifier

This module verifies the consistency and correctness of the L1 data that is used to construct the sequencing chain.

### Synd-Sequencing-Block-Verifier

Validates the structure and integrity of blocks from the sequencing chain using Arbitrum Nitro STF

### Synd-Appchain-Verifer

This module verifies the consistency and correctness of the settlement & sequencing chains data that is used to construct the appchain.

### Synd-Appchain-Block-Verifer

Validates the structure and integrity of blocks from the appchain using Arbitrum Nitro STF

### Synd-Enclave

The secure enclave environment executes core logic for verifying withdrawal proofs. This is a forked version of [base/op-enclave](https://github.com/base/op-enclave).
