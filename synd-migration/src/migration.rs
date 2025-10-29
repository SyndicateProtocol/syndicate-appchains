//! Module for migrating the DataAvailabilityCommittee flag in Nitro chain configs.

use alloy::{
    primitives::B256,
    rlp::{Decodable, RlpDecodable},
};
use eyre::{Context, Result};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::{io::Read, path::Path};
use tracing::{error, info, warn};

/// Arbitrum-specific chain configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ArbitrumChainParams {
    /// Whether ArbOS is enabled
    #[serde(default)]
    pub enable_arb_os: bool,

    /// Allow debug precompiles
    #[serde(default)]
    pub allow_debug_precompiles: bool,

    /// Data Availability Committee flag - this is what we're migrating
    pub data_availability_committee: bool,

    /// Initial ArbOS version
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

    /// EigenDA flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eigen_da: Option<bool>,
}

/// Ethereum chain configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arbitrum: Option<ArbitrumChainParams>,
}

/// Database key prefixes used by Nitro
const CHAIN_CONFIG_PREFIX: &[u8] = b"ethereum-config-";

/// Rollup state information
#[derive(Debug, Clone)]
struct RollupState {
    block_number: u64,
    block_hash: B256,
    batch_count: u64,
    batch_acc: B256,
    parent_chain_block: u64,
    delayed_msgs_count: u64,
    delayed_msgs_acc: B256,
}

pub async fn migration(nitro_db_path: &Path) -> Result<()> {
    info!("Nitro DB path: {:?}", nitro_db_path);
    let chaindata_path = nitro_db_path.join("l2chaindata");
    if !chaindata_path.exists() {
        eyre::bail!("L2 chaindata path does not exist: {:?}. Make sure you're pointing to the Nitro database directory (parent of l2chaindata)", chaindata_path);
    }

    // Open the database with read-write access if we're modifying, read-only otherwise
    let mut opts = Options::default();
    opts.create_if_missing(false);
    let db = DB::open(&opts, &chaindata_path).context("Failed to open database")?;

    // Also open the arbitrumdata database which contains Arbitrum-specific state
    let arb_db_path = nitro_db_path.join("arbitrumdata");
    let arb_db = DB::open_for_read_only(&opts, &arb_db_path, false).unwrap();

    // Get the rollup state
    let rollup_state = get_rollup_state(&db, &arb_db);

    // Get the chain config
    let (mut chain_config, genesis_hash, config_key) = get_chain_config(&db)?;

    error!("test: {:?}", rollup_state);

    // Display comprehensive rollup state
    // display_rollup_state(&rollup_state, &genesis_hash, &chain_config)?;

    let current_dac_value = if let Some(ref arb_params) = chain_config.arbitrum {
        arb_params.data_availability_committee
    } else {
        eyre::bail!("No Arbitrum chain parameters found in chain config");
    };

    if !current_dac_value {
        info!("DataAvailabilityCommittee is already set to false");
        return Ok(());
    }

    if let Some(ref mut arb_params) = chain_config.arbitrum {
        arb_params.data_availability_committee = false;
    }

    // Write the updated config
    // update_chain_config(&db, &chain_config, &config_key)?;
    info!("Configuration updated successfully");

    Ok(())
}

/// Retrieves the chain config from the database.
///
/// Returns the chain config, genesis hash, and the database key used.
fn get_chain_config(db: &DB) -> Result<(ChainConfig, B256, Vec<u8>)> {
    // In Nitro/Geth, chain configs are stored with the key "ethereum-config-<genesis_hash>"
    // We need to find the genesis hash first by iterating over keys with the prefix

    let prefix = CHAIN_CONFIG_PREFIX;
    let mut found_key: Option<Vec<u8>> = None;
    let mut found_value: Option<Vec<u8>> = None;

    // Iterate through keys with the chain config prefix
    let iter = db.prefix_iterator(prefix);
    for item in iter {
        let (key, value) = item.context("Failed to read from database iterator")?;

        // Take the first match (there should only be one genesis)
        if key.starts_with(prefix) {
            found_key = Some(key.to_vec());
            found_value = Some(value.to_vec());
            break;
        }
    }

    if found_key.is_none() || found_value.is_none() {
        eyre::bail!("No chain config found in database. Is this a Nitro chaindata directory?");
    }

    let key = found_key.unwrap_or_default();
    let value = found_value.unwrap_or_default();

    // Extract genesis hash from key (remove prefix)
    let genesis_hash_bytes = &key[prefix.len()..];
    if genesis_hash_bytes.len() != 32 {
        eyre::bail!("Invalid genesis hash length: {}", genesis_hash_bytes.len());
    }
    let genesis_hash = B256::from_slice(genesis_hash_bytes);

    // Decode the chain config (stored as JSON in Nitro)
    let chain_config: ChainConfig =
        serde_json::from_slice(&value).context("Failed to deserialize chain config")?;

    Ok((chain_config, genesis_hash, key))
}

/// Updates the chain config in the database.
fn update_chain_config(db: &DB, config: &ChainConfig, key: &[u8]) -> Result<()> {
    let encoded = serde_json::to_vec(config).context("Failed to serialize chain config")?;

    db.put(key, encoded).context("Failed to write chain config to database")?;

    Ok(())
}

/// Decodes a big-endian encoded unsigned integer from bytes
fn decode_uint(bytes: &[u8]) -> Option<u64> {
    if bytes.is_empty() {
        return None;
    }

    // Handle variable-length encoding used by Nitro
    let mut result: u64 = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        if i >= 8 {
            // Would overflow u64
            return None;
        }
        result = (result << 8) | u64::from(byte);
    }
    Some(result)
}

/// Encodes a u64 as an 8-byte big-endian value
fn encode_uint64_be(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

/// Constructs a database key with a single-byte prefix and 8-byte big-endian number
fn make_key(prefix: u8, number: u64) -> Vec<u8> {
    let mut key = Vec::with_capacity(9);
    key.push(prefix);
    key.extend_from_slice(&encode_uint64_be(number));
    key
}

// sequencerBatchCountKey         = []byte("_sequencerBatchCount")
// delayedMessageCountKey         = []byte("_delayedMessageCount")
// sequencerBatchMetaPrefix       = []byte("s")
// rlpDelayedMessagePrefix        = []byte("e")
// legacyDelayedMessagePrefix     = []byte("d")
// parentChainBlockNumberPrefix   = []byte("p")

#[derive(Debug, Clone, RlpDecodable)]
struct BatchMetadata {
    acc: B256,
    message_count: u64,
    delayed_message_count: u64,
    parent_chain_block: u64,
}

/// Retrieves rollup state information from the database
#[allow(clippy::unwrap_used)]
fn get_rollup_state(db: &DB, arb_db: &DB) -> RollupState {
    let block_hash =
        db.get(b"LastBlock").unwrap().and_then(|bytes| Some(B256::from_slice(&bytes))).unwrap();

    let mut block_number_key = [0u8; 33];
    block_number_key[0] = b'H';
    block_number_key[1..].copy_from_slice(&block_hash.as_ref());

    let block_number = db
        .get(block_number_key)
        .unwrap()
        .map(|bytes| u64::from_be_bytes(bytes[..8].try_into().unwrap()))
        .unwrap();

    let batch_count = arb_db
        .get(b"_sequencerBatchCount")
        .unwrap()
        .map(|bytes| u64::decode(&mut &bytes[..]).unwrap())
        .unwrap();

    let delayed_msgs_count = arb_db
        .get(b"_delayedMessageCount")
        .unwrap()
        .map(|bytes| u64::decode(&mut &bytes[..]).unwrap())
        .unwrap();
    // ---
    let batch_data = arb_db
        .get(make_key(b"s"[0], batch_count - 1))
        .unwrap()
        .map(|bytes| BatchMetadata::decode(&mut &bytes[..]).unwrap())
        .unwrap();

    let delayed_msgs_acc = arb_db
        .get(make_key(b"e"[0], batch_data.delayed_message_count - 1))
        .unwrap()
        .map(|bytes| B256::from_slice(&bytes[..32]))
        .unwrap();

    RollupState {
        block_number,
        block_hash,
        batch_count,
        batch_acc: batch_data.acc,
        parent_chain_block: batch_data.parent_chain_block,
        delayed_msgs_count,
        delayed_msgs_acc,
    }
}
