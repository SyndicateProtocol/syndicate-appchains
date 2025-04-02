//! Integration tests for the metabased-translator handling termination signals

use alloy::primitives::Address;
use e2e_tests::{
    full_meta_node::{start_anvil, start_mchain},
    port_manager::PortManager,
};
use eyre::Result;
use reqwest::Client;
use std::{process::Command, time::Duration};
use tokio::{process::Command as TokioCommand, time::sleep};

async fn wait_for_service(url: &str) -> Result<()> {
    let client = Client::new();
    loop {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Metabased-translator is now running.");
                return Ok(());
            }
            _ => {
                println!("Waiting for metabased-translator to start...");
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

async fn run_metabased_translator(signal: &str) -> Result<()> {
    let (seq_port, _seq_instance, _seq_provider) = start_anvil(15).await?;
    let (set_port, _set_instance, _set_provider) = start_anvil(20).await?;

    let (mchain_port, _mchain) = start_mchain(13331370, Address::ZERO).await?;
    let metrics_port = PortManager::instance().next_port();
    let mut metabased_process = TokioCommand::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("metabased-translator")
        .current_dir("../bin/metabased-translator")
        .arg("--")
        .args([
            "--mchain-rpc-url",
            &format!("http://localhost:{mchain_port}"),
            "--sequencing-contract-address",
            "0x0000000000000000000000000000000000000001",
            "--arbitrum-bridge-address",
            "0x0000000000000000000000000000000000000002",
            "--arbitrum-inbox-address",
            "0x0000000000000000000000000000000000000003",
            "--sequencing-rpc-url",
            &format!("http://localhost:{}", seq_port),
            "--sequencing-start-block",
            "0",
            "--settlement-rpc-url",
            &format!("http://localhost:{}", set_port),
            "--settlement-start-block",
            "0",
            "--metrics-port",
            &metrics_port.to_string(),
        ])
        .spawn()?;

    // Wait for metabased-translator to be ready. We can do that by hitting the metrics endpoint
    wait_for_service(&format!("http://localhost:{metrics_port}/metrics")).await?;

    let pid = metabased_process.id().ok_or_else(|| eyre::eyre!("Failed to get process ID"))?;

    Command::new("kill").arg(signal).arg(pid.to_string()).status()?;

    metabased_process.wait().await?;

    match metabased_process.try_wait()? {
        Some(status) => {
            assert!(status.success(), "Metabased process did not exit cleanly");
        }
        None => {
            panic!("Metabased process is still running after {}", signal);
        }
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_metabased_sigterm() -> Result<()> {
    run_metabased_translator("-TERM").await
}

#[ignore] // TODO (SEQ-758) - flaky test that fails in CI
#[tokio::test(flavor = "multi_thread")]
async fn test_metabased_sigint() -> Result<()> {
    run_metabased_translator("-INT").await
}
