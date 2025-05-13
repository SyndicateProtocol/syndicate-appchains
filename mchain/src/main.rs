//! The metachain is used for rollup block derivation.

use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    rpc::client::ClientBuilder,
};
use clap::Parser;
use contract_bindings::syndicate::{arbchainconfig, arbconfigmanager::ArbConfigManager};
use jsonrpsee::server::{RandomStringIdProvider, middleware::http::ProxyGetRequestLayer, RpcServiceBuilder, Server};
use mchain::{metrics::MchainMetrics, server::start_mchain};
use rocksdb::DB;
use shared::{
    logger::set_global_default_subscriber,
    service_start_utils::{start_metrics_and_health, MetricsState},
};
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

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
    let arb_chain_config_contract = ArbChainConfig::new(config_address._0, provider);

    let rollup_owner = arb_chain_config_contract.ROLLUP_OWNER().call().await?;
    Ok(rollup_owner._0)
}

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    #[allow(clippy::unwrap_used)]
    set_global_default_subscriber().unwrap();

    let cfg = Config::parse();
    info!("loading rockdb db {}", cfg.datadir);
    let db = DB::open_default(cfg.datadir)?;
    let mut metrics_state = MetricsState::default();
    let metrics = MchainMetrics::new(&mut metrics_state.registry);

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

    info!("starting mchain server on port {}", cfg.port);
    let module = start_mchain(
        cfg.appchain_chain_id,
        initial_appchain_owner,
        cfg.finality_delay,
        db,
        metrics,
    );

    let handle = Server::builder()
        .ws_only()
        .set_id_provider(RandomStringIdProvider::new(64))
        .set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
        .start(module);
    tokio::spawn(start_metrics_and_health(metrics_state, cfg.metrics_port));

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

    _ = handle.stop();
    handle.stopped().await;
    Ok(())
}
