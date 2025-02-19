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

use crate::types::{BlockAndReceipts, BlockRef, Slot};
use async_trait::async_trait;
use bincode;
use rocksdb::DB;
use serde_json;
use thiserror::Error;

const KEY_SAFE_SEQ: &[u8] = b"safe/seq";
const KEY_SAFE_SETTLE: &[u8] = b"safe/settle";
const KEY_UNSAFE_SEQ: &[u8] = b"unsafe/seq";
const KEY_UNSAFE_SETTLE: &[u8] = b"unsafe/settle";
const KEY_SLOT_SAFE: &[u8] = b"slot/latest_safe";
const KEY_SLOT_UNSAFE: &[u8] = b"slot/latest_unsafe";

/// Latest safe state of the translator
#[derive(Debug)]
pub struct KnownState {
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
    /// Saves the latest safe slot and block refs to the database
    /// NOTE: `save_safe_slot` will keep track of latest safe blocks from each slot that is passed,
    /// it expects all safe slots to be passed in order
    async fn save_safe_slot(&self, slot: &Slot) -> Result<(), DbError>;

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
    async fn get_safe_state(&self) -> Result<Option<KnownState>, DbError>;

    /// Saves the latest unsafe slot and block refs to the database
    /// NOTE: `save_unsafe_slot` will keep track of latest unsafe blocks from each slot that is
    /// passed, it expects all unsafe slots to be passed in order
    async fn save_unsafe_slot(&self, slot: &Slot) -> Result<(), DbError>;

    /// Returns the latest unsafe state from the database, if one exists.
    ///
    /// The unsafe state represents the last known unsafe state (that will be reorged) of the
    /// system, including:
    /// - The latest unsafe sequencing block
    /// - The latest unsafe settlement block
    /// - The latest unsafe slot
    ///
    /// This is used for resuming the translator from a previous shutdown.
    ///
    /// NOTE: if an unsafe state is not found, or a block is missing for a chain, it is considered
    /// that the DB is empty and the translator will start from the genesis block/slot
    ///
    /// # Returns
    /// * `Ok(Some(SafeState))` if an unsafe state was found in the database
    /// * `Ok(None)` if no unsafe state exists yet
    /// * `Err(DbError)` if there was an error reading from the database
    async fn get_unsafe_state(&self) -> Result<Option<KnownState>, DbError>;
}

/// Dummy implementation of the [`TranslatorStore`] trait that does nothing
#[derive(Debug)]
pub struct DummyStore;

#[async_trait]
impl TranslatorStore for DummyStore {
    async fn save_safe_slot(&self, _slot: &Slot) -> Result<(), DbError> {
        Ok(())
    }

    async fn get_safe_state(&self) -> Result<Option<KnownState>, DbError> {
        Ok(None)
    }

    async fn save_unsafe_slot(&self, _slot: &Slot) -> Result<(), DbError> {
        Ok(())
    }

    async fn get_unsafe_state(&self) -> Result<Option<KnownState>, DbError> {
        Ok(None)
    }
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

    async fn get_state(
        &self,
        slot_key: &[u8],
        seq_key: &[u8],
        settle_key: &[u8],
    ) -> Result<Option<KnownState>, DbError> {
        let slot_bytes = match self.db.get(slot_key)? {
            Some(v) => v,
            None => return Ok(None),
        };

        let slot: Slot = serde_json::from_slice(&slot_bytes)?;
        let seq = self.db.get(seq_key)?.and_then(|v| bincode::deserialize(&v).ok());
        let settle = self.db.get(settle_key)?.and_then(|v| bincode::deserialize(&v).ok());

        match (seq, settle) {
            (Some(seq), Some(settle)) => {
                Ok(Some(KnownState { slot, sequencing_block: seq, settlement_block: settle }))
            }
            _ => Ok(None),
        }
    }
}

#[async_trait]
impl TranslatorStore for RocksDbStore {
    async fn save_safe_slot(&self, slot: &Slot) -> Result<(), DbError> {
        let mut batch = rocksdb::WriteBatch::default();
        batch.put(KEY_SLOT_SAFE, serde_json::to_vec(slot)?);

        batch.put(KEY_SAFE_SEQ, bincode::serialize(&BlockRef::new(&slot.sequencing_block.block))?);

        if let Some(last_settle) = slot.settlement_blocks.last() {
            batch.put(KEY_SAFE_SETTLE, bincode::serialize(&BlockRef::new(&last_settle.block))?);
        }

        self.db.write(batch)?;
        Ok(())
    }

    async fn save_unsafe_slot(&self, slot: &Slot) -> Result<(), DbError> {
        let mut batch = rocksdb::WriteBatch::default();
        batch.put(KEY_SLOT_UNSAFE, serde_json::to_vec(slot)?);

        batch
            .put(KEY_UNSAFE_SEQ, bincode::serialize(&BlockRef::new(&slot.sequencing_block.block))?);

        if let Some(last_settle) = slot.settlement_blocks.last() {
            batch.put(KEY_UNSAFE_SETTLE, bincode::serialize(&BlockRef::new(&last_settle.block))?);
        }

        self.db.write(batch)?;
        Ok(())
    }

    async fn get_safe_state(&self) -> Result<Option<KnownState>, DbError> {
        self.get_state(KEY_SLOT_SAFE, KEY_SAFE_SEQ, KEY_SAFE_SETTLE).await
    }

    async fn get_unsafe_state(&self) -> Result<Option<KnownState>, DbError> {
        self.get_state(KEY_SLOT_UNSAFE, KEY_UNSAFE_SEQ, KEY_UNSAFE_SETTLE).await
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::{Block, BlockAndReceipts, SlotState};
    use alloy::primitives::B256;
    use test_utils::test_path;

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
    async fn test_save_and_get_safe_state() {
        let store = RocksDbStore::new(test_path("rocksdb_test_safe").as_str()).unwrap();
        assert!(store.get_safe_state().await.unwrap().is_none());

        let mut slot =
            Slot::new(1, 1000, BlockAndReceipts { block: create_test_block(1), receipts: vec![] });
        let seq_block = create_test_block(1);
        let settle_block = create_test_block(2);

        slot.sequencing_block = BlockAndReceipts { block: seq_block.clone(), receipts: vec![] };
        slot.settlement_blocks
            .push(BlockAndReceipts { block: settle_block.clone(), receipts: vec![] });
        slot.state = SlotState::Safe;
        store.save_safe_slot(&slot).await.unwrap();

        let latest = store.get_safe_state().await.unwrap().unwrap();
        assert_eq!(latest.slot, slot);
        assert_eq!(latest.sequencing_block, BlockRef::new(&seq_block));
        assert_eq!(latest.settlement_block, BlockRef::new(&settle_block));

        // Verify unsafe state is empty
        assert!(store.get_unsafe_state().await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_save_and_get_unsafe_state() {
        let store = RocksDbStore::new(test_path("rocksdb_test_unsafe").as_str()).unwrap();
        assert!(store.get_unsafe_state().await.unwrap().is_none());

        let mut slot =
            Slot::new(1, 1000, BlockAndReceipts { block: create_test_block(1), receipts: vec![] });
        let seq_block = create_test_block(1);
        let settle_block = create_test_block(2);

        slot.sequencing_block = BlockAndReceipts { block: seq_block.clone(), receipts: vec![] };
        slot.settlement_blocks
            .push(BlockAndReceipts { block: settle_block.clone(), receipts: vec![] });
        slot.state = SlotState::Closed;

        store.save_unsafe_slot(&slot).await.unwrap();

        let latest = store.get_unsafe_state().await.unwrap().unwrap();
        assert_eq!(latest.slot, slot);
        assert_eq!(latest.sequencing_block, BlockRef::new(&seq_block));
        assert_eq!(latest.settlement_block, BlockRef::new(&settle_block));

        // Verify safe state is empty
        assert!(store.get_safe_state().await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_safe_and_unsafe_states_are_independent() {
        let store = RocksDbStore::new(test_path("rocksdb_test_both").as_str()).unwrap();

        // Create and save safe state
        let mut safe_slot =
            Slot::new(1, 1000, BlockAndReceipts { block: create_test_block(1), receipts: vec![] });
        let safe_seq = create_test_block(1);
        let safe_settle = create_test_block(2);
        safe_slot.sequencing_block = BlockAndReceipts { block: safe_seq.clone(), receipts: vec![] };
        safe_slot
            .settlement_blocks
            .push(BlockAndReceipts { block: safe_settle.clone(), receipts: vec![] });
        safe_slot.state = SlotState::Safe;
        store.save_safe_slot(&safe_slot).await.unwrap();

        // Create and save unsafe state
        let mut unsafe_slot =
            Slot::new(2, 2000, BlockAndReceipts { block: create_test_block(3), receipts: vec![] });
        let unsafe_seq = create_test_block(3);
        let unsafe_settle = create_test_block(4);
        unsafe_slot.sequencing_block =
            BlockAndReceipts { block: unsafe_seq.clone(), receipts: vec![] };
        unsafe_slot
            .settlement_blocks
            .push(BlockAndReceipts { block: unsafe_settle.clone(), receipts: vec![] });
        unsafe_slot.state = SlotState::Closed;

        store.save_unsafe_slot(&unsafe_slot).await.unwrap();

        // Verify safe state
        let safe_state = store.get_safe_state().await.unwrap().unwrap();
        assert_eq!(safe_state.slot, safe_slot);
        assert_eq!(safe_state.sequencing_block, BlockRef::new(&safe_seq));
        assert_eq!(safe_state.settlement_block, BlockRef::new(&safe_settle));

        // Verify unsafe state
        let unsafe_state = store.get_unsafe_state().await.unwrap().unwrap();
        assert_eq!(unsafe_state.slot, unsafe_slot);
        assert_eq!(unsafe_state.sequencing_block, BlockRef::new(&unsafe_seq));
        assert_eq!(unsafe_state.settlement_block, BlockRef::new(&unsafe_settle));
    }
}
