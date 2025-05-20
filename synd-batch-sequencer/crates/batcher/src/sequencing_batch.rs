//! The `additive_compression` module provides functionality for incrementally compressing
//! transactions using zlib.

use alloy::primitives::Bytes;
use flate2::{write::ZlibEncoder, Compression};
use rlp::RlpStream;
use shared::zlib_compression::is_valid_cm_bits_8_only;
use std::io::{Error, Write};

/// A batch of transactions.
#[derive(Debug, Clone)]
pub enum SequencingBatch {
    /// A batch of zlib compressed transactions.
    Compressed(Vec<u8>),
    /// A batch of uncompressed transactions.
    Uncompressed(Vec<Vec<u8>>),
}

impl SequencingBatch {
    /// Returns the length of the batch.
    pub fn len(&self) -> usize {
        match self {
            Self::Compressed(batch) => batch.len(),
            Self::Uncompressed(batch) => batch.iter().map(|tx| tx.len()).sum(),
        }
    }

    /// Returns true if the batch is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Compressed(batch) => batch.is_empty(),
            Self::Uncompressed(batch) => batch.is_empty(),
        }
    }
}

/// Creates an uncompressed batch from a list of transactions and a new transaction.
///
/// This function takes an existing list of transactions and a new transaction, combines them
/// into an RLP-encoded list, and returns the result as a vector of vectors of bytes.
///
/// # Arguments
///
/// * `txs` - A slice of existing transactions to be included in the batch
/// * `tx` - A new transaction to be appended to the list
///
/// # Returns
///
/// * `SequencingBatch::Uncompressed(Vec<Vec<u8>>)` - The uncompressed batch
pub fn uncompressed_batch(txs: &[Bytes], tx: &Bytes) -> SequencingBatch {
    SequencingBatch::Uncompressed(vec![
        txs.iter().flat_map(|tx| tx.as_ref()).copied().collect(),
        tx.as_ref().to_vec(),
    ])
}

/// Compresses a list of transactions along with a new transaction using zlib compression.
///
/// This function takes an existing list of transactions and a new transaction, combines them
/// into an RLP-encoded list, and compresses the result using zlib compression. The compression
/// is validated to ensure it meets the required format (8-bit CM bits only).
///
/// # Arguments
///
/// * `txs` - A slice of existing transactions to be included in the compression
/// * `tx` - A new transaction to be appended to the list and included in the compression
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The compressed data if successful
/// * `Err(Error)` - An IO error if compression fails or if the compressed data doesn't meet format
///   requirements
///
/// # Example
///
/// ```
/// use alloy::primitives::Bytes;
/// use batcher::sequencing_batch::compress_batch;
///
/// let existing_txs = [Bytes::from(vec![1, 2, 3])];
/// let new_tx = Bytes::from(vec![4, 5, 6]);
/// let compressed = compress_batch(&existing_txs, &new_tx).unwrap();
/// ```
pub fn compress_batch(txs: &[Bytes], tx: &Bytes) -> Result<SequencingBatch, Error> {
    let mut stream = RlpStream::new_list(txs.len() + 1);
    for t in txs {
        stream.append(&t.as_ref());
    }
    stream.append(&tx.as_ref());

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&stream.out())?;
    let compressed = encoder.finish()?;

    is_valid_cm_bits_8_only(&compressed)?;

    Ok(SequencingBatch::Compressed(compressed))
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
    fn test_compress_single_transaction() {
        let txn = sample_txn(&[0x10, 0x20, 0x30]);
        let compressed = compress_batch(&[], &txn).unwrap();
        assert!(!compressed.is_empty());
        let compressed = match compressed {
            SequencingBatch::Compressed(compressed) => compressed,
            SequencingBatch::Uncompressed(_) => panic!("Expected compressed batch"),
        };

        // Decompress and decode
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();
        assert_eq!(decoded_txns.len(), 1, "Expected 1 transaction in the decoded list");
        assert_eq!(decoded_txns[0], txn.to_vec(), "Decoded transaction does not match original");
    }

    #[test]
    fn test_compress_multiple_transactions() {
        let txns = [sample_txn(&[1, 2, 3]), sample_txn(&[4, 5, 6])];

        let compressed = compress_batch(&txns[..1], &txns[1]).unwrap();
        assert!(!compressed.is_empty());

        let compressed = match compressed {
            SequencingBatch::Compressed(compressed) => compressed,
            SequencingBatch::Uncompressed(_) => panic!("Expected compressed batch"),
        };

        // Decompress and check the RLP list
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();
        assert_eq!(decoded_txns.len(), 2);
        assert_eq!(decoded_txns[0], txns[0].to_vec());
        assert_eq!(decoded_txns[1], txns[1].to_vec());
    }

    #[test]
    fn test_compress_empty_transactions() {
        let result = compress_batch(&[], &Bytes::new());
        assert!(result.is_ok(), "Should be able to compress empty transactions");
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compress_large_transactions() {
        let txns: Vec<Bytes> = vec![
            sample_txn(&[0xAA, 0xBB]),
            sample_txn(&[0xCC, 0xDD, 0xEE]),
            sample_txn(&[0x01, 0x02, 0x03, 0x04]),
        ];

        let batch = compress_batch(&txns[..2], &txns[2]).unwrap();
        let compressed = match batch {
            SequencingBatch::Compressed(compressed) => compressed,
            SequencingBatch::Uncompressed(_) => panic!("Expected compressed batch"),
        };
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();

        assert_eq!(decoded_txns.len(), txns.len());
        for (original, decoded) in txns.iter().zip(decoded_txns.iter()) {
            assert_eq!(original.as_ref(), decoded, "Decoded transaction does not match original");
        }
    }
}
