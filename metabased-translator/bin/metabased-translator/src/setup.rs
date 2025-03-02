use crate::{config::MetabasedConfig, types::RuntimeError};
use block_builder::block_builder::BlockBuilder;
use common::{
    db::{self, KnownState, TranslatorStore},
    types::{BlockAndReceipts, Chain},
};
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
use tracing::{debug, info};

async fn get_safe_start_block(
    config: &mut MetabasedConfig,
    safe_state: &Option<KnownState>,
    sequencing_client: &Arc<dyn RPCClient>,
    settlement_client: &Arc<dyn RPCClient>,
) -> Result<Option<u64>, RuntimeError> {
    if let Some(state) = &safe_state {
        info!(%state.slot, %state.sequencing_block, %state.settlement_block, "Resuming from known state in DB");
        // TODO(SEQ-630) - auto log any changes to core `config` after the fact
        config.sequencing.sequencing_start_block = state.sequencing_block.number + 1;
        config.settlement.settlement_start_block = state.settlement_block.number + 1;
    } else {
        info!("No known state found in DB, starting from configured start blocks");
        // Initial timestamp is only needed if we aren't resuming from db
        config.set_initial_timestamp(settlement_client, sequencing_client).await?;
    }
    let safe_block_number = safe_state.as_ref().map(|state| state.slot.number);
    Ok(safe_block_number)
}

pub async fn clients(
    config: &mut MetabasedConfig,
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

pub async fn init_db(
    config: &mut MetabasedConfig,
    sequencing_client: &Arc<dyn RPCClient>,
    settlement_client: &Arc<dyn RPCClient>,
) -> Result<(Arc<dyn TranslatorStore + Send + Sync>, Option<KnownState>, Option<u64>), RuntimeError>
{
    let db_path = format!("{}/db", config.datadir);
    let db = Arc::new(
        db::RocksDbStore::new(&db_path)
            .map_err(|e| RuntimeError::Initialization(format!("Failed to open DB: {e}")))?,
    );

    let safe_state = if config.restore_from_safe_state {
        debug!("Resuming from latest safe state");
        db.get_safe_state().await.map_err(|e| {
            RuntimeError::Initialization(format!("Failed to get latest safe state: {e}"))
        })?
    } else {
        debug!("Resuming from latest unsafe state");
        db.get_unsafe_state().await.map_err(|e| {
            RuntimeError::Initialization(format!("Failed to get latest unsafe state: {e}"))
        })?
    };

    // Override start blocks if we're resuming from a database
    let safe_block_number =
        get_safe_start_block(config, &safe_state, sequencing_client, settlement_client).await?;

    Ok((db, safe_state, safe_block_number))
}

pub async fn init_metrics(config: &mut MetabasedConfig) -> (TranslatorMetrics, JoinHandle<()>) {
    let registry = Registry::default();
    let mut metrics_state = MetricsState { registry };
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    let metrics_task = start_metrics(metrics_state, config.metrics.metrics_port).await;
    (metrics, metrics_task)
}

// TODO(SEQ-628): `init` all components without a channel, `start` all components with required
//channel
pub async fn create_node_components(
    config: &mut MetabasedConfig,
    sequencing_client: Arc<dyn RPCClient>,
    settlement_client: Arc<dyn RPCClient>,
    db: Arc<dyn TranslatorStore + Send + Sync>,
    safe_state: Option<KnownState>,
    metrics: TranslatorMetrics,
) -> Result<
    (
        Ingestor,
        Receiver<Arc<BlockAndReceipts>>,
        Ingestor,
        Receiver<Arc<BlockAndReceipts>>,
        Slotter,
        BlockBuilder,
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

    let (slotter, slot_rx) = Slotter::new(&config.slotter, safe_state, db.clone(), metrics.slotter);
    let block_builder =
        BlockBuilder::new(slot_rx, &config.block_builder, db, metrics.block_builder).await?;
    Ok((
        sequencing_ingestor,
        sequencing_rx,
        settlement_ingestor,
        settlement_rx,
        slotter,
        block_builder,
    ))
}
