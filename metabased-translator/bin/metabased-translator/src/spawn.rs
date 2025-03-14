use crate::{
    config::MetabasedConfig,
    shutdown_channels::{ShutdownRx, ShutdownTx},
    types::RuntimeError,
};
use block_builder::{connectors::mchain::MetaChainProvider, rollups::shared::RollupAdapter};
use common::{
    eth_client::{EthClient, RPCClient},
    types::{BlockAndReceipts, Chain, KnownState},
};
use eyre::Report;
use ingestor::config::ChainIngestorConfig;
use metrics::metrics::{start_metrics, MetricsState, TranslatorMetrics};
use prometheus_client::registry::Registry;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::{sync::mpsc::channel, task::JoinHandle};
use tracing::{error, log::info};

pub struct ComponentHandles {
    pub sequencing: JoinHandle<eyre::Result<(), Report>>,
    pub settlement: JoinHandle<eyre::Result<(), Report>>,
    pub slotter: JoinHandle<eyre::Result<(), Report>>,
}

impl ComponentHandles {
    pub fn spawn(
        config: &MetabasedConfig,
        known_state: Option<KnownState>,
        sequencing_client: Arc<dyn RPCClient>,
        settlement_client: Arc<dyn RPCClient>,
        metrics: TranslatorMetrics,
        mchain: MetaChainProvider<impl RollupAdapter>,
        shutdown_rx: ShutdownRx,
    ) -> Self {
        let (sequencing_tx, sequencing_rx) =
            channel::<Arc<BlockAndReceipts>>(config.sequencing.sequencing_buffer_size);
        let (settlement_tx, settlement_rx) =
            channel::<Arc<BlockAndReceipts>>(config.settlement.settlement_buffer_size);

        let mut sequencing_config: ChainIngestorConfig = config.sequencing.clone().into();
        let mut settlement_config: ChainIngestorConfig = config.settlement.clone().into();

        // start the ingestors from the known safe state
        if let Some(state) = &known_state {
            sequencing_config.start_block = state.sequencing_block.number + 1;
            settlement_config.start_block = state.settlement_block.number + 1;
        }

        let slotter_config = config.slotter.clone();

        let sequencing = tokio::spawn(async move {
            ingestor::run(
                Chain::Sequencing,
                &sequencing_config,
                sequencing_client,
                sequencing_tx,
                metrics.ingestor_sequencing,
                shutdown_rx.sequencing,
            )
            .await
        });

        let settlement = tokio::spawn(async move {
            ingestor::run(
                Chain::Settlement,
                &settlement_config,
                settlement_client,
                settlement_tx,
                metrics.ingestor_settlement,
                shutdown_rx.settlement,
            )
            .await
        });

        let slotter = tokio::spawn(async move {
            slotter::run(
                &slotter_config,
                known_state,
                sequencing_rx,
                settlement_rx,
                mchain,
                metrics.slotter,
                shutdown_rx.slotter,
            )
            .await
        });

        Self { sequencing, settlement, slotter }
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

pub async fn clients(
    config: &MetabasedConfig,
) -> Result<(Arc<dyn RPCClient>, Arc<dyn RPCClient>), RuntimeError> {
    let sequencing_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(&config.sequencing.sequencing_rpc_url, Chain::Sequencing)
            .await
            .map_err(RuntimeError::RPCClient)?,
    );
    let settlement_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(&config.settlement.settlement_rpc_url, Chain::Settlement)
            .await
            .map_err(RuntimeError::RPCClient)?,
    );
    Ok((sequencing_client, settlement_client))
}

/// These extra fields are added to every log event for additional context. Add more as needed
pub fn get_extra_fields_for_logging(base_config: MetabasedConfig) -> Vec<(String, Value)> {
    vec![("chain_id".to_string(), json!(base_config.block_builder.target_chain_id))]
}

pub async fn init_metrics(config: &MetabasedConfig) -> (TranslatorMetrics, JoinHandle<()>) {
    let registry = Registry::default();
    let mut metrics_state = MetricsState { registry };
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    let metrics_task = start_metrics(metrics_state, config.metrics.metrics_port).await;
    (metrics, metrics_task)
}
