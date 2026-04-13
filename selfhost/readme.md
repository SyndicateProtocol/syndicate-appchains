# Self-Hosting a Syndicate Appchain Node

Run your own Syndicate appchain RPC node with Docker Compose.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) with the Compose plugin (v2.20+)
- `curl` and `tar` (for snapshot download)
- Disk space: 1 TB+ recommended
- WebSocket RPC access to your sequencing chain and settlement chain (e.g. via Alchemy, Infura, or a self-hosted node)

## Setup

### 1. Copy the env template

```bash
cp .env.example .env
```

### 2. Fill in `.env`

Open `.env` and supply your chain-specific values. The fields are grouped and commented. Required fields are:

| Field | Description |
|-------|-------------|
| `APPCHAIN_CHAIN_ID` | Your appchain's chain ID |
| `SEQUENCING_INGESTOR_WS_URLS` | WebSocket RPC URL(s) for the sequencing chain |
| `SEQUENCING_INGESTOR_START_BLOCK` | Block to start ingesting from on the sequencing chain |
| `SETTLEMENT_INGESTOR_WS_URLS` | WebSocket RPC URL(s) for the settlement chain |
| `SETTLEMENT_INGESTOR_START_BLOCK` | Block to start ingesting from on the settlement chain |
| `SEQUENCING_CONTRACT_ADDRESS` | Address of the sequencing chain inbox contract |
| `ARBITRUM_BRIDGE_ADDRESS` | Arbitrum bridge contract address on the sequencing chain |
| `ARBITRUM_INBOX_ADDRESS` | Arbitrum inbox contract address on the sequencing chain |
| `BATCHER_PRIVATE_KEY` | Private key for the wallet that submits transaction batches |
| `SEQUENCING_RPC_URLS` | HTTP RPC URL(s) for the sequencing chain |
| `CHAIN_RPC_URLS` | JSON map of chain ID → HTTP RPC URL used by maestro (e.g. `{"888991":"https://..."}`) |
| `NITRO_CHAIN_INFO__JSON` | Chain info JSON blob for the Nitro node |
| `GENESIS_CONFIG` | EVM genesis config JSON for the appchain |
| `MCHAIN_SNAPSHOT_URL` | URL to a `.tar` or `.tar.gz` snapshot of the mchain data directory _(optional but recommended)_ |
| `NITRO_SNAPSHOT_URL` | URL to a `.tar` or `.tar.gz` snapshot of the Nitro data directory _(optional but recommended)_ |

At least one snapshot URL (`MCHAIN_SNAPSHOT_URL` or `NITRO_SNAPSHOT_URL`) is strongly recommended — syncing from genesis can take many hours. Both are skipped automatically if the target directory already contains data.

Fields in the **Migration** and **Proposer** sections are only required if your appchain uses those features.

> All values for your specific appchain can be provided by the Syndicate team.

### 3. Start

```bash
bash start.sh
```

The script will:
1. Create local data directories under `DATA_DIR` (default: `./data`)
2. Download and extract the mchain snapshot (if `SNAPSHOT_URL` is set and the data directory is empty)
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
├── nitro/        # Nitro node state (~/.arbitrum)
├── settlement/   # filesystem cache for the settlement ingestor
├── sequencing/   # filesystem cache for the sequencing ingestor
└── valkey/       # Valkey (Redis-compatible) persistence
```

To change the location, set `DATA_DIR` in your `.env` before running `start.sh`.
