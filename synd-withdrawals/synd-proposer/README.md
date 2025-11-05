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

If you encounter import errors related to missing packages like `github.com/offchainlabs/nitro/solgen/go/*`, you need to regenerate the Go bindings from the Nitro contracts:

1. Build legacy contracts:
   ```bash
   cd ../synd-enclave/nitro/contracts-legacy
   yarn build
   ```

2. Generate Go bindings:
   ```bash
   cd ..
   go run solgen/gen.go
   ```

3. Update Go modules:
   ```bash
   cd ../../synd-proposer
   go mod tidy
   ```

**One-liner from synd-proposer directory:**
```bash
cd ../synd-enclave/nitro/contracts-legacy && yarn build && cd .. && mkdir -p solgen/go/ && go run solgen/gen.go && cd ../../synd-proposer && go mod tidy
```

**Note:** These bindings are auto-generated from Solidity contracts and are not checked into version control.
