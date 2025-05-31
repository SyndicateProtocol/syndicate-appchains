//! This db uses two append-only files to persist variable-size items to disk.

use crate::fixed_size_append_only_db::{FixedSizeAppendOnlyDB, FixedSizeAppendOnlyT as _};
use fs2::FileExt as _;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::{FileExt, MetadataExt as _},
};
use tracing::warn;

/// Append-only db trait
#[allow(missing_docs)]
pub trait AppendOnlyT {
    /// 0 returns the header, elements start from index 1 onwards
    fn get(&self, index: u64) -> Vec<u8>;
    fn append(&mut self, item: &[u8]);
    fn truncate(&mut self, count: u64);
    fn count(&self) -> u64;
}

#[derive(Debug)]
#[allow(missing_docs)]
pub struct AppendOnlyDB {
    file: File,
    index_db: FixedSizeAppendOnlyDB<8>,
}

#[allow(missing_docs)]
impl AppendOnlyDB {
    #[allow(clippy::cognitive_complexity)]
    pub fn open(file_name: &str, header: &[u8]) -> std::io::Result<Self> {
        let mut index_db = FixedSizeAppendOnlyDB::open(
            &(file_name.to_owned() + ".index"),
            &header.len().to_be_bytes(),
        )?;
        let mut file = OpenOptions::new().read(true).append(true).create(true).open(file_name)?;
        file.lock_exclusive()?;
        let file_size = file.metadata()?.size();
        let mut last_index_end_offset = u64::from_be_bytes(index_db.get(index_db.count));
        while index_db.count > 0 && file_size < last_index_end_offset {
            warn!("removing corrupt entry {} from db", index_db.count);
            index_db.truncate(index_db.count - 1);
            last_index_end_offset = u64::from_be_bytes(index_db.get(index_db.count));
        }
        #[allow(clippy::comparison_chain)]
        if file_size < last_index_end_offset {
            if file_size > 0 {
                warn!("file is fully corrupt! clearing db");
                file.set_len(0)?;
            }
            file.write_all(header)?;
        } else if file_size > last_index_end_offset {
            warn!("removing corrupt entry from db");
            file.set_len(last_index_end_offset)?;
        }
        let mut new_header = vec![0; header.len()];
        file.read_exact_at(&mut new_header, 0)?;
        assert_eq!(header, new_header);
        Ok(Self { file, index_db })
    }
}

#[allow(clippy::unwrap_used)]
impl AppendOnlyT for AppendOnlyDB {
    fn get(&self, index: u64) -> Vec<u8> {
        let start = (index > 0)
            .then(|| u64::from_be_bytes(self.index_db.get(index - 1)))
            .unwrap_or_default();
        let end = u64::from_be_bytes(self.index_db.get(index));
        assert!(end >= start);
        let mut buf = vec![0; (end - start) as usize];
        self.file.read_exact_at(&mut buf, start).unwrap();
        buf
    }

    fn append(&mut self, item: &[u8]) {
        self.file.write_all(item).unwrap();
        let prev_size = u64::from_be_bytes(self.index_db.get(self.index_db.count));
        self.index_db.append(&(prev_size + item.len() as u64).to_be_bytes());
    }

    fn truncate(&mut self, count: u64) {
        if count < self.index_db.count {
            self.index_db.truncate(count);
            self.file.set_len(u64::from_be_bytes(self.index_db.get(count))).unwrap();
        }
    }

    fn count(&self) -> u64 {
        self.index_db.count
    }
}
