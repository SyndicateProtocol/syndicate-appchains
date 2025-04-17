//! The `additive_compression` module provides functionality for incrementally compressing
use crate::zlib_compression::is_valid_cm_bits_8_only;
use alloy::primitives::Bytes;
use bytes::BytesMut;
use flate2::{write::ZlibEncoder, Compression};
use std::io::{Error, Write};

/// A streaming compressor that incrementally compresses transactions using zlib
#[derive(Debug)]
pub struct AdditiveCompressor {
    buffer: BytesMut,
    num_transactions: u32,
    encoder: ZlibEncoder<Vec<u8>>,
}

impl Default for AdditiveCompressor {
    fn default() -> Self {
        Self::new()
    }
}

impl AdditiveCompressor {
    /// Creates a new `AdditiveCompressor`
    pub fn new() -> Self {
        let mut buffer = BytesMut::new();
        buffer.extend_from_slice(&0u32.to_be_bytes());

        Self {
            buffer,
            num_transactions: 0,
            encoder: ZlibEncoder::new(Vec::new(), Compression::default()),
        }
    }

    /// Attempts to add a transaction and returns the total compressed size so far
    pub fn try_push(&mut self, txn: &Bytes) -> Result<usize, Error> {
        // Append length prefix and transaction data to buffer
        self.buffer.extend_from_slice(&(txn.len() as u32).to_be_bytes());
        self.buffer.extend_from_slice(txn);
        self.num_transactions += 1;

        // Rewrite transaction count in header (overwrite the first 4 bytes)
        let mut with_count = self.buffer.clone();
        with_count[0..4].copy_from_slice(&(self.num_transactions).to_be_bytes());

        // Reset encoder to compress full buffer
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&with_count)?;
        let compressed = encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;

        Ok(compressed.len())
    }

    /// Attempts to add a transaction and returns the total compressed size so far
    pub fn peek_push_size(&self, txn: &Bytes) -> Result<usize, Error> {
        let mut buffer = self.buffer.clone();
        let tx_count = self.num_transactions + 1;

        buffer.extend_from_slice(&(txn.len() as u32).to_be_bytes());
        buffer.extend_from_slice(txn);
        buffer[0..4].copy_from_slice(&tx_count.to_be_bytes());

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&buffer)?;
        let compressed = encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;
        Ok(compressed.len())
    }

    /// Finalizes the stream and returns the full compressed batch
    pub fn finish(mut self) -> Result<Bytes, Error> {
        // Write the final transaction count
        self.buffer[0..4].copy_from_slice(&(self.num_transactions).to_be_bytes());

        self.encoder.write_all(&self.buffer)?;
        let compressed = self.encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;

        Ok(Bytes::from(compressed))
    }
}
