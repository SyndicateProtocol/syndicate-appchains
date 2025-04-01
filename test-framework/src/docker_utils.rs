//! Docker components for the integration tests

use crate::rollup_utils::{chain_config, rollup_config, rollup_info};
use alloy::{
    primitives::Address,
    providers::{IpcConnect, Provider, ProviderBuilder, RootProvider},
};
use eyre::Result;
use std::{env, time::Duration};
use test_utils::port_manager::PortManager;
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

#[derive(Debug)]
pub struct NodeInfo {
    pub ipc: String,
    pub auth_ipc: String,
    pub http_port: u16,
}

pub async fn start_reth(
    chain_id: u64,
    tag: &str,
) -> Result<(NodeInfo, (Docker, Option<(Docker, Docker, Docker, Docker)>))> {
    let manager = PortManager::instance();
    let port = manager.next_port();
    let auth_port = manager.next_port();
    let http_port = manager.next_port();
    let dir = env!("CARGO_MANIFEST_DIR");
    let ipc = format!("{dir}/{port}.ipc");
    let auth_ipc = format!("{dir}/{auth_port}.ipc");
    let chain_cfg = chain_config(chain_id);
    let reth = Docker(
        Command::new("docker")
            .arg("run")
            .arg("--init")
            .arg("--rm")
            .arg("--entrypoint")
            .arg("/bin/sh")
            .arg("-v")
            .arg(if cfg!(target_os = "macos") { "ipc" } else { dir }.to_string() + ":/ipc")
            .arg("-p")
            .arg(format!("{http_port}:{http_port}"))
            .arg(format!("ghcr.io/syndicateprotocol/reth:{tag}"))
            .arg("-c")
            .arg(format!("umask 0 && exec reth node --http --http.addr=0.0.0.0 --http.port={http_port} --ipcpath=/ipc/{port}.ipc --auth-ipc --auth-ipc.path=/ipc/{auth_port}.ipc --chain='{chain_cfg}'"))
            .spawn()?
    );
    #[cfg(not(target_os = "macos"))]
    let socat = None;

    // on mac, use socat to route traffic over tcp as unix domain sockets cannot cross the os
    // boundary. host port.ipc -> host socat -> container socat -> container port.ipc
    // on linux, the port.ipc socket file can be shared between the docker container and host os
    // directly.
    #[cfg(target_os = "macos")]
    let socat = Some((
        Docker(
            Command::new("socat")
                .arg(format!("UNIX-LISTEN:{ipc},reuseaddr,fork"))
                .arg(format!("TCP4:127.0.0.1:{port}"))
                .spawn()?,
        ),
        Docker(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("-p")
                .arg(format!("{port}:{port}"))
                .arg("-v")
                .arg("ipc:/ipc")
                .arg("alpine/socat:1.8.0.1")
                .arg(format!("TCP4-LISTEN:{port},reuseaddr,fork,bind=0.0.0.0"))
                .arg(format!("UNIX-CONNECT:/ipc/{port}.ipc,retry=1200,interval=0.1"))
                .spawn()?,
        ),
        Docker(
            Command::new("socat")
                .arg(format!("UNIX-LISTEN:{auth_ipc},reuseaddr,fork"))
                .arg(format!("TCP4:127.0.0.1:{auth_port}"))
                .spawn()?,
        ),
        Docker(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("-p")
                .arg(format!("{auth_port}:{auth_port}"))
                .arg("-v")
                .arg("ipc:/ipc")
                .arg("alpine/socat:1.8.0.1")
                .arg(format!("TCP4-LISTEN:{auth_port},reuseaddr,fork,bind=0.0.0.0"))
                .arg(format!("UNIX-CONNECT:/ipc/{auth_port}.ipc,retry=1200,interval=0.1"))
                .spawn()?,
        ),
    ));
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        let mut rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(ipc.clone())).await;
        while rollup.is_err() {
            sleep(Duration::from_millis(100)).await;
            rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(ipc.clone())).await;
        }
        let mut auth_rollup =
            ProviderBuilder::new().on_ipc(IpcConnect::new(auth_ipc.clone())).await;
        while auth_rollup.is_err() {
            sleep(Duration::from_millis(100)).await;
            auth_rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(auth_ipc.clone())).await;
        }
        #[allow(clippy::unwrap_used)]
        let r = rollup.unwrap();
        while r.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }
        #[allow(clippy::unwrap_used)]
        let r = auth_rollup.unwrap();
        while r.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(100)).await;
        }
        Ok::<_, eyre::Error>((NodeInfo { ipc, auth_ipc, http_port }, (reth, socat)))
    })
    .await?
}

pub async fn launch_nitro_node(
    chain_id: u64,
    chain_owner: Address,
    mchain_port: u16,
    sequencer_port: u16,
    tag: &str,
) -> Result<(Docker, RootProvider, String)> {
    let port = PortManager::instance().next_port();
    let nitro = Command::new("docker")
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg(format!("offchainlabs/nitro-node:{tag}"))
        .arg(format!("--parent-chain.connection.url=http://localhost:{mchain_port}"))
        .arg("--node.dangerous.disable-blob-reader")
        .arg(format!("--execution.forwarding-target=http://localhost:{sequencer_port}"))
        .arg("--execution.parent-chain-reader.old-header-timeout=1000h")
        .arg("--node.inbox-reader.check-delay=10ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg(format!(
            "--chain.info-json={}",
            rollup_info(&rollup_config(chain_id, chain_owner), "test")
        ))
        .arg("--http.addr=0.0.0.0")
        .arg(format!("--http.port={}", port))
        .arg("--log-level=info")
        .spawn()?;
    let rollup = ProviderBuilder::default().on_http(format!("http://localhost:{}", port).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((Docker(nitro), rollup, format!("http://localhost:{}", port)))
    })
    .await?
}
