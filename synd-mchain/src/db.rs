use alloy::{
    primitives::{keccak256, Address, Bytes, FixedBytes, U256},
    sol_types::SolValue as _,
};
use jsonrpsee::types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned};
use rocksdb::{DBWithThreadMode, ThreadMode};
use serde::{Deserialize, Serialize};
use std::fmt;

/// VERSION must be bumped whenever a breaking change is made
const VERSION: u64 = 1;

/// Each delayed message is used to derive an appchain block
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DelayedMessage {
    pub kind: u8,
    pub sender: Address,
    pub data: Bytes,
    pub base_fee_l1: U256,
}

/// Slot metadata used to track reorgs
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Slot {
    /// The sequencing block that seals the slot, which is also included in the slot
    pub seq_block_number: u64,
    pub seq_block_hash: FixedBytes<32>,
    /// The settlement block that seals the slot, which is not included in the slot
    pub set_block_number: u64,
    pub set_block_hash: FixedBytes<32>,
}

/// The current state of the synd-mchain
// `batch_count` is the latest number of batches
// `batch_acc` is the latest batch accumulator
// `message_count` is the latest number of messages
// `message_acc` is the latest message accumulator
// `timestamp` is the timestamp of the pending slot
// `slot` is the pending `Slot`
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    pub batch_count: u64,
    pub batch_acc: FixedBytes<32>,
    pub message_count: u64,
    pub message_acc: FixedBytes<32>,
    pub timestamp: u64,
    pub slot: Slot,
}

/// `MBlock` contains all information necessary to build a `Block`
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MBlock {
    pub timestamp: u64,
    pub slot: Slot,
    pub payload: Option<(Bytes, Vec<DelayedMessage>)>,
}

/// Block data stored in rocksdb
/// Note that the block hash does not affect derived block hashes and therefore
/// this implementation should be fully compatible with existing reth `MockChains`.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Block {
    /// block epoch timestamp in seconds
    pub timestamp: u64,
    /// batch data
    pub batch: Bytes,
    /// accumulator
    pub after_batch_acc: FixedBytes<32>,
    /// delayed messages included in the batch & accumulator values
    pub messages: Vec<(DelayedMessage, FixedBytes<32>)>,
    /// previous sequencer inbox accumulator
    /// note that this is used to detect reorgs instead of block hash
    pub before_batch_acc: FixedBytes<32>,
    /// previous delayed message (inbox) accumulator
    /// note that this is used to detect reorgs instead of block hash
    pub before_message_acc: FixedBytes<32>,
    /// previous delayed messages read
    pub before_message_count: u64,
    /// reorg data
    pub slot: Slot,
}

#[allow(missing_docs)]
impl Block {
    pub fn after_message_acc(&self) -> FixedBytes<32> {
        self.messages.last().map_or(self.before_message_acc, |x| x.1)
    }
    pub fn after_message_count(&self) -> u64 {
        self.before_message_count + self.messages.len() as u64
    }
}

/// rocksdb implements the key-value trait
#[allow(clippy::unwrap_used)]
impl<T: ThreadMode> ArbitrumDB for DBWithThreadMode<T> {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
        self.get(key).unwrap().map(|x| x.into())
    }
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
        self.put(key, value).unwrap();
    }
    fn delete<K: AsRef<[u8]>>(&self, key: K) {
        self.delete(key).unwrap();
    }
}

/// Database key types for different stored values
#[derive(Debug, Clone)]
pub enum DBKey {
    /// Block data with block number
    Block(u64),
    /// State of the chain at the latest head
    State,
    /// Message accumulator with message number
    MessageAcc(u64),
    /// DB schema version
    Version,
}

impl fmt::Display for DBKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(num) => write!(f, "b{num}"),
            Self::State => write!(f, "s"),
            Self::MessageAcc(num) => write!(f, "m{num}"),
            Self::Version => write!(f, "v"),
        }
    }
}

/// generic db trait for reading and writing block data
#[allow(clippy::unwrap_used)]
pub trait ArbitrumDB {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes>;
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V);
    fn delete<K: AsRef<[u8]>>(&self, key: K);
    fn get_block(&self, key: u64) -> Result<Block, ErrorObjectOwned> {
        let state = self.get_state();
        if key <= state.batch_count { self.get(DBKey::Block(key).to_string()) } else { None }
            .map_or_else(
                || Err(to_err(format!("could not find block {}", key))),
                |x| Ok(bincode::deserialize(&x).unwrap()),
            )
    }
    fn put_block(&self, key: u64, value: &Block) {
        self.put(DBKey::Block(key).to_string(), bincode::serialize(value).unwrap())
    }
    fn delete_block(&self, key: u64) {
        self.delete(DBKey::Block(key).to_string())
    }
    fn get_message_acc(&self, key: u64) -> Result<FixedBytes<32>, ErrorObjectOwned> {
        let state = self.get_state();
        if key < state.message_count { self.get(DBKey::MessageAcc(key).to_string()) } else { None }
            .map_or_else(
                || Err(to_err(format!("could not find message acc {}", key))),
                |x| Ok(bincode::deserialize(&x).unwrap()),
            )
    }
    fn put_message_acc(&self, key: u64, value: &FixedBytes<32>) {
        self.put(DBKey::MessageAcc(key).to_string(), bincode::serialize(value).unwrap())
    }
    fn delete_message_acc(&self, key: u64) {
        self.delete(DBKey::MessageAcc(key).to_string())
    }
    fn get_state(&self) -> State {
        self.get(DBKey::State.to_string())
            .map(|x| bincode::deserialize(&x).unwrap())
            .unwrap_or_default()
    }
    fn put_state(&self, value: &State) {
        self.put(DBKey::State.to_string(), bincode::serialize(value).unwrap())
    }
    fn check_version(&self) {
        match self.get(DBKey::Version.to_string()) {
            Some(version) => {
                assert_eq!(bincode::deserialize::<u64>(&version).unwrap(), VERSION);
            }
            None => {
                // version 0 uses the "n" key to store the current block number
                assert_eq!(self.get("n"), None, "version mismatch: found 0 expected {}", VERSION);
                self.put(DBKey::Version.to_string(), bincode::serialize(&VERSION).unwrap());
            }
        }
    }
    // returns the block number if a new block is added
    fn add_batch(&self, mblock: MBlock) -> Result<Option<u64>, ErrorObjectOwned> {
        let state = self.get_state();
        if state.batch_count == 0 && mblock.payload.is_none() {
            return Err(to_err("invalid first batch: must contain a payload"))
        }
        if state.batch_count > 0 &&
            (mblock.timestamp < state.timestamp ||
                (state.slot.seq_block_number > 0 &&
                    mblock.slot.seq_block_number != state.slot.seq_block_number + 1) ||
                mblock.slot.seq_block_number <= state.slot.seq_block_number ||
                mblock.slot.set_block_number < state.slot.set_block_number)
        {
            return Err(to_err(format!(
                "invalid batch: timestamp {} < {} or seq block {} != {} or set block {} < {}",
                mblock.timestamp,
                state.timestamp,
                mblock.slot.seq_block_number,
                state.slot.seq_block_number + 1,
                mblock.slot.set_block_number,
                state.slot.set_block_number
            )))
        }
        // if the payload is empty, update the state with pending slot / timestamp info and return
        let (batch, messages) = match mblock.payload {
            Some(payload) => payload,
            None => {
                self.put_state(&State { timestamp: mblock.timestamp, slot: mblock.slot, ..state });
                return Ok(None);
            }
        };
        let block_number = state.batch_count + 1;
        let mut block = Block {
            timestamp: mblock.timestamp,
            batch,
            slot: mblock.slot,
            before_batch_acc: state.batch_acc,
            before_message_count: state.message_count,
            before_message_acc: state.message_acc,
            messages: messages.iter().map(|x| (x.to_owned(), FixedBytes::ZERO)).collect(),
            after_batch_acc: Default::default(),
        };
        let mut before_inbox_acc = block.before_message_acc;
        for (i, (msg, acc)) in block.messages.iter_mut().enumerate() {
            let message_hash = keccak256(
                (
                    [msg.kind],
                    msg.sender,
                    block_number,
                    mblock.timestamp,
                    U256::from(block.before_message_count + i as u64),
                    msg.base_fee_l1,
                    keccak256(&msg.data),
                )
                    .abi_encode_packed(),
            );
            before_inbox_acc = keccak256((before_inbox_acc, message_hash).abi_encode_packed());
            *acc = before_inbox_acc;
            self.put_message_acc(block.before_message_count + i as u64, &before_inbox_acc);
        }
        let data_hash = keccak256(
            (
                0u64,     // minTimestamp
                u64::MAX, // maxTimestamp
                0u64,     // minBlockNumber
                u64::MAX, // maxBlockNumber
                block.after_message_count(),
                &block.batch,
            )
                .abi_encode_packed(),
        );
        block.after_batch_acc = keccak256(
            (block.before_batch_acc, data_hash, block.after_message_acc()).abi_encode_packed(),
        );
        self.put_block(block_number, &block);
        // update the state last - incomplete blocks can be ignored / overwritten
        self.put_state(&State {
            batch_count: block_number,
            batch_acc: block.after_batch_acc,
            message_count: block.after_message_count(),
            message_acc: block.after_message_acc(),
            timestamp: block.timestamp,
            slot: block.slot,
        });
        Ok(Some(block_number))
    }
}

pub(crate) fn to_err<T: ToString>(err: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, err.to_string(), None::<()>)
}

// fully in-memory kv db for testing
#[cfg(test)]
pub(crate) mod tests {
    use super::ArbitrumDB;
    use crate::db::{MBlock, Slot};
    use alloy::primitives::Bytes;
    use std::{collections::HashMap, sync::RwLock};

    #[allow(clippy::redundant_pub_crate)]
    #[derive(Debug)]
    pub(crate) struct TestDB(pub(crate) RwLock<HashMap<Bytes, Bytes>>);

    impl TestDB {
        pub(crate) fn new() -> Self {
            Self(RwLock::new(HashMap::new()))
        }
    }

    #[allow(clippy::unwrap_used)]
    impl ArbitrumDB for TestDB {
        fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
            self.0.read().unwrap().get(key.as_ref()).cloned()
        }
        fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
            self.0
                .write()
                .unwrap()
                .insert(key.as_ref().to_owned().into(), value.as_ref().to_owned().into());
        }
        fn delete<K: AsRef<[u8]>>(&self, key: K) {
            self.0.write().unwrap().remove(key.as_ref());
        }
    }

    #[test]
    fn invalid_batch() -> eyre::Result<()> {
        let db = TestDB::new();

        // first batch must contain a payload
        assert!(db.add_batch(MBlock { payload: None, ..Default::default() }).is_err());

        for payload in [None, Some(Default::default())] {
            let db = TestDB::new();
            db.add_batch(MBlock { payload: Some(Default::default()), ..Default::default() })?;

            // not incrementing the seq block number fails
            assert!(db
                .add_batch(MBlock { timestamp: 1, payload: payload.clone(), ..Default::default() })
                .is_err());

            // initially the seq block number can be set arbitarily high
            db.add_batch(MBlock {
                slot: Slot { seq_block_number: 2, ..Default::default() },
                payload: payload.clone(),
                ..Default::default()
            })?;

            // it can no longer be incremented by more than 1 thereafter
            assert!(db
                .add_batch(MBlock {
                    slot: Slot { seq_block_number: 4, ..Default::default() },
                    payload: payload.clone(),
                    ..Default::default()
                })
                .is_err());

            // not incrementing timestamp is okay
            db.add_batch(MBlock {
                slot: Slot { seq_block_number: 3, ..Default::default() },
                payload: payload.clone(),
                ..Default::default()
            })?;

            // incrementing both timestamp and seq block number is okay
            db.add_batch(MBlock {
                timestamp: 1,
                slot: Slot { seq_block_number: 4, ..Default::default() },
                payload: payload.clone(),
            })?;

            // decrementing timestamp fails
            assert!(db
                .add_batch(MBlock {
                    slot: Slot { seq_block_number: 5, ..Default::default() },
                    payload: payload.clone(),
                    ..Default::default()
                })
                .is_err());

            // decrementing seq block number fails, even if ts is incremented
            assert!(db
                .add_batch(MBlock {
                    timestamp: 2,
                    slot: Slot { seq_block_number: 3, ..Default::default() },
                    payload,
                })
                .is_err());
        }
        Ok(())
    }
}
