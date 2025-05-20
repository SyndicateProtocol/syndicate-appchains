# Run a Syndicate Appchain RPC Node Locally

Use the provided config files and docker-compose file to build and run an Appchain RPC node locally.

## Appchain Config

There are several example chain configs in the provided `./config` directory.

## Run in Docker

Run all of the components together in docker, using the provided `./docker-compose.yml` file. Replace all of the templated values.

```sh
docker compose build
docker compose --env-file config/[burrata/manchego/cheddar].env up
```

## Clean up

Run the following to stop and clean up docker compose resources.

```sh
docker compose down -v
```
