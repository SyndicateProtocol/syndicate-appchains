use block_builder::block_builder::BlockBuilder;
use common::{
    db::{self, KnownState, TranslatorStore},
    tracing::{init_tracing_with_extra_fields, TracingError},
    types::Chain,
};
use eyre::Result;
use ingestor::{
    config::ChainIngestorConfig,
    eth_client::{EthClient, RPCClient, RPCClientError},
    ingestor::Ingestor,
};
use metabased_translator::config::MetabasedConfig;
use metrics::metrics::{start_metrics, MetricsState, TranslatorMetrics};
use prometheus_client::registry::Registry;
use serde_json::{json, Value};
use slotter::Slotter;
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::oneshot,
};
use tracing::{debug, error, info};

// TODO(SEQ-515): Improve this executable, research async tasks
async fn run(
    config: &mut MetabasedConfig,
    db_path: &str,
    registry: Registry,
) -> Result<(), RuntimeError> {
    // Create shutdown channels
    let (main_shutdown_tx, main_shutdown_rx) = oneshot::channel();
    let (seq_shutdown_tx, seq_shutdown_rx) = oneshot::channel();
    let (settle_shutdown_tx, settle_shutdown_rx) = oneshot::channel();
    let (slotter_shutdown_tx, slotter_shutdown_rx) = oneshot::channel();
    let (builder_shutdown_tx, builder_shutdown_rx) = oneshot::channel();
    init_signal_handler(main_shutdown_tx);

    // Initialize the DB with the restore flag
    let (db, safe_state) = init_db(db_path, config.restore_from_safe_state).await?;

    // Initialize ETH clients
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

    // Override start blocks if we're resuming from a database
    if let Some(state) = &safe_state {
        info!(%state.slot, %state.sequencing_block, %state.settlement_block, "Resuming from known state in DB");
        config.sequencing.sequencing_start_block = state.sequencing_block.number + 1;
        config.settlement.settlement_start_block = state.settlement_block.number + 1;
    } else {
        info!("No known state found in DB, starting from configured start blocks");
        // Initial timestamp is only needed if we aren't resuming from db
        config.set_initial_timestamp(&settlement_client, &sequencing_client).await?;
    }
    let safe_block_number = safe_state.as_ref().map(|state| state.slot.number);

    // Initialize metrics
    let mut metrics_state = MetricsState { registry };
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    let metrics_task = start_metrics(metrics_state, config.metrics.metrics_port).await;

    let seq_config: ChainIngestorConfig = (&config.sequencing).into();
    let set_config: ChainIngestorConfig = (&config.settlement).into();
    // Initialize components
    let (sequencing_ingestor, sequencing_rx) = Ingestor::new(
        Chain::Sequencing,
        sequencing_client,
        &seq_config,
        metrics.ingestor_sequencing,
    )
    .await?;
    let (settlement_ingestor, settlement_rx) = Ingestor::new(
        Chain::Settlement,
        settlement_client,
        &set_config,
        metrics.ingestor_settlement,
    )
    .await?;
    let (slotter, slot_rx) = Slotter::new(&config.slotter, safe_state, db.clone(), metrics.slotter);
    let block_builder = BlockBuilder::new(
        slot_rx,
        &config.block_builder,
        config.slotter.slot_duration,
        db,
        metrics.block_builder,
    )
    .await?;

    // Start component tasks
    info!("Starting Metabased Translator");
    let mut sequencing_handle =
        tokio::spawn(async move { sequencing_ingestor.start_polling(seq_shutdown_rx).await });
    let mut settlement_handle =
        tokio::spawn(async move { settlement_ingestor.start_polling(settle_shutdown_rx).await });
    let mut slotter_handle = tokio::spawn(async move {
        slotter.start(sequencing_rx, settlement_rx, slotter_shutdown_rx).await
    });
    let mut block_builder_handle =
        tokio::spawn(
            async move { block_builder.start(safe_block_number, builder_shutdown_rx).await },
        );

    // Wait for either shutdown signal or task failure
    tokio::select! {
        _ = main_shutdown_rx => {
            // Perform graceful shutdown
            info!("Received shutdown signal");

            // 1. Stop ingestors first
            info!("Shutting down ingestors...");
            let _ = seq_shutdown_tx.send(());
            let _ = settle_shutdown_tx.send(());
            if let Err(e) = sequencing_handle.await {
                error!("Error shutting down sequencing ingestor: {}", e);
            }
            if let Err(e) = settlement_handle.await {
                error!("Error shutting down settlement ingestor: {}", e);
            }

            // 2. Stop slotter after ingestors are done
            info!("Shutting down slotter...");
            let _ = slotter_shutdown_tx.send(());
            if let Err(e) = slotter_handle.await {
                error!("Error shutting down slotter: {}", e);
            }

            // 3. Finally stop block builder
            info!("Shutting down block builder...");
            let _ = builder_shutdown_tx.send(());
            if let Err(e) = block_builder_handle.await {
                error!("Error shutting down block builder: {}", e);
            }
            info!("Metabased Translator shutdown complete");
        }
        res = &mut sequencing_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Sequencing chain ingestor unrecoverable error: {}", e)));
            }
        }
        res = &mut settlement_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Settlement chain ingestor unrecoverable error: {}", e)));
            }
        }
        res = &mut slotter_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Slotter unrecoverable error: {}", e)));
            }
        }
        res = &mut block_builder_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Block builder unrecoverable error: {}", e)));
            }
        }
        res = metrics_task => {
            if let Err(e) = res {
                error!("Metrics task failed: {}", e)
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), RuntimeError> {
    let mut base_config = MetabasedConfig::initialize();
    let extra_fields = get_extra_fields_for_logging(base_config.clone());
    init_tracing_with_extra_fields(extra_fields)?;

    debug!("Base configuration {:?}", base_config);
    if let Err(e) = base_config.validate() {
        error!("Failed to initialize MetabasedConfig: {}", e);
        std::process::exit(1);
    };

    // Create and run async runtime
    let runtime =
        tokio::runtime::Runtime::new().map_err(|e| RuntimeError::Initialization(e.to_string()))?;

    // Init the path for the DB
    let db_path = format!("{}/db", base_config.datadir);
    // Initiate the metrics registry
    let registry = Registry::default();

    // Run the async process
    runtime.block_on(run(&mut base_config, db_path.as_str(), registry))?;

    Ok(())
}

/// These extra fields are added to every log event for additional context. Add more as needed
fn get_extra_fields_for_logging(base_config: MetabasedConfig) -> Vec<(String, Value)> {
    vec![("chain_id".to_string(), json!(base_config.block_builder.target_chain_id))]
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to initialize component: {0}")]
    Initialization(String),

    #[error("Component shutdown error: {0}")]
    Shutdown(String),

    #[error("Task error: {0}")]
    TaskFailure(String),

    #[error("Invalid config")]
    InvalidConfig(String),

    #[error(transparent)]
    Tracing(#[from] TracingError),

    #[error(transparent)]
    BlockBuilderConfig(#[from] block_builder::config::ConfigError),

    #[error(transparent)]
    SlotterConfig(#[from] slotter::config::ConfigError),

    #[error(transparent)]
    IngestorConfig(#[from] ingestor::config::ConfigError),

    #[error(transparent)]
    TranslatorConfig(#[from] metabased_translator::config::ConfigError),

    #[error(transparent)]
    RPCClient(#[from] RPCClientError),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

fn init_signal_handler(main_shutdown_tx: oneshot::Sender<()>) {
    tokio::spawn(async move {
        // SIGINT is triggered when the user presses Ctrl+C in the terminal
        let mut sigint =
            signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
        // SIGTERM is typically sent when stopping a Docker container
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                info!("Received SIGINT (Ctrl+C), initiating shutdown...");
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM, initiating shutdown...");
            }
        }

        if main_shutdown_tx.send(()).is_err() {
            panic!("Failed to send shutdown signal");
        }
    });
}

async fn init_db(
    db_path: &str,
    restore_from_safe_state: bool,
) -> Result<(Arc<dyn TranslatorStore + Send + Sync>, Option<KnownState>), RuntimeError> {
    let db = Arc::new(
        db::RocksDbStore::new(db_path)
            .map_err(|e| RuntimeError::Initialization(format!("Failed to open DB: {e}")))?,
    );

    let safe_state = if restore_from_safe_state {
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

    Ok((db, safe_state))
}
