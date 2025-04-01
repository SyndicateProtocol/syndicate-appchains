# `Maestro`

The `maestro` service is a high-performance Ethereum transaction dispatcher for use across multiple wallets. It is designed to increase Transactions Per Second (TPS) throughput on Syndicate Network Appchains.

### Testing 
`Maestro` supports both the `cargo-nextest` [next-gen test runner](https://nexte.st/). 
To use, 
1. Install `nextest` on your machine via 

`cargo install cargo-nextest --locked` 

or other methods [here](https://nexte.st/docs/installation/).

2. From `/metabased-rollup` root, run 

`cargo nextest run -p maestro`