use crate::presentation::configuration::Configuration;
use crate::presentation::server;
use tracing_subscriber::EnvFilter;

pub fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
}

pub async fn run() -> anyhow::Result<()> {
    let args = Configuration::parse()?;
    let (addr, handle) = server::run(
        args.port,
        args.chain_contract_address,
        args.chain_rpc_address,
        args.private_key,
    )
    .await?;

    println!("RPC Server started on {addr}");

    // Keep the server running
    handle.stopped().await;

    Ok(())
}
