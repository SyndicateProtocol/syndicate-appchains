mod config;

use crate::config::MetabasedConfig;
use block_builder::{block_builder::BlockBuilder, config::BlockBuilderConfig};
use common::tracing::{init_tracing, TracingError};
use eyre::Result;
use ingestor::{config::IngestionPipelineConfig, ingestor::Ingestor};
use slotting::{config::SlottingConfig, slotting::Slotter};
use tokio::sync::oneshot;
use tracing::{error, info};

// TODO(SEQ-515): Improve this executable, research async tasks
async fn run(
    block_builder_config: BlockBuilderConfig,
    slotting_config: SlottingConfig,
    ingestion_config: IngestionPipelineConfig,
) -> Result<(), RuntimeError> {
    // Create shutdown channel
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

    // TODO(SEQ-515): Improve Ctrl+C handler
    let shutdown_tx_clone = shutdown_tx;
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.map_err(|e| RuntimeError::TaskFailure(e.to_string()))?;
        info!("Received shutdown signal");
        shutdown_tx_clone
            .send(())
            .map_err(|_| RuntimeError::Shutdown("Failed to send shutdown signal".into()))?;
        Ok::<_, RuntimeError>(())
    });

    // Initialize components with their specific configs
    let sequencing_config = ingestion_config.sequencing;
    let settlement_config = ingestion_config.settlement;

    let (mut sequencing_ingestor, sequencer_rx) =
        Ingestor::new(sequencing_config.into()).await.map_err(|e| {
            RuntimeError::Initialization(format!(
                "Failed to create ingestor for sequencing chain: {}",
                e
            ))
        })?;

    let (mut settlement_ingestor, settlement_rx) =
        Ingestor::new(settlement_config.into()).await.map_err(|e| {
            RuntimeError::Initialization(format!(
                "Failed to create ingestor for settlement chain: {}",
                e
            ))
        })?;

    let slotter = Slotter::new(sequencer_rx, settlement_rx, slotting_config);

    // TODO(SEQ-515): refactor me to get the channel without starting the slotter already?
    // TODO(SEQ-515): slotter assumes that it starts first, or else it errors here
    let slot_rx = slotter.await.start();

    let block_builder = BlockBuilder::new(slot_rx, block_builder_config).await.map_err(|e| {
        RuntimeError::Initialization(format!("Failed to create block builder: {}", e))
    })?;

    info!("Starting Metabased Translator");
    let sequencing_ingestor_handle = tokio::spawn(async move {
        if let Err(e) = sequencing_ingestor.start_polling().await {
            error!("Ingestor error: {}", e);
        }
    });

    let settlement_ingestor_handle = tokio::spawn(async move {
        if let Err(e) = settlement_ingestor.start_polling().await {
            error!("Ingestor error: {}", e);
        }
    });

    // TODO(SEQ-515): Block builder doesn't error
    let block_builder_handle = tokio::spawn(async move { block_builder.start().await });

    // Main control loop
    tokio::select! {
        // Wait for shutdown signal
        _ = &mut shutdown_rx => {
            info!("Metabased Translator shutting down...");
        }
        // Watch for task completion/errors
        res = settlement_ingestor_handle => {
            if let Err(e) = res {
                error!("Settlement chain ingestor task failed: {}", e);
            }
        }
        res = sequencing_ingestor_handle => {
            if let Err(e) = res {
                error!("Sequencing chain ingestor task failed: {}", e);
            }
        }
        res = block_builder_handle => {
            if let Err(e) = res {
                error!("Block builder task failed: {}", e);
            }
        }
    }

    info!("Metabased Translator shutdown complete");
    Ok(())
}

fn main() -> Result<(), RuntimeError> {
    init_tracing()?;

    // Parse base config from CLI/env
    let base_config = MetabasedConfig::parse();

    // Create and run async runtime
    tokio::runtime::Runtime::new()
        .map_err(|e| RuntimeError::Initialization(e.to_string()))?
        .block_on(run(
            base_config.block_builder,
            base_config.slotter,
            IngestionPipelineConfig {
                sequencing: base_config.sequencing,
                settlement: base_config.settlement,
            },
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
