use crate::{config::MetabasedConfig, types::RuntimeError};
use block_builder::rollups::arbitrum::arbitrum_adapter::ArbitrumAdapter;
use chain_ingestor::client::{IngestorProvider, Provider as IProvider};
use eyre::Result;
use ingestor::config::ChainIngestorConfig;
use mchain::client::{MProvider, Provider};
use metrics::metrics::TranslatorMetrics;
use shared::metrics::{start_metrics, MetricsState};
use std::sync::Arc;
use tracing::{error, log::info};

/// Entry point for the async runtime
pub async fn run(config: &MetabasedConfig) -> Result<(), RuntimeError> {
    info!("Initializing Metabased Translator components");

    let metrics = init_metrics(config).await;

    loop {
        info!("Starting Metabased Translator");
        match start_slotter(config, &metrics).await {
            Ok(()) => std::process::exit(0),
            Err(e) => error!("restarting the translator components: {e}"),
        };
    }
}

pub async fn init_metrics(config: &MetabasedConfig) -> TranslatorMetrics {
    let mut metrics_state = MetricsState::default();
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    start_metrics(metrics_state, config.metrics.metrics_port).await;
    metrics
}

async fn start_slotter(config: &MetabasedConfig, metrics: &TranslatorMetrics) -> Result<()> {
    let mchain = MProvider::new(&config.block_builder.mchain_rpc_url)
        .await
        .map_err(|e| RuntimeError::InvalidConfig(format!("Invalid mchain rpc url: {}", e)))?;
    assert_eq!(config.appchain_owner, Some(mchain.rollup_owner().await));

    let sequencing_client = IngestorProvider::new(
        config.sequencing.sequencing_rpc_url.as_ref().unwrap(),
        config.sequencing.sequencing_rpc_timeout,
    )
    .await;

    let settlement_client = IngestorProvider::new(
        &config.settlement.settlement_rpc_url,
        config.settlement.settlement_rpc_timeout,
    )
    .await;

    let safe_state =
        mchain.reconcile_mchain_with_source_chains(&sequencing_client, &settlement_client).await?;

    let mut sequencing_config: ChainIngestorConfig = config.sequencing.clone().into();
    let mut settlement_config: ChainIngestorConfig = config.settlement.clone().into();

    // start the ingestors from the known safe state
    if let Some(state) = &safe_state {
        sequencing_config.start_block = state.sequencing_block.number + 1;
        // re-ingest the last known settlement block as it is not included in the latest mchain
        // block
        settlement_config.start_block = state.settlement_block.number;
    }

    let arbitrum_adapter = Arc::new(ArbitrumAdapter::new(&config.block_builder));

    let adapter = arbitrum_adapter.clone();
    let sequencing = sequencing_client
        .get_blocks(sequencing_config.start_block, adapter.sequencer_addresses(), adapter)
        .await?;

    let settlement = settlement_client
        .get_blocks(
            settlement_config.start_block,
            arbitrum_adapter.settlement_addresses(),
            arbitrum_adapter,
        )
        .await?;

    let settlement_delay = config.settlement_delay;

    Ok(slotter::slotter::run(
        settlement_delay.unwrap(),
        safe_state,
        sequencing,
        settlement,
        &mchain,
        &metrics.slotter,
    )
    .await?)
}
