//! The `eth_client` module provides a client for interacting with an Ethereum-like blockchain.

use crate::types::{Block, PartialBlock, Receipt};
use alloy::{
    providers::{IpcConnect, Provider, ProviderBuilder, WsConnect},
    rpc::{
        client::{BuiltInConnectionString, ClientBuilder, RpcClient},
        types::BlockNumberOrTag,
    },
    transports::{RpcError, TransportErrorKind},
};
use async_trait::async_trait;
use std::{fmt::Debug, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::time::timeout;
use url::Url;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub enum Client {
    Subscription(Arc<dyn Provider>),
    Http(Arc<dyn RPCClient>),
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Invalid RPC URL {0}")]
    InvalidRpcURL(String),
}

pub async fn client(url: &str) -> Result<Client, ClientError> {
    let parsed_url = url
        .parse::<BuiltInConnectionString>()
        .map_err(|_| ClientError::InvalidRpcURL(url.to_string()))?;
    match parsed_url {
        BuiltInConnectionString::Http(_) => {
            let client = EthClient::new(url, Duration::from_secs(10)) //TODO pass the configured timeout
                .await
                .map_err(|_| ClientError::InvalidRpcURL(url.to_string()))?;
            Ok(Client::Http(Arc::new(client)))
        }
        BuiltInConnectionString::Ws(ws_url, _authorization) => {
            // TODO handle WS with auth
            let ws = WsConnect::new(ws_url);
            let provider = ProviderBuilder::new()
                .on_ws(ws)
                .await
                .map_err(|_| ClientError::InvalidRpcURL(url.to_string()))?;

            Ok(Client::Subscription(Arc::new(provider)))
        }
        BuiltInConnectionString::Ipc(ipc_url) => {
            let ipc = IpcConnect::new(ipc_url);
            let provider = ProviderBuilder::new()
                .on_ipc(ipc)
                .await
                .map_err(|_| ClientError::InvalidRpcURL(url.to_string()))?;

            Ok(Client::Subscription(Arc::new(provider)))
        }
        _ => todo!(),
    }
}

impl Client {
    pub async fn get_block_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> Result<PartialBlock, RPCClientError> {
        match self {
            Self::Subscription(provider) => {
                let block = provider
                    .get_block_by_number(block_number)
                    .await
                    .map_err(|e| RPCClientError::RpcError(e))?
                    .ok_or_else(|| RPCClientError::BlockNotFound(block_number.to_string()))?;
                Ok(PartialBlock::new(block.header, vec![]))
            }
            Self::Http(client) => {
                let block = client.get_block_by_number(block_number).await?;
                Ok(PartialBlock {
                    number: block.number,
                    hash: block.hash,
                    timestamp: block.timestamp,
                    parent_hash: block.parent_hash,
                    logs: vec![],
                })
            }
        }
    }
}
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

    /// Error returned when the RPC request times out.
    #[error("RPC request timed out")]
    Timeout,

    /// Generic RPC communication failure
    #[error("RPC request failed: {0}")]
    RpcError(#[from] RpcError<TransportErrorKind>),
}

/// Trait defining methods for interacting with a blockchain
#[async_trait]
pub trait RPCClient: Send + Sync + Debug {
    /// Retrieves a block by its number with a timeout.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The number of the block to retrieve.
    ///
    /// # Returns
    ///
    /// A result containing the `Block` if found, or an error if the block is not found or the
    /// request times out.
    async fn get_block_by_number(
        &self,
        block_identifier: BlockNumberOrTag,
    ) -> Result<Block, RPCClientError>;

    /// Retrieves the transaction receipts for a given block hash with a timeout.
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
    timeout: Duration,
}

impl EthClient {
    /// Creates a new `EthClient` instance.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The URL of the Ethereum JSON-RPC endpoint.
    /// * `timeout` - The timeout duration for RPC requests.
    ///
    /// # Returns
    ///
    /// A result containing the `EthClient` instance if successful, or an error if the connection
    /// fails.
    pub async fn new(rpc_url: &str, timeout: Duration) -> Result<Self, RPCClientError> {
        let url =
            Url::parse(rpc_url).map_err(|_e| RPCClientError::InvalidRpcURL(rpc_url.to_string()))?;
        let client = ClientBuilder::default().http(url);
        Ok(Self { client, timeout })
    }
}

#[async_trait]
impl RPCClient for EthClient {
    /// Retrieves a block by its number with a timeout.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The number of the block to retrieve.
    ///
    /// # Returns
    ///
    /// A result containing the `Block` if found, or an error if the block is not found or the
    /// request times out.
    async fn get_block_by_number(
        &self,
        block_identifier: BlockNumberOrTag,
    ) -> Result<Block, RPCClientError> {
        let request_future = self
            .client
            .request::<_, Option<Block>>("eth_getBlockByNumber", (block_identifier, true));

        match timeout(self.timeout, request_future).await {
            Ok(Ok(Some(block))) => Ok(block),
            Ok(Ok(None)) => Err(RPCClientError::BlockNotFound(block_identifier.to_string())),
            Ok(Err(rpc_err)) => Err(RPCClientError::RpcError(rpc_err)),
            Err(_) => Err(RPCClientError::Timeout),
        }
    }

    /// Retrieves the transaction receipts for a given block hash with a timeout.
    async fn get_block_receipts(
        &self,
        block_number_hex: &str,
    ) -> Result<Vec<Receipt>, RPCClientError> {
        let request_future = self
            .client
            .request::<_, Option<Vec<Receipt>>>("eth_getBlockReceipts", (block_number_hex,));

        match timeout(self.timeout, request_future).await {
            Ok(Ok(Some(receipts))) => Ok(receipts),
            Ok(Ok(None)) => {
                Err(RPCClientError::BlockReceiptsNotFound(block_number_hex.to_string()))
            }
            Ok(Err(rpc_err)) => Err(RPCClientError::RpcError(rpc_err)),
            Err(_) => Err(RPCClientError::Timeout),
        }
    }
}
