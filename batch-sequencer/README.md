# Batch Sequencer

The Batch Sequencer is a service that reads transactions from Redis, bundles them into size-limited batches, and submits them directly to a sequencing contract on an EVM-compatible chain using Alloy or to TC.

Sample cmd:

```
cargo run -p batch-sequencer -- \
  --redis-url redis://127.0.0.1:6379 \
  --chain-id 11155111 \
  --max-batch-size 90KB \
  --batcher-polling-interval 100ms \
  --private-key <hex-private-key> \
  --wallet-pool-address 0x1111... \
  --sequencing-contract-address 0x2222... \
  --sequencing-rpc-url http://localhost:8545
```
