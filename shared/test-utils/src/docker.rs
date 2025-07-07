//! Docker components for the integration tests

use crate::{
    chain_info::{test_account1, ChainInfo, ProcessInstance},
    nitro_chain::{nitro_chain_info_json, NitroChainInfoArgs, NitroDeployment},
    port_manager::PortManager,
    utils::test_path,
    wait_until,
};
use alloy::{
    hex,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::Client,
};
use eyre::Result;
use redis::aio::ConnectionManager;
use std::{
    env,
    future::Future,
    process::{ExitStatus, Stdio},
    time::Duration,
};
use synd_mchain::client::MProvider;
use synd_tee_attestation_zk_proofs_submitter::{self, get_signer_public_key};
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    process::{Child, Command},
};
use tracing::{info, warn};

/// Wraps a `tokio::process::Command`, prefixes and redirects the output to the test logger, and
/// handles cleanup.
#[derive(Debug)]
pub struct E2EProcess(Child);

impl E2EProcess {
    pub fn new(cmd: &mut Command, command_name: &str) -> Result<Self> {
        let mut child =
            cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        let stdout = child.stdout.take().unwrap();
        // force tests to capture output from stdout
        let command_name_clone = command_name.to_string();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Some(line) = reader.next_line().await.unwrap() {
                println!("{command_name_clone}: {line}");
            }
        });
        // force tests to capture output from stderr
        let stderr = child.stderr.take().unwrap();
        let command_name_clone = command_name.to_string();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line) = reader.next_line().await.unwrap() {
                eprintln!("{command_name_clone}: {line}");
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

impl Drop for E2EProcess {
    fn drop(&mut self) {
        if self.0.try_wait().is_ok_and(|status| status.is_some()) {
            info!("process for Drop already exited or was not running.");
            return;
        }

        let Some(pid) = self.0.id() else {
            info!("process for Drop had no PID, likely already exited or failed to start.");
            return;
        };

        info!("Attempting to stop process (PID: {}) via Drop", pid);
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
) -> Result<E2EProcess> {
    info!("launching {}", executable_name);
    let needs_rocksdb = executable_name == "synd-mchain";
    let mut docker = if let Ok(tag) =
        env::var(executable_name.to_uppercase().replace("-", "_") + "_TAG")
    {
        E2EProcess::new(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("--net=host")
                .arg(format!("ghcr.io/syndicateprotocol/{executable_name}:{tag}"))
                .args(args),
            executable_name,
        )
    } else if executable_name == "synd-proposer" {
        // Synd-proposer is a Go service, so we need to use the Go command to run it
        let mut cmd = Command::new("go");
        cmd.current_dir(format!("{}/synd-withdrawals/synd-proposer", env!("CARGO_WORKSPACE_DIR")));
        cmd.arg("run").arg("./cmd/synd-proposer");
        cmd.args(args);
        E2EProcess::new(&mut cmd, executable_name)
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
        E2EProcess::new(&mut cmd, executable_name)
    }?;

    health_check(executable_name, api_port, &mut docker).await;
    Ok(docker)
}

pub async fn health_check(executable_name: &str, api_port: u16, docker: &mut E2EProcess) {
    let client = Client::new();
    wait_until!(
        if let Some(status) = docker.try_wait()? {
            panic!("{executable_name} exited with {status}");
        };
        client
            .get(format!("http://localhost:{api_port}/health"))
            .send()
            .await
            .is_ok_and(|x| x.status().is_success()),
        Duration::from_secs(10*60)  // give it time to download the image if necessary
    );
}

pub async fn start_mchain(
    appchain_chain_id: u64,
    finality_delay: u64,
) -> Result<(String, E2EProcess, MProvider)> {
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

pub enum NitroSequencerMode {
    // No sequencer mode - used for the appchain that will simply derive the state
    None,
    // Forwarding mode - used for using an external sequencer, used to test the synd-sequencer
    Forwarding(u16),
    // Sequencer mode - used for base chains when E2E's Nitro mode is enabled, these chains will
    // post batch data to L1
    Sequencer,
    // TODO SEQ-1032: add EigenDA mode
}

pub struct NitroNodeArgs {
    pub chain_id: u64,
    pub chain_name: String,
    pub chain_owner: Address,
    pub parent_chain_url: String,
    pub parent_chain_id: u64,
    pub sequencer_mode: NitroSequencerMode,
    pub deployment: NitroDeployment,
    pub sequencer_private_key: Option<String>,
}

/// Starts nitro instance
pub async fn launch_nitro_node(args: NitroNodeArgs) -> Result<ChainInfo> {
    let tag = env::var("NITRO_TAG").unwrap_or("v3.6.2-5b41a2d-slim".to_string());
    let port = PortManager::instance().next_port().await;

    let log_level = env::var("NITRO_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    let sequencer_args = match args.sequencer_mode {
        NitroSequencerMode::None => vec!["--execution.forwarding-target=null".to_string()],
        NitroSequencerMode::Forwarding(port) => {
            vec![format!("--execution.forwarding-target=http://localhost:{port}")]
        }
        NitroSequencerMode::Sequencer => {
            vec![
                "--node.sequencer=true".to_string(),
                "--node.batch-poster.enable=true".to_string(),
                "--execution.sequencer.enable=true".to_string(),
                "--node.delayed-sequencer.enable=true".to_string(),
                "--node.delayed-sequencer.require-full-finality=false".to_string(),
                "--node.delayed-sequencer.use-merge-finality=false".to_string(),
                "--node.delayed-sequencer.finalize-distance=0".to_string(),
                "--node.dangerous.no-sequencer-coordinator=true".to_string(),
                args.sequencer_private_key
                    .map(|key| {
                        format!(
                            "--node.batch-poster.parent-chain-wallet.private-key={}",
                            key.strip_prefix("0x").unwrap()
                        )
                    })
                    .unwrap_or_default(),
                "--node.batch-poster.data-poster.wait-for-l1-finality=false".to_string(),
                "--node.parent-chain-reader.use-finality-data=false".to_string(),
                "--execution.parent-chain-reader.use-finality-data=false".to_string(),
                "--node.batch-poster.reorg-resistance-margin=1ms".to_string(),
                "--execution.parent-chain-reader.use-finality-data=false".to_string(),
                "--execution.sync-monitor.safe-block-wait-for-block-validator=false".to_string(),
                "--node.batch-poster.l1-block-bound=ignore".to_string(),
                "--execution.sync-monitor.finalized-block-wait-for-block-validator=false"
                    .to_string(),
                "--node.batch-poster.max-delay=1s".to_string(),
                "--node.batch-poster.wait-for-max-delay=false".to_string(),
            ]
        }
    };

    let mut nitro = E2EProcess::new(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("--net=host")
            .arg(format!("offchainlabs/nitro-node:{tag}"))
            .arg(format!("--parent-chain.connection.url={}", args.parent_chain_url))
            .arg("--node.dangerous.disable-blob-reader")
            .arg("--node.inbox-reader.check-delay=100ms")
            .arg("--node.parent-chain-reader.poll-interval=100ms")
            .arg("--node.staker.enable=false")
            .arg("--execution.tx-pre-checker.strictness=20")
            .arg("--ensure-rollup-deployment=false")
            .arg(format!(
                "--chain.info-json={}",
                nitro_chain_info_json(NitroChainInfoArgs {
                    chain_id: args.chain_id,
                    parent_chain_id: args.parent_chain_id,
                    chain_owner: args.chain_owner,
                    chain_name: args.chain_name.clone(),
                    deployment: args.deployment,
                })
            ))
            .arg("--http.addr=0.0.0.0")
            .arg("--http.api=net,web3,eth,debug,trace")
            .arg(format!("--http.port={port}"))
            .arg("--ws.expose-all")
            .arg(format!("--ws.port={port}"))
            .arg("--ws.addr=0.0.0.0")
            .arg("--ws.origins=\\*")
            .arg(format!("--log-level={log_level}"))
            .args(sequencer_args),
        format!("nitro-{}", args.chain_name).as_str(),
    )?;

    let http_url = format!("http://localhost:{port}");
    let ws_url = format!("ws://localhost:{port}");

    let provider =
        ProviderBuilder::new().wallet(test_account1().signer.clone()).connect(&http_url).await?;
    wait_until!(
        if let Some(status) = nitro.try_wait()? {
            panic!("nitro node exited with {status}");
        };
        provider.get_chain_id().await.is_ok(),
        Duration::from_secs(5*60)  // give it time to download the image if necessary
    );
    Ok(ChainInfo { instance: ProcessInstance::Docker(nitro), provider, ws_url, http_url })
}

pub async fn start_valkey() -> Result<(E2EProcess, String)> {
    let port = PortManager::instance().next_port().await;
    let mut valkey = E2EProcess::new(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("-p")
            .arg(format!("{port}:6379"))
            .arg("valkey/valkey:latest")
            .arg("--loglevel debug"),
        "valkey",
    )?;

    // Not a typo - `valkey` URL should have the `redis` prefix
    let valkey_url = format!("redis://0.0.0.0:{port}/");

    let valkey_client = redis::Client::open(valkey_url.as_str()).unwrap();
    wait_until!(
        if let Some(status) = valkey.try_wait()? {
            panic!("cache exited with {status}");
        };
        ConnectionManager::new(valkey_client.clone()).await.is_ok(),
        Duration::from_secs(5 * 60) // give it time to download the image if necessary
    );
    Ok((valkey, valkey_url))
}

pub async fn launch_enclave_server() -> Result<(E2EProcess, String, Address)> {
    info!("launching enclave server");

    let project_root = env!("CARGO_WORKSPACE_DIR");
    let enclave_path = format!("{project_root}/synd-withdrawals/synd-enclave");
    let image_name = "ghcr.io/syndicateprotocol/enclave-server:local-dev".to_string();

    info!("building enclave server docker image - NOTE: this may take a while");
    let status = E2EProcess::new(
        Command::new("docker")
            .arg("buildx")
            .arg("build")
            .arg("--load")
            .arg("--progress=plain")
            .arg(&enclave_path)
            .arg("--tag")
            .arg(&image_name)
            .arg("--target")
            .arg("local-dev"),
        "building-enclave-server",
    )?
    .wait()
    .await?;

    if !status.success() {
        return Err(eyre::eyre!(
            "failed to build enclave-server docker image. Exit status: {}",
            status
        ));
    }
    info!("enclave server docker image built successfully");

    let port = PortManager::instance().next_port().await;
    let docker = E2EProcess::new(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("-p")
            .arg(format!("{port}:1234"))
            .arg(image_name),
        "enclave-server",
    )?;

    let enclave_rpc_url = format!("http://localhost:{port}");

    wait_until!(
        get_signer_public_key(enclave_rpc_url.clone()).await.is_ok(),
        Duration::from_secs(5 * 60)
    );
    // NOTE: in theory we we should get the attestation doc instead, but it's hard to get that
    // function to work outside the AWS enclave. We'll just use the public key for now.
    let signer_pub_key_hex = get_signer_public_key(enclave_rpc_url.clone()).await?;
    let signer_pub_key_bytes = hex::decode(signer_pub_key_hex).unwrap();
    let signer_address = Address::from_raw_public_key(&signer_pub_key_bytes[..64]);

    Ok((docker, enclave_rpc_url, signer_address))
}
