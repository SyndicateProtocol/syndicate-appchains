use block_builder::{
    config::TargetRollupType::{ARBITRUM, OPTIMISM},
    rollups::{
        arbitrum::arbitrum_adapter::ArbitrumAdapter, optimism::optimism_adapter::OptimismAdapter,
    },
};
use eyre::Result;
use metabased_translator::{config::MetabasedConfig, spawn::run};
use shared::logger::set_global_default_subscriber;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::oneshot,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    let base_config = MetabasedConfig::initialize();

    info!("Base configuration {:?}", base_config);
    if let Err(e) = base_config.validate() {
        error!("Failed to initialize MetabasedConfig: {}", e);
        std::process::exit(1);
    };

    tokio::spawn(async move {
        let mut sigint =
            signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                info!("Received SIGINT (Ctrl+C), terminating...");
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM, initiating shutdown...");
            }
        }
        std::process::exit(0);
    });

    // Run the async process
    let (_shutdown_tx, mut shutdown_rx) = oneshot::channel();
    match base_config.block_builder.target_rollup_type {
        OPTIMISM => {
            run(&base_config, OptimismAdapter::new(&base_config.block_builder), &mut shutdown_rx)
                .await?;
        }
        ARBITRUM => {
            run(&base_config, ArbitrumAdapter::new(&base_config.block_builder), &mut shutdown_rx)
                .await?;
        }
    }

    Ok(())
}
