# Run a Syndicate Appchain RPC Node Locally

Use the provided config files and Docker Compose file to build and run a Syndicate Appchain RPC node locally.

## Appchain Config

There are several example chain configs in the provided `./config` directory.

## Run in Docker

Run all of the components together in Docker, using the provided `./docker-compose.yml` file. Replace all of the templated values.

```sh
docker compose build
docker compose --env-file config/[burrata/manchego/cheddar].env up
```

## Clean up

Run the following to stop and clean up Docker Compose resources.

```sh
docker compose down -v
```
