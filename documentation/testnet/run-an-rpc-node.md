# Run a Syndicate Appchain RPC Node

A Syndicate Appchain RPC Node is comprised of 3 components: `translator`, `mchain`, and [`nitro`](https://github.com/OffchainLabs/nitro). This guide explains how to run them together in docker to operate your own RPC node for a Syndicate appchain.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run a readonly RPC node.

CPU/Memory:

- 4 vCPUs
- 16GB memory

Storage:

- translator data disk: 5GB
- nitro data disk: 50GB

> This data grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull `translator` and `mchain` images from `ghcr.io/syndicateprotocol/syndicate-appchains/` (private) registry
- Access to Syndicate's sequencing json-rpc API, in order to forward `eth_sendRawTransaction` requests there
- Ingestors running for the Settlement and Sequencing chains, see docs for running your own ingestors here

## Chain Config

Obtain the appchain's `chain_info` config from Syndicate. It should look something like the provided `./config/chain_info.json.example` file, but with chain-specific values:

This chain config file will be mounted into the `nitro` container in the next step.

## Docker Compose

Run all of the components together in docker, using the provided `./docker/rpc-node-docker-compose.yml` file. Replace all of the templated values. Some of the values come from the chain config above, some will be your own values, and the rest should come from Syndicate.

After running that docker compose file, use the `slotter_last_processed_block` metric (with `settlement` and `sequencing` tags) from the `translator` container to monitor the node's syncing progress. If that number is increasing over time, then the node is successfully syncing. Once it syncs up to the settlement/sequencing chain heads, you can begin sending transactions to this RPC node.