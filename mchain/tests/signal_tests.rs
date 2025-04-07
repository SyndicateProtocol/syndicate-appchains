//! Integration tests for the mchain handling termination signals

use alloy::primitives::Address;
use eyre::Result;
use std::process::Command;
use test_utils::docker::start_mchain;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

async fn run_mchain(signal: &str) -> Result<()> {
    let (_, mut mchain, _) = start_mchain(13331370, Address::ZERO, 0).await?;
    let pid = mchain.id().ok_or_else(|| eyre::eyre!("Failed to get process ID"))?;
    Command::new("kill").arg(signal).arg(pid.to_string()).status()?;
    let status = mchain.wait().await?;
    assert_eq!(status.code(), Some(0));

    Ok(())
}

#[tokio::test]
async fn test_metabased_sigterm() -> Result<()> {
    run_mchain("-TERM").await
}

#[tokio::test]
async fn test_metabased_sigint() -> Result<()> {
    run_mchain("-INT").await
}
