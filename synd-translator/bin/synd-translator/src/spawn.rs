use crate::{
    config::{ChainIngestorConfig, TranslatorConfig},
    types::RuntimeError,
};
use eyre::Result;
use metrics::metrics::TranslatorMetrics;
use shared::{
    service_start_utils::{start_http_server_with_aux_handlers, MetricsState},
    tracing::SpanKind,
};
use std::{sync::Arc, time::Duration};
use synd_block_builder::appchains::arbitrum::arbitrum_adapter::ArbitrumAdapter;
use synd_chain_ingestor::{
    client::{IngestorProvider, IngestorProviderConfig, Provider as IProvider},
    eth_client::EthClient,
};
use synd_mchain::client::{MProvider, Provider};
use tracing::{error, instrument, log::info};
use url::Url;

/// Entry point for the async runtime
#[instrument(skip(config), err, fields(otel.kind = ?SpanKind::Internal))]
pub async fn run(config: &TranslatorConfig) -> Result<(), RuntimeError> {
    info!("Initializing Syndicate Translator components");

    let mut metrics_state = MetricsState::default();
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    start_http_server_with_aux_handlers(metrics_state, config.port, None, None).await;

    loop {
        info!("Starting Syndicate Translator");
        match start_slotter(config, &metrics).await {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                error!("restarting the translator components: {e}");
                // Sleep for 1 second to avoid spamming the logs on unrecoverable errors
                // TODO [SEQ-985]: Review errors thrown by slotter and handle them appropriately
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        };
    }
}

#[instrument(skip(metrics), err, fields(otel.kind = ?SpanKind::Internal))]
async fn start_slotter(config: &TranslatorConfig, metrics: &TranslatorMetrics) -> Result<()> {
    let mchain = MProvider::new(&config.block_builder.mchain_ws_url)
        .await
        .map_err(|e| RuntimeError::InvalidConfig(format!("Invalid synd-mchain rpc url: {e}")))?;

    let sequencing_client = IngestorProvider::new(
        config.sequencing.sequencing_ws_url.as_ref().unwrap(),
        IngestorProviderConfig {
            timeout: config.ws_request_timeout,
            max_buffer_capacity_per_subscription: config.max_buffer_capacity_per_subscription,
            max_response_size: config.max_response_size,
            max_blocks_per_request: config.get_logs_max_blocks_per_request,
        },
    )
    .await;

    let settlement_client = IngestorProvider::new(
        config.settlement.settlement_ws_url.as_ref(),
        IngestorProviderConfig {
            timeout: config.ws_request_timeout,
            max_buffer_capacity_per_subscription: config.max_buffer_capacity_per_subscription,
            max_response_size: config.max_response_size,
            max_blocks_per_request: config.get_logs_max_blocks_per_request,
        },
    )
    .await;

    wait_until_ingestors_are_ready(
        &sequencing_client,
        &settlement_client,
        config.ingestor_ready_check_interval,
    )
    .await?;

    let safe_state =
        mchain.reconcile_mchain_with_source_chains(&sequencing_client, &settlement_client).await?;

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

    let seq_urls = sequencing_client
        .get_urls()
        .await?
        .into_iter()
        .map(|s| Url::parse(&s))
        .collect::<Result<Vec<_>, _>>()?;

    let seq_client = EthClient::new(
        seq_urls,
        config.ws_request_timeout,
        config.get_logs_timeout,
        1024,
        config.rpc_retry_interval,
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

    let set_urls = settlement_client
        .get_urls()
        .await?
        .into_iter()
        .map(|s| Url::parse(&s))
        .collect::<Result<Vec<_>, _>>()?;

    let set_client = EthClient::new(
        set_urls,
        config.ws_request_timeout,
        config.get_logs_timeout,
        1024,
        config.rpc_retry_interval,
    )
    .await;
    let settlement = settlement_client
        .get_blocks(
            settlement_config.start_block,
            arbitrum_adapter.settlement_addresses(),
            arbitrum_adapter,
            set_client,
        )
        .await?;

    let settlement_delay = config.settlement_delay;

    Ok(synd_slotter::slotter::run(
        settlement_delay.unwrap(),
        safe_state,
        sequencing,
        settlement,
        &mchain,
        &metrics.slotter,
    )
    .await?)
}

async fn wait_until_ingestors_are_ready(
    sequencing_client: &IngestorProvider,
    settlement_client: &IngestorProvider,
    ingestor_ready_check_interval: Duration,
) -> Result<()> {
    let interval_str = humantime::format_duration(ingestor_ready_check_interval);
    while sequencing_client.get_block_number().await.is_err() {
        info!("Sequencing ingestor is not ready yet - waiting for {interval_str}");
        tokio::time::sleep(ingestor_ready_check_interval).await;
    }
    while settlement_client.get_block_number().await.is_err() {
        info!("Settlement ingestor is not ready yet - waiting for {interval_str}",);
        tokio::time::sleep(ingestor_ready_check_interval).await;
    }
    info!("Ingestors are ready");
    Ok(())
}
