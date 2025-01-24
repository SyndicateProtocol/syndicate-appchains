use crate::types::{Block, Slot};
use async_trait::async_trait;
use rocksdb::DB;
use std::hash::{DefaultHasher, Hash, Hasher};
use thiserror::Error;

const PREFIX_SLOT: &[u8] = b"slot/";
const KEY_SEQ_LATEST: &[u8] = b"latest/seq";
const KEY_SETTLE_LATEST: &[u8] = b"latest/settle";

/// Latest safe state of the translator
#[derive(Debug)]
pub struct SafeState {
    /// The latest slot that was marked as safe
    pub slot: Slot,
    /// The latest block from the sequencing chain that has been slotted
    pub sequencing_block: Block,
    /// The latest block from the settlement chain that has been slotted
    pub settlement_block: Block,
}

#[async_trait]
pub trait TranslatorStore {
    /// Saves the latest slot and latest blocks to the database
    async fn save_slot(&self, slot: &Slot) -> Result<(), DbError>;
    /// Returns the latest safe state (latest slot and latest blocks from each chain)
    async fn get_latest(&self) -> Result<Option<SafeState>, DbError>;
}

pub struct RocksDbStore {
    db: DB,
}

impl RocksDbStore {
    pub fn new(path: &str) -> Result<Self, DbError> {
        Ok(Self { db: DB::open_default(path)? })
    }

    fn slot_key(slot_number: u64) -> Vec<u8> {
        let mut key = PREFIX_SLOT.to_vec();
        key.extend_from_slice(&slot_number.to_be_bytes());
        key
    }
}

#[async_trait]
impl TranslatorStore for RocksDbStore {
    async fn save_slot(&self, slot: &Slot) -> Result<(), DbError> {
        let mut batch = rocksdb::WriteBatch::default();
        batch.put(Self::slot_key(slot.number), bincode::serialize(slot)?);
        batch.put(
            KEY_SEQ_LATEST,
            bincode::serialize(&slot.sequencing_chain_blocks.last().map(|b| &b.block))?,
        );
        batch.put(
            KEY_SETTLE_LATEST,
            bincode::serialize(&slot.settlement_chain_blocks.last().map(|b| &b.block))?,
        );
        self.db.write(batch)?;
        Ok(())
    }

    async fn get_latest(&self) -> Result<Option<SafeState>, DbError> {
        let iter = self.db.prefix_iterator(PREFIX_SLOT);
        match iter.last().transpose()? {
            None => Ok(None),
            Some((_, v)) => {
                let slot = bincode::deserialize(&v)?;
                let seq = self.db.get(KEY_SEQ_LATEST)?.and_then(|v| bincode::deserialize(&v).ok());
                let settle =
                    self.db.get(KEY_SETTLE_LATEST)?.and_then(|v| bincode::deserialize(&v).ok());

                match (seq, settle) {
                    (Some(seq), Some(settle)) => Ok(Some(SafeState {
                        slot,
                        sequencing_block: seq,
                        settlement_block: settle,
                    })),
                    _ => Ok(None),
                }
            }
        }
    }
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database error: {0}")]
    Db(#[from] rocksdb::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
}

// Test utility function, panics are acceptable
/// Returns a unique temporary path for `RocksDB` test databases.
///
/// The path is constructed by:
/// 1. Getting the caller's source location (file and line)
/// 2. Appending the current timestamp in nanoseconds
/// 3. Hashing the combined string
/// 4. Creating a path in the system temp directory with format `"rocksdb_test_{hash}"`
///
/// This ensures unique paths for concurrent tests and includes the test location
/// for debugging.
#[allow(clippy::unwrap_used)] // Test utility function, panics are acceptable
pub fn test_path() -> String {
    use std::{
        panic,
        time::{SystemTime, UNIX_EPOCH},
    };

    let location = panic::Location::caller();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

    let mut hasher = DefaultHasher::new();
    format!("{}:{}", location, timestamp).hash(&mut hasher);
    let hash = hasher.finish();

    std::env::temp_dir().join(format!("rocksdb_test_{:x}", hash)).to_str().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::BlockAndReceipts;
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
        slot.sequencing_chain_blocks
            .push(BlockAndReceipts { block: create_test_block(1), receipts: vec![] });
        slot.settlement_chain_blocks
            .push(BlockAndReceipts { block: create_test_block(2), receipts: vec![] });
        store.save_slot(&slot).await.unwrap();

        let latest = store.get_latest().await.unwrap().unwrap();
        assert_eq!(latest.slot.number, 1);
        assert_eq!(latest.sequencing_block.number, 1);
        assert_eq!(latest.settlement_block.number, 2);
    }
}

// In-memory implementation for testing ------------------------------------------

// /// In-memory store for testing
// #[derive(Debug)]
// pub struct InMemoryStore<T> {
//     slots: Mutex<BTreeMap<Vec<u8>, T>>,
// }

// impl<T> InMemoryStore<T> {
//     pub const fn new() -> Self {
//         Self {
//             slots: Mutex::new(BTreeMap::new()),
//         }
//     }
// }

// impl<T> Default for InMemoryStore<T> {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// #[async_trait]
// #[allow(clippy::unwrap_used)] // Safe to unwrap in test code
// impl<T: Serialize + DeserializeOwned + Clone + Send + Sync> Store<T> for InMemoryStore<T> {
//     async fn save(&self, key: &[u8], value: &T) -> Result<(), DbError> {
//         self.slots
//             .lock()
//             .unwrap()
//             .insert(key.to_vec(), value.clone());
//         Ok(())
//     }

//     async fn get(&self, key: &[u8]) -> Result<Option<T>, DbError> {
//         let slots = self.slots.lock().unwrap();
//         Ok(slots.get(key).cloned())
//     }

//     async fn get_latest(&self) -> Result<Option<T>, DbError> {
//         let slots = self.slots.lock().unwrap();
//         Ok(slots.values().next_back().cloned())
//     }
// }
