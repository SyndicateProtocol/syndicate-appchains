//! The `additive_compression` module provides functionality for incrementally compressing
//! transactions using zlib.

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

    /// Clones the internal buffer, appends the txn, compresses, and returns compressed Vec
    pub fn clone_and_compress_with_txn(&self, txn: &Bytes) -> Result<Vec<u8>, Error> {
        let mut buffer = self.buffer.clone();
        let tx_count = self.num_transactions + 1;

        buffer.extend_from_slice(&(txn.len() as u32).to_be_bytes());
        buffer.extend_from_slice(txn);
        buffer[0..4].copy_from_slice(&tx_count.to_be_bytes());

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&buffer)?;
        let compressed = encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;

        Ok(compressed)
    }

    /// Caller provides already compressed buffer (for optimized flow)
    pub fn push_with_precompressed(
        &mut self,
        txn: &Bytes,
        _compressed: Vec<u8>,
    ) -> Result<(), Error> {
        // Only mutate state — compression result was already validated
        self.buffer.extend_from_slice(&(txn.len() as u32).to_be_bytes());
        self.buffer.extend_from_slice(txn);
        self.num_transactions += 1;
        self.buffer[0..4].copy_from_slice(&self.num_transactions.to_be_bytes());
        Ok(())
    }
    /// Finalizes the stream and returns the full compressed batch
    pub fn finish(mut self) -> Result<Bytes, Error> {
        self.buffer[0..4].copy_from_slice(&(self.num_transactions).to_be_bytes());

        self.encoder.write_all(&self.buffer)?;
        let compressed = self.encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;

        Ok(Bytes::from(compressed))
    }

    /// Resets the compressor to its initial state
    pub fn reset(&mut self) {
        self.buffer = BytesMut::new();
        self.num_transactions = 0;
        self.encoder = ZlibEncoder::new(Vec::new(), Compression::default());
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
    use flate2::read::ZlibDecoder;
    use std::io::Read;

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
    fn test_clone_and_compress_is_consistent() {
        let compressor = AdditiveCompressor::new();
        let txn = sample_txn(&[0x10, 0x20, 0x30]);

        let compressed = compressor.clone_and_compress_with_txn(&txn).unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_push_with_precompressed_updates_state() {
        let mut compressor = AdditiveCompressor::new();
        let txn = sample_txn(&[0x42, 0x42]);

        let compressed = compressor.clone_and_compress_with_txn(&txn).unwrap();
        compressor.push_with_precompressed(&txn, compressed).unwrap();

        assert_eq!(compressor.num_transactions(), 1);
        assert!(!compressor.is_empty());
    }

    #[test]
    fn test_multiple_transactions_accumulate_properly() {
        let mut compressor = AdditiveCompressor::new();

        for i in 0..3 {
            let txn = sample_txn(&[i; 4]);
            let compressed = compressor.clone_and_compress_with_txn(&txn).unwrap();
            compressor.push_with_precompressed(&txn, compressed).unwrap();
        }

        assert_eq!(compressor.num_transactions(), 3);
    }

    #[test]
    fn test_finish_returns_valid_compression() {
        let mut compressor = AdditiveCompressor::new();
        let txn1 = sample_txn(&[1, 2, 3]);
        let txn2 = sample_txn(&[4, 5, 6]);

        let c1 = compressor.clone_and_compress_with_txn(&txn1).unwrap();
        compressor.push_with_precompressed(&txn1, c1).unwrap();

        let c2 = compressor.clone_and_compress_with_txn(&txn2).unwrap();
        compressor.push_with_precompressed(&txn2, c2).unwrap();

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
    fn test_decoded_transactions_match_originals() {
        let mut compressor = AdditiveCompressor::new();

        let txns: Vec<Bytes> = vec![
            sample_txn(&[0xAA, 0xBB]),
            sample_txn(&[0xCC, 0xDD, 0xEE]),
            sample_txn(&[0x01, 0x02, 0x03, 0x04]),
        ];

        // Push transactions
        for txn in &txns {
            let compressed = compressor.clone_and_compress_with_txn(txn).unwrap();
            compressor.push_with_precompressed(txn, compressed).unwrap();
        }

        // Finalize and decompress
        let compressed = compressor.finish().unwrap();
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();

        // Decode the transactions from the decompressed buffer
        let mut cursor = 4; // skip the first 4 bytes (transaction count)
        let mut decoded_txns = Vec::new();
        while cursor < decompressed.len() {
            let len_bytes: [u8; 4] = decompressed[cursor..cursor + 4].try_into().unwrap();
            let txn_len = u32::from_be_bytes(len_bytes) as usize;
            cursor += 4;

            let txn = decompressed[cursor..cursor + txn_len].to_vec();
            decoded_txns.push(Bytes::from(txn));
            cursor += txn_len;
        }

        assert_eq!(decoded_txns.len(), txns.len());
        for (original, decoded) in txns.iter().zip(decoded_txns.iter()) {
            assert_eq!(original, decoded, "Decoded transaction does not match original");
        }
    }
}
