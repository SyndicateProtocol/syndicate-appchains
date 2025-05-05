//! The chain-ingestor servers blocks from a chain to all rollups that use the chain.

use chain_ingestor::{eth_client::EthClient, ingestor, metrics::ChainIngestorMetrics, server};
use clap::{command, Parser};
use humantime::parse_duration;
use jsonrpsee::server::Server;
use shared::{
    logger::set_global_default_subscriber,
    metrics::{start_metrics, MetricsState},
};
use std::time::Duration;
use tracing::error;

/// CLI args for the chain ingestor executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
struct Config {
    // time delay until a block is considered finalized
    #[arg(long, env = "RPC_URL")]
    rpc_url: String,
    #[arg(long, env = "DB_FILE")]
    db_file: String,
    #[arg(long, env = "START_BLOCK")]
    start_block: u64,
    #[arg(long, env = "BUFFER_SIZE", default_value_t = 1000)]
    buffer_size: u64,
    #[arg(long, env = "PARALLEL_SYNC_REQUESTS", default_value_t = 100)]
    parallel_sync_requests: u64,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    port: u16,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    metrics_port: u16,
    #[arg(
        long,
        env = "REQUEST_TIMEOUT",
        default_value = "10s",
        value_parser = parse_duration
    )]
    request_timeout: Duration,
    #[arg(long, env = "GET_LOGS_TIMEOUT", default_value = "300s", value_parser = parse_duration)]
    get_logs_timeout: Duration,
}

async fn new_provider(cfg: &Config) -> EthClient {
    EthClient::new(&cfg.rpc_url, cfg.request_timeout, cfg.get_logs_timeout).await
}

#[tokio::main]
async fn main() {
    // Initialize logging
    set_global_default_subscriber().unwrap();

    let cfg = Config::parse();
    let mut provider = new_provider(&cfg).await;
    let (module, ctx) = server::start(
        provider.clone(),
        &cfg.rpc_url,
        &cfg.db_file,
        cfg.start_block,
        cfg.parallel_sync_requests,
    )
    .await
    .unwrap();
    let _handle = Server::builder()
        .ws_only()
        .build(format!("0.0.0.0:{}", cfg.port))
        .await
        .unwrap()
        .start(module);
    let mut metrics_state = MetricsState::default();
    let metrics = ChainIngestorMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics(metrics_state, cfg.metrics_port));
    loop {
        if let Err(err) = ingestor::run(&ctx, &provider, cfg.buffer_size, &metrics).await {
            error!("ingestor failed: {}", err);
            // manually recreate the ws connection just in case.
            // the old connection still exists & will keep retrying endlessly.
            provider = new_provider(&cfg).await;
            ctx.lock().unwrap().provider = provider.clone();
        }
    }
}
