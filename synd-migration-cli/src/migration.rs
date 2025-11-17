//! Module for migrating the `DataAvailabilityCommittee` flag in Nitro chain configs.
use alloy::{
    primitives::B256,
    rlp::{Decodable, RlpDecodable},
};
use eyre::{eyre, Context, Result};
use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

/// Arbitrum-specific chain configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ArbitrumChainParams {
    /// Whether arbOS is enabled
    #[serde(default, rename = "EnableArbOS")]
    pub enable_arb_os: bool,

    /// Allow debug precompiles
    #[serde(default)]
    pub allow_debug_precompiles: bool,

    /// Data Availability Committee flag - this is what we're migrating
    pub data_availability_committee: bool,

    /// Initial arbOS version
    #[serde(default, rename = "InitialArbOSVersion")]
    pub initial_arb_os_version: u64,

    /// Genesis block number
    #[serde(default)]
    pub genesis_block_num: u64,

    /// Maximum code size
    #[serde(default)]
    pub max_code_size: u64,

    /// Maximum init code size
    #[serde(default)]
    pub max_init_code_size: u64,

    /// Initial chain owner address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_chain_owner: Option<String>,

    /// Syndicate flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syndicate: Option<bool>,

    /// eigenDA flag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eigen_da: Option<bool>,
}

/// Clique consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(missing_docs)]
pub struct CliqueConfig {
    pub period: u64,
    pub epoch: u64,
}

/// Ethereum chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainConfig {
    /// Homestead block
    #[serde(default)]
    pub homestead_block: u64,

    /// DAO fork block
    pub dao_fork_block: Option<u64>,

    /// DAO fork support
    #[serde(default)]
    pub dao_fork_support: bool,

    /// EIP150 block
    #[serde(default)]
    pub eip150_block: u64,

    /// EIP150 hash
    #[serde(default)]
    pub eip150_hash: B256,

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

    /// Muir Glacier block
    #[serde(default)]
    pub muir_glacier_block: u64,

    /// Berlin block
    #[serde(default)]
    pub berlin_block: u64,

    /// London block
    #[serde(default)]
    pub london_block: u64,

    /// Clique consensus configuration
    #[serde(default)]
    pub clique: CliqueConfig,

    /// Arbitrum-specific parameters
    #[serde(default)]
    pub arbitrum: ArbitrumChainParams,

    /// Chain ID
    pub chain_id: u64,
}

/// Rollup state information
#[derive(Debug, Clone, Default)]
pub struct RollupState {
    /// The last block number
    pub last_block_number: u64,
    /// The last block hash
    pub last_block_hash: B256,
    /// The safe block number
    pub safe_block_number: Option<u64>,
    /// The safe block hash
    pub safe_block_hash: Option<B256>,
    /// The batch count
    pub batch_count: u64,
    /// The batch accumulator
    pub batch_acc: B256,
    /// The parent chain block
    pub parent_chain_block: u64,
    /// The delayed messages count
    pub delayed_msgs_count: u64,
    /// The delayed messages accumulator
    pub delayed_msgs_acc: B256,
    /// Arb message count
    pub batch_message_count: u64,
}

/// Migration command - it inspects a given Nitro database, extracts relevant information and sets
/// `DataAvailabilityCommittee` to false in the chain config.
#[allow(clippy::unwrap_used, clippy::cognitive_complexity)]
pub async fn get_migration_data(nitro_db_path: &Path) -> Result<(RollupState, Vec<u8>)> {
    info!("Nitro DB path: {:?}", nitro_db_path);
    let chaindata_path = nitro_db_path.join("l2chaindata");
    if !chaindata_path.exists() {
        eyre::bail!(
            "L2 chaindata path does not exist: {:?}. Make sure you're pointing to the Nitro database directory (parent of l2chaindata)",
            chaindata_path
        );
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
    let (chain_config, _config_key, raw_genesis_from_db) = get_chain_config(&db)?;

    debug!("rollup state: {:#?}", rollup_state);
    debug!("chain config: {:#?}", chain_config);

    println!("\n---------------TRANSLATOR / MCHAIN config ---------------\n");
    println!("MIGRATED_BATCH_ACC: {}", rollup_state.batch_acc);
    println!("MIGRATED_BATCH_COUNT: {}", rollup_state.batch_count);
    println!("MIGRATED_DELAYED_MSGS_ACC: {}", rollup_state.delayed_msgs_acc);
    println!("MIGRATED_DELAYED_MSGS_COUNT: {}", rollup_state.delayed_msgs_count);
    println!("MIGRATED_APPCHAIN_BLOCK_HASH: {:?}", rollup_state.last_block_hash);
    println!("SETTLEMENT_START_BLOCK: {}", rollup_state.parent_chain_block);
    println!("GENESIS_CONFIG: '{}'", std::str::from_utf8(&raw_genesis_from_db).unwrap());
    println!("\n------------------------------\n\n");

    println!("\n--------------- NITRO configuration ---------------\n");
    println!(
        "--chain.info-json={}",
        get_nitro_chain_cfg(
            chain_config.chain_id.to_string(),
            rollup_state.parent_chain_block.to_string()
        )
    );
    if chain_config.arbitrum.data_availability_committee {
        println!("--node.data-availability.enable=true");
        println!("--node.data-availability.rest-aggregator.urls=https://no.op");
        println!("--node.data-availability.rest-aggregator.enable=true");
    }

    println!("\n------------------------------\n\n");

    println!("last batch arb msg count: {}", rollup_state.batch_message_count);

    println!(
        "last rollup block: {:?} - {:?}",
        rollup_state.last_block_number, rollup_state.last_block_hash
    );
    println!(
        "safe rollup block: {:?} - {:?}",
        rollup_state.safe_block_number, rollup_state.safe_block_hash
    );

    if rollup_state.safe_block_hash.is_some() &&
        rollup_state.safe_block_hash.unwrap() == rollup_state.last_block_hash
    {
        println!("✅✅✅✅✅ Rollup is in safe state to be migrated");
    } else {
        println!(
            "❌❌❌❌❌ Rollup is not in safe state to be migrated - a reorg is likely to happen"
        );
    }

    Ok((rollup_state, raw_genesis_from_db))
}

fn get_nitro_chain_cfg(appchain_chain_id: String, deployed_at: String) -> String {
    format!("--chain.info-json=[{{\"chain-id\":{appchain_chain_id},\"parent-chain-id\":511000,\"parent-chain-is-arbitrum\":false,\"chain-name\":\"unite-testnet\",\"chain-config\":{{\"homesteadBlock\":0,\"daoForkBlock\":null,\"daoForkSupport\":true,\"eip150Block\":0,\"eip150Hash\":\"0x0000000000000000000000000000000000000000000000000000000000000000\",\"eip155Block\":0,\"eip158Block\":0,\"byzantiumBlock\":0,\"constantinopleBlock\":0,\"petersburgBlock\":0,\"istanbulBlock\":0,\"muirGlacierBlock\":0,\"berlinBlock\":0,\"londonBlock\":0,\"clique\":{{\"period\":0,\"epoch\":0}},\"arbitrum\":{{\"EnableArbOS\":true,\"AllowDebugPrecompiles\":false,\"DataAvailabilityCommittee\":true,\"InitialArbOSVersion\":32,\"GenesisBlockNum\":0,\"MaxCodeSize\":24576,\"MaxInitCodeSize\":49152,\"InitialChainOwner\":\"0x6dedc20540fd54348fa0d7b0af2378f5494ab240\"}},\"chainId\":{appchain_chain_id}}},\"rollup\":{{\"bridge\":\"0x0000000000000000000000000000000000511000\",\"inbox\":\"0x0000000000000000000000000000000000511000\",\"sequencer-inbox\":\"0x0000000000000000000000000000000000511000\",\"rollup\":\"0x0000000000000000000000000000000000511000\",\"validator-utils\":\"0x0000000000000000000000000000000000511000\",\"validator-wallet-creator\":\"0x0000000000000000000000000000000000511000\",\"deployed-at\":{deployed_at}}}}}]")
}

/// Retrieves the chain config from the database.
/// `DBkeys` used can be found in <https://github.com/SyndicateProtocol/go-ethereum/blob/HEAD/core/rawdb/schema.go>
///
/// Returns the chain config, and the database key used.
#[allow(clippy::unwrap_used)]
fn get_chain_config(db: &DB) -> Result<(ChainConfig, Vec<u8>, Vec<u8>)> {
    // headerHashKey = headerPrefix + num (uint64 big endian) + headerHashSuffix
    let mut raw_genesis: Vec<u8> = vec![];
    let genesis_hash = db
        .get(make_numbered_key(b"h", 0, b"n"))
        .unwrap()
        .map(|bytes| B256::from_slice(&bytes[..32]))
        .unwrap();

    let config_key = make_key(b"ethereum-config-", genesis_hash.as_ref(), &[]);
    let chain_config: ChainConfig = db
        .get(config_key.clone())
        .unwrap()
        .map(|bytes| {
            raw_genesis = bytes.clone();
            serde_json::from_slice(&bytes).unwrap()
        })
        .unwrap();

    Ok((chain_config, config_key, raw_genesis))
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

#[allow(clippy::unwrap_used)]
fn get_block_info(db: &DB, key: &[u8]) -> Result<(Option<u64>, Option<B256>)> {
    match db.get(key)?.map(|bytes| B256::from_slice(&bytes)) {
        Some(block_hash) => {
            let mut block_number_key = [0u8; 33];
            block_number_key[0] = b'H';
            block_number_key[1..].copy_from_slice(block_hash.as_ref());

            let block_number = db
                .get(block_number_key)?
                .map(|bytes| u64::from_be_bytes(bytes[..8].try_into().unwrap()));
            Ok((block_number, Some(block_hash)))
        }
        None => Ok((None, None)),
    }
}

/// Retrieves rollup state information from the database
/// `DBkeys` used can be found in <https://github.com/SyndicateProtocol/go-ethereum/blob/HEAD/core/rawdb/schema.go>
#[allow(clippy::unwrap_used)]
fn get_rollup_state(db: &DB, arb_db: &DB) -> Result<RollupState> {
    let (last_block_number, last_block_hash) = get_block_info(db, b"LastBlock")?;
    let (safe_block_number, safe_block_hash) = get_block_info(db, b"LastFinalized")?;

    let last_block_hash = last_block_hash.ok_or_else(|| eyre!("last block hash is None"))?;
    let last_block_number = last_block_number.ok_or_else(|| eyre!("last block number is None"))?;

    let batch_count_bytes =
        arb_db.get(b"_sequencerBatchCount")?.ok_or_else(|| eyre!("Failed to get batch count"))?;

    let batch_count = u64::decode(&mut &batch_count_bytes[..])
        .map_err(|e| eyre!("Failed to decode batch count: {e}"))?;

    // SequencerBatchMetaPrefix is "s", and maps batch_seq_num to batch metadata
    let batch_data = arb_db
        .get(make_numbered_key(b"s", batch_count - 1, &[]))?
        .map(|bytes| BatchMetadata::decode(&mut &bytes[..]).unwrap())
        .ok_or_else(|| eyre!("Failed to get batch data"))?;
    debug!("batch_data: {:#?}", batch_data);

    let delayed_msgs_count = batch_data.delayed_message_count;

    // RlpDelayedMessagePrefix is "e" and maps delayed messages sequence_num to
    // [delayedMsgsAcc(32bytes) + RLP encoded L1IncomingMessage]
    let delayed_msgs_acc = arb_db
        .get(make_numbered_key(b"e", delayed_msgs_count - 1, &[]))?
        .map(|bytes| B256::from_slice(&bytes[..32]))
        .ok_or_else(|| eyre!("Failed to get delayed message accumulator"))?;

    Ok(RollupState {
        last_block_number,
        last_block_hash,
        safe_block_number,
        safe_block_hash,
        batch_count,
        batch_acc: batch_data.acc,
        parent_chain_block: batch_data.parent_chain_block,
        delayed_msgs_count,
        delayed_msgs_acc,
        batch_message_count: batch_data.message_count,
    })
}
