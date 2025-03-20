//!  The `poller` polls information from the appchain on a `polling_interval` frequency

use crate::types::NitroBlock;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use eyre::{eyre, Result};
use std::{sync::Arc, time::Duration};
use tokio::sync::{mpsc::Sender, oneshot};
use tracing::{error, info};
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
    info!("Starting poller...");
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_poller_shutdown() {
        let (sender, _receiver) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let rpc_url = Url::parse("http://localhost:8545").unwrap(); // Replace with an actual RPC if needed

        let poller = Poller {
            provider: ProviderBuilder::default().on_http(rpc_url),
            polling_interval: Duration::from_millis(100),
            sender,
        };

        let poller_task = tokio::spawn(poller.main_loop(shutdown_rx));

        // Send shutdown signal
        shutdown_tx.send(()).unwrap();
        let result = poller_task.await.unwrap();

        assert!(result.is_ok(), "Poller should exit cleanly on shutdown");
    }
}
