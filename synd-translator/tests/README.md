# Integration Tests

This directory contains integration tests for the project, which are located in the `tests` directory.

## Test Categories

### Environment

Located in `tests/environment_tests.rs`, these tests can be used to validate that a target environment is working as expected.
It makes cross-chain interactions and ensures the correct effect is observed.

### Integration

Located in `tests/integration_tests.rs`, these tests verify that the software is working as expected. It tests the entire rollup lifecycle using a local environment.

### Signal Tests

Located in `tests/signal_tests.rs`, these tests verify that the software handles OS signals as expected.

## Running Tests

`Integration` tests are executed by default when running `cargo test`.

The tests can also be run standalone with:

```bash
cargo test --package e2e-tests --test environment_tests --features env-tests
```
