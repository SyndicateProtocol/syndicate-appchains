# Run a Testnet Syndicate Appchain RPC Node

A Syndicate Appchain RPC Node consists of 3 components: `synd-translator`, `synd-mchain`, and Arbitrum [`nitro`](https://github.com/OffchainLabs/nitro). This guide explains how to run them together in Docker to operate your own RPC node for a Syndicate Appchain.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run an RPC node.

CPU/Memory:

- 8 vCPUs
- 16GB memory

Storage:

- `mchain` data disk: 5 GB
- `nitro` data disk: 50 GB

> This data grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull private Syndicate Docker images. Contact Syndicate for access instructions.
  - `ghcr.io/syndicateprotocol/syndicate-appchains/synd-mchain`
  - `ghcr.io/syndicateprotocol/syndicate-appchains/synd-translator`
  - `ghcr.io/syndicateprotocol/nitro/nitro`
- Access to Syndicate's sequencing JSON-RPC API, in order to forward `eth_sendRawTransaction` requests there
- Websocket URLs for the Settlement and Sequencing chains
  - For faster sync times, consider running your own Settlement and Sequencing Ingestors (see the `ingestor.md` doc for more info)
  - Need to support `getBlockByNumber`, `getReceiptsByBlock`, `get_logs`, and `eth_subscribe`

## Chain Config

Obtain the Appchain's `chain_info` config from Syndicate. It should look something like the provided `../config/chain_info.json.example` file, but with chain-specific values.

This chain config file will be mounted into the `nitro` container in the next step.

> **Note**: The container images run as non-root users. Ensure that any mounted directories have appropriate permissions for the container user to read/write.

## Docker Compose

Run all of the components together in Docker, using the provided `./rpc-node-docker-compose.yml` file. Replace all of the templated values. Up-to-date chain config values can be found in the [helm values overrides files](https://github.com/SyndicateProtocol/helm/blob/main/charts/appchains/values/testnet/values.yaml).

After running that Docker Compose file, use the `last_processed_block` metric (with `settlement` and `sequencing` tags) from the `synd-translator` container to monitor the node's syncing progress. If that number is increasing over time, then the node is successfully syncing. Once it syncs up to the settlement/sequencing chain heads, you can begin sending transactions to this RPC node.

## Liveness and Monitoring

Each service has different endpoint configurations:

### Synd-mchain

This service runs two independent HTTP servers:

- One on the main port (default `8545`) for JSON-RPC + `health`/`ready` endpoints
- One on the metrics port (default `8546`) for Prometheus metrics exposed on `/metrics` endpoint

### Synd-translator

This service runs a single HTTP server on port `9090` (default) that exposes all endpoints:

- `/metrics` - Prometheus metrics
- `/health` - Health check
- `/ready` - Readiness check

### Nitro

By default, nitro's http server is exposed on port 8547 and metrics on 6070. See their [docs](https://github.com/OffchainLabs/community-helm-charts/blob/main/charts/nitro/values.yaml) for more info.
