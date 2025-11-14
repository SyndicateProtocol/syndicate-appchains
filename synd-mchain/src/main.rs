//! The `MockChain` is used for appchain block derivation.

use alloy::primitives::B256;
use clap::Parser;
use shared::parse::parse_hash;
use synd_mchain::db::MigrationParams;
#[cfg(feature = "rocksdb")]
use {
    jsonrpsee::server::middleware::http::ProxyGetRequestLayer,
    jsonrpsee::server::{RandomStringIdProvider, Server},
    rocksdb::DB,
    shared::service_start_utils::start_http_server_with_metrics_only,
    shared::{service_start_utils::MetricsState, tracing::setup_global_logging},
    synd_mchain::{metrics::MchainMetrics, server::start_mchain},
    tokio::signal::unix::{signal, SignalKind},
    tracing::info,
};

/// CLI args for the `synd-mchain` executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
struct MchainConfig {
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

    /// The genesis config for a migrated appchain
    #[arg(long, env = "GENESIS_CONFIG")]
    genesis_config: Option<String>,

    /// The block number at which the migration took place (last known L1 block from the rollup
    /// PoV)
    #[arg(long = "settlement-start-block", env = "SETTLEMENT_START_BLOCK")]
    pub settlement_start_block: Option<u64>,

    /// The batch accumulator at the point of migration
    #[arg(long, env = "MIGRATED_BATCH_ACC", value_parser = parse_hash)]
    pub migrated_batch_acc: Option<B256>,

    /// The batch accumulator at the point of migration
    #[arg(long, env = "MIGRATED_BATCH_COUNT")]
    pub migrated_batch_count: Option<u64>,

    /// The delayed messages accumulator at the point of migration
    #[arg(long, env = "MIGRATED_DELAYED_MSGS_ACC", value_parser = parse_hash)]
    pub migrated_delayed_msgs_acc: Option<B256>,

    /// The delayed messages count at the point of migration
    #[arg(long, env = "MIGRATED_DELAYED_MSGS_COUNT")]
    pub migrated_delayed_msgs_count: Option<u64>,
}

impl MchainConfig {
    pub fn migration_config(&self) -> Option<MigrationParams> {
        self.migrated_batch_acc?;

        let settlement_start_block = self
            .settlement_start_block
            .unwrap_or_else(|| panic!("migration initial settlement block is none"));
        let batch_acc = self.migrated_batch_acc.unwrap_or_else(|| panic!("batch acc is none"));
        let batch_count =
            self.migrated_batch_count.unwrap_or_else(|| panic!("batch count is none"));
        let delayed_msgs_acc =
            self.migrated_delayed_msgs_acc.unwrap_or_else(|| panic!("delayed msgs acc is none"));
        let delayed_msgs_count = self
            .migrated_delayed_msgs_count
            .unwrap_or_else(|| panic!("delayed msgs count is none"));

        Some(MigrationParams {
            settlement_start_block,
            batch_acc,
            batch_count,
            delayed_msgs_acc,
            delayed_msgs_count,
        })
    }
}

#[tokio::main]
#[cfg(feature = "rocksdb")]
async fn main() -> eyre::Result<()> {
    use jsonrpsee::{
        server::{PingConfig, ServerConfigBuilder},
        ws_client::RpcServiceBuilder,
    };
    // Initialize logging
    setup_global_logging()?;

    let cfg = MchainConfig::parse();
    info!("loading rocksdb db {}", cfg.datadir);
    let db = DB::open_default(cfg.datadir.clone())?;

    let mut metrics_state = MetricsState::default();
    let metrics = MchainMetrics::new(&mut metrics_state.registry);

    // TODO read migration cfg from config_manager

    info!("starting synd-mchain server on port {}", cfg.port);
    tokio::spawn(start_http_server_with_metrics_only(metrics_state, cfg.metrics_port));
    let module = start_mchain(
        cfg.appchain_chain_id,
        cfg.finality_delay,
        cfg.genesis_config.clone(),
        cfg.migration_config(),
        db,
        metrics,
    );
    let jsonrpsee_cfg = ServerConfigBuilder::new()
        .enable_ws_ping(PingConfig::default())
        .set_id_provider(RandomStringIdProvider::new(64))
        .build();

    let http_middleware = tower::builder::ServiceBuilder::new()
        .layer(ProxyGetRequestLayer::new([("/health", "health"), ("/ready", "ready")])?);
    let handle = Server::builder()
        .set_config(jsonrpsee_cfg)
        .set_http_middleware(http_middleware)
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
    }

    _ = handle.stop();
    handle.stopped().await;
    Ok(())
}
#[tokio::main]
#[cfg(not(feature = "rocksdb"))]
async fn main() -> eyre::Result<()> {
    Ok(())
}
