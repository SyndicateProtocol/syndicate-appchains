use block_builder::{block_builder::BlockBuilder, config::BlockBuilderConfig};
use common::{
    db::{self, SafeState, TranslatorStore},
    tracing::{init_tracing, TracingError},
    types::Chain,
};
use eyre::Result;
use ingestor::{config::IngestionPipelineConfig, ingestor::Ingestor};
use metabased_translator::config::MetabasedConfig;
use metrics::{
    config::MetricsConfig,
    metrics::{start_metrics, MetricsState, TranslatorMetrics},
};
use prometheus_client::registry::Registry;
use slotting::{config::SlottingConfig, slotting::Slotter};
use thiserror::Error;
use tokio::sync::oneshot;
use tracing::{error, info};

// TODO(SEQ-515): Improve this executable, research async tasks
async fn run(
    block_builder_config: BlockBuilderConfig,
    slotting_config: SlottingConfig,
    ingestion_config: IngestionPipelineConfig,
    db_path: &str,
    metrics_config: MetricsConfig,
    registry: Registry,
) -> Result<(), RuntimeError> {
    // Create shutdown channels
    let (main_shutdown_tx, main_shutdown_rx) = oneshot::channel();
    let (seq_shutdown_tx, seq_shutdown_rx) = oneshot::channel();
    let (settle_shutdown_tx, settle_shutdown_rx) = oneshot::channel();
    let (slotter_shutdown_tx, slotter_shutdown_rx) = oneshot::channel();
    let (builder_shutdown_tx, builder_shutdown_rx) = oneshot::channel();
    init_ctrl_c_handler(main_shutdown_tx);

    // Initialize the DB
    let (db, safe_state) = init_db(db_path).await?;

    // Override start blocks if we're resuming from a database
    let mut sequencing_config = ingestion_config.sequencing;
    let mut settlement_config = ingestion_config.settlement;
    if let Some(state) = &safe_state {
        sequencing_config.sequencing_start_block = state.sequencing_block.number;
        settlement_config.settlement_start_block = state.settlement_block.number;
    }
    let safe_block_number = safe_state.as_ref().map(|state| state.slot.number);

    // Initialize metrics
    let mut metrics_state = MetricsState { registry };
    let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
    let metrics_task = start_metrics(metrics_state, metrics_config.metrics_port).await;

    // Initialize components
    let (sequencing_ingestor, sequencing_rx) =
        Ingestor::new(Chain::Sequencing, sequencing_config.into(), metrics.ingestor_sequencing)
            .await?;
    let (settlement_ingestor, settlement_rx) =
        Ingestor::new(Chain::Settlement, settlement_config.into(), metrics.ingestor_settlement)
            .await?;
    let (slotter, slot_rx) = Slotter::new(
        sequencing_rx,
        settlement_rx,
        slotting_config,
        safe_state,
        db,
        metrics.slotting,
    );
    let block_builder =
        BlockBuilder::new(slot_rx, block_builder_config, metrics.block_builder).await?;

    // Start component tasks
    info!("Starting Metabased Translator");
    let mut sequencing_handle =
        tokio::spawn(async move { sequencing_ingestor.start_polling(seq_shutdown_rx).await });
    let mut settlement_handle =
        tokio::spawn(async move { settlement_ingestor.start_polling(settle_shutdown_rx).await });
    let mut slotter_handle = tokio::spawn(async move { slotter.start(slotter_shutdown_rx).await });
    let mut block_builder_handle =
        tokio::spawn(
            async move { block_builder.start(safe_block_number, builder_shutdown_rx).await },
        );

    // Wait for either shutdown signal or task failure
    tokio::select! {
        _ = main_shutdown_rx => {
            info!("Received shutdown signal");
            // Perform graceful shutdown
            tokio::time::timeout(std::time::Duration::from_secs(5 * 60), async {
                // 1. Stop ingestors first
                info!("Shutting down ingestors...");
                let _ = seq_shutdown_tx.send(());
                let _ = settle_shutdown_tx.send(());
                let _ = sequencing_handle.await;
                let _ = settlement_handle.await;

                // 2. Stop slotter after ingestors are done
                info!("Shutting down slotter...");
                let _ = slotter_shutdown_tx.send(());
                let _ = slotter_handle.await;

                // 3. Finally stop block builder
                info!("Shutting down block builder...");
                let _ = builder_shutdown_tx.send(());
                let _ = block_builder_handle.await;
                info!("Metabased Translator shutdown complete");
            })
            .await
            .map_err(|_| RuntimeError::Initialization("Shutdown timeout exceeded".into()))?;
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
    init_tracing()?;

    // Parse base config from CLI/env
    let base_config = MetabasedConfig::parse();

    // init the paths for the db and anvil state
    let db_path = format!("{}/db", base_config.datadir);
    let anvil_state_path = format!("{}/anvil_state", base_config.datadir);
    let block_builder_config = BlockBuilderConfig { anvil_state_path, ..base_config.block_builder };
    // Initiates the metrics registry
    let registry = Registry::default();

    // Create and run async runtime
    tokio::runtime::Runtime::new()
        .map_err(|e| RuntimeError::Initialization(e.to_string()))?
        .block_on(run(
            block_builder_config,
            base_config.slotter,
            IngestionPipelineConfig {
                sequencing: base_config.sequencing,
                settlement: base_config.settlement,
            },
            db_path.as_str(),
            base_config.metrics,
            registry,
        ))?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Block builder config error: {0}")]
    BlockBuilder(String),

    #[error("Slotter config error: {0}")]
    Slotter(String),

    #[error("Ingestor config error: {0}")]
    Ingestor(String),

    #[error("Metrics config error: {0}")]
    Metrics(String),
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
}

impl From<TracingError> for RuntimeError {
    fn from(err: TracingError) -> Self {
        RuntimeError::Initialization(format!("Tracing initialization failed: {}", err))
    }
}
impl From<block_builder::config::ConfigError> for ConfigError {
    fn from(err: block_builder::config::ConfigError) -> Self {
        ConfigError::BlockBuilder(format!("{}", err))
    }
}

impl From<slotting::config::ConfigError> for ConfigError {
    fn from(err: slotting::config::ConfigError) -> Self {
        ConfigError::Slotter(format!("{}", err))
    }
}

impl From<ingestor::config::ConfigError> for ConfigError {
    fn from(err: ingestor::config::ConfigError) -> Self {
        ConfigError::Ingestor(format!("{}", err))
    }
}

impl From<metrics::config::ConfigError> for ConfigError {
    fn from(err: metrics::config::ConfigError) -> Self {
        ConfigError::Metrics(format!("{}", err))
    }
}

impl From<eyre::Report> for RuntimeError {
    fn from(err: eyre::Report) -> Self {
        RuntimeError::TaskFailure(err.to_string())
    }
}

// Add this function before run()
fn init_ctrl_c_handler(main_shutdown_tx: oneshot::Sender<()>) {
    tokio::spawn(async move {
        if let Err(e) = tokio::signal::ctrl_c().await {
            panic!("Failed to listen for ctrl-c: {}", e);
        }
        info!("Received shutdown signal");
        if main_shutdown_tx.send(()).is_err() {
            panic!("Failed to send shutdown signal");
        }
    });
}

async fn init_db(
    db_path: &str,
) -> Result<(Box<dyn TranslatorStore + Send + Sync>, Option<SafeState>), RuntimeError> {
    let db = Box::new(
        db::RocksDbStore::new(db_path)
            .map_err(|e| RuntimeError::Initialization(format!("Failed to open DB: {e}")))?,
    );
    let safe_state = db
        .get_latest()
        .await
        .map_err(|e| RuntimeError::Initialization(format!("Failed to get latest state: {e}")))?;

    Ok((db, safe_state))
}
