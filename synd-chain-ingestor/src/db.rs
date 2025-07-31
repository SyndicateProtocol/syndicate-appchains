//! The `synd-chain-ingestor` db uses an append-only file format to persist fixed-size items to
//! disk.
use crate::metrics::ChainIngestorMetrics;
use alloy::{
    primitives::{Bytes, B256},
    rpc::types::Header,
};
use fs2::FileExt as _;
use shared::{tracing::SpanKind, types::BlockRef};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::{FileExt, MetadataExt as _},
};
use tracing::{debug, info, instrument, warn};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct DB {
    file: File,
    pub start_block: u64,
    pub count: u64,
}

/// 4 bytes for the block timestamp + 32 bytes for the block hash
///
/// The first item is the header - this contains the version byte followed by the start block number
/// (`u64`) followed by the chain id (`u64`). The remaining 19 bytes are empty and reserved for
/// custom metadata.
pub const ITEM_SIZE: u64 = 36;

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
#[allow(clippy::unwrap_used, clippy::cognitive_complexity)]
impl DB {
    #[instrument(fields(otel.kind = ?SpanKind::Internal))]
    pub fn open(file_name: &str, start_block: u64, chain_id: u64) -> std::io::Result<Self> {
        let mut file = OpenOptions::new().read(true).append(true).create(true).open(file_name)?;
        file.lock_exclusive()?;
        let metadata = file.metadata()?;
        let metadata_size = metadata.size();
        if metadata_size < ITEM_SIZE {
            info!("creating new db {}", file_name);
            file.set_len(0)?;
            file.write_all(&[1])?;
            file.write_all(&start_block.to_be_bytes())?;
            file.write_all(&chain_id.to_be_bytes())?;
            file.write_all(&[0; 19])?;
        } else if metadata_size % ITEM_SIZE != 0 {
            info!("removing corrupt entry from db");
            file.set_len(metadata_size - (metadata_size % ITEM_SIZE))?;
        }
        let size = file.metadata()?.size();
        assert!(size >= ITEM_SIZE && size % ITEM_SIZE == 0, "unexpected file size found: {size}");
        let mut version = [0];
        file.read_exact_at(&mut version, 0)?;
        assert_eq!(version, [1]);
        let mut buf = [0; 8];
        file.read_exact_at(&mut buf, 1)?;
        let db_start_block = u64::from_be_bytes(buf);
        assert!(
            db_start_block <= start_block,
            "configured db start block {db_start_block} greater than actual db start block {start_block}"
        );
        file.read_exact_at(&mut buf, 9)?;
        assert_eq!(chain_id, u64::from_be_bytes(buf));
        let count = size / ITEM_SIZE - 1;
        Ok(Self { file, start_block: db_start_block, count })
    }

    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Internal))]
    pub fn get_block(&self, block: u64) -> BlockRef {
        assert!(self.in_range(block));
        let mut data = [0; ITEM_SIZE as usize];
        self.file.read_exact_at(&mut data, (block - self.start_block + 1) * ITEM_SIZE).unwrap();
        BlockRef {
            number: block,
            timestamp: u32::from_be_bytes(data[..4].try_into().unwrap()) as u64,
            hash: B256::from_slice(data[4..].into()),
        }
    }

    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Internal))]
    pub fn get_block_bytes(&self, from: u64) -> Bytes {
        assert!(self.in_range(from));
        let mut data = vec![0; ((self.next_block() - from) * ITEM_SIZE) as usize];
        self.file.read_exact_at(&mut data, (from - self.start_block + 1) * ITEM_SIZE).unwrap();
        data.into()
    }

    fn add_block(&mut self, ts: u32, hash: B256) {
        debug!("adding block {}: ts={}, hash={}", self.next_block(), ts, hash);
        self.file.write_all(&[ts.to_be_bytes().as_slice(), hash.as_slice()].concat()).unwrap();
        self.count += 1;
    }

    fn reorg_block(&mut self, next: u64) {
        warn!("reorging next block from {} to {}", self.next_block(), next);
        assert!(self.in_range(next));
        self.count = next - self.start_block;
        self.file.set_len((self.count + 1) * ITEM_SIZE).unwrap();
    }

    #[instrument(skip(self, metrics), fields(otel.kind = ?SpanKind::Internal))]
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
        if self.count > 0 {
            let prev = self.get_block(header.number - 1);
            if header.parent_hash != prev.hash {
                self.reorg_block(prev.number);
                metrics.record_reorg(1);
                return BlockUpdateResult::Reorged;
            }
        }
        self.add_block(header.timestamp as u32, header.hash);
        metrics.record_block(header.number, header.timestamp);
        BlockUpdateResult::Added
    }

    pub const fn next_block(&self) -> u64 {
        self.start_block + self.count
    }

    pub const fn in_range(&self, block: u64) -> bool {
        block >= self.start_block && block < self.next_block()
    }
}
