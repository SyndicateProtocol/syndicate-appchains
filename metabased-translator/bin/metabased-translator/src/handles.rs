use crate::{
    shutdown::{ShutdownRx, ShutdownTx},
    types::RuntimeError,
};
use block_builder::{block_builder::BlockBuilder, rollups::shared::RollupAdapter};
use common::types::BlockAndReceipts;
use eyre::Report;
use ingestor::ingestor::Ingestor;
use slotter::Slotter;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, log::info};

pub struct ComponentHandles {
    pub sequencing: JoinHandle<eyre::Result<(), Report>>,
    pub settlement: JoinHandle<eyre::Result<(), Report>>,
    pub slotter: JoinHandle<eyre::Result<(), Report>>,
    pub block_builder: JoinHandle<eyre::Result<(), Report>>,
}

impl ComponentHandles {
    #[allow(clippy::too_many_arguments)]
    pub fn spawn<R: RollupAdapter>(
        safe_block_number: Option<u64>,
        sequencing_ingestor: Ingestor,
        sequencing_rx: tokio::sync::mpsc::Receiver<Arc<BlockAndReceipts>>,
        settlement_ingestor: Ingestor,
        settlement_rx: tokio::sync::mpsc::Receiver<Arc<BlockAndReceipts>>,
        slotter: Slotter,
        block_builder: BlockBuilder<R>,
        rx: ShutdownRx,
    ) -> Self {
        let sequencing =
            tokio::spawn(async move { sequencing_ingestor.start_polling(rx.sequencer).await });

        let settlement =
            tokio::spawn(async move { settlement_ingestor.start_polling(rx.settlement).await });

        let slotter =
            tokio::spawn(
                async move { slotter.start(sequencing_rx, settlement_rx, rx.slotter).await },
            );

        let block_builder =
            tokio::spawn(async move { block_builder.start(safe_block_number, rx.builder).await });

        Self { sequencing, settlement, slotter, block_builder }
    }

    pub async fn graceful_shutdown(self, tx: ShutdownTx) -> eyre::Result<(), RuntimeError> {
        info!("Received shutdown signal");

        // 1. Stop ingestors first
        info!("Shutting down ingestors...");
        let _ = tx.sequencer.send(());
        let _ = tx.settlement.send(());
        if let Err(e) = self.sequencing.await {
            error!("Error shutting down sequencing ingestor: {}", e);
        }
        if let Err(e) = self.settlement.await {
            error!("Error shutting down settlement ingestor: {}", e);
        }

        // 2. Stop slotter
        info!("Shutting down slotter...");
        let _ = tx.slotter.send(());
        if let Err(e) = self.slotter.await {
            error!("Error shutting down slotter: {}", e);
        }

        // 3. Stop block builder
        info!("Shutting down block builder...");
        let _ = tx.builder.send(());
        if let Err(e) = self.block_builder.await {
            error!("Error shutting down block builder: {}", e);
        }

        info!("Metabased Translator shutdown complete");
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
