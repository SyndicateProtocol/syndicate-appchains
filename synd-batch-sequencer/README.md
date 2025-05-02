# Batch Sequencer

The Batch Sequencer is a Rust-based service responsible for reading transactions from a Redis stream, bundling them into size-limited batches, and submitting them to a sequencing contract on an EVM-compatible chain. It supports submission via Alloy or to Transaction Chains (TC) directly, depending on configuration.

### Sample cmd:

```
cargo run -p synd-batch-sequencer -- \
  --redis-url redis://127.0.0.1:6379 \
  --chain-id 11155111 \
  --max-batch-size 90KB \
  --batcher-polling-interval 100ms \
  --private-key <hex-private-key> \
  --wallet-pool-address 0x1111... \
  --sequencing-contract-address 0x2222... \
  --sequencing-rpc-url http://localhost:8545
```

## Notes:

- `--private-key`: This is the private key of the wallet used to sign and submit batches. This wallet must be funded with enough native tokens (e.g., ETH) to cover gas costs.
  ⚠️ Important: Do not commit this key to version control. Use environment variables or secret managers to inject it at runtime securely.
