//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, metrics::IngestorMetrics, run_http, run_subscription};
use alloy::primitives::Address;
use common::{
    eth_client::Client,
    types::{BlockRef, Chain, PartialBlock},
};
use eyre::Error;
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, oneshot};

pub async fn run(
    chain: Chain,
    config: &ChainIngestorConfig,
    addresses: Vec<Address>,
    client: &Client,
    sender: Sender<Arc<PartialBlock>>,
    metrics: IngestorMetrics,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    match client {
        Client::Http(client) => {
            run_http(chain, config, addresses, client.clone(), sender, metrics, shutdown_rx).await
        }
        Client::Subscription(client) => {
            run_subscription(chain, config, addresses, client.clone(), sender, metrics, shutdown_rx)
                .await
        }
    }
}

pub fn assert_no_reorg(current_block: &BlockRef, next_block: &PartialBlock) {
    assert!(next_block.number >= current_block.number, "TODO reorg");
    assert!(next_block.parent_hash == current_block.hash, "TODO reorg");
}
