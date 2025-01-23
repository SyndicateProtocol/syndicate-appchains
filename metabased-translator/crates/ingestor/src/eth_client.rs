//! The `eth_client` module provides a client for interacting with an Ethereum-like blockchain.

use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::BoxTransport,
};
use common::types::{Block, Receipt};
use eyre::{eyre, Error};

/// A client for interacting with an Ethereum-like blockchain.
///
/// This client is designed to retrieve blockchain data such as blocks and receipts
/// by interacting with an Ethereum JSON-RPC endpoint.
#[derive(Debug)]
#[allow(unreachable_pub)] // TODO: remove when used
pub struct EthClient {
    chain: RootProvider<BoxTransport>,
}

impl EthClient {
    /// Creates a new `EthClient` instance.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The URL of the Ethereum JSON-RPC endpoint.
    ///
    /// # Returns
    ///
    /// A result containing the `EthClient` instance if successful, or an error if the connection fails.
    pub(crate) async fn new(rpc_url: &str) -> Result<Self, Error> {
        let chain = ProviderBuilder::new().on_builtin(rpc_url).await?;
        Ok(Self { chain })
    }

    /// Retrieves a block by its number.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The number of the block to retrieve as a string.
    ///
    /// # Returns
    ///
    /// A result containing the `Block` if found, or an error if the block is not found.
    pub(crate) async fn get_block_by_number(&self, block_number: u64) -> Result<Block, Error> {
        let block_number_hex = format!("0x{:x}", block_number);
        self.chain
            .client()
            .request::<_, Option<Block>>("eth_getBlockByNumber", (block_number_hex, true))
            .await?
            .ok_or_else(|| eyre!("Block not found"))
    }

    /// Retrieves the receipts of all transactions in a block.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The number of the block for which to retrieve receipts as a string.
    ///
    /// # Returns
    ///
    /// A result containing a vector of `Receipt` if found, or an error if no receipts are found.
    pub(crate) async fn get_block_receipts(
        &self,
        block_number: u64,
    ) -> Result<Vec<Receipt>, Error> {
        let block_number_hex = format!("0x{:x}", block_number);
        self.chain
            .client()
            .request::<_, Option<Vec<Receipt>>>("eth_getBlockReceipts", (block_number_hex,))
            .await?
            .ok_or_else(|| eyre!("Block receipts not found"))
    }
}
