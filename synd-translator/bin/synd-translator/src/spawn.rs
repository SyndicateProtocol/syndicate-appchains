use crate::{
    config::{ChainIngestorConfig, TranslatorConfig},
    types::RuntimeError,
};
use eyre::Result;
use metrics::metrics::TranslatorMetrics;
use shared::service_start_utils::{start_metrics_and_health, MetricsState};
use std::sync::Arc;
use synd_block_builder::appchains::arbitrum::arbitrum_adapter::ArbitrumAdapter;
use synd_chain_ingestor::{
    client::{IngestorProvider, Provider as IProvider},
    eth_client::EthClient,
};
use synd_mchain::client::{MProvider, Provider};
use tracing::{error, log::info};

/// Entry point for the async runtime
pub async fn run(config: &TranslatorConfig) -> Result<(), RuntimeError> {
    info!("Initializing Syndicate Translator components");

    let mut metrics_state = MetricsState::default();
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    start_metrics_and_health(metrics_state, config.metrics.metrics_port, None).await;

    loop {
        info!("Starting Syndicate Translator");
        match start_slotter(config, &metrics).await {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                error!("restarting the translator components: {e}");
                // Sleep for 1 second to avoid spamming the logs on unrecoverable errors
                // TODO [SEQ-985]: Review errors thrown by slotter and handle them appropriately
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        };
    }
}

async fn start_slotter(config: &TranslatorConfig, metrics: &TranslatorMetrics) -> Result<()> {
    // TODO(LBL) - is this a `ws_url` now?
    let mchain = MProvider::new(&config.block_builder.mchain_rpc_url)
        .await
        .map_err(|e| RuntimeError::InvalidConfig(
            format!("Invalid synd-mchain rpc url: {} error: {}", config.block_builder.mchain_rpc_url, e)))?;

    let sequencing_client = IngestorProvider::new(
        config.sequencing.sequencing_rpc_url.as_ref().unwrap(),
        config.rpc_timeout,
    )
    .await;

    let settlement_ingestor_client =
        IngestorProvider::new(&config.settlement.settlement_rpc_url, config.rpc_timeout).await;

    let safe_state =
        mchain.reconcile_mchain_with_source_chains(&sequencing_client, &settlement_ingestor_client).await?;

    let mut sequencing_config: ChainIngestorConfig = config.sequencing.clone().into();
    let mut settlement_config: ChainIngestorConfig = config.settlement.clone().into();

    // start the ingestors from the known safe state
    if let Some(state) = &safe_state {
        sequencing_config.start_block = state.sequencing_block.number + 1;
        // re-ingest the last known settlement block as it is not included in the latest synd-mchain
        // block
        settlement_config.start_block = state.settlement_block.number;
    }

    let arbitrum_adapter = Arc::new(ArbitrumAdapter::new(&config.block_builder));

    let adapter = arbitrum_adapter.clone();

    let seq_client = EthClient::new(
        &sequencing_client.get_url().await?,
        config.rpc_timeout,
        config.get_logs_timeout,
        1024,
    )
    .await;

    let sequencing = sequencing_client
        .get_blocks(
            sequencing_config.start_block,
            adapter.sequencer_addresses(),
            adapter,
            seq_client,
        )
        .await?;

    let settlement_client = EthClient::new(
        &settlement_ingestor_client.get_url().await?,
        config.rpc_timeout,
        config.get_logs_timeout,
        1024,
    )
    .await;
    let settlement = settlement_ingestor_client
        .get_blocks(
            settlement_config.start_block,
            arbitrum_adapter.settlement_addresses(),
            arbitrum_adapter,
            settlement_client,
        )
        .await?;

    let settlement_delay = config.settlement_delay.ok_or_else(
        || RuntimeError::InvalidConfig("settlement_delay unset".into())
    )?;

    Ok(synd_slotter::slotter::run(
        settlement_delay,
        safe_state,
        sequencing,
        settlement,
        &mchain,
        &metrics.slotter,
    )
    .await?)
}
