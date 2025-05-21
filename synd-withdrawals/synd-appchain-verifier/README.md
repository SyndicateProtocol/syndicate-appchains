# Synd-Appchain-Verifier

`Synd-Appchain-Verifier` is a Rust-based component used in the TEE-based withdrawal validation pipeline. It verifies Ethereum-compatible input data from both the sequencing and settlement chains and produces a validated BlockVerifierInput—the canonical artifact used to construct or validate rollup state in trusted execution environments (TEEs).

## Role in TEE Withdrawals

In the TEE withdrawal flow:

- This component runs inside a Trusted Execution Environment (TEE).

- It receives structured input data from the settlement and sequencing layers.

- It verifies the correctness and alignment of all provided data.

- It outputs a validated `BlockVerifierInput` to be used by the synd-block-verifier component.

This ensures that all onchain and offchain state transitions are cryptographically verified and consistent with the rollup’s canonical rules.
