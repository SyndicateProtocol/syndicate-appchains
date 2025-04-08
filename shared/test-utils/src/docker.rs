//! Docker components for the integration tests

use crate::{port_manager::PortManager, rollup::rollup_info, utils::test_path, wait_until};
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, RootProvider},
};
use eyre::Result;
use mchain::mchain::{rollup_config, MProvider};
use std::{
    env,
    process::{ExitStatus, Stdio},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    process::{Child, Command},
};
use tracing::info;

#[derive(Debug)]
pub struct Docker(Child);

impl Docker {
    pub fn new(cmd: &mut Command) -> Result<Self> {
        let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        let stdout = child.stdout.take().unwrap();
        // force tests to capture output from stdout
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Some(line) = reader.next_line().await.unwrap() {
                println!("{}", line);
            }
        });
        // force tests to capture output from stderr
        let stderr = child.stderr.take().unwrap();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line) = reader.next_line().await.unwrap() {
                eprintln!("{}", line);
            }
        });
        Ok(Self(child))
    }
    pub fn id(&self) -> Option<u32> {
        self.0.id()
    }
    pub async fn wait(&mut self) -> std::io::Result<ExitStatus> {
        self.0.wait().await
    }
}

impl Drop for Docker {
    fn drop(&mut self) {
        if let Some(pid) = self.0.id() {
            if let Ok(None) = self.0.try_wait() {
                // drop the file descriptors to suppress termination output.
                self.0.stdin.take();
                self.0.stdout.take();
                self.0.stderr.take();

                let _ = std::process::Command::new("kill").arg(pid.to_string()).spawn();
            }
        }
    }
}

pub async fn start_component(
    executable_name: &str,
    args: Vec<String>,
    cargs: Vec<String>,
) -> Result<Docker> {
    info!("launching {}", executable_name);
    if let Ok(tag) = env::var(executable_name.to_uppercase().replace("-", "_") + "_TAG") {
        Docker::new(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("--net=host")
                .arg(format!("ghcr.io/syndicateprotocol/{executable_name}:{tag}"))
                .args(args),
        )
    } else {
        Docker::new(
            Command::new("cargo")
                // ring has a custom build.rs script that rebuilds whenever certain environment vars
                // change
                .env_remove("CARGO_MANIFEST_DIR")
                .env_remove("CARGO_PKG_NAME")
                .env_remove("CARGO_PKG_VERSION_MAJOR")
                .env_remove("CARGO_PKG_VERSION_MINOR")
                .env_remove("CARGO_PKG_VERSION_PATCH")
                .env_remove("CARGO_PKG_VERSION_PRE")
                .env_remove("CARGO_MANIFEST_LINKS")
                .current_dir(env!("CARGO_WORKSPACE_DIR"))
                .arg("run")
                .arg("--bin")
                .arg(executable_name)
                .arg("--")
                .args(args)
                .args(cargs),
        )
    }
}

pub async fn start_mchain(
    chain_id: u64,
    chain_owner: Address,
    finality_delay: u64,
) -> Result<(String, Docker, MProvider)> {
    let temp = test_path("mchain");
    let port = PortManager::instance().next_port();
    let metric_port = PortManager::instance().next_port();
    let docker = start_component(
        "mchain",
        vec![
            "--chain-id".to_string(),
            chain_id.to_string(),
            "--chain-owner".to_string(),
            chain_owner.to_string(),
            "--port".to_string(),
            port.to_string(),
            "--metrics-port".to_string(),
            metric_port.to_string(),
            "--finality-delay".to_string(),
            finality_delay.to_string(),
        ],
        vec!["--datadir".to_string(), temp.to_string()],
    )
    .await?;
    let url = format!("http://localhost:{port}");
    let mchain = MProvider::new(url.parse()?);
    wait_until!(mchain.connected().await, Duration::from_secs(120));
    Ok((url, docker, mchain))
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

    let nitro = Docker::new(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("--net=host")
            .arg(format!("offchainlabs/nitro-node:{tag}"))
            .arg(format!("--parent-chain.connection.url={mchain_url}"))
            .arg("--node.dangerous.disable-blob-reader")
            .arg("--node.inbox-reader.check-delay=50ms")
            .arg("--node.staker.enable=false")
            .arg("--ensure-rollup-deployment=false")
            .arg(format!(
                "--chain.info-json={}",
                rollup_info(&rollup_config(chain_id, chain_owner), "test")
            ))
            .arg("--http.addr=0.0.0.0")
            .arg("--http.api=net,web3,eth,debug,trace")
            .arg(format!("--http.port={}", port))
            .arg(format!("--log-level={log_level}"))
            .arg(if let Some(port) = sequencer_port {
                format!("--execution.forwarding-target=http://localhost:{port}")
            } else {
                "--execution.forwarding-target=null".to_string()
            }),
    )?;

    let url = format!("http://localhost:{}", port);

    let rollup = ProviderBuilder::default().on_http(url.parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    wait_until!(rollup.get_chain_id().await.is_ok(), Duration::from_secs(120));
    Ok((nitro, rollup, url))
}
