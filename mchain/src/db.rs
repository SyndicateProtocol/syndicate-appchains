use alloy::{
    primitives::{keccak256, Address, Bytes, FixedBytes, U256},
    sol_types::SolValue as _,
};
use jsonrpsee::types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned};
use rocksdb::{DBWithThreadMode, ThreadMode};
use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(missing_docs)]
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
    pub seq_block_number: u64,
    pub seq_block_hash: FixedBytes<32>,
    pub set_block_number: u64,
    pub set_block_hash: FixedBytes<32>,
}

/// `MBlock` contains all information necessary to build a `Block`
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MBlock {
    pub timestamp: u64,
    pub messages: Vec<DelayedMessage>,
    pub batch: Bytes,
    pub slot: Slot,
}

/// Block data stored in rocksdb
/// Note that the block hash does not affect derived block hashes and therefore
/// this implementation should be fully compatible with existing reth metachains.
#[allow(missing_docs)]
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

/// key-value db trait
#[allow(missing_docs)]
pub trait KVDB {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes>;
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V);
    fn delete<K: AsRef<[u8]>>(&self, key: K);
}

/// rocksdb implements the key-value trait
impl<T: ThreadMode> KVDB for DBWithThreadMode<T> {
    fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
        #[allow(clippy::unwrap_used)]
        self.get(key).unwrap().map(|x| x.into())
    }
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
        #[allow(clippy::unwrap_used)]
        self.put(key, value).unwrap();
    }
    fn delete<K: AsRef<[u8]>>(&self, key: K) {
        #[allow(clippy::unwrap_used)]
        self.delete(key).unwrap();
    }
}

/// Database key types for different stored values
#[derive(Debug, Clone)]
pub enum DBKey {
    /// Block data with block number
    Block(u64),
    /// Block number counter
    CurrentBlockNumber,
    /// Message accumulator with message number
    MessageAcc(u64),
    /// The last ingested slot
    CurrentSlot,
}

impl fmt::Display for DBKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(num) => write!(f, "b{num}"),
            Self::CurrentBlockNumber => write!(f, "n"),
            Self::MessageAcc(num) => write!(f, "m{num}"),
            Self::CurrentSlot => write!(f, "s"),
        }
    }
}

/// generic db trait for reading and writing block data
#[allow(missing_docs)]
pub trait ArbitrumDB {
    fn get_block(&self, key: u64) -> Result<Block, ErrorObjectOwned>;
    fn put_block(&self, key: u64, value: &Block);
    fn delete_block(&self, key: u64);
    fn get_block_number(&self) -> u64;
    fn put_block_number(&self, value: u64);
    fn get_message_acc(&self, key: u64) -> Result<FixedBytes<32>, ErrorObjectOwned>;
    fn put_message_acc(&self, key: u64, value: &FixedBytes<32>);
    fn delete_message_acc(&self, key: u64);
    fn get_slot(&self) -> Option<Slot>;
    fn put_slot(&self, value: &Slot);
    /// Create a new block that a contains a batch
    fn add_batch(&self, block: MBlock) -> Result<u64, ErrorObjectOwned>;
}

impl<T: KVDB> ArbitrumDB for T {
    fn get_block(&self, key: u64) -> Result<Block, ErrorObjectOwned> {
        self.get(DBKey::Block(key).to_string()).map_or_else(
            || Err(to_err(format!("could not find block {}", key))),
            #[allow(clippy::unwrap_used)]
            |x| Ok(bincode::deserialize(&x).unwrap()),
        )
    }
    fn put_block(&self, key: u64, value: &Block) {
        #[allow(clippy::unwrap_used)]
        self.put(DBKey::Block(key).to_string(), bincode::serialize(value).unwrap())
    }
    fn delete_block(&self, key: u64) {
        #[allow(clippy::unwrap_used)]
        self.delete(DBKey::Block(key).to_string())
    }
    // count starts at and defaults to 0
    fn get_block_number(&self) -> u64 {
        #[allow(clippy::unwrap_used)]
        self.get(DBKey::CurrentBlockNumber.to_string())
            .map_or(0, |x| bincode::deserialize(&x).unwrap())
    }
    fn put_block_number(&self, value: u64) {
        #[allow(clippy::unwrap_used)]
        self.put(DBKey::CurrentBlockNumber.to_string(), bincode::serialize(&value).unwrap())
    }
    fn get_message_acc(&self, key: u64) -> Result<FixedBytes<32>, ErrorObjectOwned> {
        self.get(DBKey::MessageAcc(key).to_string()).map_or_else(
            || Err(to_err(format!("could not find message acc {}", key))),
            #[allow(clippy::unwrap_used)]
            |x| Ok(bincode::deserialize(&x).unwrap()),
        )
    }
    fn put_message_acc(&self, key: u64, value: &FixedBytes<32>) {
        #[allow(clippy::unwrap_used)]
        self.put(DBKey::MessageAcc(key).to_string(), bincode::serialize(value).unwrap())
    }
    fn delete_message_acc(&self, key: u64) {
        self.delete(DBKey::MessageAcc(key).to_string())
    }
    fn get_slot(&self) -> Option<Slot> {
        #[allow(clippy::unwrap_used)]
        self.get(DBKey::CurrentSlot.to_string()).map(|x| bincode::deserialize(&x).unwrap())
    }
    fn put_slot(&self, value: &Slot) {
        #[allow(clippy::unwrap_used)]
        self.put(DBKey::CurrentSlot.to_string(), bincode::serialize(value).unwrap())
    }
    fn add_batch(&self, mblock: MBlock) -> Result<u64, ErrorObjectOwned> {
        let block_number = self.get_block_number() + 1;
        let prev_block = if block_number > 1 {
            self.get_block(block_number - 1)?
        } else {
            // genesis block
            Block::default()
        };
        if block_number > 1 &&
            (mblock.timestamp < prev_block.timestamp ||
                mblock.slot.seq_block_number <= prev_block.slot.seq_block_number ||
                mblock.slot.set_block_number < prev_block.slot.set_block_number)
        {
            return Err(to_err(format!(
                "invalid batch: timestamp {} < {} or seq block {} <= {} or set block {} < {}",
                mblock.timestamp,
                prev_block.timestamp,
                mblock.slot.seq_block_number,
                prev_block.slot.seq_block_number,
                mblock.slot.set_block_number,
                prev_block.slot.set_block_number
            )));
        }
        let mut block = Block {
            timestamp: mblock.timestamp,
            batch: mblock.batch,
            slot: mblock.slot,
            before_batch_acc: prev_block.after_batch_acc,
            before_message_count: prev_block.after_message_count(),
            before_message_acc: prev_block.after_message_acc(),
            messages: mblock.messages.iter().map(|x| (x.to_owned(), FixedBytes::ZERO)).collect(),
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
            (0u64, u64::MAX, 0u64, u64::MAX, block.after_message_count()).abi_encode_packed(),
        );
        block.after_batch_acc = keccak256(
            (block.before_batch_acc, data_hash, block.after_message_acc()).abi_encode_packed(),
        );
        self.put_block(block_number, &block);
        self.put_slot(&block.slot);
        // update the block number last - incomplete blocks can be ignored
        self.put_block_number(block_number);
        Ok(block_number)
    }
}

#[allow(missing_docs)]
pub fn to_err<T: ToString>(err: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, err.to_string(), None::<()>)
}

// fully in-memory kv db for testing
#[cfg(test)]
pub(crate) mod tests {
    use super::KVDB;
    use crate::db::{ArbitrumDB as _, MBlock, Slot};
    use alloy::primitives::Bytes;
    use std::{collections::HashMap, sync::RwLock};

    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct TestDB(pub RwLock<HashMap<Bytes, Bytes>>);
    impl TestDB {
        pub(crate) fn new() -> Self {
            Self(RwLock::new(HashMap::new()))
        }
    }
    impl KVDB for TestDB {
        fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
            #[allow(clippy::unwrap_used)]
            self.0.read().unwrap().get(key.as_ref()).cloned()
        }
        fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
            #[allow(clippy::unwrap_used)]
            self.0
                .write()
                .unwrap()
                .insert(key.as_ref().to_owned().into(), value.as_ref().to_owned().into());
        }
        fn delete<K: AsRef<[u8]>>(&self, key: K) {
            #[allow(clippy::unwrap_used)]
            self.0.write().unwrap().remove(key.as_ref());
        }
    }

    #[test]
    fn invalid_batch() -> eyre::Result<()> {
        let db = TestDB::new();
        db.add_batch(Default::default())?;

        // not incrementing the seq block number fails
        assert!(db.add_batch(MBlock { timestamp: 1, ..Default::default() }).is_err());

        // not incrementing timestamp is okay
        db.add_batch(MBlock {
            slot: Slot { seq_block_number: 1, ..Default::default() },
            ..Default::default()
        })?;

        // incrementing both timestamp and seq block number is okay
        db.add_batch(MBlock {
            timestamp: 1,
            slot: Slot { seq_block_number: 2, ..Default::default() },
            ..Default::default()
        })?;

        // decrementing timestamp fails
        assert!(db
            .add_batch(MBlock {
                slot: Slot { seq_block_number: 3, ..Default::default() },
                ..Default::default()
            })
            .is_err());

        // decrementing seq block number fails
        assert!(db
            .add_batch(MBlock {
                timestamp: 2,
                slot: Slot { seq_block_number: 2, ..Default::default() },
                ..Default::default()
            })
            .is_err());

        Ok(())
    }
}
