use crate::presentation::configuration::Configuration;
use crate::presentation::server;
use core::fmt;
use std::error::Error;
use std::fmt::Debug;
use tracing::info;
use tracing_subscriber::{fmt as subscriber_fmt, EnvFilter};

#[derive(Debug)]
pub enum TracingError {
    SubscriberInit(String),
}

impl fmt::Display for TracingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TracingError::SubscriberInit(msg) => {
                write!(f, "Failed to initialize subscriber: {}", msg)
            }
        }
    }
}

impl Error for TracingError {}

pub fn init_tracing_subscriber() -> Result<(), TracingError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    subscriber_fmt()
        .json()
        .with_target(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;

    Ok(())
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

    info!(
        addr = %addr,
        "Interceptor server running"
    );

    // Keep the server running
    handle.stopped().await;

    Ok(())
}
