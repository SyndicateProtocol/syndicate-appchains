//! This db uses an append-only file to persist fixed-size items to disk.
use fs2::FileExt as _;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::{FileExt, MetadataExt as _},
};
use tracing::{info, warn};

#[allow(missing_docs)]
pub trait FixedSizeAppendOnlyT<const ITEM_SIZE: usize> {
    /// 0 returns the header, elements start from index 1 onwards
    fn get(&self, index: u64) -> [u8; ITEM_SIZE];
    fn append(&mut self, item: &[u8; ITEM_SIZE]);
    fn truncate(&mut self, count: u64);
    fn count(&self) -> u64;
}

#[derive(Debug)]
#[allow(missing_docs)]
pub struct FixedSizeAppendOnlyDB<const ITEM_SIZE: usize> {
    pub file: File,
    pub count: u64,
}

#[allow(missing_docs)]
impl<const ITEM_SIZE: usize> FixedSizeAppendOnlyDB<ITEM_SIZE> {
    const _OK: () = assert!(ITEM_SIZE > 0);

    pub fn open(file_name: &str, header: &[u8; ITEM_SIZE]) -> std::io::Result<Self> {
        let mut file = OpenOptions::new().read(true).append(true).create(true).open(file_name)?;
        file.lock_exclusive()?;
        let metadata_size = file.metadata()?.size();
        if metadata_size < ITEM_SIZE as u64 {
            info!("creating new db {}", file_name);
            file.set_len(0)?;
            file.write_all(header)?;
        } else if metadata_size % ITEM_SIZE as u64 != 0 {
            warn!("removing corrupt entry from fixed size db");
            file.set_len(metadata_size - (metadata_size % ITEM_SIZE as u64))?;
        }
        let size = file.metadata()?.size();
        assert!(
            size >= ITEM_SIZE as u64 && size % ITEM_SIZE as u64 == 0,
            "unexpected file size found: {}",
            size
        );
        let mut new_header = [0; ITEM_SIZE];
        file.read_exact_at(&mut new_header, 0)?;
        assert_eq!(header, &new_header);
        let count = size / ITEM_SIZE as u64 - 1;
        Ok(Self { file, count })
    }

    pub fn get_bytes(&self, start_index: u64) -> Vec<u8> {
        let mut data = vec![0; (self.count - start_index + 1) as usize * ITEM_SIZE];
        #[allow(clippy::unwrap_used)]
        self.file.read_exact_at(&mut data, start_index * ITEM_SIZE as u64).unwrap();
        data
    }
}

#[allow(clippy::unwrap_used)]
impl<const ITEM_SIZE: usize> FixedSizeAppendOnlyT<ITEM_SIZE> for FixedSizeAppendOnlyDB<ITEM_SIZE> {
    fn get(&self, index: u64) -> [u8; ITEM_SIZE] {
        let mut data = [0; ITEM_SIZE];
        self.file.read_exact_at(&mut data, index * ITEM_SIZE as u64).unwrap();
        data
    }

    fn append(&mut self, item: &[u8; ITEM_SIZE]) {
        self.file.write_all(item).unwrap();
        self.count += 1;
    }

    fn truncate(&mut self, count: u64) {
        if count < self.count {
            self.count = count;
            self.file.set_len((count + 1) * ITEM_SIZE as u64).unwrap();
        }
    }

    fn count(&self) -> u64 {
        self.count
    }
}
