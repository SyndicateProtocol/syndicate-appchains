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

    // Run the async process
    match base_config.block_builder.target_rollup_type {
        OPTIMISM => {
            runtime
                .block_on(run(&base_config, OptimismAdapter::new(&base_config.block_builder)))?;
        }
        ARBITRUM => {
            runtime
                .block_on(run(&base_config, ArbitrumAdapter::new(&base_config.block_builder)))?;
        }
    }

    Ok(())
}
