# Run a Syndicate Appchain RPC Node Locally

Use the provided config files and Docker Compose file to build and run a Syndicate Appchain RPC node locally.

## Appchain Config

The `./config` directory has example chain configs, but we store the most updated versions in [Notion](https://www.notion.so/syndicateprotocol/1b294475eae6801eb7a9e9320398a902?v=1b294475eae6804982f2000cd7f74d9d).

## Run in Docker

Run all of the components together in Docker, using the provided `./docker-compose.yml` file. Replace all of the templated values with values from Notion and GCP Secrets Manager.

> **Note**: The container images run as non-root users. Ensure that any mounted directories have appropriate permissions for the container user to read/write.

```sh
docker compose build
docker compose up
```

## Clean up

Run the following to stop and clean up Docker Compose resources.

```sh
docker compose down -v
```
