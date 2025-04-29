//! The `additive_compression` module provides functionality for incrementally compressing
//! transactions using zlib.

use crate::zlib_compression::is_valid_cm_bits_8_only;
use alloy::primitives::Bytes;
use flate2::{write::ZlibEncoder, Compression};
use rlp::RlpStream;
use std::io::{Error, Write};

/// A streaming compressor that incrementally compresses transactions using zlib
#[derive(Debug, Default)]
pub struct AdditiveCompressor {
    transactions: Vec<Bytes>,
}

impl AdditiveCompressor {
    /// Creates a new `AdditiveCompressor`
    pub const fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    /// Clones the compressor and compresses a transaction
    pub fn clone_and_compress_with_txn(&self, txn: &Bytes) -> Result<Vec<u8>, Error> {
        let mut txns = self.transactions.clone();
        txns.push(txn.clone());

        let mut stream = RlpStream::new_list(txns.len());
        for t in &txns {
            stream.append(&t.as_ref());
        }
        let encoded = stream.out();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&encoded)?;
        let compressed = encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;

        Ok(compressed)
    }

    /// Pushes a transaction and its precompressed data into the compressor
    pub fn push_with_precompressed(
        &mut self,
        txn: &Bytes,
        _compressed: Vec<u8>,
    ) -> Result<(), Error> {
        self.transactions.push(txn.clone());
        Ok(())
    }

    /// Finishes the compression process and returns the compressed data
    pub fn finish(self) -> Result<Bytes, Error> {
        let mut stream = RlpStream::new_list(self.transactions.len());
        for t in &self.transactions {
            stream.append(&t.as_ref());
        }
        let encoded = stream.out();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&encoded)?;
        let compressed = encoder.finish()?;

        is_valid_cm_bits_8_only(&compressed)?;

        Ok(Bytes::from(compressed))
    }

    /// Drains the transactions from the compressor
    pub fn drain_transactions(&mut self) -> Vec<Bytes> {
        std::mem::take(&mut self.transactions)
    }

    /// Resets the compressor to its initial state
    pub fn reset(&mut self) {
        self.transactions.clear();
    }

    /// Returns the number of transactions in the compressor
    pub fn num_transactions(&self) -> usize {
        self.transactions.len()
    }

    /// Returns true if the compressor is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;
    use flate2::read::ZlibDecoder;
    use rlp::Rlp;
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

        // Decompress and decode
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();
        assert_eq!(decoded_txns.len(), 1);
        assert_eq!(decoded_txns[0], txn.to_vec());
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

        // Decompress and check the RLP list
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();
        assert_eq!(decoded_txns.len(), 2);
        assert_eq!(decoded_txns[0], txn1.to_vec());
        assert_eq!(decoded_txns[1], txn2.to_vec());
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
        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();

        assert_eq!(decoded_txns.len(), txns.len());
        for (original, decoded) in txns.iter().zip(decoded_txns.iter()) {
            assert_eq!(original.as_ref(), decoded, "Decoded transaction does not match original");
        }
    }
}
