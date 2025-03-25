//! The `eth_client` module provides a client for interacting with an Ethereum-like blockchain.

use crate::types::{Block, BlockAndReceipts, Chain, Receipt};
use alloy::{
    rpc::{
        client::{ClientBuilder, RpcClient},
        types::BlockNumberOrTag,
    },
    transports::{RpcError, TransportErrorKind},
};
use async_trait::async_trait;
use std::{fmt::Debug, time::Duration};
use thiserror::Error;
use tokio::time::Instant;
use tracing::trace;
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
    BlockReceiptsNotFound(u64),

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
    async fn batch_get_blocks_and_receipts(
        &self,
        block_numbers: Vec<u64>,
    ) -> Result<(Vec<BlockAndReceipts>, Duration), RPCClientError>;
}

/// A client for interacting with an Ethereum-like blockchain.
///
/// This client is designed to retrieve blockchain data such as blocks and receipts
/// by interacting with an Ethereum JSON-RPC endpoint.
#[derive(Debug)]
pub struct EthClient {
    client: RpcClient,
    chain: Chain,
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
    pub async fn new(rpc_url: &str, chain: Chain) -> Result<Self, RPCClientError> {
        let url =
            Url::parse(rpc_url).map_err(|_e| RPCClientError::InvalidRpcURL(rpc_url.to_string()))?;
        let client = ClientBuilder::default().http(url);
        Ok(Self { client, chain })
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

    /// Retrieves blocks and their receipts for multiple block numbers in a single batch request.
    ///
    /// This function sends batch RPC calls to fetch both the block data and the receipts of all
    /// transactions in the specified blocks, reducing the number of network requests.
    ///
    /// # Arguments
    ///
    /// * `block_numbers` - A vector of block numbers for which to retrieve blocks and receipts.
    ///
    /// # Returns
    ///
    /// A result containing a vector of `BlockAndReceipts` objects if successful, or an
    /// `RPCClientError` if any request fails.
    ///
    /// # Errors
    ///
    /// Returns an `RPCClientError` if there is an issue with the RPC request, batch execution,
    /// or response parsing.
    async fn batch_get_blocks_and_receipts(
        &self,
        block_numbers: Vec<u64>,
    ) -> Result<(Vec<BlockAndReceipts>, Duration), RPCClientError> {
        let mut batch = self.client.new_batch();
        let mut block_futures = Vec::new();
        let mut receipt_futures = Vec::new();

        for block_number in &block_numbers {
            let block_number_hex = format!("0x{:x}", block_number);

            let block_fut = batch
                .add_call("eth_getBlockByNumber", &(block_number_hex.clone(), true))?
                .map_resp(move |resp: Option<Block>| {
                    if resp.is_none() {
                        trace!(
                            "Block #{:?} not available on {:?} chain.",
                            block_number.clone(),
                            self.chain
                        );
                    }
                    resp
                });
            let receipt_fut = batch
                .add_call("eth_getBlockReceipts", &[block_number_hex])?
                .map_resp(|resp: Option<Vec<Receipt>>| {
                    if resp.is_none() {
                        trace!(
                            "Receipts not available for block #{:?} on {:?} chain.",
                            block_number.clone(),
                            self.chain
                        );
                    }
                    resp
                });

            block_futures.push(block_fut);
            receipt_futures.push(receipt_fut);
        }

        let start_time = Instant::now();
        batch.send().await?;
        let duration = start_time.elapsed();

        let mut results = Vec::new();
        for (block_fut, receipt_fut) in block_futures.into_iter().zip(receipt_futures.into_iter()) {
            let (block_opt, receipts_opt) = tokio::try_join!(block_fut, receipt_fut)?;

            if let (Some(block), Some(receipts)) = (block_opt, receipts_opt) {
                results.push(BlockAndReceipts { block, receipts });
            }
        }

        Ok((results, duration))
    }
}
