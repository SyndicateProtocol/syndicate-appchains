//! The `spawn` module handles spawns

use crate::{
    config::Config,
    poster,
    shutdown_channels::{ShutdownRx, ShutdownTx},
    types::RuntimeError,
};
use eyre::Report;
use tokio::task::JoinHandle;
use tracing::{error, info};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct ComponentHandles {
    pub poster: JoinHandle<eyre::Result<(), Report>>,
}

#[allow(missing_docs)]
impl ComponentHandles {
    pub fn spawn(config: Config, shutdown_rx: ShutdownRx) -> Self {
        let poster_handle =
            tokio::spawn(async move { poster::run(&config, shutdown_rx.poller).await });

        Self { poster: poster_handle }
    }

    pub async fn graceful_shutdown(self, tx: ShutdownTx) -> eyre::Result<(), RuntimeError> {
        info!("Received shutdown signal");

        // 1. Stop poller first
        info!("Shutting down poller...");
        let _ = tx.poller.send(());
        if let Err(e) = self.poster.await {
            error!("Error shutting down poster: {}", e);
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
