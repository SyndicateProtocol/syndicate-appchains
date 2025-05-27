//! This db uses two files to safely store a single variable-sized value

use fs2::FileExt as _;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    mem::swap,
    os::unix::fs::{FileExt, MetadataExt as _},
    time::SystemTime,
};

#[allow(missing_docs)]
pub trait SingleValueT {
    fn get(&self) -> Option<&Vec<u8>>;
    fn set(&mut self, value: Vec<u8>);
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SingleValueDB {
    files: [File; 2],
    is_first: bool,
    data: Option<Vec<u8>>,
}

#[allow(clippy::type_complexity)]
fn open_file(file_name: &str) -> std::io::Result<(File, SystemTime, Option<Vec<u8>>)> {
    let file = OpenOptions::new().read(true).append(true).create(true).open(file_name)?;
    file.lock_exclusive()?;
    let metadata = file.metadata()?;
    let size = metadata.size();
    let modified = metadata.modified()?;
    if size >= 8 {
        let mut len_buf = [0; 8];
        file.read_exact_at(&mut len_buf, 0)?;
        let len = u64::from_be_bytes(len_buf);
        if size == len + 8 {
            let mut data = vec![0; len as usize];
            file.read_exact_at(&mut data, 8)?;
            return Ok((file, modified, Some(data)))
        }
    }
    file.set_len(0)?;
    Ok((file, modified, None))
}

#[allow(missing_docs)]
impl SingleValueDB {
    pub fn open(file_name: &str) -> std::io::Result<Self> {
        let (mut file_a, modified_a, mut data_a) = open_file(&(file_name.to_owned() + ".1"))?;
        let (mut file_b, modified_b, mut data_b) = open_file(&(file_name.to_owned() + ".2"))?;
        if modified_a < modified_b {
            swap(&mut file_a, &mut file_b);
            swap(&mut data_a, &mut data_b);
        }
        Ok(Self { files: [file_a, file_b], is_first: data_a.is_some(), data: data_a.or(data_b) })
    }
}

impl SingleValueT for SingleValueDB {
    fn get(&self) -> Option<&Vec<u8>> {
        self.data.as_ref()
    }
    #[allow(clippy::unwrap_used)]
    fn set(&mut self, value: Vec<u8>) {
        if self.get() == Some(&value) {
            return;
        }
        self.is_first = !self.is_first;
        let file = &mut self.files[if self.is_first { 0 } else { 1 }];
        file.set_len(0).unwrap();
        file.write_all(&(value.len() as u64).to_be_bytes()).unwrap();
        file.write_all(&value).unwrap();
        self.files[if self.is_first { 1 } else { 0 }].set_len(0).unwrap();
        self.data = Some(value);
    }
}
