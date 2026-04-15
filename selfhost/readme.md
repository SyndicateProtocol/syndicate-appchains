# Self-Hosting a Syndicate Appchain Node

Run your own Syndicate appchain RPC node with Docker Compose.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with the Compose plugin (v2.20+)
- `curl` and `tar` (for snapshot download)
- Disk space: 1 TB+ recommended
- RAM: 16 GB+ recommended
- CPU: 4+ cores recommended
- Ports `8545` (HTTP RPC) and `8548` (WebSocket RPC) available on the host
- WebSocket RPC access to your sequencing chain and settlement chain (e.g. via Alchemy, Infura, or a self-hosted node)

> [!NOTE]
> For synd-TEE withdrawals to be functional, you'll need to set up a synd-enclave on an AWS Nitro TEE instance and provide that URL as `ENCLAVE_RPC_URL`. See the [TEE enclave setup guide](https://docs.syndicate.io/en/docs/syndicate-stack/guides/run-withdrawals-infra#run-synd-enclave-in-aws-tee) for instructions.

## Setup


### 1. Copy the env template

```bash
cp .env.example .env
```

### 2. Fill in `.env`

Open `.env` and supply your chain-specific values.
All initial values for your specific appchain can be provided by the Syndicate team.


> [!WARNING]
> `BATCHER_PRIVATE_KEY` and `PROPOSER_PRIVATE_KEY` must have funds on the sequencing / settlement chains respectively. Additionally, the BATCHER must be authorized to sequence on the sequencing contract.

To change the location where data is persisted, set `DATA_DIR` in your `.env` before running `start.sh`.

### 3. Start

```bash
bash start.sh
```

The script will:
1. Create local data directories under `DATA_DIR` (default: `./data`)
2. Download and extract the nitro snapshot if `NITRO_SNAPSHOT_URL` is set
3. Start all services with `docker compose` (the mchain container handles its own snapshot download via `MCHAIN_SNAPSHOT_URL` if set)

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

WebSocket RPC is also available at `ws://localhost:8548`.

## Common commands

```bash
# Follow logs for all services
docker compose logs -f

# Follow logs for a specific service
docker compose logs -f nitro

# Stop all services
docker compose down

# Stop and remove all persisted data (bind-mounted)
docker compose down && rm -rf ${DATA_DIR:-./data}

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

At this point you should have a functional synd-stack rollup deriving state from the parent chains.
You can assert that the rollup node is synced by checking the `eth_blockNumber` rpc call result.
You should also be able to send new transactions by calling `eth_sendRawTransaction` on the rollup node.
Withdrawals should also be functional, you can assert this by monitoring the `TEEModule` contract for `assertionPosted` and `closeChallengeWindow` events ([example](https://basescan.org/address/0xA61C573986bf21D1B93010c8D50909a6c313Dd61#events))

