//! The `eth_client` module provides a client for interacting with an Ethereum-like blockchain.

use crate::types::{Block, Receipt};
use alloy::{
    rpc::{
        client::{ClientBuilder, RpcClient},
        types::BlockNumberOrTag,
    },
    transports::{RpcError, TransportErrorKind},
};
use async_trait::async_trait;
use std::fmt::Debug;
use thiserror::Error;
use url::Url;

/// Errors that can occur while interacting with the Ethereum RPC client.
#[derive(Debug, Error)]
pub enum RPCClientError {
    /// Error returned when the RPC URL isn't valid
    #[error("Invalid RPC URL {0}")]
    InvalidRpcURL(String),

    /// Error returned when a block is not found for the given block number.
    #[error("Block {0} not found")]
    BlockNotFound(String),

    /// Error returned when transaction receipts for a block are not found.
    #[error("Receipts for block {0} not found")]
    BlockReceiptsNotFound(String),

    /// Generic RPC communication failure
    #[error("RPC request failed: {0}")]
    RpcError(#[from] RpcError<TransportErrorKind>),
}

/// Trait defining methods for interacting with a blockchain
#[async_trait]
pub trait RPCClient: Send + Sync + Debug {
    /// Retrieves a block by its number.
    async fn get_block_by_number(
        &self,
        block_identifier: BlockNumberOrTag,
    ) -> Result<Block, RPCClientError>;

    /// Retrieves multiple blocks & its receipts in a batch call
    async fn get_block_receipts(
        &self,
        block_number_hex: &str,
    ) -> Result<Vec<Receipt>, RPCClientError>;
}

/// A client for interacting with an Ethereum-like blockchain.
///
/// This client is designed to retrieve blockchain data such as blocks and receipts
/// by interacting with an Ethereum JSON-RPC endpoint.
#[derive(Debug)]
pub struct EthClient {
    client: RpcClient,
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
    async fn get_block_by_number(
        &self,
        block_identifier: BlockNumberOrTag,
    ) -> Result<Block, RPCClientError> {
        self.client
            .request::<_, Option<Block>>("eth_getBlockByNumber", (block_identifier, true))
            .await?
            .ok_or_else(|| RPCClientError::BlockNotFound(block_identifier.to_string()))
    }

    async fn get_block_receipts(
        &self,
        block_number_hex: &str,
    ) -> Result<Vec<Receipt>, RPCClientError> {
        self.client
            .request::<_, Option<Vec<Receipt>>>("eth_getBlockReceipts", (block_number_hex,))
            .await?
            .ok_or_else(|| RPCClientError::BlockReceiptsNotFound(block_number_hex.to_string()))
    }
}
