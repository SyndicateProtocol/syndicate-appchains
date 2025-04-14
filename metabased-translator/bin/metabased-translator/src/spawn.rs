use crate::{
    config::MetabasedConfig,
    shutdown_channels::{ShutdownChannels, ShutdownRx, ShutdownTx},
    types::RuntimeError,
};
use block_builder::{connectors::mchain::MetaChainProvider, rollups::shared::RollupAdapter};
use common::{
    eth_client::{client, Client, EthClient, RPCClient},
    types::{Chain, KnownState, PartialBlock},
};
use eyre::Result;
use ingestor::{
    config::ChainIngestorConfig,
    ingestor::{IngestorArgs, IngestorError},
};
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

    let sequencing_client =
        client(&config.sequencing.sequencing_rpc_url, config.sequencing.sequencing_rpc_timeout)
            .await?;
    let settlement_client =
        client(&config.settlement.settlement_rpc_url, config.settlement.settlement_rpc_timeout)
            .await?;

    let metrics = init_metrics(config).await;

    let mchain = MetaChainProvider::start(
        &config.block_builder,
        metrics.block_builder.clone(),
        rollup_adapter,
    )
    .await?;

    loop {
        let known_state = mchain
            .reconcile_mchain_with_source_chains(&sequencing_client, &settlement_client)
            .await?;

        let (tx, rx) = ShutdownChannels::new().split();
        let component_tasks = ComponentHandles::spawn(
            config,
            known_state,
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
    fn handle_ingestor_result(
        result: Result<Result<(), IngestorError>, tokio::task::JoinError>,
        component: &str,
    ) -> Result<(), TerminationError> {
        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(IngestorError::ReorgDetected {
                chain,
                current_block,
                received_block,
                received_parent_hash,
            })) => {
                error!("restarting the translator components due to {chain} chain reorg: current={:?}, received={:?}, parent_hash={:?}", 
                    current_block, received_block, received_parent_hash);
                Err(TerminationError::ReorgDetected())
            }
            Ok(Err(e)) => Err(TerminationError::Err(RuntimeError::TaskFailedUnrecoverable(
                format!("{component}: {e}"),
            ))),
            Err(e) => Err(TerminationError::Err(RuntimeError::TaskFailedUnrecoverable(format!(
                "{component}: {e}"
            )))),
        }
    }

    fn handle_slotter_result(
        result: Result<Result<(), SlotterError>, tokio::task::JoinError>,
    ) -> Result<(), TerminationError> {
        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(TerminationError::Err(RuntimeError::TaskFailedUnrecoverable(
                format!("Slotter: {e}"),
            ))),
            Err(e) => Err(TerminationError::Err(RuntimeError::TaskFailedUnrecoverable(format!(
                "Slotter: {e}"
            )))),
        }
    }

    // MAIN SELECT LOOP - wait for shutdown signal or task failure
    let result = tokio::select! {
        res = &mut handles.sequencing => {
            handle_ingestor_result(res, "Sequencing chain ingestor")
        },
        res = &mut handles.settlement => {
            handle_ingestor_result(res, "Settlement chain ingestor")
        },
        res = &mut handles.slotter => {
            handle_slotter_result(res)
        },
    };

    handles.graceful_shutdown(tx).await;

    result
}

struct ComponentHandles {
    sequencing: JoinHandle<Result<(), IngestorError>>,
    settlement: JoinHandle<Result<(), IngestorError>>,
    slotter: JoinHandle<Result<(), SlotterError>>,
}

impl ComponentHandles {
    fn spawn(
        config: &MetabasedConfig,
        known_state: Option<KnownState>,
        sequencing_client: Client,
        settlement_client: Client,
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

        let sequencing_known_state =
            known_state.as_ref().map(|state| state.sequencing_block.clone());

        let sequencing = tokio::spawn(async move {
            ingestor::ingestor::run(
                IngestorArgs {
                    chain: Chain::Sequencing,
                    config: sequencing_config,
                    addresses: sequencing_addresses,
                    sender: sequencing_tx,
                    known_block: sequencing_known_state,
                    metrics: metrics.ingestor_sequencing,
                    shutdown_rx: shutdown_rx.sequencing,
                },
                &sequencing_client,
            )
            .await
        });

        let settlement_known_state =
            known_state.as_ref().map(|state| state.settlement_block.clone());

        let settlement = tokio::spawn(async move {
            ingestor::ingestor::run(
                IngestorArgs {
                    chain: Chain::Settlement,
                    config: settlement_config,
                    addresses: settlement_addresses,
                    sender: settlement_tx,
                    known_block: settlement_known_state,
                    metrics: metrics.ingestor_settlement,
                    shutdown_rx: shutdown_rx.settlement,
                },
                &settlement_client,
            )
            .await
        });

        let settlement_delay = config.settlement_delay;
        let slotter = tokio::spawn(async move {
            slotter::slotter::run(
                settlement_delay,
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

        // if this function is called on reorg, an ingestor will already be stopped
        if !self.sequencing.is_finished() {
            if let Err(e) = self.sequencing.await {
                warn!("Error shutting down sequencing ingestor: {}", e);
            }
        }
        if !self.settlement.is_finished() {
            if let Err(e) = self.settlement.await {
                warn!("Error shutting down settlement ingestor: {}", e);
            }
        }
        if !self.slotter.is_finished() {
            self.slotter.abort();
        }

        info!("Metabased Translator shutdown complete");
    }
}

pub async fn clients(
    config: &MetabasedConfig,
) -> Result<(Arc<dyn RPCClient>, Arc<dyn RPCClient>), RuntimeError> {
    let sequencing_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(
            &config.sequencing.sequencing_rpc_url,
            config.sequencing.sequencing_rpc_timeout,
        )
        .await?,
    );
    let settlement_client: Arc<dyn RPCClient> = Arc::new(
        EthClient::new(
            &config.settlement.settlement_rpc_url,
            config.settlement.settlement_rpc_timeout,
        )
        .await?,
    );
    Ok((sequencing_client, settlement_client))
}

pub async fn init_metrics(config: &MetabasedConfig) -> TranslatorMetrics {
    let mut metrics_state = MetricsState::default();
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    start_metrics(metrics_state, config.metrics.metrics_port).await;
    metrics
}
