use alloy::{
    primitives::{keccak256, Address, Bytes, FixedBytes, U256},
    sol_types::SolValue as _,
};
use jsonrpsee::types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned};
use serde::{Deserialize, Serialize};
use shared::{
    append_only_db::AppendOnlyT, fixed_size_append_only_db::FixedSizeAppendOnlyT,
    single_value_db::SingleValueT,
};
use std::fmt;
use tracing::warn;

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

#[derive(Debug)]
#[allow(missing_docs)]
pub struct ArbitrumDB<
    AppendOnlyDB: AppendOnlyT,
    FixedSizeAppendOnlyDB: FixedSizeAppendOnlyT<32>,
    SingleValueDB: SingleValueT,
> {
    pub block: AppendOnlyDB,
    pub message_acc: FixedSizeAppendOnlyDB,
    pub state: SingleValueDB,
}

/// generic db trait for reading and writing block data
#[allow(clippy::unwrap_used)]
impl<T: AppendOnlyT, U: FixedSizeAppendOnlyT<32>, V: SingleValueT> ArbitrumDB<T, U, V> {
    /// this function should be called after loading the db from disk
    pub fn sync_to_state(&mut self) {
        let state = self.get_state();
        if self.block.count() != state.batch_count {
            warn!("removing corrupt blocks");
            self.block.truncate(state.batch_count);
        }
        assert_eq!(self.block.count(), state.batch_count);
        if self.message_acc.count() != state.message_count {
            warn!("removing corrupt messages");
            self.message_acc.truncate(state.message_count);
        }
        assert_eq!(self.message_acc.count(), state.message_count);
    }
    pub fn get_block(&self, block: u64) -> Result<Block, ErrorObjectOwned> {
        if block > 0 && block <= self.block.count() {
            Ok(bincode::deserialize(&self.block.get(block)).unwrap())
        } else {
            Err(to_err(format!("could not find block {}", block)))
        }
    }
    pub fn get_message_acc(&self, index: u64) -> Result<FixedBytes<32>, ErrorObjectOwned> {
        if index < self.message_acc.count() {
            Ok(self.message_acc.get(index + 1).into())
        } else {
            Err(to_err(format!("could not find message acc {}", index)))
        }
    }
    pub fn get_state(&self) -> State {
        self.state.get().map(|x| bincode::deserialize(x).unwrap()).unwrap_or_default()
    }
    pub fn rollback_to_block(&mut self, block_number: u64) -> Block {
        let block = self.get_block(block_number).unwrap();

        // first update the state - it is okay if the other deletions fail, incomplete
        // data is ignored.
        self.state.set(
            bincode::serialize(&State {
                batch_count: block_number,
                batch_acc: block.after_batch_acc,
                message_count: block.after_message_count(),
                message_acc: block.after_message_acc(),
                timestamp: block.timestamp,
                slot: block.slot.clone(),
            })
            .unwrap(),
        );

        // next delete blocks, messages to free up space.
        self.block.truncate(block_number);
        self.message_acc.truncate(block.after_message_count());

        block
    }
    // returns the block number if a new block is added
    pub fn add_batch(&mut self, mblock: MBlock) -> Result<Option<u64>, ErrorObjectOwned> {
        let state = self.get_state();
        assert_eq!(state.batch_count, self.block.count());
        assert_eq!(state.message_count, self.message_acc.count());
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
                self.state.set(
                    bincode::serialize(&State {
                        timestamp: mblock.timestamp,
                        slot: mblock.slot,
                        ..state
                    })
                    .unwrap(),
                );
                return Ok(None);
            }
        };
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
                    block.slot.seq_block_number,
                    mblock.timestamp,
                    U256::from(block.before_message_count + i as u64),
                    msg.base_fee_l1,
                    keccak256(&msg.data),
                )
                    .abi_encode_packed(),
            );
            before_inbox_acc = keccak256((before_inbox_acc, message_hash).abi_encode_packed());
            *acc = before_inbox_acc;
            self.message_acc.append(&before_inbox_acc);
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
        self.block.append(&bincode::serialize(&block).unwrap());
        self.state.set(
            bincode::serialize(&State {
                batch_count: state.batch_count + 1,
                batch_acc: block.after_batch_acc,
                message_count: block.after_message_count(),
                message_acc: block.after_message_acc(),
                timestamp: block.timestamp,
                slot: block.slot.clone(),
            })
            .unwrap(),
        );
        Ok(Some(state.batch_count + 1))
    }
}

pub(crate) fn to_err<T: ToString>(err: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, err.to_string(), None::<()>)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::ArbitrumDB;
    use crate::db::{MBlock, Slot};
    use shared::{
        append_only_db::AppendOnlyT, fixed_size_append_only_db::FixedSizeAppendOnlyT,
        single_value_db::SingleValueT,
    };

    #[allow(clippy::redundant_pub_crate)]
    #[derive(Debug, Default)]
    /// fully in-memory db for testing
    pub(crate) struct TestDB(pub(crate) Vec<Vec<u8>>);

    impl TestDB {
        fn truncate(&mut self, count: u64) {
            self.0.truncate(count as usize);
        }
        fn count(&self) -> u64 {
            self.0.len() as u64
        }
    }

    impl AppendOnlyT for TestDB {
        fn get(&self, index: u64) -> Vec<u8> {
            self.0[index as usize - 1].clone()
        }
        fn append(&mut self, item: &[u8]) {
            self.0.push(item.to_vec());
        }
        fn truncate(&mut self, count: u64) {
            self.truncate(count)
        }
        fn count(&self) -> u64 {
            self.count()
        }
    }

    impl FixedSizeAppendOnlyT<32> for TestDB {
        fn get(&self, index: u64) -> [u8; 32] {
            self.0[index as usize - 1].as_slice().try_into().unwrap()
        }
        fn append(&mut self, item: &[u8; 32]) {
            self.0.push(item.to_vec());
        }
        fn truncate(&mut self, count: u64) {
            self.truncate(count)
        }
        fn count(&self) -> u64 {
            self.count()
        }
    }

    impl SingleValueT for TestDB {
        fn get(&self) -> Option<&Vec<u8>> {
            self.0.first()
        }
        fn set(&mut self, value: Vec<u8>) {
            self.0 = vec![value];
        }
    }

    #[test]
    fn invalid_batch() -> eyre::Result<()> {
        let mut db = ArbitrumDB {
            block: TestDB::default(),
            message_acc: TestDB::default(),
            state: TestDB::default(),
        };

        // first batch must contain a payload
        assert!(db.add_batch(MBlock { payload: None, ..Default::default() }).is_err());

        for payload in [None, Some(Default::default())] {
            let mut db = ArbitrumDB {
                block: TestDB::default(),
                message_acc: TestDB::default(),
                state: TestDB::default(),
            };
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
