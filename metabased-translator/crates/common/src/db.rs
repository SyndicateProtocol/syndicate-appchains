//! The `db` module contains the `TranslatorStore` trait and its implementation for `RocksDB`
//!
//! The `TranslatorStore` trait is used to save and retrieve the latest slot and latest blocks from
//! each chain
//!
//! The current implementation relies on `block-builder` saving the full state.
//! The resume from DB functionality works as follows:
//! - the lastest known `Safe` `Slot` and the latest slotted block for each chain are taken from the
//!   DB
//! - the Ingestor will start from the lastest slotted block for each chain and continue from there
//! - the `Slotter` will then start from the lastest known `Safe` `Slot` and continue to slot blocks
//!   from the chains
//! - the `BlockBuilder` will reorg the Anvil state to match the latest known safe slot and continue
//!   from there
//!
//! The `RocksDbStore` implementation is used for the `TranslatorStore` trait

use crate::types::{BlockRef, Slot};
use async_trait::async_trait;
use bincode;
use rocksdb::DB;
use serde_json;
use std::hash::{DefaultHasher, Hash, Hasher};
use thiserror::Error;

const KEY_SLOT: &[u8] = b"slot/latest";
const KEY_SEQ_LATEST: &[u8] = b"latest/seq";
const KEY_SETTLE_LATEST: &[u8] = b"latest/settle";

/// Latest safe state of the translator
#[derive(Debug)]
pub struct SafeState {
    /// The latest slot that was marked as safe
    pub slot: Slot,
    /// The latest block from the sequencing chain that has been slotted
    pub sequencing_block: BlockRef,
    /// The latest block from the settlement chain that has been slotted
    pub settlement_block: BlockRef,
}

#[async_trait]
/// A trait for storing and retrieving the latest safe state of the translator
///
/// The safe state consists of:
/// - The latest slot that was marked as safe
/// - The latest block from the sequencing chain that has been slotted
/// - The latest block from the settlement chain that has been slotted
///
/// This state is used to resume the translator after a restart:
/// - The Ingestor will start from the latest slotted blocks
/// - The Slotter will start from the latest safe slot
/// - The BlockBuilder will reorg to match the latest safe slot
pub trait TranslatorStore {
    /// Saves the latest slot and latest blocks to the database
    ///
    /// # Arguments
    /// * `slot` - The slot to save
    ///
    /// # Returns
    /// * `Ok(())` if the save was successful
    /// * `Err(DbError)` if there was an error saving to the database
    async fn save_slot(&self, slot: &Slot) -> Result<(), DbError>;

    /// Returns the latest safe state from the database, if one exists.
    ///
    /// The safe state represents the last known safe state (that won't be reorged) of the system,
    /// including:
    /// - The latest safe sequencing block
    /// - The latest safe settlement block
    /// - The latest safe slot
    ///
    /// This is used for resuming the translator from a previous shutdown.
    ///
    /// NOTE: if a safe state is not found, or a block is missing for a chain, it is considered that
    /// the DB is empty and the translator will start from the genesis block/slot
    ///
    /// # Returns
    /// * `Ok(Some(SafeState))` if a safe state was found in the database
    /// * `Ok(None)` if no safe state exists yet
    /// * `Err(DbError)` if there was an error reading from the database
    async fn get_latest(&self) -> Result<Option<SafeState>, DbError>;
}

/// A RocksDB-backed implementation of the [`TranslatorStore`] trait
#[derive(Debug)]
pub struct RocksDbStore {
    /// The underlying `RocksDB` instance
    db: DB,
}

impl RocksDbStore {
    /// Creates a new `RocksDbStore` instance
    ///
    /// # Arguments
    /// * `path` - The path to the `RocksDB` database
    ///
    /// # Returns
    /// * `Result<Self, DbError>` - The new `RocksDbStore` instance
    pub fn new(path: &str) -> Result<Self, DbError> {
        Ok(Self { db: DB::open_default(path)? })
    }
}

#[async_trait]
impl TranslatorStore for RocksDbStore {
    async fn save_slot(&self, slot: &Slot) -> Result<(), DbError> {
        let mut batch = rocksdb::WriteBatch::default();
        batch.put(KEY_SLOT, serde_json::to_vec(slot)?);

        // Store just the latest block refs
        if let Some(last_seq) = slot.sequencing_chain_blocks.last() {
            batch.put(KEY_SEQ_LATEST, bincode::serialize(&BlockRef::new(&last_seq.block))?);
        }
        if let Some(last_settle) = slot.settlement_chain_blocks.last() {
            batch.put(KEY_SETTLE_LATEST, bincode::serialize(&BlockRef::new(&last_settle.block))?);
        }

        self.db.write(batch)?;
        Ok(())
    }

    async fn get_latest(&self) -> Result<Option<SafeState>, DbError> {
        let slot_bytes = match self.db.get(KEY_SLOT)? {
            Some(v) => v,
            None => return Ok(None),
        };

        let slot: Slot = serde_json::from_slice(&slot_bytes)?;
        let seq = self.db.get(KEY_SEQ_LATEST)?.and_then(|v| bincode::deserialize(&v).ok());
        let settle = self.db.get(KEY_SETTLE_LATEST)?.and_then(|v| bincode::deserialize(&v).ok());

        match (seq, settle) {
            (Some(seq), Some(settle)) => {
                Ok(Some(SafeState { slot, sequencing_block: seq, settlement_block: settle }))
            }
            _ => Ok(None),
        }
    }
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Db(#[from] rocksdb::Error),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Bincode serialization error: {0}")]
    BincodeSerialization(#[from] bincode::Error),
}

// Test utility function, panics are acceptable
/// Returns a unique temporary path for `RocksDB` test databases.
///
/// The path is constructed by:
/// 1. Getting the caller's source location (file and line)
/// 2. Appending the current timestamp in nanoseconds and thread ID
/// 3. Hashing the combined string
/// 4. Creating a path in the system temp directory with format `"rocksdb_test_{hash}"`
///
/// This ensures unique paths for concurrent tests by including both the test location
/// and thread ID for debugging.
#[allow(clippy::unwrap_used)] // Test utility function, panics are acceptable
pub fn test_path() -> String {
    use std::{
        panic, thread,
        time::{SystemTime, UNIX_EPOCH},
    };

    let location = panic::Location::caller();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let thread_id = thread::current().id();

    let mut hasher = DefaultHasher::new();
    format!("{}:{}:{:?}", location, timestamp, thread_id).hash(&mut hasher);
    let hash = hasher.finish();

    std::env::temp_dir().join(format!("rocksdb_test_{:x}", hash)).to_str().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{Block, BlockAndReceipts};
    use alloy::primitives::B256;

    fn create_test_block(number: u64) -> Block {
        Block {
            hash: B256::ZERO,
            number,
            parent_hash: B256::ZERO,
            logs_bloom: "0x0".to_string(),
            transactions_root: "0x0".to_string(),
            state_root: "0x0".to_string(),
            receipts_root: "0x0".to_string(),
            timestamp: number * 1000,
            transactions: vec![],
        }
    }

    #[tokio::test]
    async fn test_save_and_get_latest() {
        let store = RocksDbStore::new(test_path().as_str()).unwrap();
        assert!(store.get_latest().await.unwrap().is_none());

        let mut slot = Slot::new(1, 1000);
        let seq_block = create_test_block(1);
        let settle_block = create_test_block(2);

        slot.sequencing_chain_blocks
            .push(BlockAndReceipts { block: seq_block.clone(), receipts: vec![] });
        slot.settlement_chain_blocks
            .push(BlockAndReceipts { block: settle_block.clone(), receipts: vec![] });
        store.save_slot(&slot).await.unwrap();
        let latest = store.get_latest().await.unwrap().unwrap();

        assert_eq!(latest.slot, slot);
        assert_eq!(latest.sequencing_block, BlockRef::new(&seq_block));
        assert_eq!(latest.settlement_block, BlockRef::new(&settle_block));
    }
}
