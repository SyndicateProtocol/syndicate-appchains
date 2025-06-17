# `Synd-Proposer`

The Syndicate Proposer is responsible for extracting the appchain root state and submitting assertions to the settlement chain `AssertionPoster` contract.

## Structure

- `cmd/` - Main entry point for the service
- `pkg/` - Reusable packages
- `internal/` - Internal logic
- `tests/` - Tests

## Getting Started

1. Install Go 1.22 or later
2. Run `go mod tidy` to install dependencies
3. Build and run the service:
   ```sh
   go run ./cmd/synd-proposer
   ```
