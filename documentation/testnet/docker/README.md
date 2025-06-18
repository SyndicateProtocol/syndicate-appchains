# Run a Testnet Syndicate Appchain RPC Node

A Syndicate Appchain RPC Node consists of 3 components: `synd-translator`, `synd-mchain`, and Arbitrum [`nitro`](https://github.com/OffchainLabs/nitro). This guide explains how to run them together in Docker to operate your own RPC node for a Syndicate Appchain.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run an RPC node.

CPU/Memory:

- 4 vCPUs
- 16GB memory

Storage:

- `mchain` data disk: 5 GB
- `nitro` data disk: 50 GB

> This data grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull private Syndicate Docker images. Contact Syndicate for access instructions.
  - `synd-mchain`
  - `synd-translator`
- Access to Syndicate's sequencing JSON-RPC API, in order to forward `eth_sendRawTransaction` requests there
- Websocket URLs for the Settlement and Sequencing chains
  - For faster sync times, consider running your own Settlement and Sequencing Ingestors (see the `ingestor.md` doc for more info)
  - Need to support `getBlockByNumber`, `getReceiptsByBlock`, `get_logs`, and `eth_subscribe`

## Chain Config

Obtain the Appchain's `chain_info` config from Syndicate. It should look something like the provided `../config/chain_info.json.example` file, but with chain-specific values.

This chain config file will be mounted into the `nitro` container in the next step.

## Docker Compose

Run all of the components together in Docker, using the provided `./rpc-node-docker-compose.yml` file. Replace all of the templated values. Some of the values come from the chain config above, some will be your own values, and the rest should come from Syndicate.

After running that Docker Compose file, use the `slotter_last_processed_block` metric (with `settlement` and `sequencing` tags) from the `synd-translator` container to monitor the node's syncing progress. If that number is increasing over time, then the node is successfully syncing. Once it syncs up to the settlement/sequencing chain heads, you can begin sending transactions to this RPC node.
