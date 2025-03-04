use alloy::eips::BlockNumberOrTag;
use block_builder::{
    connectors::anvil::MetaChainProvider,
    rollups::{
        arbitrum::arbitrum_adapter::ArbitrumAdapter, optimism::optimism_adapter::OptimismAdapter,
        shared::RollupAdapter,
    },
};
use common::{tracing::init_tracing_with_extra_fields, types::KnownState};
use eyre::Result;
use ingestor::eth_client::RPCClient;
use metabased_translator::{
    config::MetabasedConfig,
    handles::ComponentHandles,
    setup::{clients, create_node_components, get_extra_fields_for_logging, init_metrics},
    shutdown::{ShutdownChannels, ShutdownTx},
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
        block_builder::config::TargetRollupType::OPTIMISM => {
            runtime
                .block_on(run(&base_config, OptimismAdapter::new(&base_config.block_builder)))?;
        }
        block_builder::config::TargetRollupType::ARBITRUM => {
            runtime
                .block_on(run(&base_config, ArbitrumAdapter::new(&base_config.block_builder)))?;
        }
    }

    Ok(())
}

/// Entry point for the async runtime
/// This function initializes the database, creates the node components, and starts the translator.
async fn run(
    config: &MetabasedConfig,
    rollup_adapter: impl RollupAdapter,
) -> Result<(), RuntimeError> {
    info!("Initializing Metabased Translator components");
    let shutdown_channels = ShutdownChannels::new();
    let (sequencing_client, settlement_client) = clients(config).await?;

    let (metrics, metrics_task) = init_metrics(config).await;

    // TODO maybe there is a way to avoid creating this here (?)
    let mchain = MetaChainProvider::start(
        &config.block_builder,
        config.datadir.as_str(),
        &metrics.block_builder.mchain_metrics,
    )
    .await?;

    let (safe_state, safe_block_number) = obtain_safe_state(
        &mchain,
        sequencing_client.clone(),
        settlement_client.clone(),
        &rollup_adapter,
    )
    .await?;

    let (
        sequencing_ingestor,
        sequencing_rx,
        settlement_ingestor,
        settlement_rx,
        slotter,
        block_builder,
    ) = create_node_components(
        config,
        sequencing_client,
        settlement_client,
        rollup_adapter,
        safe_state,
        metrics,
    )
    .await?;
    let (main_shutdown_rx, tx, rx) = shutdown_channels.split();
    let component_tasks = ComponentHandles::spawn(
        safe_block_number,
        sequencing_ingestor,
        sequencing_rx,
        settlement_ingestor,
        settlement_rx,
        slotter,
        block_builder,
        rx,
    );

    info!("Starting Metabased Translator");
    start_translator(main_shutdown_rx, tx, component_tasks, metrics_task).await
}

async fn obtain_safe_state(
    mchain: &MetaChainProvider,
    sequencing_client: std::sync::Arc<dyn RPCClient>,
    settlement_client: std::sync::Arc<dyn RPCClient>,
    rollup_adapter: &impl RollupAdapter,
) -> Result<(Option<KnownState>, Option<u64>)> {
    // TODO avoid creating this provider multiple times (?)

    let (known_state, block_number) =
        rollup_adapter.get_processed_blocks(&mchain.provider, BlockNumberOrTag::Latest).await?;

    // TODO - assert the hash still matches using the client,
    // if hashes do not match, walk back the chain until we find a safe spot.
    //...........
    // TODO re-use this in case of reorg

    Ok((known_state, block_number))
}

#[allow(clippy::too_many_arguments)]
async fn start_translator(
    main_shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    tx: ShutdownTx,
    mut handles: ComponentHandles,
    metrics_task: JoinHandle<()>,
) -> Result<(), RuntimeError> {
    //TODO(SEQ-628) - some crate task failures don't actually get propagated back. We probably want
    // to, likely useful for operating nodes

    /* Oleksii chat
    if JoinError, just panic

    eyre vs anyhow
    - eyre has subcrate color-eyre for readable local dev

    - run should be handleable, panic in `main()` if you want
     */

    // MAIN SELECT LOOP - wait for shutdown signal or task failure
    tokio::select! {
        _ = main_shutdown_rx => handles.graceful_shutdown(tx).await,
        res = &mut handles.sequencing => handles.check_error(res, "Sequencing chain ingestor"),
        res = &mut handles.settlement => handles.check_error(res, "Settlement chain ingestor"),
        res = &mut handles.slotter => handles.check_error(res, "Slotter"),
        res = &mut handles.block_builder => handles.check_error(res, "Block builder"),
        res = metrics_task => {
            if let Err(e) = res {
                error!("Metrics task failed: {}", e)
            }
            Ok(())
        }
    }
}
