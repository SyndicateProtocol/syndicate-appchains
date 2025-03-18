//!  The `poller` polls information from the appchain on a `polling_interval` frequency

use crate::types::NitroBlock;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use eyre::{eyre, Result};
use std::{sync::Arc, time::Duration};
use tokio::sync::{mpsc::Sender, oneshot};
use tracing::error;
use url::Url;

/// Polls information from the `rpc_url` on a `polling_interval` frequency
#[derive(Debug)]
struct Poller {
    provider: RootProvider,
    polling_interval: Duration,
    sender: Sender<Arc<NitroBlock>>,
}

/// Starts the poller task
pub async fn run(
    rpc_url: Url,
    polling_interval: Duration,
    sender: Sender<Arc<NitroBlock>>,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<()> {
    let poller =
        Poller { polling_interval, provider: ProviderBuilder::default().on_http(rpc_url), sender };
    poller.main_loop(shutdown_rx).await
}

impl Poller {
    async fn main_loop(self, mut shutdown_rx: oneshot::Receiver<()>) -> Result<()> {
        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            tokio::select! {
                _ = &mut shutdown_rx => {
                    drop(self.sender);
                    return Ok(());
                }
                _ = interval.tick() => {
                    if let Err(err) = self.fetch_and_push_block().await {
                        error!("Failed to fetch and push block: {:?}", err);
                    }
                }
            }
        }
    }

    async fn fetch_and_push_block(&self) -> Result<()> {
        let block: NitroBlock = self
            .provider
            .raw_request("eth_getBlockByNumber".into(), ("latest", false))
            .await
            .map_err(|err| eyre!("eth_getBlockByNumber request failed: {:?}", err))?;

        self.sender.send(Arc::new(block)).await.map_err(|_| eyre!("Failed to send block"))?;

        Ok(())
    }
}
