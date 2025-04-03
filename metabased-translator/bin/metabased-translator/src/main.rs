use block_builder::{
    config::TargetRollupType::{ARBITRUM, OPTIMISM},
    rollups::{
        arbitrum::arbitrum_adapter::ArbitrumAdapter, optimism::optimism_adapter::OptimismAdapter,
    },
};
use common::tracing::init_tracing_with_extra_fields;
use eyre::Result;
use metabased_translator::{
    config::MetabasedConfig,
    spawn::{get_extra_fields_for_logging, run},
    types::RuntimeError,
};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::oneshot,
};
use tracing::{error, info};

fn main() -> Result<(), RuntimeError> {
    let base_config = MetabasedConfig::initialize();
    let extra_fields = get_extra_fields_for_logging(base_config.clone());
    init_tracing_with_extra_fields(extra_fields)?;

    info!("Base configuration {:?}", base_config);
    if let Err(e) = base_config.validate() {
        error!("Failed to initialize MetabasedConfig: {}", e);
        std::process::exit(1);
    };

    let runtime =
        tokio::runtime::Runtime::new().map_err(|e| RuntimeError::Initialization(e.to_string()))?;
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
    init_signal_handler(&runtime, shutdown_tx);

    // Run the async process
    match base_config.block_builder.target_rollup_type {
        OPTIMISM => {
            runtime.block_on(run(
                &base_config,
                OptimismAdapter::new(&base_config.block_builder),
                &mut shutdown_rx,
            ))?;
        }
        ARBITRUM => {
            runtime.block_on(run(
                &base_config,
                ArbitrumAdapter::new(&base_config.block_builder),
                &mut shutdown_rx,
            ))?;
        }
    }

    Ok(())
}

fn init_signal_handler(runtime: &tokio::runtime::Runtime, shutdown_tx: oneshot::Sender<()>) {
    runtime.spawn(async move {
        let mut sigint =
            signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
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

        if shutdown_tx.send(()).is_err() {
            panic!("Failed to send shutdown signal");
        }
    });
}
