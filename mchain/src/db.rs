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

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MBlock {
    pub timestamp: u64,
    pub messages: Vec<DelayedMessage>,
    pub batch: Bytes,
    pub seq_block_number: u64,
    pub seq_block_hash: FixedBytes<32>,
    pub set_block_number: u64,
    pub set_block_hash: FixedBytes<32>,
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
}

impl fmt::Display for DBKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(num) => write!(f, "b{num}"),
            Self::CurrentBlockNumber => write!(f, "n"),
            Self::MessageAcc(num) => write!(f, "m{num}"),
        }
    }
}

/// generic db trait for reading and writing block data
#[allow(missing_docs)]
pub trait ArbitrumDB {
    fn get_block(&self, key: u64) -> Result<Block, ErrorObjectOwned>;
    fn put_block(&self, key: u64, value: Block);
    fn delete_block(&self, key: u64);
    fn get_block_number(&self) -> u64;
    fn put_block_number(&self, value: u64);
    fn get_message_acc(&self, key: u64) -> Result<FixedBytes<32>, ErrorObjectOwned>;
    fn put_message_acc(&self, key: u64, value: FixedBytes<32>);
    fn delete_message_acc(&self, key: u64);
    /// Create a new block that a contains a batch
    fn add_batch(&self, block: MBlock) -> Result<(), ErrorObjectOwned>;
}

impl<T: KVDB> ArbitrumDB for T {
    fn get_block(&self, key: u64) -> Result<Block, ErrorObjectOwned> {
        self.get(DBKey::Block(key).to_string()).map_or_else(
            || Err(to_err(format!("could not find block {}", key))),
            #[allow(clippy::unwrap_used)]
            |x| Ok(bincode::deserialize(&x).unwrap()),
        )
    }
    fn put_block(&self, key: u64, value: Block) {
        #[allow(clippy::unwrap_used)]
        self.put(DBKey::Block(key).to_string(), bincode::serialize(&value).unwrap())
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
    fn put_message_acc(&self, key: u64, value: FixedBytes<32>) {
        #[allow(clippy::unwrap_used)]
        self.put(DBKey::MessageAcc(key).to_string(), bincode::serialize(&value).unwrap())
    }
    fn delete_message_acc(&self, key: u64) {
        self.delete(DBKey::MessageAcc(key).to_string())
    }
    fn add_batch(&self, mblock: MBlock) -> Result<(), ErrorObjectOwned> {
        let block_number = self.get_block_number() + 1;
        let prev_block = if block_number > 1 {
            self.get_block(block_number - 1)?
        } else {
            // genesis block
            Block::default()
        };
        if mblock.timestamp < prev_block.timestamp {
            return Err(to_err(format!(
                "batch timestamp too low: {} < {}",
                mblock.timestamp, prev_block.timestamp
            )));
        }
        let mut block = Block {
            timestamp: mblock.timestamp,
            batch: mblock.batch,
            seq_block_number: mblock.seq_block_number,
            seq_block_hash: mblock.seq_block_hash,
            set_block_number: mblock.set_block_number,
            set_block_hash: mblock.set_block_hash,
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
            self.put_message_acc(block.before_message_count + i as u64, before_inbox_acc);
        }
        let data_hash = keccak256(
            (0u64, u64::MAX, 0u64, u64::MAX, block.after_message_count()).abi_encode_packed(),
        );
        block.after_batch_acc = keccak256(
            (block.before_batch_acc, data_hash, block.after_message_acc()).abi_encode_packed(),
        );
        self.put_block(block_number, block);
        // update the block number last - incomplete blocks can be ignored
        self.put_block_number(block_number);
        Ok(())
    }
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
    pub seq_block_number: u64,
    pub seq_block_hash: FixedBytes<32>,
    pub set_block_number: u64,
    pub set_block_hash: FixedBytes<32>,
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

#[allow(missing_docs)]
pub fn to_err<T: ToString>(err: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, err.to_string(), None::<()>)
}
