# synd-migration

CLI tool for migrating Syndicate appchain (Nitro) database configurations.

## Overview

`synd-migration` provides utilities to safely modify chain configuration parameters stored in a Nitro node's database. This is particularly useful for operational changes that require updating the on-chain configuration without redeploying the entire chain.

## Features

- **DataAvailabilityCommittee Migration**: Update the DAC flag to switch between AnyTrust and Rollup modes
- **Dry Run Mode**: Preview changes without modifying the database
- **Verification**: Automatically verifies configuration changes after applying them
- **Safe Operations**: Built-in checks to prevent common mistakes

## Installation

Build from the workspace root:

```bash
cargo build -p synd-migration --release
```

The binary will be available at `target/release/synd-migration`.

## Usage

### Migrate DataAvailabilityCommittee Flag

The `migrate-dac` command updates the `DataAvailabilityCommittee` flag in a Nitro node's chain configuration.

**⚠️ IMPORTANT: Always backup your chaindata directory before running migrations!**

#### Basic Usage

```bash
# Disable DAC (migrate to Rollup mode)
synd-migration migrate-dac \
  --chaindata-path /data/nitro/l2chaindata \
  --new-value false

# Enable DAC (migrate to AnyTrust mode)
synd-migration migrate-dac \
  --chaindata-path /data/nitro/l2chaindata \
  --new-value true
```

#### Dry Run (Recommended First Step)

Always test with `--dry-run` first to verify the tool can read your database and understand what changes will be made:

```bash
synd-migration migrate-dac \
  --chaindata-path /data/nitro/l2chaindata \
  --new-value false \
  --dry-run
```

#### Using Environment Variables

You can also specify the chaindata path via environment variable:

```bash
export CHAINDATA_PATH=/data/nitro/l2chaindata
synd-migration migrate-dac --new-value false
```

### Command Options

#### `migrate-dac`

| Option | Short | Environment Variable | Required | Description |
|--------|-------|---------------------|----------|-------------|
| `--chaindata-path` | `-d` | `CHAINDATA_PATH` | Yes | Path to the Nitro chaindata database directory |
| `--new-value` | `-v` | - | Yes | The new value for DataAvailabilityCommittee (true/false) |
| `--dry-run` | - | - | No | Preview changes without modifying the database |

## Migration Process

When migrating the DataAvailabilityCommittee flag, follow these steps:

### 1. Stop Your Nitro Node

```bash
# Stop the nitro node (method depends on your deployment)
systemctl stop nitro-node
# or
docker stop nitro-node
```

### 2. Backup Your Database

```bash
# Create a backup of the chaindata directory
cp -r /data/nitro/l2chaindata /data/nitro/l2chaindata.backup
```

### 3. Run Dry Run

```bash
synd-migration migrate-dac \
  -d /data/nitro/l2chaindata \
  -v false \
  --dry-run
```

Review the output to ensure the tool can read your configuration correctly.

### 4. Run the Migration

```bash
synd-migration migrate-dac \
  -d /data/nitro/l2chaindata \
  -v false
```

If successful, you'll see:

```
================================================================================
✓ MIGRATION COMPLETED SUCCESSFULLY
================================================================================

DataAvailabilityCommittee has been set to: false

Next steps:
  1. Update your node configuration to set:
     --node.data-availability.enable=false
  2. Restart your Nitro node
```

### 5. Update Node Configuration

Update your Nitro node configuration to match the new DAC setting:

**If disabling DAC (Rollup mode):**
```bash
# In your node config or command line args
--node.data-availability.enable=false
```

**If enabling DAC (AnyTrust mode):**
```bash
# In your node config or command line args
--node.data-availability.enable=true
# Plus any additional DAC-specific configuration
```

### 6. Restart Your Node

```bash
# Restart the nitro node
systemctl start nitro-node
# or
docker start nitro-node
```

### 7. Verify

Check the node logs to ensure it starts up correctly with the new configuration.

## Error Handling

The tool includes several safety checks:

- **Path validation**: Verifies the chaindata path exists before opening the database
- **Config validation**: Ensures the chain config contains Arbitrum parameters
- **No-op detection**: Warns if the value is already set to the target value
- **Verification**: Automatically verifies the change was applied correctly
- **Atomic updates**: Uses RocksDB's atomic write operations

If an error occurs, the tool will:
1. Display a clear error message
2. Exit with a non-zero status code
3. Leave the database unchanged (in case of read errors or validation failures)

## Technical Details

### Database Structure

Nitro stores chain configurations in its LevelDB/Pebble database with the key format:
- Key: `ethereum-config-<genesis_hash>`
- Value: JSON-encoded `ChainConfig` object

### Chain Config Format

The tool reads and writes chain configs in the Nitro JSON format:

```json
{
  "chainId": 510000,
  "homesteadBlock": 0,
  "eip150Block": 0,
  "eip155Block": 0,
  "eip158Block": 0,
  "byzantiumBlock": 0,
  "constantinopleBlock": 0,
  "petersburgBlock": 0,
  "istanbulBlock": 0,
  "berlinBlock": 0,
  "londonBlock": 0,
  "arbitrum": {
    "EnableArbOS": true,
    "AllowDebugPrecompiles": false,
    "DataAvailabilityCommittee": false,
    "InitialArbOSVersion": 32,
    "InitialChainOwner": "0x...",
    "GenesisBlockNum": 0,
    "Syndicate": true
  }
}
```

## Troubleshooting

### "No chain config found in database"

- Ensure the path points to the correct chaindata directory (should contain `.sst` files)
- Verify this is a Nitro/Geth-compatible database

### "Failed to open database"

- Check file permissions
- Ensure the Nitro node is stopped (database must not be in use)
- Verify the path exists and is accessible

### "Verification failed"

- This indicates the write succeeded but the value wasn't updated correctly
- Check database permissions and disk space
- Restore from backup and try again

## Development

### Running Tests

```bash
cargo test -p synd-migration
```

### Building for Release

```bash
cargo build -p synd-migration --release
```

The optimized binary will be at `target/release/synd-migration`.

## Safety and Best Practices

1. **Always backup** your chaindata before running migrations
2. **Always run with `--dry-run`** first to verify the tool works correctly
3. **Stop your node** before running the migration
4. **Test in a non-production environment** first if possible
5. **Keep your backup** until you've verified the node runs correctly with the new config

## Contributing

When adding new migration commands, follow these patterns:

1. Create a new module in `src/` (e.g., `src/migrate_foo.rs`)
2. Add the module to `src/lib.rs`
3. Define an `Args` struct using `clap::Args`
4. Implement the migration function with proper error handling
5. Add the command to the `Commands` enum in `src/main.rs`
6. Include verification steps to ensure the migration succeeded
7. Add tests and documentation

## License

See the workspace license.
