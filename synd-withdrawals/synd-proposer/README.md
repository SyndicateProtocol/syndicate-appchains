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
