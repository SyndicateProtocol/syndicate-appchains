
# synd-chain-ingestor

A high-performance blockchain data ingestion service that subscribes to Ethereum-compatible blockchains via WebSocket connections and provides real-time block and log data to connected appchains.

## Overview

The `synd-chain-ingestor` serves as a critical infrastructure component that:

- **Ingests blockchain data** from Ethereum-compatible chains using WebSocket subscriptions
- **Provides real-time block streaming** to multiple appchain subscribers
- **Handles blockchain reorganizations** automatically with proper rollback mechanisms
- **Persists block data** to disk using an efficient append-only file format
- **Exposes metrics** for monitoring ingestion performance and blockchain state
- **Offers filtered subscriptions** allowing clients to subscribe to specific contract addresses

## Architecture

The service consists of several key components:

- **`EthClient`**: WebSocket client for connecting to Ethereum RPC endpoints with automatic reconnection
- **Ingestor**: Core ingestion logic that handles block subscriptions and reorganization detection
- **Database**: Append-only file-based storage for efficient block data persistence
- **Server**: JSON-RPC server providing subscription endpoints for appchain clients
- **Metrics**: Prometheus metrics for monitoring block ingestion and chain health

## Features

- **Real-time block ingestion** via WebSocket subscriptions instead of polling
- **Automatic reorganization handling** with metrics tracking reorg depth
- **Filtered log subscriptions** - clients can subscribe to specific contract addresses
- **Parallel block synchronization** during initial sync with configurable request limits
- **Fault tolerance** with automatic reconnection and error recovery
- **Prometheus metrics** including block numbers, timestamps, delays, and reorg counts
- **Health monitoring** with built-in health check endpoints

## Configuration

The service is configured via command-line arguments or environment variables.

See `config.rs` for details.

## Usage

### Sample Command

```bash
cargo run -p synd-chain-ingestor -- \
  --ws-url ws://localhost:8546 \
  --db-file ./chain-data.db \
  --start-block 1000000 \
  --port 8545 \
  --metrics-port 8546 \
  --channel-size 1024 \
  --parallel-sync-requests 190 \
  --request-timeout 10s
```

## JSON-RPC Methods

The service exposes the following WebSocket JSON-RPC methods:

### `subscribe_blocks`

Subscribe to receive filtered block data for specific contract addresses.

**Parameters:**
- `start_block` (number): Block number to start streaming from
- `addresses` (array): Array of contract addresses to filter logs for

**Returns:** Subscription ID

### `block`

Get a specific block by number.

**Parameters:**
- `block_number` (number): Block number to retrieve

**Returns:** Block data or null if not available

### `eth_blockNumber`

Get the latest ingested block number.

**Returns:** Latest block number

### `url`

Get the upstream WebSocket URL.

**Returns:** WebSocket URL string

## Database Format

The service uses a custom append-only file format:

- **Header**: Version (1 byte) + Start Block (8 bytes) + Chain ID (8 bytes) + Reserved (19 bytes)
- **Block Items**: Timestamp (4 bytes) + Block Hash (32 bytes) per block

This format provides efficient storage and fast random access while supporting automatic reorg handling through file truncation.

## Monitoring

Prometheus metrics are available at `http://localhost:{{PORT}}/metrics`:

- `block_number`: Latest ingested block number
- `block_timestamp`: Latest block timestamp
- `block_delay`: Delay between block production and ingestion
- `reorg_blocks`: Total number of blocks affected by reorgs

## Error Handling

- **Connection Failures**: Automatic reconnection to upstream WebSocket with exponential backoff
- **Reorgs**: Automatic detection and recovery by comparing block hashes
- **Timeouts**: Configurable timeouts with retry logic for all RPC requests
- **Database Corruption**: Automatic truncation of corrupted database entries

## Notes

- The service maintains consistency by validating block hashes against receipts before committing
- Initial sync uses parallel block fetching for improved performance
- Log queries automatically use binary search when encountering rate limits
- The database file is locked exclusively to prevent concurrent access
- Clients can subscribe to receive only logs from contracts they're interested in, reducing bandwidth usage
