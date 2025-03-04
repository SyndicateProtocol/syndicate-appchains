use crate::{config::MetabasedConfig, types::RuntimeError};
use block_builder::{block_builder::BlockBuilder, rollups::shared::RollupAdapter};
use common::types::{BlockAndReceipts, Chain, KnownState};
use eyre::Result;
use ingestor::{
    config::ChainIngestorConfig,
    eth_client::{EthClient, RPCClient},
    ingestor::Ingestor,
};
use metrics::metrics::{start_metrics, MetricsState, TranslatorMetrics};
use prometheus_client::registry::Registry;
use serde_json::{json, value::Value};
use slotter::Slotter;
use std::sync::Arc;
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

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

// TODO(SEQ-628): `init` all components without a channel, `start` all components with required
//channel
pub async fn create_node_components<R: RollupAdapter>(
    config: &MetabasedConfig,
    sequencing_client: Arc<dyn RPCClient>,
    settlement_client: Arc<dyn RPCClient>,
    rollup_adapter: R,
    safe_state: Option<KnownState>,
    metrics: TranslatorMetrics,
) -> Result<
    (
        Ingestor,
        Receiver<Arc<BlockAndReceipts>>,
        Ingestor,
        Receiver<Arc<BlockAndReceipts>>,
        Slotter,
        BlockBuilder<R>,
    ),
    RuntimeError,
> {
    let sequencing_config: ChainIngestorConfig = (&config.sequencing).into();
    let settlement_config: ChainIngestorConfig = (&config.settlement).into();

    // Initialize components
    let (sequencing_ingestor, sequencing_rx) = Ingestor::new(
        Chain::Sequencing,
        sequencing_client,
        &sequencing_config,
        metrics.ingestor_sequencing,
    )
    .await?;

    let (settlement_ingestor, settlement_rx) = Ingestor::new(
        Chain::Settlement,
        settlement_client,
        &settlement_config,
        metrics.ingestor_settlement,
    )
    .await?;

    let (slotter, slot_rx) = Slotter::new(&config.slotter, safe_state, metrics.slotter);
    let block_builder =
        BlockBuilder::new(slot_rx, &config.block_builder, rollup_adapter, metrics.block_builder)
            .await?;
    Ok((
        sequencing_ingestor,
        sequencing_rx,
        settlement_ingestor,
        settlement_rx,
        slotter,
        block_builder,
    ))
}
