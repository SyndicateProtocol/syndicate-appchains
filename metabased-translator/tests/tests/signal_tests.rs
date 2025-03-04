//! Integration tests for the metabased-translator handling termination signals

use block_builder::connectors::mchain::MCHAIN_ID;
use e2e_tests::full_meta_node::{start_anvil, start_reth};
use eyre::Result;
use reqwest::Client;
use serial_test::serial;
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

    let (node, _mchain) = start_reth(MCHAIN_ID).await?;
    let mut metabased_process = TokioCommand::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("metabased-translator")
        .current_dir("../bin/metabased-translator")
        .arg("--")
        .args([
            "--mchain-ipc-path",
            &node.ipc,
            "--mchain-auth-ipc-path",
            &node.auth_ipc,
            "--sequencing-contract-address",
            "0x0000000000000000000000000000000000000001",
            "--bridge-address",
            "0x0000000000000000000000000000000000000002",
            "--inbox-address",
            "0x0000000000000000000000000000000000000003",
            "--sequencing-rpc-url",
            &format!("http://localhost:{}", seq_port),
            "--sequencing-start-block",
            "0",
            "--settlement-rpc-url",
            &format!("http://localhost:{}", set_port),
            "--settlement-start-block",
            "0",
        ])
        .spawn()?;

    // Wait for metabased-translator to be ready. We can do that by hitting the metrics endpoint
    wait_for_service("http://localhost:9090/metrics").await?;

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
#[serial]
async fn test_metabased_sigterm() -> Result<()> {
    run_metabased_translator("-TERM").await
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_metabased_sigint() -> Result<()> {
    run_metabased_translator("-INT").await
}
