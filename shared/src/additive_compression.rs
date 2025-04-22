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

    /// Returns the number of transactions in the buffer
    pub const fn num_transactions(&self) -> u32 {
        self.num_transactions
    }

    /// Returns true if the buffer is empty
    pub const fn is_empty(&self) -> bool {
        self.num_transactions == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;

    fn sample_txn(data: &[u8]) -> Bytes {
        Bytes::from(data.to_vec())
    }

    #[test]
    fn test_default_state() {
        let compressor = AdditiveCompressor::default();
        assert_eq!(compressor.num_transactions(), 0);
        assert!(compressor.is_empty());
    }

    #[test]
    fn test_try_push_single_transaction() {
        let mut compressor = AdditiveCompressor::new();
        let txn = sample_txn(&[0xde, 0xad, 0xbe, 0xef]);

        let size = compressor.try_push(&txn).unwrap();
        assert!(size > 0);
        assert_eq!(compressor.num_transactions(), 1);
        assert!(!compressor.is_empty());
    }

    #[test]
    fn test_try_push_multiple_transactions() {
        let mut compressor = AdditiveCompressor::new();

        for i in 0..5 {
            let data = vec![i; 10]; // 10-byte payloads
            let txn = sample_txn(&data);
            let size = compressor.try_push(&txn).unwrap();
            assert!(size > 0);
        }

        assert_eq!(compressor.num_transactions(), 5);
    }

    #[test]
    fn test_peek_push_does_not_mutate_state() {
        let mut compressor = AdditiveCompressor::new();
        let txn = sample_txn(&[0xaa, 0xbb]);

        let peek_size = compressor.peek_push_size(&txn).unwrap();
        assert!(peek_size > 0);

        // After peek, compressor state should be unchanged
        assert_eq!(compressor.num_transactions(), 0);
        assert!(compressor.is_empty());

        // Now actually push and verify
        compressor.try_push(&txn).unwrap();
        assert_eq!(compressor.num_transactions(), 1);
    }

    #[test]
    fn test_finish_returns_valid_compression() {
        let mut compressor = AdditiveCompressor::new();
        let txn1 = sample_txn(&[1, 2, 3]);
        let txn2 = sample_txn(&[4, 5, 6]);

        compressor.try_push(&txn1).unwrap();
        compressor.try_push(&txn2).unwrap();

        let compressed = compressor.finish().unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_finish_on_empty_buffer() {
        let compressor = AdditiveCompressor::new();
        let result = compressor.finish();

        assert!(result.is_ok(), "Should be able to finish with 0 transactions");
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_txn_count_increments_properly() {
        let mut compressor = AdditiveCompressor::new();

        assert_eq!(compressor.num_transactions(), 0);
        compressor.try_push(&sample_txn(&[10])).unwrap();
        assert_eq!(compressor.num_transactions(), 1);
        compressor.try_push(&sample_txn(&[20])).unwrap();
        assert_eq!(compressor.num_transactions(), 2);
    }
}
