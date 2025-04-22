use crate::{
    config::MetabasedConfig,
    shutdown_channels::{ShutdownChannels, ShutdownRx, ShutdownTx},
    types::RuntimeError,
};
use block_builder::{connectors::mchain::MetaChainProvider, rollups::shared::RollupAdapter};
use common::{
    eth_client::{EthClient, RPCClient},
    types::{Chain, KnownState, PartialBlock},
};
use eyre::{Report, Result};
use ingestor::config::ChainIngestorConfig;
use metrics::metrics::TranslatorMetrics;
use shared::metrics::{start_metrics, MetricsState};
use slotter::slotter::SlotterError;
use std::sync::Arc;
use tokio::{sync::mpsc::channel, task::JoinHandle};
use tracing::{error, log::info, warn};

/// Entry point for the async runtime
/// This function holds the application lifecycle. It starts the translator components and sets up
/// the shutdown handling.
pub async fn run(
    config: &MetabasedConfig,
    rollup_adapter: impl RollupAdapter,
) -> Result<(), RuntimeError> {
    info!("Initializing Metabased Translator components");

    let (sequencing_client, settlement_client) = clients(config).await?;

    let metrics = init_metrics(config).await;

    let mchain = MetaChainProvider::start(
        &config.block_builder,
        metrics.block_builder.clone(),
        rollup_adapter,
    )
    .await?;
    mchain
        .assert_rollup_owner(config.rollup_owner)
        .await
        .map_err(|e| RuntimeError::Initialization(e.to_string()))?;

    loop {
        let safe_state = mchain
            .reconcile_mchain_with_source_chains(
                &sequencing_client.clone(),
                &settlement_client.clone(),
            )
            .await?;

        let (tx, rx) = ShutdownChannels::new().split();
        let component_tasks = ComponentHandles::spawn(
            config,
            safe_state,
            sequencing_client.clone(),
            settlement_client.clone(),
            metrics.clone(),
            mchain.clone(),
            rx,
        );

        info!("Starting Metabased Translator");
        match termination_handling(tx, component_tasks).await {
            Ok(()) => std::process::exit(0),
            Err(e) => match e {
                TerminationError::Err(e) => {
                    error!("Metabased Translator terminated with error: {}", e);
                    std::process::exit(1);
                }
                TerminationError::ReorgDetected() => {
                    continue;
                }
            },
        };
    }
}

enum TerminationError {
    Err(RuntimeError),
    ReorgDetected(),
}

async fn termination_handling(
    tx: ShutdownTx,
    mut handles: ComponentHandles,
) -> Result<(), TerminationError> {
    // MAIN SELECT LOOP - wait for shutdown signal or task failure
    tokio::select! {
        res = &mut handles.sequencing => handles.check_error(res, "Sequencing chain ingestor").map_err(TerminationError::Err),
        res = &mut handles.settlement => handles.check_error(res, "Settlement chain ingestor").map_err(TerminationError::Err),
        res = &mut handles.slotter => {
            match res {
                Ok(Ok(_)) => Ok(()),
                Ok(Err(e @ SlotterError::ReorgDetected { .. })) => {
                    error!("restarting the translator components: {e}");
                    handles.graceful_shutdown(tx).await;
                    Err(TerminationError::ReorgDetected())
                }
                Ok(Err(e)) => Err(TerminationError::Err(RuntimeError::TaskFailedRecoverable(format!("Slotter: {e}")))),
                Err(e) => Err(TerminationError::Err(RuntimeError::TaskFailedUnrecoverable(format!("Slotter: {e}")))),
            }
        },
    }
}

struct ComponentHandles {
    sequencing: JoinHandle<Result<(), Report>>,
    settlement: JoinHandle<Result<(), Report>>,
    slotter: JoinHandle<Result<(), SlotterError>>,
}

impl ComponentHandles {
    fn spawn(
        config: &MetabasedConfig,
        known_state: Option<KnownState>,
        sequencing_client: Arc<dyn RPCClient>,
        settlement_client: Arc<dyn RPCClient>,
        metrics: TranslatorMetrics,
        mchain: MetaChainProvider<impl RollupAdapter>,
        shutdown_rx: ShutdownRx,
    ) -> Self {
        let (sequencing_tx, sequencing_rx) =
            channel::<PartialBlock>(config.sequencing.sequencing_buffer_size);
        let (settlement_tx, settlement_rx) =
            channel::<PartialBlock>(config.settlement.settlement_buffer_size);

        let mut sequencing_config: ChainIngestorConfig = config.sequencing.clone().into();
        let mut settlement_config: ChainIngestorConfig = config.settlement.clone().into();

        // start the ingestors from the known safe state
        if let Some(state) = &known_state {
            sequencing_config.start_block = state.sequencing_block.number + 1;
            settlement_config.start_block = state.settlement_block.number + 1;
        }

        let sequencing_addresses = mchain.rollup_adapter.sequencing_addresses_to_monitor();
        let settlement_addresses = mchain.rollup_adapter.settlement_addresses_to_monitor();

        let sequencing = tokio::spawn(async move {
            ingestor::run(
                Chain::Sequencing,
                &sequencing_config,
                sequencing_addresses,
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
                settlement_addresses,
                settlement_client,
                settlement_tx,
                metrics.ingestor_settlement,
                shutdown_rx.settlement,
            )
            .await
        });

        let settlement_delay = config.settlement_delay;
        let slotter = tokio::spawn(async move {
            slotter::slotter::run(
                settlement_delay.unwrap(),
                known_state,
                sequencing_rx,
                settlement_rx,
                mchain,
                metrics.slotter,
            )
            .await
        });

        Self { sequencing, settlement, slotter }
    }

    pub async fn graceful_shutdown(self, tx: ShutdownTx) {
        info!("Received shutdown signal");

        // 1. Stop ingestors first
        info!("Shutting down ingestors...");
        let _ = tx.sequencer.send(());
        let _ = tx.settlement.send(());
        if let Err(e) = self.sequencing.await {
            warn!("Error shutting down sequencing ingestor: {}", e);
        }
        if let Err(e) = self.settlement.await {
            warn!("Error shutting down settlement ingestor: {}", e);
        }

        info!("Metabased Translator shutdown complete");
    }

    /// Outer error is unrecoverable task panic, inner error is recoverable
    pub fn check_error(
        self,
        handle_result: Result<eyre::Result<(), Report>, tokio::task::JoinError>,
        component: &str,
    ) -> Result<(), RuntimeError> {
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
        EthClient::new(
            config.sequencing.sequencing_rpc_url.as_ref().unwrap(),
            config.sequencing.sequencing_rpc_timeout,
        )
        .await
        .map_err(RuntimeError::RPCClient)?,
    );
    let settlement_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(
            &config.settlement.settlement_rpc_url,
            config.settlement.settlement_rpc_timeout,
        )
        .await
        .map_err(RuntimeError::RPCClient)?,
    );
    Ok((sequencing_client, settlement_client))
}

pub async fn init_metrics(config: &MetabasedConfig) -> TranslatorMetrics {
    let mut metrics_state = MetricsState::default();
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    start_metrics(metrics_state, config.metrics.metrics_port).await;
    metrics
}
