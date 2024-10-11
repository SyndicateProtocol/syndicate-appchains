mod app;
mod endpoint;
mod json_rpc_errors;
mod server;
mod transaction;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");

    let port = 8456;
    let (addr, handle) = server::run(port).await?;
    println!("RPC Server started on {}", addr);

    // Keep the server running
    handle.stopped().await;

    Ok(())
}
