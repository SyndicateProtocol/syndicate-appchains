# Run a Testnet Syndicate Appchain RPC Node

A Syndicate Appchain RPC Node consists of 3 components: `synd-translator`, `synd-mchain`, and Arbitrum [`nitro`](https://github.com/OffchainLabs/nitro). This guide explains how to run them together in Kubernetes (via Helm) to operate your own RPC node for a Syndicate Appchain.

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

- Kubernetes cluster (version 1.25+) and Helm
- Access to pull private Syndicate Docker images. Contact Syndicate for access instructions.
  - `ghcr.io/syndicateprotocol/syndicate-appchains/synd-mchain`
  - `ghcr.io/syndicateprotocol/syndicate-appchains/synd-translator`
  - `ghcr.io/syndicateprotocol/nitro/nitro`
- Access to pull private Syndicate Helm charts. Contact Syndicate for access instructions.
  - repoURL: `https://github.com/SyndicateProtocol/helm`
  - path: `charts/syndicate-rpc`
  - targetRevision: `HEAD`
- Access to Syndicate's sequencing JSON-RPC API, in order to forward `eth_sendRawTransaction` requests there
- Websocket URLs for the Settlement and Sequencing chains
  - For faster sync times, consider running your own Settlement and Sequencing Ingestors (see the `ingestor.md` doc for more info)
  - Need to support `getBlockByNumber`, `getReceiptsByBlock`, `get_logs`, and `eth_subscribe`

## Chain Config

Obtain the Appchain's `chain_info` config from Syndicate. It should look something like the provided `../config/chain_info.json.example` file, but with chain-specific values.

This chain config file will injected into the `nitro` Helm chart in the next step.

> **Note**: The container images run as non-root users. Ensure that any mounted directories have appropriate permissions for the container user to read/write.

## Helm

Run all of the components together via Helm, using the appropriate [values](https://github.com/SyndicateProtocol/helm/blob/main/charts/appchains/values/testnet/values.yaml) for your chain.

Confirm the RPC node is ready for use by testing the rpc endpoint:

```bash
curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["latest", true],"id":1}' http://<service>.<namespace>.svc.cluster.local:<port>/rpc
```
