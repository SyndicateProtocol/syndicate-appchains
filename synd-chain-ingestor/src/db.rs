//! The synd-chain-ingestor db uses an append-only file to persist fixed-size items to disk.
use crate::metrics::ChainIngestorMetrics;
use alloy::{
    primitives::{Bytes, B256},
    rpc::types::Header,
};
use shared::{
    fixed_size_append_only_db::{FixedSizeAppendOnlyDB, FixedSizeAppendOnlyT},
    types::BlockRef,
};
use tracing::{debug, warn};

/// 4 bytes for the block timestamp + 32 bytes for the block hash
// The first item is the header - this contains the version byte followed by the start block number
// (u64) followed by the chain id (u64). The remaining 19 bytes are empty and reserved for custom
// metadata.
pub const ITEM_SIZE: usize = 36;

#[derive(Debug)]
#[allow(missing_docs)]
pub struct DB<FixedSizeAppendOnlyDB: FixedSizeAppendOnlyT<ITEM_SIZE>> {
    db: FixedSizeAppendOnlyDB,
    pub start_block: u64,
}

/// The effect of an `update_block()` call on the database
#[derive(Debug, PartialEq, Eq)]
pub enum BlockUpdateResult {
    /// The block is added to the db, with or without a reorg
    Added,
    /// The db is reorged to the parent of the parent of the block
    Reorged,
    /// The db is not updated as the block hash matches the one in the db
    Verified,
}

#[allow(missing_docs)]
impl DB<FixedSizeAppendOnlyDB<ITEM_SIZE>> {
    pub fn open(file_name: &str, start_block: u64, chain_id: u64) -> std::io::Result<Self> {
        let header = [
            [1].as_slice(),
            start_block.to_be_bytes().as_slice(),
            chain_id.to_be_bytes().as_slice(),
            [0; 19].as_slice(),
        ]
        .concat();
        Ok(Self {
            #[allow(clippy::unwrap_used)]
            db: FixedSizeAppendOnlyDB::open(file_name, &header.try_into().unwrap())?,
            start_block,
        })
    }

    pub fn get_block_bytes(&self, from: u64) -> Bytes {
        assert!(self.in_range(from));
        self.db.get_bytes(from - self.start_block + 1).into()
    }
}

#[allow(missing_docs)]
#[allow(clippy::unwrap_used)]
impl<T: FixedSizeAppendOnlyT<ITEM_SIZE>> DB<T> {
    pub fn get_block(&self, block: u64) -> BlockRef {
        assert!(self.in_range(block));
        let data = self.db.get(block - self.start_block + 1);
        BlockRef {
            number: block,
            timestamp: u32::from_be_bytes(data[..4].try_into().unwrap()) as u64,
            hash: B256::from_slice(data[4..].into()),
        }
    }

    fn reorg_block(&mut self, next: u64) {
        warn!("reorging next block from {} to {}", self.next_block(), next);
        assert!(self.in_range(next));
        self.db.truncate(next - self.start_block);
    }

    pub fn update_block(
        &mut self,
        header: &Header,
        metrics: &ChainIngestorMetrics,
    ) -> BlockUpdateResult {
        let next_block = self.next_block();
        if header.number < next_block {
            let block = self.get_block(header.number);
            if block.hash == header.hash {
                return BlockUpdateResult::Verified;
            }
            self.reorg_block(header.number);
            metrics.record_reorg(next_block - header.number);
        }
        let next_block = self.next_block();
        assert_eq!(header.number, next_block);
        if self.count() > 0 {
            let prev = self.get_block(header.number - 1);
            if header.parent_hash != prev.hash {
                self.reorg_block(prev.number);
                metrics.record_reorg(1);
                return BlockUpdateResult::Reorged;
            }
        }
        debug!(
            "adding block {}: ts={}, hash={}",
            self.next_block(),
            header.timestamp as u32,
            header.hash
        );
        self.db.append(
            &[(header.timestamp as u32).to_be_bytes().as_slice(), header.hash.as_slice()]
                .concat()
                .try_into()
                .unwrap(),
        );
        metrics.record_block(header.number, header.timestamp);
        BlockUpdateResult::Added
    }

    pub fn next_block(&self) -> u64 {
        self.start_block + self.count()
    }

    pub fn in_range(&self, block: u64) -> bool {
        block >= self.start_block && block < self.next_block()
    }

    pub fn count(&self) -> u64 {
        self.db.count()
    }
}

#[cfg(test)]
mod tests {
    use super::{DB, ITEM_SIZE};
    use shared::fixed_size_append_only_db::FixedSizeAppendOnlyT;

    struct TestDB(Vec<Vec<u8>>);

    impl FixedSizeAppendOnlyT<ITEM_SIZE> for TestDB {
        fn get(&self, index: u64) -> [u8; ITEM_SIZE] {
            self.0[index as usize - 1].as_slice().try_into().unwrap()
        }
        fn append(&mut self, item: &[u8; ITEM_SIZE]) {
            self.0.push(item.to_vec());
        }
        fn truncate(&mut self, count: u64) {
            self.0.truncate(count as usize);
        }
        fn count(&self) -> u64 {
            self.0.len() as u64
        }
    }

    #[allow(dead_code)]
    fn test_db(start_block: u64) -> DB<TestDB> {
        DB { db: TestDB(Default::default()), start_block }
    }
}
