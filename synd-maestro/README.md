# `Synd-Maestro`

The `synd-maestro` service is the transaction ingestion and ordering component of a high-performance Ethereum transaction sequencing system. It is designed to work as a transaction mempool and make transaction submission more efficient. This service is complemented by a consumer component, such as the `synd-batch-sequencer`.

## Features
The `synd-maestro` service receives Appchain `eth_sendRawTransaction` requests, validates them, then enqueues them to a Redis stream for consumption by other components like the `synd-batch-sequencer`. 

It is also capable of receiving transactions out of order and caching them for a short time, until it receives a transaction which fills this nonce gap. 

## Requirements
`Synd-maestro` requires a persistent connection to a Redis instance, as well as functional RPC endpoints for all chains on which it will receive transactions. Transactions for a given chain will not be processed if that chain's RPC is unavailable. 

See `src/config.rs` for more details.

## Testing 
`Synd-maestro` supports the `cargo-nextest` [next-gen test runner](https://nexte.st/). 
To use, 
1. Install `nextest` on your machine via 

`cargo install cargo-nextest --locked` 

or other methods [here](https://nexte.st/docs/installation/).

2. From `/syndicate-appchains` root, run 

`cargo nextest run -p synd-maestro`