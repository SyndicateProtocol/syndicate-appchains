# Run a Testnet Syndicate Appchain RPC Node

A Syndicate Appchain RPC Node consists of 3 components: `synd-translator`, `synd-mchain`, and Arbitrum [`nitro`](https://github.com/OffchainLabs/nitro). This guide explains how to run them together in Kubernetes (via Helm) to operate your own RPC node for a Syndicate Appchain.

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

- Kubernetes cluster (version 1.25+) and Helm
- Access to pull private Syndicate Helm charts. Contact Syndicate for access instructions.
  - repoURL: `https://github.com/SyndicateProtocol/helm`
  - path: `charts/rpc`
  - targetRevision: `HEAD`
- Access to Syndicate's sequencing JSON-RPC API, in order to forward `eth_sendRawTransaction` requests there
- Websocket URLs for the Settlement and Sequencing chains
  - For faster sync times, consider running your own Settlement and Sequencing Ingestors (see the `ingestor.md` doc for more info)
  - Need to support `getBlockByNumber`, `getReceiptsByBlock`, `get_logs`, and `eth_subscribe`

## Chain Config

Obtain the Appchain's `chain_info` config from Syndicate. It should look something like the provided `../config/chain_info.json.example` file, but with chain-specific values.

This chain config file will injected into the `nitro` Helm chart in the next step.

## Helm

Run all of the components together via Helm, using the provided values files. Replace all of the templated values. Some of the values come from the chain config above, some will be your own values, and the rest should come from Syndicate.

Once the Kubernetes resources are running, use the `slotter_last_processed_block` metric (with `settlement` and `sequencing` tags) from the `synd-translator` container to monitor the node's syncing progress. If that number is increasing over time, then the node is successfully syncing. Once it syncs up to the settlement/sequencing chain heads, you can begin sending transactions to this RPC node.
