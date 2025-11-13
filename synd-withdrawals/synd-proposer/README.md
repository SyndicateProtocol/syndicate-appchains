# `Synd-Proposer`

The Syndicate Proposer is responsible for extracting the appchain root state and submitting assertions to the settlement chain `AssertionPoster` contract.

## Getting Started

1. Install Go 1.24 or later
2. Run `go mod tidy` to install dependencies
3. Run `make build-node-deps` from `~/synd-withdrawals/synd-enclave/nitro`
   - you may be missing the following tools:
      - `wat2wasm` -> can be installed via `brew install wabt`
      - `cbindgen` -> `brew install cbindgen` 
4. Build and run the service:
   ```sh
   go run ./cmd/synd-proposer
   ```

## Troubleshooting

### Import Errors

If you encounter import errors related to undefined types in `github.com/offchainlabs/nitro/arbos/util`, such as:
- `undefined: pgen.ArbRetryableTxRedeemScheduled`
- `undefined: pgen.ArbSysL2ToL1Transaction`

This means the precompile Go bindings haven't been generated. Normally, `make build-node-deps` or `make contracts` from the nitro directory would handle this automatically. However, these targets currently fail due to a Foundry linting error on `contracts/yul/Reader4844.yul` when using nightly builds.

**Workaround:** Manually build the precompile contracts (skipping Yul files) and regenerate the Go bindings:

1. Build precompile contracts (critical step):
   ```bash
   cd ../synd-enclave/nitro/contracts-local
   make build-forge-sol
   ```

2. Build legacy contracts:
   ```bash
   cd ../contracts-legacy
   yarn build
   ```

3. Generate Go bindings:
   ```bash
   cd ..
   go run solgen/gen.go
   ```

4. Return to synd-proposer and verify build:
   ```bash
   cd ../../synd-proposer
   go build ./...
   ```

**One-liner from synd-proposer directory:**
```bash
cd ../synd-enclave/nitro/contracts-local && make build-forge-sol && cd ../contracts-legacy && yarn build && cd .. && mkdir -p solgen/go/ && go run solgen/gen.go && cd ../../synd-proposer && go build ./...
```

**Note:** These bindings are auto-generated from Solidity contracts and are not checked into version control. The workaround bypasses the failing Yul build by directly building only Solidity files (`make build-forge-sol`), which includes the precompile contracts in `contracts-local/src/precompiles/` needed to generate the required ABIs.
