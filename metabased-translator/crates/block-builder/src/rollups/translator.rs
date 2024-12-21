use alloy_rpc_types::{Block, Transaction};

/// Translates blocks from settlement and sequence chains into a list of transactions
pub trait Translator {
    /// Takes blocks from settlement and sequence chains and produces a list of transactions
    /// that should be included in the next meta-chain block
    ///
    /// # Arguments
    /// * `settlement_blocks` - Vector of settlement blocks to process
    /// * `sequence_blocks` - Vector of sequence blocks to process
    ///
    /// # Returns
    /// Vector of transactions to include in next sequence block
    fn translate_blocks(
        &self,
        settlement_blocks: Vec<Block>,
        sequence_blocks: Vec<Block>,
    ) -> Vec<Transaction>;
}
