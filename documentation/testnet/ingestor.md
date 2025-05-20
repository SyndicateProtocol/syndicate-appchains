# Run an Ingestor

A Syndicate "Ingestor" is a necessary component for running Syndicate Appchain RPC nodes. They optimize the syncing process by quickly ingesting and storing the Sequencing and Settlement chain history, and making that compressed data available to the RPC nodes via a websocket. This guide explains how to run ingestor(s) for your Syndicate Appchain RPC node.

## Hardware Requirements

Below are the _minimum_ hardware requirements to run a readonly RPC node.

CPU/Memory:

- CPU: 500m
- Memory: 256Mi

Storage:

- 10Gi

> This data grows over time depending on chain activity, so consider enabling dynamic resizing if available.

Other:

- Docker version 28+
- Access to pull `ingestor` image from `ghcr.io/syndicateprotocol/syndicate-appchains/` (private) registry


## Docker Compose

Run your ingestors in docker, using the provided `./docker/ingestor-docker-compose.yml` file. Replace all of the templated values. Some of the values come from Syndicate, others will be your own values.

After running that docker compose file, the container logs will track the ingesting progress and indicate when the websocket is ready to use.