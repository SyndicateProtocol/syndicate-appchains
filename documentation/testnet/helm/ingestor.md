# Run an Ingestor

A Syndicate Ingestor is an optional service you can run to optimize your Syndicate Appchain RPC nodes. Ingestors speed up the syncing process by ingesting and storing compressed Sequencing and Settlement chain history and exposing that data via a websocket to one or more Syndicate Appchain RPC nodes.

This guide explains how to run ingestor(s) for your Syndicate Appchain RPC node in Kubernetes via Helm.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run an ingestor.

- CPU: 1 core
- Memory: 256 MB
- Storage: 10 GB

> The disk usage grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Kubernetes cluster (version 1.25+) and Helm
- Access to pull the `ghcr.io/syndicateprotocol/syndicate-appchains/synd-chain-ingestor` private image. Contact Syndicate for access instructions.
- Access to pull private Syndicate Helm charts. Contact Syndicate for access instructions.
  - repoURL: `https://github.com/SyndicateProtocol/helm`
  - path: `charts/ingestor`
  - targetRevision: `HEAD`

## Helm

Run your ingestors in Kubernetes, using Helm and the appropriate [values](https://github.com/SyndicateProtocol/helm/blob/main/charts/apps/values/testnet/values.yaml) for your chain.

> **Note**: The container images run as non-root users. Ensure that any mounted directories have appropriate permissions for the container user to read/write.

Confirm the ingestor is ready for use by testing the websocket:

```bash
websocat -1 ws://your-ingestor:your-ingestor-port '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}'
```

## Configuring RPC Node(s)

Now that your ingestors are running, use the following config values for your Syndicate Appchain RPC Node(s):

```yaml
SETTLEMENT_WS_URL: "wss://your-settlement-ingestor"
SEQUENCING_WS_URL: "wss://your-sequencing-ingestor"
```
