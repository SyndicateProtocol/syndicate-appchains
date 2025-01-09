# Integration Tests

This directory contains integration tests for the project, which are located in the `tests` directory.
These tests are meant to verify the full system functionality by simulating cross-chain interactions like sequencing transactions, deposits and withdrawals between the sequencing chain, the settlement chain and the L3 chain.

## Running Tests

To run all tests:

```bash
cargo test --features e2e-tests
```

or, in the root directory:

```bash
just e2e-tests
```
