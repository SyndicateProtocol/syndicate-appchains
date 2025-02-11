//! The `eth_client` module provides a client for interacting with an Ethereum-like blockchain.

use alloy::{
    rpc::client::{ClientBuilder, RpcClient},
    transports::http::{Client, Http},
};
use async_trait::async_trait;
use common::types::{Block, BlockAndReceipts, Receipt};
use eyre::eyre;
use std::fmt::Debug;
use thiserror::Error;
use tracing::debug;
use url::Url;

/// Errors that can occur while interacting with the Ethereum RPC client.
#[derive(Debug, Error)]
pub enum RPCClientError {
    /// Error returned when the RPC URL isn't valid
    #[error("Invalid RPC URL {0}")]
    InvalidRpcURL(String),

    /// Error returned when a block is not found for the given block number.
    #[error("Block {0} not found")]
    BlockNotFound(u64),

    /// Error returned when transaction receipts for a block are not found.
    #[error("Receipts for block {0} not found")]
    BlockReceiptsNotFound(u64),

    /// Generic RPC communication failure
    #[error("RPC request failed: {0}")]
    RpcError(#[source] eyre::Error),
}

/// Trait defining methods for interacting with a blockchain
#[async_trait]
pub trait RPCClient: Send + Sync + Debug {
    /// Retrieves a block by its number.
    async fn get_block_by_number(&self, block_number: u64) -> Result<Block, RPCClientError>;

    /// Retrieves the blocks and its receipts in a batch call.
    async fn get_block_and_receipts(
        &self,
        block_number: u64,
    ) -> Result<BlockAndReceipts, RPCClientError>;
}

/// A client for interacting with an Ethereum-like blockchain.
///
/// This client is designed to retrieve blockchain data such as blocks and receipts
/// by interacting with an Ethereum JSON-RPC endpoint.
#[derive(Debug)]
pub struct EthClient {
    client: RpcClient<Http<Client>>,
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
    /// A result containing the `EthClient` instance if successful, or an error if the connection
    /// fails.
    pub async fn new(rpc_url: &str) -> Result<Self, RPCClientError> {
        let url =
            Url::parse(rpc_url).map_err(|_e| RPCClientError::InvalidRpcURL(rpc_url.to_string()))?;
        let client = ClientBuilder::default().http(url);
        Ok(Self { client })
    }
}

#[async_trait]
impl RPCClient for EthClient {
    /// Retrieves a block by its number.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The number of the block to retrieve.
    ///
    /// # Returns
    ///
    /// A result containing the `Block` if found, or an error if the block is not found.
    async fn get_block_by_number(&self, block_number: u64) -> Result<Block, RPCClientError> {
        let block_number_hex = format!("0x{:x}", block_number);
        self.client
            .request::<_, Option<Block>>("eth_getBlockByNumber", (block_number_hex, true))
            .await
            .map_err(|e| RPCClientError::RpcError(eyre!(e)))?
            .ok_or_else(|| RPCClientError::BlockNotFound(block_number))
    }

    /// Retrieves the block & receipts of all transactions in a block.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The number of the block for which to retrieve receipts.
    ///
    /// # Returns
    ///
    /// A result containing a `BlockAndReceipts` object if found, or an error if not found.
    async fn get_block_and_receipts(
        &self,
        block_number: u64,
    ) -> Result<BlockAndReceipts, RPCClientError> {
        debug!("get_blocks_and_receipts {}", block_number);
        let block_number_hex = format!("0x{:x}", block_number);

        let mut batch = self.client.new_batch();

        let block_fut = batch
            .add_call("eth_getBlockByNumber", &(block_number_hex.clone(), true))
            .map_err(|e| RPCClientError::RpcError(eyre!(e)))?
            .map_resp(|resp: Block| resp);
        let receipt_fut = batch
            .add_call("eth_getBlockReceipts", &[block_number_hex])
            .map_err(|e| RPCClientError::RpcError(eyre!(e)))?
            .map_resp(|resp: Vec<Receipt>| resp);

        batch.send().await.map_err(|e| RPCClientError::RpcError(eyre!(e)))?;

        let (block, receipts) = tokio::try_join!(block_fut, receipt_fut)
            .map_err(|e| RPCClientError::RpcError(eyre!(e)))?;
        debug!("get_blocks_and_receipts response: {:?} & {:?}", block, receipts);

        Ok(BlockAndReceipts { block, receipts })
    }
}
