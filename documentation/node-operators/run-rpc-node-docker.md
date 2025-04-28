# Run a Readonly Metabased Testnet RPC Node

A Metabased RPC node is comprised of 3 components: translator, mchain, and nitro. This guide explains how to run them together in docker to operate your own readonly RPC node for a Metabased appchain (rollup).

## Hardware Requirements

Below are the _minimum_ hardware requirements to run a readonly RPC node.

CPU/Memory:

- 4 vCPUs
- 16GB memory

Two SSDs (we use GCP NAS right now):

- translator data disk: 5GB
- nitro data disk: 50GB

> Note: This data grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull images from `us-central1-docker.pkg.dev/metabased-testnet/images/metabased-rollup` or `ghcr.io/syndicateprotocol/metabased-rollup` registries
- Access to Metabased sequencer json-rpc API, in order to forward `eth_sendRawTransaction` requests there

## Chain Config

Obtain the appchain's config file from Syndicate. It should look similar to the example in the `infra/testnet/` directory, with chain-specific values. The chain config file will be mounted into the nitro container in the next step.

## Docker Compose

Run all of the components together in docker, using the docker-compose file in the `infra/testnet/` directory. Replace all of the templated values. Some of the values come from the chain config above, some will be your own values, and the rest should come from Syndicate. After running that docker compose file, use the `slotter_last_processed_block` metric (with `settlement` and `sequencing` tags) from translator to monitor the node's syncing progress. The default metrics port is `9090`.