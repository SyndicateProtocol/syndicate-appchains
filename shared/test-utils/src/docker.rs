//! Docker components for the integration tests

use crate::{port_manager::PortManager, rollup::rollup_info, utils::test_path};
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, RootProvider},
};
use eyre::Result;
use mchain::mchain::{rollup_config, MProvider};
use std::{env, time::Duration};
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::{sleep, timeout},
};

#[derive(Debug)]
pub struct Docker(pub Child);

impl Drop for Docker {
    fn drop(&mut self) {
        if let Some(x) = self.0.id() {
            _ = std::process::Command::new("kill").arg(x.to_string()).output();
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    _ = self.0.wait().await;
                })
            })
        }
    }
}

pub async fn start_mchain(
    chain_id: u64,
    chain_owner: Address,
    finality_delay: u64,
) -> Result<(String, Docker, MProvider)> {
    let temp = test_path("mchain");
    let port = PortManager::instance().next_port();
    let docker = Docker(
        Command::new("cargo")
            .current_dir("../")
            .arg("run")
            .arg("--bin")
            .arg("mchain")
            .arg("--")
            .args([
                "--chain-id",
                &chain_id.to_string(),
                "--chain-owner",
                &chain_owner.to_string(),
                "--port",
                &port.to_string(),
                "--datadir",
                &temp.to_string(),
                "--finality-delay",
                &finality_delay.to_string(),
            ])
            .spawn()?,
    );
    let url = format!("http://localhost:{port}");
    let mchain = MProvider::new(url.parse()?);
    timeout(Duration::from_secs(120), async {
        while !mchain.connected().await {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((url, docker, mchain))
    })
    .await?
}

/// Starts nitro instance
pub async fn launch_nitro_node(
    chain_id: u64,
    chain_owner: Address,
    mchain_url: &str,
    sequencer_port: Option<u16>,
) -> Result<(Docker, RootProvider, String)> {
    let tag = env::var("NITRO_TAG").unwrap_or("v3.4.0-d896e9c-slim".to_string());
    let port = PortManager::instance().next_port();

    let log_level = env::var("NITRO_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    let mut cmd = Command::new("docker");
    cmd.arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg(format!("offchainlabs/nitro-node:{tag}"))
        .arg(format!("--parent-chain.connection.url={mchain_url}"))
        .arg("--node.dangerous.disable-blob-reader")
        .arg("--node.inbox-reader.check-delay=100ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg(format!(
            "--chain.info-json={}",
            rollup_info(&rollup_config(chain_id, chain_owner), "test")
        ))
        .arg("--http.addr=0.0.0.0")
        .arg("--http.api=net,web3,eth,debug,trace")
        .arg(format!("--http.port={}", port))
        .arg(format!("--log-level={log_level}"));

    match sequencer_port {
        Some(port) => {
            cmd.arg(format!("--execution.forwarding-target=http://localhost:{port}"));
        }
        None => {
            cmd.arg("--execution.forwarding-target=null");
        }
    }

    let nitro = Docker(cmd.spawn()?);
    let rollup = ProviderBuilder::default().on_http(format!("http://localhost:{}", port).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((nitro, rollup, format!("http://localhost:{}", port)))
    })
    .await?
}
