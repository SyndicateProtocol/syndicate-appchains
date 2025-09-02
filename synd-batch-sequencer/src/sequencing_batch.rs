//! The `additive_compression` module provides functionality for incrementally compressing
//! transactions using zlib.

use tracing::instrument;

/// A batch of transactions.
#[derive(Debug, Clone)]
pub enum SequencingBatch {
    /// A batch of uncompressed transactions.
    Uncompressed(Vec<TxWithValkeyId>),
}

/// A transaction with a valkey ID.
pub type TxWithValkeyId = (Vec<u8>, String);

impl SequencingBatch {
    /// Returns the length of the batch.
    pub fn len(&self) -> usize {
        match self {
            Self::Uncompressed(batch) => batch.iter().map(|tx| tx.0.len()).sum(),
        }
    }

    /// Returns the uncompressed size of batch content (sum of transaction bytes)
    pub fn uncompressed_size(&self) -> usize {
        self.txs().iter().map(|tx| tx.0.len()).sum()
    }

    /// Returns true if the batch is empty.
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Uncompressed(batch) => batch.is_empty(),
        }
    }

    /// Returns a reference to the transactions in the batch.
    pub const fn txs(&self) -> &Vec<TxWithValkeyId> {
        match self {
            Self::Uncompressed(txs) => txs,
        }
    }

    /// Takes ownership of the transactions in the batch.
    pub fn into_owned_txs(self) -> Vec<TxWithValkeyId> {
        match self {
            Self::Uncompressed(txs) => txs,
        }
    }

    /// Creates a new batch with the given transaction added.
    pub fn create_new_with_tx(&self, tx: TxWithValkeyId) -> std::io::Result<Self> {
        match self {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        }
    }
}
