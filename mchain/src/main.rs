//! The metachain is used for rollup block derivation.

use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    rpc::client::ClientBuilder,
};
use clap::Parser;
use contract_bindings::metabased::{arbchainconfig, arbconfigmanager::ArbConfigManager};
use jsonrpsee::server::{RpcServiceBuilder, Server};
use mchain::{metrics::MchainMetrics, server::start_mchain};
use rocksdb::DB;
use shared::{
    logger::set_global_default_subscriber,
    metrics::{start_metrics, MetricsState},
};
use tokio::signal::unix::{signal, SignalKind};

/// CLI args for the mchain executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
struct Config {
    // time delay until a block is considered finalized
    #[arg(long, env = "FINALITY_DELAY", default_value_t = 60)]
    finality_delay: u64,
    #[arg(long, env = "DATADIR", default_value = "./datadir")]
    datadir: String,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    port: u64,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    metrics_port: u16,
    #[arg(long, env = "APPCHAIN_CHAIN_ID")]
    appchain_chain_id: u64,
    #[arg(long, env = "APPCHAIN_OWNER")]
    appchain_owner: Option<Address>,
    #[arg(long, env = "SETTLEMENT_RPC_URL")]
    settlement_rpc_url: Option<String>,
    #[arg(long, env = "CONFIG_MANAGER_ADDRESS")]
    config_manager_address: Option<Address>,
}

async fn get_rollup_owner(
    config_manager_address: Option<Address>,
    appchain_chain_id: u64,
    settlement_rpc_url: &str,
) -> eyre::Result<Address> {
    let address = config_manager_address.map_or_else(
        || {
            panic!("No config_manager_address provided, cannot fetch on-chain config");
        },
        |addr| addr,
    );

    #[allow(clippy::unwrap_used)]
    let provider = ProviderBuilder::new()
        .on_client(ClientBuilder::default().connect(settlement_rpc_url).await.unwrap());

    let config_manager_contract = ArbConfigManager::new(address, provider.clone());
    let config_address = config_manager_contract
        .getArbChainConfigAddress(U256::from(appchain_chain_id))
        .call()
        .await?;
    let arb_chain_config_contract =
        arbchainconfig::ArbChainConfig::new(config_address._0, provider);

    let rollup_owner = arb_chain_config_contract.ROLLUP_OWNER().call().await?;
    Ok(rollup_owner._0)
}

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    let cfg = Config::parse();
    let db = DB::open_default(cfg.datadir)?;
    let mut metrics_state = MetricsState::default();
    let metrics = MchainMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics(metrics_state, cfg.metrics_port));

    let initial_appchain_owner = match cfg.appchain_owner {
        Some(owner) => owner,
        None => {
            let settlement_rpc_url = cfg.settlement_rpc_url.as_deref().ok_or_else(|| {
                eyre::eyre!("SETTLEMENT_RPC_URL must be provided when APPCHAIN_OWNER is not set")
            })?;

            get_rollup_owner(cfg.config_manager_address, cfg.appchain_chain_id, settlement_rpc_url)
                .await?
        }
    };

    let module = start_mchain(
        cfg.appchain_chain_id,
        initial_appchain_owner,
        cfg.finality_delay,
        db,
        metrics,
    )
    .await;
    let handle = Server::builder()
        .set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
        .start(module);

    #[allow(clippy::expect_used)]
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    #[allow(clippy::expect_used)]
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {
            println!("Received SIGINT (Ctrl+C), initiating shutdown...");
        }
        _ = sigterm.recv() => {
            println!("Received SIGTERM, initiating shutdown...");
        }
    };

    handle.stop()?;
    handle.stopped().await;
    Ok(())
}
