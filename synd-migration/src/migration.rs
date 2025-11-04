//! Module for migrating the `DataAvailabilityCommittee` flag in Nitro chain configs.

use alloy::{
    primitives::B256,
    rlp::{Decodable, RlpDecodable},
};
use eyre::{Context, Result};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

/// Arbitrum-specific chain configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ArbitrumChainParams {
    /// Whether arbOS is enabled
    #[serde(default)]
    pub enable_arb_os: bool,

    /// Allow debug precompiles
    #[serde(default)]
    pub allow_debug_precompiles: bool,

    /// Data Availability Committee flag - this is what we're migrating
    pub data_availability_committee: bool,

    /// Initial arbOS version
    #[serde(default)]
    pub initial_arb_os_version: u64,

    /// Initial chain owner address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_chain_owner: Option<String>,

    /// Genesis block number
    #[serde(default)]
    pub genesis_block_num: u64,

    /// Maximum code size
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_code_size: Option<u64>,

    /// Maximum init code size
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_init_code_size: Option<u64>,

    /// Syndicate flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syndicate: Option<bool>,

    /// eigenDA flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eigen_da: Option<bool>,
}

/// Ethereum chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    /// Chain ID
    pub chain_id: u64,

    /// Homestead block
    #[serde(default)]
    pub homestead_block: u64,

    /// EIP150 block
    #[serde(default)]
    pub eip150_block: u64,

    /// EIP155 block
    #[serde(default)]
    pub eip155_block: u64,

    /// EIP158 block
    #[serde(default)]
    pub eip158_block: u64,

    /// Byzantium block
    #[serde(default)]
    pub byzantium_block: u64,

    /// Constantinople block
    #[serde(default)]
    pub constantinople_block: u64,

    /// Petersburg block
    #[serde(default)]
    pub petersburg_block: u64,

    /// Istanbul block
    #[serde(default)]
    pub istanbul_block: u64,

    /// Berlin block
    #[serde(default)]
    pub berlin_block: u64,

    /// London block
    #[serde(default)]
    pub london_block: u64,

    /// Arbitrum-specific parameters
    #[serde(default)]
    pub arbitrum: ArbitrumChainParams,
}

/// Rollup state information
#[derive(Debug, Clone, Default)]
pub struct RollupState {
    pub block_number: u64,
    pub block_hash: B256,
    pub batch_count: u64,
    pub before_batch_acc: B256,
    pub batch_acc: B256,
    pub parent_chain_block: u64,
    pub delayed_msgs_count: u64,
    pub delayed_msgs_acc: B256,
}

/// Migration command - it inspects a given Nitro database, extracts relevant information and sets
/// `DataAvailabilityCommittee` to false in the chain config.
#[allow(clippy::unwrap_used, clippy::cognitive_complexity)]
pub async fn get_migration_data(nitro_db_path: &Path) -> Result<RollupState> {
    info!("Nitro DB path: {:?}", nitro_db_path);
    let chaindata_path = nitro_db_path.join("l2chaindata");
    if !chaindata_path.exists() {
        eyre::bail!("L2 chaindata path does not exist: {:?}. Make sure you're pointing to the Nitro database directory (parent of l2chaindata)", chaindata_path);
    }

    // Open the database with read-write access if we're modifying, read-only otherwise
    let mut opts = Options::default();
    opts.create_if_missing(false);
    let db =
        DB::open_for_read_only(&opts, &chaindata_path, false).context("Failed to open database")?;

    // Also open the arbitrumdata database which contains Arbitrum-specific state
    let arb_db_path = nitro_db_path.join("arbitrumdata");
    let arb_db = DB::open_for_read_only(&opts, &arb_db_path, false).unwrap();

    // Get the rollup state
    let rollup_state = get_rollup_state(&db, &arb_db)?;

    // Get the chain config
    let (mut chain_config, config_key) = get_chain_config(&db)?;

    debug!("rollup state: {:#?}", rollup_state);
    debug!("chain config: {:#?}", chain_config);

    println!("\n---------------\n");
    println!("MIGRATED_BEFORE_BATCH_ACC {}", rollup_state.before_batch_acc);
    println!("MIGRATED_BATCH_ACC: {}", rollup_state.batch_acc);
    println!("MIGRATED_BATCH_COUNT: {}", rollup_state.batch_count);
    println!("MIGRATED_DELAYED_MSGS_ACC: {}", rollup_state.delayed_msgs_acc);
    println!("MIGRATED_DELAYED_MSGS_COUNT: {}", rollup_state.delayed_msgs_count);
    println!("MIGRATED_APPCHAIN_BLOCK_HASH: {}", rollup_state.block_hash);
    println!("SETTLEMENT_START_BLOCK: {}", rollup_state.parent_chain_block);
    println!("\n---------------\n");

    Ok(rollup_state)
}

/// Retrieves the chain config from the database.
/// `DBkeys` used can be found in <https://github.com/SyndicateProtocol/go-ethereum/blob/HEAD/core/rawdb/schema.go>
///
/// Returns the chain config, and the database key used.
#[allow(clippy::unwrap_used)]
fn get_chain_config(db: &DB) -> Result<(ChainConfig, Vec<u8>)> {
    // headerHashKey = headerPrefix + num (uint64 big endian) + headerHashSuffix
    let genesis_hash = db
        .get(make_numbered_key(b"h", 0, b"n"))
        .unwrap()
        .map(|bytes| B256::from_slice(&bytes[..32]))
        .unwrap();

    let config_key = make_key(b"ethereum-config-", genesis_hash.as_ref(), &[]);
    let chain_config: ChainConfig = db
        .get(config_key.clone())
        .unwrap()
        .map(|bytes| serde_json::from_slice(&bytes).unwrap())
        .unwrap();

    Ok((chain_config, config_key))
}

/// Updates the chain config in the database.
fn update_chain_config(db: &DB, config: &ChainConfig, key: &[u8]) -> Result<()> {
    let encoded = serde_json::to_vec(config).context("Failed to serialize chain config")?;
    db.put(key, encoded).context("Failed to write chain config to database")?;
    db.flush().context("Failed to flush database")?;
    Ok(())
}

fn make_numbered_key(prefix: &[u8], number: u64, suffix: &[u8]) -> Vec<u8> {
    let mut key = prefix.to_vec();
    key.extend_from_slice(&number.to_be_bytes());
    key.extend_from_slice(suffix);
    key
}

fn make_key(prefix: &[u8], middle: &[u8], suffix: &[u8]) -> Vec<u8> {
    let mut key = prefix.to_vec();
    key.extend_from_slice(middle);
    key.extend_from_slice(suffix);
    key
}

#[derive(Debug, Clone, RlpDecodable)]
#[allow(dead_code)]
struct BatchMetadata {
    acc: B256,
    message_count: u64,
    delayed_message_count: u64,
    parent_chain_block: u64,
}

/// Retrieves rollup state information from the database
/// `DBkeys` used can be found in <https://github.com/SyndicateProtocol/go-ethereum/blob/HEAD/core/rawdb/schema.go>
#[allow(clippy::unwrap_used)]
fn get_rollup_state(db: &DB, arb_db: &DB) -> Result<RollupState> {
    let block_hash = db.get(b"LastBlock").unwrap().map(|bytes| B256::from_slice(&bytes)).unwrap();

    let mut block_number_key = [0u8; 33];
    block_number_key[0] = b'H';
    block_number_key[1..].copy_from_slice(block_hash.as_ref());

    let block_number = db
        .get(block_number_key)?
        .map(|bytes| u64::from_be_bytes(bytes[..8].try_into().unwrap()))
        .ok_or_else(|| eyre::eyre!("Failed to get block number"))?;

    let batch_count_bytes = arb_db
        .get(b"_sequencerBatchCount")?
        .ok_or_else(|| eyre::eyre!("Failed to get batch count"))?;

    let batch_count = u64::decode(&mut &batch_count_bytes[..])
        .map_err(|e| eyre::eyre!("Failed to decode batch count: {e}"))?;

    let delayed_msgs_count = arb_db
        .get(b"_delayedMessageCount")?
        .map(|bytes| u64::decode(&mut &bytes[..]).unwrap())
        .ok_or_else(|| eyre::eyre!("Failed to get delayed message count"))?;

    let batch_data = arb_db
        .get(make_numbered_key(b"s", batch_count - 1, &[]))?
        .map(|bytes| BatchMetadata::decode(&mut &bytes[..]).unwrap())
        .ok_or_else(|| eyre::eyre!("Failed to get batch data"))?;

    let delayed_msgs_acc = arb_db
        .get(make_numbered_key(b"e", batch_data.delayed_message_count, &[]))?
        .map(|bytes| {
            println!("bytes: {}", alloy::hex::encode(&bytes));

            return B256::from_slice(&bytes[..32]);
        })
        .ok_or_else(|| eyre::eyre!("Failed to get delayed message accumulator"))?;

    let before_batch_acc = arb_db
        .get(make_numbered_key(b"s", batch_count - 2, &[]))?
        .map(|bytes| BatchMetadata::decode(&mut &bytes[..]).unwrap())
        .ok_or_else(|| eyre::eyre!("Failed to get batch data"))?
        .acc;

    Ok(RollupState {
        block_number,
        block_hash,
        batch_count,
        before_batch_acc,
        batch_acc: batch_data.acc,
        parent_chain_block: batch_data.parent_chain_block,
        delayed_msgs_count,
        delayed_msgs_acc,
    })
}
