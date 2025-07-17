//! The `eth_client` is used by both the `synd-chain-ingestor` server and client crates for
//! interacting with the Ethereum-like chain.

use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider as _, ProviderBuilder, RootProvider, WsConnect},
    pubsub::Subscription,
    rpc::types::{Filter, FilterBlockOption, Header},
    transports::{ws::WebSocketConfig, RpcError, TransportErrorKind},
};
use shared::{tracing::SpanKind, types::Receipt};
use std::time::Duration;
use tokio::time::timeout;
use tracing::{error, info, instrument, warn};

/// A client for interacting with an Ethereum-like blockchain.
///
/// This client is designed to retrieve blockchain data such as blocks and receipts
/// by interacting with an Ethereum JSON-RPC endpoint.
#[derive(Debug, Clone)]
pub struct EthClient {
    /// The underlying client for the Ethereum-like chains
    pub client: RootProvider,
    timeout: Duration,
    log_timeout: Duration,
    retry_interval: Duration,
}

fn handle_rpc_error(name: &str, err: &RpcError<TransportErrorKind>) {
    error!("{}: {}", name, err);
    if let RpcError::Transport(err) = err {
        // TODO(SEQ-1055): Revisit `recoverable()` usage if necessary
        assert!(err.recoverable(), "{}: {}: {}", name, "fatal transport error", err);
    }
}

impl EthClient {
    /// Creates a new [`EthClient`] instance. Retries indefinitely until it is able to connect.
    pub async fn new(
        ws_urls: Vec<String>,
        timeout: Duration,
        log_timeout: Duration,
        channel_size: usize,
        retry_interval: Duration,
    ) -> Self {
        loop {
            // fallback to next ws url if the current one fails
            for ws_url in ws_urls.clone() {
                match tokio::time::timeout(
                    timeout,
                    ProviderBuilder::default().connect_ws(WsConnect::new(ws_url).with_config(
                        WebSocketConfig::default().max_message_size(None).max_frame_size(None),
                    )),
                )
                .await
                {
                    Err(_) => {
                        error!("timed out connecting to websocket");
                    }
                    Ok(Err(err)) => {
                        handle_rpc_error("failed to connect to websocket", &err);
                    }
                    Ok(Ok(client)) => {
                        client.client().expect_pubsub_frontend().set_channel_size(channel_size);
                        return Self { client, timeout, log_timeout, retry_interval };
                    }
                }
            }
            tokio::time::sleep(retry_interval).await;
        }
    }
}

impl EthClient {
    /// Retrieves a block by its number with a timeout. Retries indefinitely until the request
    /// succeeds.
    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Client))]
    pub async fn get_block_header(&self, block_identifier: BlockNumberOrTag) -> Header {
        loop {
            match timeout(self.timeout, self.client.get_block_by_number(block_identifier)).await {
                Err(_) => {
                    warn!(%block_identifier, "eth_getBlockByNumber request timed out");
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to fetch a block header", &err);
                }
                Ok(Ok(None)) => {
                    warn!(%block_identifier, "fetched an empty block header");
                }
                Ok(Ok(Some(block))) => {
                    if let BlockNumberOrTag::Number(number) = block_identifier {
                        assert_eq!(block.header.number, number);
                    }
                    return block.header;
                }
            }
            tokio::time::sleep(self.retry_interval).await;
        }
    }

    /// Retrieves the transaction receipts for a given block hash with a timeout. Retries
    /// indefinitely until the request succeeds.
    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Client))]
    pub async fn get_block_receipts(&self, number: u64) -> Vec<Receipt> {
        loop {
            match timeout(
                self.timeout,
                self.client.raw_request::<_, Vec<Receipt>>(
                    "eth_getBlockReceipts".into(),
                    (BlockNumberOrTag::Number(number),),
                ),
            )
            .await
            {
                Err(_) => {
                    error!("eth_getBlockReceipts request timed out");
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to fetch receipts", &err);
                }
                Ok(Ok(receipts)) => return receipts,
            }
            tokio::time::sleep(self.retry_interval).await;
        }
    }

    /// Subscribes to blocks over the websocket connection with a timeout. Retries indefinitely
    /// until the request succeeds.
    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Consumer))]
    pub async fn subscribe_blocks(&self) -> Subscription<Header> {
        loop {
            match timeout(self.timeout, self.client.subscribe_blocks()).await {
                Err(_) => {
                    error!("eth_subscribe request timed out");
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to subscribe to blocks", &err);
                }
                Ok(Ok(sub)) => return sub,
            }
            tokio::time::sleep(self.retry_interval).await;
        }
    }

    /// Gets the chain id. Retries indefinitely until the request succeeds.
    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Client))]
    pub async fn get_chain_id(&self) -> u64 {
        loop {
            match timeout(self.timeout, self.client.get_chain_id()).await {
                Err(_) => {
                    error!("eth_chainId request timed out");
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to get chain id", &err);
                }
                Ok(Ok(chain_id)) => return chain_id,
            }
            tokio::time::sleep(self.retry_interval).await;
        }
    }

    /// Get logs, splitting the range in half if the request fails.
    #[allow(clippy::cognitive_complexity)]
    #[instrument(skip(self), fields(otel.kind = ?SpanKind::Client))]
    pub async fn get_logs(
        &self,
        filter: &Filter,
    ) -> Result<Vec<alloy::rpc::types::Log>, RpcError<TransportErrorKind>> {
        match timeout(self.log_timeout, self.client.get_logs(filter)).await {
            Err(_) => {
                warn!("eth_getLogs request timed out. Attempting to split range: {:?}", filter);
                self.handle_split_range(
                    filter,
                    TransportErrorKind::Custom("request timed out".into()).into(),
                )
                .await
            }
            Ok(Ok(x)) => Ok(x),
            Ok(Err(RpcError::ErrorResp(err))) => {
                warn!("eth_getLogs request failed. Attempting to split range: {:?}", filter);
                self.handle_split_range(filter, RpcError::ErrorResp(err)).await
            }
            Ok(Err(err)) => {
                handle_rpc_error("failed to get logs", &err);
                Err(err)
            }
        }
    }

    async fn handle_split_range(
        &self,
        filter: &Filter,
        err: RpcError<TransportErrorKind>,
    ) -> Result<Vec<alloy::rpc::types::Log>, RpcError<TransportErrorKind>> {
        // Only attempt to split the range if we have a valid block range
        let (from_block, to_block) = match filter.block_option {
            FilterBlockOption::Range {
                from_block: Some(BlockNumberOrTag::Number(from)),
                to_block: Some(BlockNumberOrTag::Number(to)),
            } => (from, to),
            _ => (0, 0),
        };

        // Error if the range is too small
        if to_block <= from_block {
            error!("failed to get logs ({:?})", filter);
            return Err(err);
        }

        // Split range in half and recursively fetch logs
        info!("splitting eth_getLogs range ({} to {})", from_block, to_block);
        let mid = (from_block + to_block) / 2;
        let lower_range =
            Box::pin(self.get_logs(&filter.clone().from_block(from_block).to_block(mid))).await?;
        let upper_range =
            Box::pin(self.get_logs(&filter.clone().from_block(mid + 1).to_block(to_block))).await?;
        Ok([lower_range, upper_range].concat())
    }
}
