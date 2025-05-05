//! Db
use alloy::primitives::B256;
use fs2::FileExt as _;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::{FileExt, MetadataExt as _},
};
use tracing::{debug, info, trace, warn};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct DB {
    file: File,
    pub start_block: u64,
    pub count: u64,
}

// 4 bytes for the block timestamp + 32 bytes for the block hash
//
// The first item is the header - this contains the version byte followed by the
// start block number (u64) followed by the chain id (u64).
// The remaining 19 bytes are empty and reserved for custom metadata.
const ITEM_SIZE: u64 = 36;

#[allow(missing_docs)]
impl DB {
    pub fn open(file_name: &str, start_block: u64, chain_id: u64) -> std::io::Result<DB> {
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
        if size < ITEM_SIZE || size % ITEM_SIZE != 0 {
            panic!("unexpected file size found: {}", size);
        }
        let mut version = [0];
        file.read_exact_at(&mut version, 0)?;
        assert_eq!(version, [1]);
        let mut buf = [0; 8];
        file.read_exact_at(&mut buf, 1)?;
        let db_start_block = u64::from_be_bytes(buf);
        assert!(db_start_block <= start_block);
        file.read_exact_at(&mut buf, 9)?;
        assert_eq!(chain_id, u64::from_be_bytes(buf));
        let count = size / ITEM_SIZE - 1;
        Ok(DB { file, start_block: db_start_block, count })
    }
    pub fn add_block(&mut self, ts: u32, hash: B256) {
        debug!("adding block {}: ts={}, hash={}", self.next_block(), ts, hash);
        self.file.write_all(&[ts.to_be_bytes().as_slice(), hash.as_slice()].concat()).unwrap();
        self.count += 1;
    }
    pub fn get_block(&self, block: u64) -> Option<(u32, B256)> {
        trace!("getting block {}", block);
        if block < self.start_block || block >= self.next_block() {
            return None
        }
        let mut data = [0; ITEM_SIZE as usize];
        self.file.read_exact_at(&mut data, (block - self.start_block + 1) * ITEM_SIZE).unwrap();
        Some((
            u32::from_be_bytes(data[..4].try_into().unwrap()),
            B256::from_slice(data[4..].try_into().unwrap()),
        ))
    }
    pub fn reorg_block(&mut self, next: u64) {
        warn!("reorging next block from {} to {}", self.next_block(), next);
        assert!(next >= self.start_block && next < self.next_block());
        self.count = next - self.start_block;
        self.file.set_len((self.count + 1) * ITEM_SIZE).unwrap();
    }
    pub fn next_block(&self) -> u64 {
        self.start_block + self.count
    }
}
