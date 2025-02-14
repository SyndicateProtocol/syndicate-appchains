//! Integration tests for the metabased stack
#![allow(missing_docs)]

use e2e_tests::{full_meta_node::start_anvil, port_manager::PortManager};
use eyre::Result;
use serial_test::serial;
use std::{process::Command, time::Duration};
use tokio::{process::Command as TokioCommand, time::sleep};

#[tokio::test]
#[serial]
async fn test_metabased_sigterm() -> Result<()> {
    let port_tracker = PortManager::instance();
    let seq_port = port_tracker.next_port();
    let (_seq_instance, _seq_provider) = start_anvil(seq_port, 15).await?;
    let set_port = port_tracker.next_port();
    let (_set_instance, _set_provider) = start_anvil(set_port, 20).await?;

    let mut metabased_process = TokioCommand::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("metabased-translator")
        .current_dir("../bin/metabased-translator")
        .arg("--") // <-- This separates Cargo arguments from binary arguments
        .args([
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

    // Wait for some time to ensure the process is running
    sleep(Duration::from_millis(3000)).await;

    let pid = metabased_process.id().expect("Failed to get process ID");

    Command::new("kill").arg("-TERM").arg(pid.to_string()).status()?;

    metabased_process.wait().await?;

    match metabased_process.try_wait()? {
        Some(status) => {
            assert!(status.success(), "Metabased process did not exit cleanly");
        }
        None => {
            panic!("Metabased process is still running after SIGTERM");
        }
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_metabased_sigint() -> Result<()> {
    let port_tracker = PortManager::instance();
    let seq_port = port_tracker.next_port();
    let (_seq_instance, _seq_provider) = start_anvil(seq_port, 15).await?;
    let set_port = port_tracker.next_port();
    let (_set_instance, _set_provider) = start_anvil(set_port, 20).await?;

    let mut metabased_process = TokioCommand::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("metabased-translator")
        .current_dir("../bin/metabased-translator")
        .arg("--") // <-- This separates Cargo arguments from binary arguments
        .args([
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

    // Wait for some time to ensure the process is running
    sleep(Duration::from_millis(3000)).await;

    let pid = metabased_process.id().expect("Failed to get process ID");

    Command::new("kill").arg("-INT").arg(pid.to_string()).status()?;

    metabased_process.wait().await?;

    match metabased_process.try_wait()? {
        Some(status) => {
            assert!(status.success(), "Metabased process did not exit cleanly");
        }
        None => {
            panic!("Metabased process is still running after SIGTERM");
        }
    }

    Ok(())
}
