# Run an Ingestor

A Syndicate Ingestor is an optional service you can run to optimize your Syndicate Appchain RPC nodes. Ingestors speed up the syncing process by ingesting and storing compressed Sequencing and Settlement chain history and exposing that data via a websocket to one or more Syndicate Appchain RPC nodes.

This guide explains how to run ingestor(s) for your Syndicate Appchain RPC node in Docker.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run an ingestor.

- CPU: 1 core
- Memory: 256 MB
- Storage: 10 GB

> The disk usage grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull the `ghcr.io/syndicateprotocol/syndicate-appchains/synd-chain-ingestor` private image. Contact Syndicate for access instructions.

## Docker Compose

Run your ingestors in Docker, using the provided `./docker/ingestor-docker-compose.yml` file. Replace all of the templated values. Some of the values come from Syndicate, others will be your own values.

> **Note**: The container images run as non-root users. Ensure that any mounted directories have appropriate permissions for the container user to read/write.

After running that Docker Compose file, the INFO-level container logs will track the ingesting progress and indicate when the websocket is ready to use. You can also confirm the ingestor is ready for use by testing the websocket:

```bash
websocat -1 ws://your-ingestor:your-ingestor-port '{"jsonrpc":"2.0","id":1,"method":"eth_blockNumber","params":[]}'
```

## Configuring RPC Node(s)

Now that your ingestors are running, use the following config values for your Syndicate Appchain RPC Node(s):

```yaml
SETTLEMENT_WS_URL: "ws://your-settlement-ingestor:your-ingestor-port"
SEQUENCING_WS_URL: "ws://your-sequencing-ingestor:your-ingestor-port"
```

## Liveness and Monitoring

The ingestor container runs two independent HTTP servers:

- One on the main port (default `8545`) for JSON-RPC + `health`/`ready` endpoints
- One on the metrics port (default `8546`) for Prometheus metrics exposed on `/metrics` endpoint
