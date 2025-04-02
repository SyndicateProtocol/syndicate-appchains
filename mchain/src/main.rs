//! The metachain is used for rollup block derivation.

use alloy::primitives::Address;
use block_builder::config::get_default_private_key_signer;
use clap::Parser;
use jsonrpsee::tokio;
use mchain::mchain::MDB;
use rocksdb::DB;
use std::sync::Arc;

/// CLI args for the mchain executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
pub struct Config {
    #[arg(long, env = "DATADIR", default_value = "./datadir")]
    pub datadir: String,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    pub port: u64,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    pub metrics_port: u64,
    #[arg(long, env = "CHAIN_ID")]
    pub chain_id: u64,
    #[arg(long, env = "CHAIN_OWNER", default_value_t = get_default_private_key_signer().address())]
    pub chain_owner: Address,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cfg = Config::parse();
    let db = Arc::new(DB::open_default(cfg.datadir)?);
    let _mdb = MDB::start(cfg.chain_id, cfg.chain_owner, cfg.port, cfg.metrics_port, db).await?;
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    }
}
