//! Metabased Poster is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

use eyre::Result;
use metabased_poster::{
    config::Config,
    shutdown_channels::{ShutdownChannels, ShutdownTx},
    spawn::ComponentHandles,
    types::RuntimeError,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), RuntimeError> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    let config = Config::initialize();
    info!("Config: {:?}", config);

    let runtime =
        tokio::runtime::Runtime::new().map_err(|e| RuntimeError::Initialization(e.to_string()))?;

    runtime.block_on(run(&config))?;
    Ok(())
}

async fn run(config: &Config) -> Result<(), RuntimeError> {
    let shutdown_channels = ShutdownChannels::new();
    // TODO (SEQ-695): Poster metrics
    let component_tasks = ComponentHandles::spawn(config, shutdown_channels.rx);
    info!("Starting Poster service...");
    start_shutdown_handling(shutdown_channels.main, shutdown_channels.tx, component_tasks).await
}

#[allow(clippy::redundant_pub_crate)]
async fn start_shutdown_handling(
    main_shutdown_rx: tokio::sync::oneshot::Receiver<()>,
    tx: ShutdownTx,
    mut handles: ComponentHandles,
) -> Result<(), RuntimeError> {
    // MAIN SELECT LOOP - wait for shutdown signal or task failure
    tokio::select! {
        _ = main_shutdown_rx => handles.graceful_shutdown(tx).await,
        res = &mut handles.poller => handles.check_error(res, "Poller"),
        res = &mut handles.submitter => handles.check_error(res, "Submitter"),
    }
}
