# Self-Hosting a Syndicate Appchain Node

Run your own Syndicate appchain RPC node with Docker Compose.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with the Compose plugin (v2.20+)
- `curl` and `tar` (for snapshot download)
- Disk space: 1 TB+ recommended
- WebSocket RPC access to your sequencing chain and settlement chain (e.g. via Alchemy, Infura, or a self-hosted node)
- For synd-TEE withdrawals to be functional you'll need to set up a synd-enclave on a AWS nitro TEE instance and provide that URL as the `ENCLAVE_RPC_URL`. (a guide on how to set up the TEE enclave can be found [here](https://docs.syndicate.io/en/docs/syndicate-stack/guides/run-withdrawals-infra#run-synd-enclave-in-aws-tee)

## Setup


### 1. Copy the env template

```bash
cp .env.example .env
```

### 2. Fill in `.env`

Open `.env` and supply your chain-specific values.
All initial values for your specific appchain can be provided by the Syndicate team.


> [!WARNING]
>`BATCHER_PRIVATE_KEY` and `PROPOSER_PRIVATE_KEY` must to have funds on the sequencing / settlement chains respectively. Aditionally, the BATCHER must be authorized to sequence on the sequencing contract.

To change the location where data is persisted, set `DATA_DIR` in your `.env` before running `start.sh`.

### 3. Start

```bash
bash start.sh
```

The script will:
1. Create local data directories under `DATA_DIR` (default: `./data`)
2. Download and extract the nitro and/or mchain snapshot if specified
3. Start all services with `docker compose`

## Verify

Check that all containers are running:

```bash
docker compose ps
```

Wait for the mchain and nitro to finish syncing, then test the RPC:

```bash
curl -s -X POST http://localhost:8545 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

## Common commands

```bash
# Follow logs for all services
docker compose logs -f

# Follow logs for a specific service
docker compose logs -f nitro

# Stop all services
docker compose down

# Stop and remove all data volumes
docker compose down -v

# Restart a single service
docker compose restart mchain
```

## Data directory layout

```
./data/
├── mchain/       # RocksDB state for the intermediate chain node
├── nitro/        # Nitro node state
├── sequencing/   # filesystem cache for the sequencing ingestor
├── settlement/   # filesystem cache for the settlement ingestor
└── valkey/       # Valkey (Redis-compatible) persistence
```

## Summary

At this point you should have a function synd-stack rollup deriving state from the parent chains.
You can assert that the rollup node is synced by checking the `eth_blockNumber` rpc call result.
You should also be able to send new transactions by calling `eth_sendRawTransaction` on the rollup node.
Withdrawals should also be functional, you can assert this by monitoring the `TEEModule` contract for `assertionPosted` and `closeChallengeWindow` events ([example](https://basescan.org/address/0xA61C573986bf21D1B93010c8D50909a6c313Dd61#events))

