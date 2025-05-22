# Run an Ingestor

A Syndicate "Ingestor" is an optional service you can run to optimize your Syndicate Appchain RPC nodes. Ingestors speed up the syncing process by ingesting and storing compressed Sequencing and Settlement chain history and exposing that data via a websocket to any number of Syndicate Appchain RPC nodes.

This guide explains how to run ingestor(s) for your Syndicate Appchain RPC node.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run a readonly RPC node.

- CPU: 0.5 cores
- Memory: 256 MB
- Storage: 10 GB

> The disk usage grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull `ingestor` image from `ghcr.io/syndicateprotocol/syndicate-appchains/` (private) registry

## Docker Compose

Run your ingestors in Docker, using the provided `./docker/ingestor-docker-compose.yml` file. Replace all of the templated values. Some of the values come from Syndicate, others will be your own values.

After running that Docker Compose file, the container logs will track the ingesting progress and indicate when the websocket is ready to use.

## Configuring the RPC Node

Now that your ingestors are running, use the following config values for the `translator` container in your Syndicate Appchain RPC Nodes:

```yaml
settlement_rpc_ws_url: "wss://your-settlement-ingestor"
sequencing_rpc_ws_url: "wss://your-sequencing-ingestor"
```
