use block_builder::{block_builder::BlockBuilder, config::BlockBuilderConfig};
use common::{
    db::{self, TranslatorStore},
    tracing::{init_tracing, TracingError},
};
use eyre::Result;
use ingestor::{config::IngestionPipelineConfig, ingestor::Ingestor};
use metabased_translator::config::MetabasedConfig;
use slotting::{config::SlottingConfig, slotting::Slotter};
use tokio::sync::oneshot;
use tracing::{error, info};

// TODO(SEQ-515): Improve this executable, research async tasks
async fn run(
    block_builder_config: BlockBuilderConfig,
    slotting_config: SlottingConfig,
    ingestion_config: IngestionPipelineConfig,
    db_path: &str,
) -> Result<(), RuntimeError> {
    // Create shutdown channels
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (seq_shutdown_tx, seq_shutdown_rx) = oneshot::channel();
    let (settle_shutdown_tx, settle_shutdown_rx) = oneshot::channel();
    let (slotter_shutdown_tx, slotter_shutdown_rx) = oneshot::channel();
    let (builder_shutdown_tx, builder_shutdown_rx) = oneshot::channel();

    // Ctrl-C handler
    let shutdown_tx_clone = shutdown_tx;
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.map_err(|e| RuntimeError::TaskFailure(e.to_string()))?;
        info!("Received shutdown signal");
        shutdown_tx_clone
            .send(())
            .map_err(|_| RuntimeError::Shutdown("Failed to send shutdown signal".into()))?;
        Ok::<_, RuntimeError>(())
    });

    // Initialize the DB
    let db = Box::new(
        db::RocksDbStore::new(db_path)
            .map_err(|e| RuntimeError::Initialization(format!("Failed to open DB: {e}")))?,
    );
    let safe_state = db
        .get_latest()
        .await
        .map_err(|e| RuntimeError::Initialization(format!("Failed to get latest state: {e}")))?;

    // Initialize components with their specific configs
    let mut sequencing_config = ingestion_config.sequencing;
    let mut settlement_config = ingestion_config.settlement;

    let safe_block_number = safe_state.as_ref().map(|state| state.slot.number);
    // Override start blocks if we're resuming from a database
    if let Some(state) = &safe_state {
        sequencing_config.sequencing_start_block = state.sequencing_block.number;
        settlement_config.settlement_start_block = state.settlement_block.number;
    }

    // Initialize components
    let (sequencing_ingestor, sequencer_rx) = Ingestor::new(sequencing_config.into()).await?;
    let (settlement_ingestor, settlement_rx) = Ingestor::new(settlement_config.into()).await?;
    let (slotter, slot_rx) =
        Slotter::new(sequencer_rx, settlement_rx, slotting_config, safe_state, db);
    let block_builder = BlockBuilder::new(slot_rx, block_builder_config).await?;

    // Start components
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
        _ = shutdown_rx => {
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
                return Err(RuntimeError::TaskFailure(format!("Sequencing chain ingestor panicked: {}", e)));
            }
        }
        res = &mut settlement_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Settlement chain ingestor panicked: {}", e)));
            }
        }
        res = &mut slotter_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Slotter panicked: {}", e)));
            }
        }
        res = &mut block_builder_handle => {
            if let Err(e) = res {
                return Err(RuntimeError::TaskFailure(format!("Block builder panicked: {}", e)));
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
        ))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
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

impl From<block_builder::config::ConfigError> for RuntimeError {
    fn from(err: block_builder::config::ConfigError) -> Self {
        RuntimeError::InvalidConfig(format!("Block builder config error: {}", err))
    }
}

impl From<slotting::config::ConfigError> for RuntimeError {
    fn from(err: slotting::config::ConfigError) -> Self {
        RuntimeError::InvalidConfig(format!("Slotter config error: {}", err))
    }
}

impl From<ingestor::config::ConfigError> for RuntimeError {
    fn from(err: ingestor::config::ConfigError) -> Self {
        RuntimeError::InvalidConfig(format!("Ingestor config error: {}", err))
    }
}

impl From<eyre::Report> for RuntimeError {
    fn from(err: eyre::Report) -> Self {
        RuntimeError::TaskFailure(err.to_string())
    }
}
