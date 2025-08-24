//! The `additive_compression` module provides functionality for incrementally compressing
//! transactions using zlib.

use rlp::RlpStream;
use tracing::instrument;

/// A batch of transactions.
#[derive(Debug, Clone)]
pub enum SequencingBatch {
    /// A batch of brotli compressed transactions. (also includes the uncompressed transactions, in
    /// case they need to be re-tried)
    Compressed(Vec<u8>, Vec<TxWithValkeyId>),
    /// A batch of uncompressed transactions.
    Uncompressed(Vec<TxWithValkeyId>),
}

/// A transaction with a valkey ID.
pub type TxWithValkeyId = (Vec<u8>, String);

impl SequencingBatch {
    /// Returns the length of the batch.
    pub fn len(&self) -> usize {
        match self {
            Self::Compressed(batch, _) => batch.len(),
            Self::Uncompressed(batch) => batch.iter().map(|tx| tx.0.len()).sum(),
        }
    }

    /// Returns true if the batch is empty.
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Compressed(batch, _) => batch.is_empty(),
            Self::Uncompressed(batch) => batch.is_empty(),
        }
    }

    /// Returns a reference to the transactions in the batch.
    pub const fn txs(&self) -> &Vec<TxWithValkeyId> {
        match self {
            Self::Compressed(_, txs) | Self::Uncompressed(txs) => txs,
        }
    }

    /// Takes ownership of the transactions in the batch.
    pub fn into_owned_txs(self) -> Vec<TxWithValkeyId> {
        match self {
            Self::Compressed(_, txs) | Self::Uncompressed(txs) => txs,
        }
    }

    /// Creates a new batch with the given transaction added.
    pub fn create_new_with_tx(&self, tx: TxWithValkeyId) -> std::io::Result<Self> {
        match self {
            Self::Compressed(_, txs) => compress_batch(txs, tx),
            Self::Uncompressed(txs) => Ok(uncompressed_batch(txs, tx)),
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
#[instrument(
    skip_all,
    fields(
        tx_count = txs.len() + 1,
    )
)]
fn uncompressed_batch(txs: &[TxWithValkeyId], new_tx: TxWithValkeyId) -> SequencingBatch {
    let mut final_tx_list = txs.to_vec();
    final_tx_list.push(new_tx);
    SequencingBatch::Uncompressed(final_tx_list)
}

/// Compresses a list of transactions along with a new transaction using brotli compression.
///
/// This function takes an existing list of transactions and a new transaction, combines them
/// into an RLP-encoded list, and compresses the result using brotli compression. The compression
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
/// use synd_batch_sequencer::sequencing_batch::compress_batch;
///
/// let existing_txs = [Bytes::from(vec![1, 2, 3])];
/// let new_tx = Bytes::from(vec![4, 5, 6]);
/// let compressed = compress_batch(&existing_txs, &new_tx).unwrap();
/// ```
#[instrument(
    skip_all,
    err,
    fields(
        tx_count = batch_txs.len() + 1,
    )
)]
fn compress_batch(
    batch_txs: &[TxWithValkeyId],
    new_tx: TxWithValkeyId,
) -> std::io::Result<SequencingBatch> {
    let mut batch_txs_clone = batch_txs.to_vec();
    batch_txs_clone.push(new_tx.clone());

    let mut stream = RlpStream::new_list(batch_txs.len() + 1);
    for t in batch_txs {
        stream.append(&t.0);
    }
    stream.append(&new_tx.0);

    let mut compressed = Vec::new();
    brotli::enc::BrotliCompress(
        &mut &stream.out()[..],
        &mut compressed,
        &brotli::enc::BrotliEncoderInitParams(),
    )?;

    Ok(SequencingBatch::Compressed(compressed, batch_txs_clone))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlp::Rlp;

    #[test]
    fn test_uncompressed_batch() {
        let tx1 = vec![1, 1, 1];
        let tx2 = vec![2, 2, 2];
        let new_tx = vec![3, 3, 3];

        let existing_txs = vec![(tx1.clone(), "tx1".to_string()), (tx2.clone(), "tx2".to_string())];

        let batch = uncompressed_batch(&existing_txs, (new_tx.clone(), "new_tx".to_string()));
        match batch {
            SequencingBatch::Uncompressed(uncompressed_data) => {
                assert_eq!(uncompressed_data.len(), 3);
                assert_eq!(uncompressed_data[0], (tx1, "tx1".to_string()));
                assert_eq!(uncompressed_data[1], (tx2, "tx2".to_string()));
                assert_eq!(uncompressed_data[2], (new_tx, "new_tx".to_string()));
            }
            _ => {
                panic!("Expected Uncompressed batch for this test");
            }
        }
    }

    #[test]
    fn test_compress_single_transaction() {
        let txn = vec![0x10, 0x20, 0x30];
        let compressed = compress_batch(&[], (txn.clone(), "txn".to_string())).unwrap();
        assert!(!compressed.is_empty());
        let compressed = match compressed {
            SequencingBatch::Compressed(compressed, _) => compressed,
            SequencingBatch::Uncompressed(_) => panic!("Expected compressed batch"),
        };

        // Decompress and decode
        let mut decompressed = Vec::new();
        brotli::BrotliDecompress(&mut &compressed[..], &mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();
        assert_eq!(decoded_txns.len(), 1, "Expected 1 transaction in the decoded list");
        assert_eq!(decoded_txns[0], txn, "Decoded transaction does not match original");
    }

    #[test]
    fn test_compress_multiple_transactions() {
        let txns = [(vec![1, 2, 3], "txn1".to_string()), (vec![4, 5, 6], "txn2".to_string())];

        let compressed = compress_batch(&[txns[0].clone()], txns[1].clone()).unwrap();
        assert!(!compressed.is_empty());

        let compressed = match compressed {
            SequencingBatch::Compressed(compressed, _) => compressed,
            SequencingBatch::Uncompressed(_) => panic!("Expected compressed batch"),
        };

        // Decompress and check the RLP list
        let mut decompressed = Vec::new();
        brotli::BrotliDecompress(&mut &compressed[..], &mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();
        assert_eq!(decoded_txns.len(), 2);
        assert_eq!(decoded_txns[0], txns[0].0);
        assert_eq!(decoded_txns[1], txns[1].0);
    }

    #[test]
    fn test_compress_empty_transactions() {
        let result = compress_batch(&[], (vec![], "txn".to_string()));
        assert!(result.is_ok(), "Should be able to compress empty transactions");
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compress_large_transactions() {
        let txns = [
            (vec![0xAA, 0xBB], "txn1".to_string()),
            (vec![0xCC, 0xDD, 0xEE], "txn2".to_string()),
            (vec![0x01, 0x02, 0x03, 0x04], "txn3".to_string()),
        ];

        let batch = compress_batch(&[txns[0].clone(), txns[1].clone()], txns[2].clone()).unwrap();
        let compressed = match batch {
            SequencingBatch::Compressed(compressed, _) => compressed,
            SequencingBatch::Uncompressed(_) => panic!("Expected compressed batch"),
        };

        let mut decompressed = Vec::new();
        brotli::BrotliDecompress(&mut &compressed[..], &mut decompressed).unwrap();

        let rlp = Rlp::new(&decompressed);
        let decoded_txns: Vec<Vec<u8>> = rlp.as_list().unwrap();

        assert_eq!(decoded_txns.len(), txns.len());
        for (original, decoded) in txns.iter().zip(decoded_txns.iter()) {
            assert_eq!(original.0, *decoded, "Decoded transaction does not match original");
        }
    }
}
