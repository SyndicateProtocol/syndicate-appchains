//! Integration tests for the `synd-mchain` handling termination signals

use eyre::Result;
use std::process::Command;
use test_utils::docker::start_mchain;

#[ctor::ctor]
fn init() {
    shared::logger::set_global_default_subscriber();
}

async fn run_mchain(signal: &str) -> Result<()> {
    let (_, mut mchain, _) = start_mchain(13331370, 0).await?;
    let pid = mchain.id().ok_or_else(|| eyre::eyre!("Failed to get process ID"))?;
    Command::new("kill").arg(signal).arg(pid.to_string()).status()?;
    let status = mchain.wait().await?;
    assert_eq!(status.code(), Some(0));

    Ok(())
}

#[tokio::test]
async fn test_mchain_sigterm() -> Result<()> {
    run_mchain("-TERM").await
}

#[tokio::test]
async fn test_mchain_sigint() -> Result<()> {
    run_mchain("-INT").await
}
