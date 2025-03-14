use block_builder::{
    config::TargetRollupType::{ARBITRUM, OPTIMISM},
    connectors::mchain::MetaChainProvider,
    rollups::{
        arbitrum::arbitrum_adapter::ArbitrumAdapter, optimism::optimism_adapter::OptimismAdapter,
        shared::RollupAdapter,
    },
};
use common::tracing::init_tracing_with_extra_fields;
use eyre::Result;
use metabased_translator::{
    config::MetabasedConfig,
    shutdown_channels::{ShutdownChannels, ShutdownTx},
    spawn::{clients, get_extra_fields_for_logging, init_metrics, ComponentHandles},
    types::RuntimeError,
};
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

fn main() -> Result<(), RuntimeError> {
    let base_config = MetabasedConfig::initialize();
    let extra_fields = get_extra_fields_for_logging(base_config.clone());
    init_tracing_with_extra_fields(extra_fields)?;

    debug!("Base configuration {:?}", base_config);
    if let Err(e) = base_config.validate() {
        error!("Failed to initialize MetabasedConfig: {}", e);
        std::process::exit(1);
    };

    let runtime =
        tokio::runtime::Runtime::new().map_err(|e| RuntimeError::Initialization(e.to_string()))?;

    // Run the async process
    match base_config.block_builder.target_rollup_type {
        OPTIMISM => {
            runtime
                .block_on(run(&base_config, OptimismAdapter::new(&base_config.block_builder)))?;
        }
        ARBITRUM => {
            runtime
                .block_on(run(&base_config, ArbitrumAdapter::new(&base_config.block_builder)))?;
        }
    }

    Ok(())
}

/// Entry point for the async runtime
/// This function holds the application lifecycle. It starts the translator components and sets up
/// the shutdown handling.
async fn run(
    config: &MetabasedConfig,
    rollup_adapter: impl RollupAdapter,
) -> Result<(), RuntimeError> {
    info!("Initializing Metabased Translator components");
    let shutdown_channels = ShutdownChannels::new();
    let (sequencing_client, settlement_client) = clients(config).await?;

    let (metrics, metrics_task) = init_metrics(config).await;

    let mchain = MetaChainProvider::start(
        &config.block_builder,
        metrics.block_builder.clone(),
        rollup_adapter,
    )
    .await?;

    let safe_state =
        mchain.reconcile_mchain_with_source_chains(&sequencing_client, &settlement_client).await?;

    let (main_shutdown_rx, tx, rx) = shutdown_channels.split();
    let component_tasks = ComponentHandles::spawn(
        config,
        safe_state,
        sequencing_client,
        settlement_client,
        metrics,
        mchain,
        rx,
    );

    info!("Starting Metabased Translator");
    start_shutdown_handling(main_shutdown_rx, tx, component_tasks, metrics_task).await
}

async fn start_shutdown_handling(
    main_shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    tx: ShutdownTx,
    mut handles: ComponentHandles,
    metrics_task: JoinHandle<()>,
) -> Result<(), RuntimeError> {
    //TODO(SEQ-628) - some crate task failures don't actually get propagated back. We probably want
    // to, likely useful for operating nodes

    // MAIN SELECT LOOP - wait for shutdown signal or task failure
    tokio::select! {
        _ = main_shutdown_rx => handles.graceful_shutdown(tx).await,
        res = &mut handles.sequencing => handles.check_error(res, "Sequencing chain ingestor"),
        res = &mut handles.settlement => handles.check_error(res, "Settlement chain ingestor"),
        res = &mut handles.slotter => handles.check_error(res, "Slotter"),
        res = metrics_task => {
            if let Err(e) = res {
                error!("Metrics task failed: {}", e)
            }
            Ok(())
        }
    }
}
