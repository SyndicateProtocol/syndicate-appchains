use eyre::Result;
use shared::tracing::{setup_global_tracing, ServiceTracingConfig};
use synd_translator::{config::TranslatorConfig, config_manager::with_onchain_config};
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    setup_global_tracing(ServiceTracingConfig::from_env(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))?;

    let base_config = TranslatorConfig::initialize();

    info!("Base configuration {:?}", base_config);
    if let Err(e) = base_config.validate() {
        error!("Failed to initialize TranslatorConfig: {e}");
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

    // Load chain config from ConfigManager if available
    let config = with_onchain_config(&base_config).await;
    config.validate_strict()?;

    synd_translator::spawn::run(&config).await?;

    Ok(())
}
