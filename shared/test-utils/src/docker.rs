//! Docker components for the integration tests

use crate::{appchain::appchain_info, port_manager::PortManager, utils::test_path, wait_until};
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::Client,
};
use eyre::Result;
use std::{
    env,
    future::Future,
    process::{ExitStatus, Stdio},
    time::Duration,
};
use synd_mchain::client::MProvider;
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    process::{Child, Command},
};
use tracing::{info, warn};

#[derive(Debug)]
pub struct Docker(Child);

impl Docker {
    pub fn new(cmd: &mut Command) -> Result<Self> {
        let mut child =
            cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
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
    pub fn wait(&mut self) -> impl Future<Output = std::io::Result<ExitStatus>> + use<'_> {
        self.0.wait()
    }
    pub fn try_wait(&mut self) -> std::io::Result<Option<ExitStatus>> {
        self.0.try_wait()
    }
}

impl Drop for Docker {
    fn drop(&mut self) {
        if self.0.try_wait().is_ok_and(|status| status.is_some()) {
            info!("Docker process for Drop already exited or was not running.");
            return;
        }

        let Some(pid) = self.0.id() else {
            info!("Docker process for Drop had no PID, likely already exited or failed to start.");
            return;
        };

        info!("Attempting to stop Docker container process (PID: {}) via Drop", pid);
        let kill_proc = match std::process::Command::new("kill")
            .arg(pid.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(proc) => proc,
            Err(e) => {
                warn!("Failed to spawn kill command for PID {}: {}", pid, e);
                return;
            }
        };

        match kill_proc.wait_with_output() {
            Ok(output) => {
                if output.status.success() {
                    info!("Successfully sent SIGTERM to PID {} and kill command exited.", pid);
                } else {
                    warn!(
                        "Kill command for PID {} completed with error. Status: {}. Stderr: {}. Stdout: {}",
                        pid,
                        output.status,
                        String::from_utf8_lossy(&output.stderr),
                        String::from_utf8_lossy(&output.stdout)
                    );
                }
            }
            Err(e) => {
                warn!("Failed to wait for kill command for PID {}: {}", pid, e);
            }
        }
    }
}

pub async fn start_component(
    executable_name: &str,
    api_port: u16,
    args: Vec<String>,
    cargs: Vec<String>,
) -> Result<Docker> {
    info!("launching {}", executable_name);
    let needs_rocksdb = executable_name == "synd-mchain";
    let mut docker = if let Ok(tag) =
        env::var(executable_name.to_uppercase().replace("-", "_") + "_TAG")
    {
        Docker::new(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("--net=host")
                .arg(format!("ghcr.io/syndicateprotocol/{executable_name}:{tag}"))
                .args(args),
        )
    } else if executable_name == "synd-proposer" {
        // Synd-proposer is a Go service, so we need to use the Go command to run it
        let mut cmd = Command::new("go");
        cmd.current_dir(format!("{}/synd-withdrawals/synd-proposer", env!("CARGO_WORKSPACE_DIR")));
        cmd.arg("run").arg("./cmd/synd-proposer");
        cmd.args(args.iter().chain(cargs.iter()));
        Docker::new(&mut cmd)
    } else {
        let mut cmd = Command::new("cargo");
        // ring has a custom build.rs script that rebuilds whenever certain environment
        // vars change
        cmd.env_remove("CARGO_MANIFEST_DIR")
            .env_remove("CARGO_PKG_NAME")
            .env_remove("CARGO_PKG_VERSION_MAJOR")
            .env_remove("CARGO_PKG_VERSION_MINOR")
            .env_remove("CARGO_PKG_VERSION_PATCH")
            .env_remove("CARGO_PKG_VERSION_PRE")
            .env_remove("CARGO_MANIFEST_LINKS")
            .current_dir(env!("CARGO_WORKSPACE_DIR"))
            .arg("run");

        if needs_rocksdb {
            cmd.arg("--features").arg("rocksdb");
        }

        cmd.arg("--bin").arg(executable_name).arg("--").args(args).args(cargs);
        Docker::new(&mut cmd)
    }?;

    health_check(executable_name, api_port, &mut docker).await;
    Ok(docker)
}

pub async fn health_check(executable_name: &str, api_port: u16, docker: &mut Docker) {
    let client = Client::new();
    wait_until!(
        if let Some(status) = docker.try_wait()? {
            panic!("{} exited with {}", executable_name, status);
        };
        client
            .get(format!("http://localhost:{api_port}/health"))
            .send()
            .await
            .is_ok_and(|x| x.status().is_success()),
        Duration::from_secs(5*60)  // give it time to download the image if necessary
    );
}

pub async fn start_mchain(
    appchain_chain_id: u64,
    finality_delay: u64,
) -> Result<(String, Docker, MProvider)> {
    let temp = test_path("synd-mchain");
    let port = PortManager::instance().next_port().await;
    let metric_port = PortManager::instance().next_port().await;

    let args = vec![
        "--appchain-chain-id".to_string(),
        appchain_chain_id.to_string(),
        "--port".to_string(),
        port.to_string(),
        "--metrics-port".to_string(),
        metric_port.to_string(),
        "--finality-delay".to_string(),
        finality_delay.to_string(),
    ];

    let docker = start_component(
        "synd-mchain",
        metric_port,
        args,
        vec!["--datadir".to_string(), temp.to_string()],
    )
    .await?;
    let url = format!("ws://localhost:{port}");
    let mchain = MProvider::new(&url).await?;
    Ok((url, docker, mchain))
}

/// Return the on-chain config for a rollup with a given chain id
fn appchain_config(chain_id: u64, chain_owner: Address) -> String {
    let mut cfg = format!(
        r#"{{
            "chainId": {chain_id},
            "homesteadBlock": 0,
            "daoForkBlock": null,
            "daoForkSupport": true,
            "eip150Block": 0,
            "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "eip155Block": 0,
            "eip158Block": 0,
            "byzantiumBlock": 0,
            "constantinopleBlock": 0,
            "petersburgBlock": 0,
            "istanbulBlock": 0,
            "muirGlacierBlock": 0,
            "berlinBlock": 0,
            "londonBlock": 0,
            "clique": {{
            "period": 0,
            "epoch": 0
            }},
            "arbitrum": {{
            "EnableArbOS": true,
            "AllowDebugPrecompiles": false,
            "DataAvailabilityCommittee": false,
            "InitialArbOSVersion": 32,
            "InitialChainOwner": "{chain_owner}",
            "GenesisBlockNum": 0
            }}
        }}"#
    );
    cfg.retain(|c| !c.is_whitespace());
    cfg.shrink_to_fit();
    cfg
}

/// Starts nitro instance
pub async fn launch_nitro_node(
    chain_id: u64,
    chain_owner: Address,
    mchain_url: &str,
    sequencer_port: Option<u16>,
) -> Result<(Docker, RootProvider, String)> {
    let tag = env::var("NITRO_TAG").unwrap_or("v3.6.2-5b41a2d-slim".to_string());
    let port = PortManager::instance().next_port().await;

    let log_level = env::var("NITRO_LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());

    let mut nitro = Docker::new(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("--net=host")
            .arg(format!("offchainlabs/nitro-node:{tag}"))
            .arg(format!("--parent-chain.connection.url={mchain_url}"))
            .arg("--node.dangerous.disable-blob-reader")
            .arg("--node.inbox-reader.check-delay=100ms")
            .arg("--node.parent-chain-reader.poll-interval=100ms")
            .arg("--node.staker.enable=false")
            .arg("--ensure-rollup-deployment=false")
            .arg(format!(
                "--chain.info-json={}",
                appchain_info(&appchain_config(chain_id, chain_owner), "test")
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

    let appchain = ProviderBuilder::default().connect(&url).await?;
    wait_until!(
        if let Some(status) = nitro.try_wait()? {
            panic!("nitro node exited with {}", status);
        };
        appchain.get_chain_id().await.is_ok(),
        Duration::from_secs(5*60)  // give it time to download the image if necessary
    );
    Ok((nitro, appchain, url))
}

pub async fn start_valkey() -> Result<(Docker, String)> {
    let port = PortManager::instance().next_port().await;
    let mut valkey = Docker::new(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("-p")
            .arg(format!("{port}:6379"))
            .arg("valkey/valkey:latest")
            .arg("--loglevel debug"),
    )?;

    // Not a typo - `valkey` URL should have the `redis` prefix
    let valkey_url = format!("redis://0.0.0.0:{port}/");

    let valkey_client = redis::Client::open(valkey_url.as_str()).unwrap();
    wait_until!(
        if let Some(status) = valkey.try_wait()? {
            panic!("cache exited with {}", status);
        };
        valkey_client.get_multiplexed_async_connection().await.is_ok(),
        Duration::from_secs(5 * 60) // give it time to download the image if necessary
    );
    Ok((valkey, valkey_url))
}
