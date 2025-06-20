//! The eth client is used by both the synd-chain-ingestor server and client crates for interacting
//! with the ethereum chain.

use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider as _, ProviderBuilder, RootProvider, WsConnect},
    pubsub::Subscription,
    rpc::types::{Filter, FilterBlockOption, Header},
    transports::{ws::WebSocketConfig, RpcError, TransportErrorKind},
};
use shared::types::Receipt;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{error, info};

/// A client for interacting with an Ethereum-like blockchain.
///
/// This client is designed to retrieve blockchain data such as blocks and receipts
/// by interacting with an Ethereum JSON-RPC endpoint.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct EthClient {
    pub client: RootProvider,
    timeout: Duration,
    log_timeout: Duration,
}

fn handle_rpc_error(name: &str, err: &RpcError<TransportErrorKind>) {
    error!("{}: {}", name, err);
    if let RpcError::Transport(err) = err {
        assert!(err.recoverable(), "{}: {}: {}", name, "fatal transport error", err);
    }
}

impl EthClient {
    /// Creates a new `EthClient` instance. Infinitely retries until it is able to connect.
    pub async fn new(
        ws_url: &str,
        timeout: Duration,
        log_timeout: Duration,
        channel_size: usize,
    ) -> Self {
        loop {
            match tokio::time::timeout(
                timeout,
                ProviderBuilder::default().on_ws(WsConnect::new(ws_url).with_config(
                    WebSocketConfig::default().max_message_size(None).max_frame_size(None),
                )),
            )
            .await
            {
                Err(_) => {
                    error!("timed out connecting to websocket");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to connect to websocket", &err);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Ok(client)) => {
                    client.client().expect_pubsub_frontend().set_channel_size(channel_size);
                    return Self { client, timeout, log_timeout };
                }
            }
        }
    }
}

impl EthClient {
    /// Retrieves a block by its number with a timeout. Infinitely retries until the request
    /// succeeds.
    pub async fn get_block_header(&self, block_identifier: BlockNumberOrTag) -> Header {
        loop {
            match timeout(self.timeout, self.client.get_block_by_number(block_identifier)).await {
                Err(_) => {
                    error!("eth_getBlockByNumber request timed out");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to fetch block header", &err);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Ok(None)) => {
                    error!("fetched empty block header");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Ok(Some(block))) => {
                    if let BlockNumberOrTag::Number(number) = block_identifier {
                        assert_eq!(block.header.number, number);
                    }
                    return block.header;
                }
            }
        }
    }

    /// Retrieves the transaction receipts for a given block hash with a timeout. Infinitely retries
    /// until the request succeeds.
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
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to fetch receipts", &err);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Ok(receipts)) => return receipts,
            }
        }
    }

    /// Subscribe to blocks over the websocket connection with a timeout. Infinitely retries until
    /// the request succeeds.
    pub async fn subscribe_blocks(&self) -> Subscription<Header> {
        loop {
            match timeout(self.timeout, self.client.subscribe_blocks()).await {
                Err(_) => {
                    error!("eth_subscribe request timed out");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to subscribe to blocks", &err);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Ok(sub)) => return sub,
            }
        }
    }

    /// Get the chain id. Infinitely retries until the request succeeds.
    pub async fn get_chain_id(&self) -> u64 {
        loop {
            match timeout(self.timeout, self.client.get_chain_id()).await {
                Err(_) => {
                    error!("eth_chainId request timed out");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Err(err)) => {
                    handle_rpc_error("failed to get chain id", &err);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Ok(Ok(chain_id)) => return chain_id,
            }
        }
    }

    /// Get logs, binary search version.
    #[allow(clippy::cognitive_complexity)]
    pub async fn get_logs(
        &self,
        filter: &Filter,
    ) -> Result<Vec<alloy::rpc::types::Log>, RpcError<TransportErrorKind>> {
        match timeout(self.log_timeout, self.client.get_logs(filter)).await {
            Err(_) => {
                error!("eth_getLogs request timed out: {:?}", filter);
                Err(TransportErrorKind::Custom("request timed out".into()).into())
            }
            Ok(Ok(x)) => Ok(x),
            Ok(Err(RpcError::ErrorResp(err))) => {
                // Only attempt binary search if we have a valid block range
                let (from_block, to_block) = match filter.block_option {
                    FilterBlockOption::Range {
                        from_block: Some(BlockNumberOrTag::Number(from)),
                        to_block: Some(BlockNumberOrTag::Number(to)),
                    } => (from, to),
                    _ => (0, 0),
                };

                // If range is too small, return error
                if to_block <= from_block {
                    error!("failed to get logs ({:?}): {}", filter, err);
                    return Err(RpcError::ErrorResp(err));
                }

                // Split range in half and recursively fetch logs
                info!(
                    "splitting eth_getLogs range ({} to {}) due to error: {}",
                    from_block, to_block, err
                );
                let mid = (from_block + to_block) / 2;
                let lower_range =
                    Box::pin(self.get_logs(&filter.clone().from_block(from_block).to_block(mid)))
                        .await?;
                let upper_range =
                    Box::pin(self.get_logs(&filter.clone().from_block(mid + 1).to_block(to_block)))
                        .await?;
                Ok([lower_range, upper_range].concat())
            }
            Ok(Err(err)) => {
                handle_rpc_error("failed to get logs", &err);
                Err(err)
            }
        }
    }
}
