//! The `spawn` module handles spawns

use crate::{
    config::Config,
    poller,
    shutdown_channels::{ShutdownRx, ShutdownTx},
    submitter,
    types::{NitroBlock, RuntimeError},
};
use eyre::Report;
use std::sync::Arc;
use tokio::{sync::mpsc::channel, task::JoinHandle};
use tracing::{error, info};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct ComponentHandles {
    pub poller: JoinHandle<eyre::Result<(), Report>>,
    pub submitter: JoinHandle<eyre::Result<(), Report>>,
}

#[allow(missing_docs)]
impl ComponentHandles {
    pub fn spawn(config: &Config, shutdown_rx: ShutdownRx) -> Self {
        let config = config.clone();
        let (blocks_tx, blocks_rx) = channel::<Arc<NitroBlock>>(config.buffer_size);
        let poller_handle = tokio::spawn(async move {
            poller::run(
                config.app_chain_rpc_url,
                config.polling_interval,
                blocks_tx,
                shutdown_rx.poller,
            )
            .await
        });

        let submitter_handler = tokio::spawn(async move {
            submitter::run(
                config.settlement_chain_rpc_url,
                config.private_key,
                config.assertion_poster_contract_address,
                blocks_rx,
                shutdown_rx.submitter,
            )
            .await
        });
        Self { poller: poller_handle, submitter: submitter_handler }
    }

    pub async fn graceful_shutdown(self, tx: ShutdownTx) -> eyre::Result<(), RuntimeError> {
        info!("Received shutdown signal");

        // 1. Stop poller first
        info!("Shutting down poller...");
        let _ = tx.poller.send(());
        if let Err(e) = self.poller.await {
            error!("Error shutting down poller: {}", e);
        }

        // 2. Stop submitter
        info!("Shutting down submitter...");
        let _ = tx.submitter.send(());
        if let Err(e) = self.submitter.await {
            error!("Error shutting down slotter: {}", e);
        }

        info!("Metabased Poster shutdown complete");
        Ok(())
    }

    /// Outer error is unrecoverable task panic, inner error is recoverable
    pub fn check_error(
        self,
        handle_result: eyre::Result<eyre::Result<(), Report>, tokio::task::JoinError>,
        component: &str,
    ) -> eyre::Result<(), RuntimeError> {
        match handle_result {
            Ok(res) => match res {
                Ok(_) => {
                    info!("{component} task completed successfully");
                    Ok(())
                }
                Err(e) => Err(RuntimeError::TaskFailedRecoverable(format!("{component}: {e}"))),
            },
            Err(e) => Err(RuntimeError::TaskFailedUnrecoverable(format!("{component}: {e}"))),
        }
    }
}
