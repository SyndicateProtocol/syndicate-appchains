use alloy_rpc_types::TransactionRequest;

/// Converts a vector of EVM blocks into a vector of meta based transactions
///
/// # Arguments
/// * `blocks` - A vector of EVM block headers
///
/// # Returns
/// A vector of meta based transactions
pub fn sequencing_chain_blocks_to_mbtxs(_blocks: Vec<String>) -> Vec<Vec<u8>> {
    // TODO: Implement
    vec![]
}

/// Trait for rollup-specific block builders that construct batches from transactions
pub trait BlockBuilder {
    /// Creates a new block builder instance
    fn new() -> Self
    where
        Self: Sized;

    /// Builds a batch of transactions into a rollup-specific batch transaction
    fn build_batch_txn(&self, txs: Vec<Vec<u8>>) -> TransactionRequest;
}
