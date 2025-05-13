# Synd-Verifier

`Synd-Verifier` is a Rust-based component used in the TEE-based withdrawal validation pipeline. It verifies batches of Ethereum-compatible blocks from both the sequencing and settlement chains, and produces a validated MBlock - a canonical data artifact used to construct or validate rollup state in trusted execution environments (TEEs).

## Role in TEE Withdrawals

In the TEE withdrawal flow:

- This component runs inside the TEE.

- It consumes settlement and sequencing block data as inputs (via chain ingestor).

- It verifies block correctness, receipt integrity, and timestamp alignment.

- It produces an MBlock—the trusted result used to on the `synd-lock-verifier` component.

This ensures that all onchain and offchain state transitions are cryptographically verified and consistent with the rollup’s canonical rules.
