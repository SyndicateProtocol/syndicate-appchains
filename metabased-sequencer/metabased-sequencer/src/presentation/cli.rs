use crate::presentation::{configuration::Configuration, server};
use core::fmt;
use eyre::Result;
use std::{error::Error, fmt::Debug};
use tracing::info;
use tracing_subscriber::{fmt as subscriber_fmt, EnvFilter};

/// Possible errors that can occur when initializing the tracing subscriber
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

pub async fn run() -> Result<()> {
    let args = Configuration::parse()?;
    let (addr, handle) = server::run(args).await?;

    info!(
        addr = %addr,
        "Metabased Sequencer server running"
    );

    // Keep the server running
    handle.stopped().await;

    Ok(())
}
