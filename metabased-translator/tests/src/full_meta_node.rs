//! Integration tests for the metabased stack
#![allow(missing_docs)]

use crate::port_manager::PortManager;
use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{address, Address, U256},
    providers::{
        ext::AnvilApi as _, IpcConnect, Provider, ProviderBuilder, RootProvider, WalletProvider,
    },
    rpc::types::anvil::MineOptions,
};
use block_builder::{
    config::{get_default_private_key_signer, get_rollup_contract_address},
    connectors::mchain::{rollup_config, FilledProvider, MetaChainProvider, MCHAIN_ID},
    rollups::arbitrum::arbitrum_adapter::ArbitrumAdapter,
};
use common::{
    eth_client::{EthClient, RPCClient},
    types::Chain,
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule,
        metabasedsequencerchain::MetabasedSequencerChain::{self, MetabasedSequencerChainInstance},
    },
};
use eyre::{eyre, Result};
use metabased_translator::{
    config::MetabasedConfig,
    shutdown_channels::{ShutdownChannels, ShutdownTx},
    spawn::ComponentHandles,
};
use metrics::metrics::{MetricsState, TranslatorMetrics};
use prometheus_client::registry::Registry;
use std::{sync::Arc, time::Duration};
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::timeout,
};

pub const PRELOAD_INBOX_ADDRESS: Address = address!("0x26eE2349212255614edCc046DD9472F2a5b7EF2b");
pub const PRELOAD_BRIDGE_ADDRESS: Address = address!("0xa0e810a42086da4Ebc5C49fEd626cA6A75B06437");
pub const PRELOAD_ROLLUP_ADDRESS: Address = address!("0x75744D0D556497B4ccb91D24328bF6160c2e0fE7");
pub const PRELOAD_OUTBOX_ADDRESS: Address = address!("0x3442A17C5AF1E664E10F6AC0e3bE2bDb9C87E948");
pub const PRELOAD_CHALLENGE_MANAGER: Address =
    address!("0xE801273F775Eacc1d74d1d43f92ec4524caBBD35");
pub const PRELOAD_ARB_SYS_PRECOMPILE_ADDRESS: Address =
    address!("0x0000000000000000000000000000000000000064");
pub const PRELOAD_NODE_INTERFACE_PRECOMPILE_ADDRESS: Address =
    address!("0x00000000000000000000000000000000000000c8");

pub const PRELOAD_WASM_MODULE_ROOT: &str =
    "0x184884e1eb9fefdc158f6c8ac912bb183bf3cf83f0090317e0bc4ac5860baa39";
#[derive(Debug)]
pub struct Docker(Child);

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

fn chain_config(chain_id: u64) -> String {
    r#"{"config": {
    "chainId": "#
        .to_string() +
        &chain_id.to_string() +
        r#",
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "constantinopleBlock": 0,
    "petersburgBlock": 0,
    "istanbulBlock": 0,
    "berlinBlock": 0,
    "londonBlock": 0,
    "terminalTotalDifficulty": 0,
    "shanghaiTime": 0,
    "cancunTime": 0
  },
  "nonce": "0x0",
  "timestamp": "0x0",
  "extraData": "0x",
  "gasLimit": "0x1c9c380",
  "difficulty": "0x0",
  "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "coinbase": "0x0000000000000000000000000000000000000000",
  "alloc": {
    "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
      "balance": "0xD3C21BCECCEDA1000000"
    }
  },
  "number": "0x0",
  "gasUsed": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}"#
}

#[derive(Debug)]
pub struct NodeInfo {
    pub ipc: String,
    pub auth_ipc: String,
    pub http_port: u16,
}

pub async fn start_reth(
    chain_id: u64,
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
            .arg("ghcr.io/syndicateprotocol/reth")
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
            tokio::time::sleep(Duration::from_millis(100)).await;
            rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(ipc.clone())).await;
        }
        let mut auth_rollup =
            ProviderBuilder::new().on_ipc(IpcConnect::new(auth_ipc.clone())).await;
        while auth_rollup.is_err() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            auth_rollup = ProviderBuilder::new().on_ipc(IpcConnect::new(auth_ipc.clone())).await;
        }
        #[allow(clippy::unwrap_used)]
        let r = rollup.unwrap();
        while r.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        #[allow(clippy::unwrap_used)]
        let r = auth_rollup.unwrap();
        while r.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok::<_, eyre::Error>((NodeInfo { ipc, auth_ipc, http_port }, (reth, socat)))
    })
    .await?
}

pub async fn start_anvil(chain_id: u64) -> Result<(u16, AnvilInstance, FilledProvider)> {
    start_anvil_with_args(chain_id, Default::default()).await
}

pub async fn start_anvil_with_args(
    chain_id: u64,
    args: &[&str],
) -> Result<(u16, AnvilInstance, FilledProvider)> {
    let port = PortManager::instance().next_port();
    let mut cmd =
        vec!["--base-fee", "0", "--gas-limit", "30000000", "--timestamp", "0", "--no-mining"];
    cmd.extend_from_slice(args);
    let anvil = Anvil::new().port(port).chain_id(chain_id).args(cmd).try_spawn()?;

    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    Ok((port, anvil, provider))
}

/// mine a block with a delay
async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    let block: common::types::Block = provider
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Latest, true))
        .await?;
    provider.evm_mine(Some(MineOptions::Timestamp(Some(block.timestamp + delay)))).await?;
    Ok(())
}

/// Get the nitro json configuration data for the rollup
fn rollup_info(rollup_config: &str, chain_name: &str) -> String {
    let rollup = get_rollup_contract_address();
    let deployed_at: u64 = 1;
    let zero = Address::ZERO;
    format!(
        r#"[{{
              "chain-name": "{chain_name}",
              "parent-chain-id": {MCHAIN_ID},
              "parent-chain-is-arbitrum": false,
              "sequencer-url": "",
              "secondary-forwarding-target": "",
              "feed-url": "",
              "secondary-feed-url": "",
              "das-index-url": "",
              "has-genesis-state": false,
              "chain-config": {rollup_config},
              "rollup": {{
                "bridge": "{rollup}",
                "inbox": "{zero}",
                "sequencer-inbox": "{rollup}",
                "deployed-at": {deployed_at},
                "rollup": "{zero}",
                "native-token": "{zero}",
                "upgrade-executor": "{zero}",
                "validator-wallet-creator": "{zero}"
              }}
            }}]"#
    )
}

pub async fn launch_nitro_node(
    chain_id: u64,
    chain_owner: Address,
    mchain_port: u16,
) -> Result<(Docker, RootProvider)> {
    let port = PortManager::instance().next_port();
    let nitro = Command::new("docker")
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg("offchainlabs/nitro-node:v3.4.0-d896e9c-slim")
        .arg(format!("--parent-chain.connection.url=http://localhost:{mchain_port}"))
        .arg("--node.dangerous.disable-blob-reader")
        .arg("--execution.forwarding-target=null")
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
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((Docker(nitro), rollup))
    })
    .await?
}

#[allow(missing_debug_implementations)]
pub struct MetaNode {
    pub sequencing_contract: MetabasedSequencerChainInstance<(), FilledProvider>,
    pub sequencing_provider: FilledProvider,
    pub sequencing_client: Arc<dyn RPCClient>,
    pub settlement_provider: FilledProvider,
    pub settlement_client: Arc<dyn RPCClient>,
    pub metabased_rollup: RootProvider,

    pub chain_id: u64,

    pub mchain: (Docker, Option<(Docker, Docker, Docker, Docker)>),
    pub mchain_provider: MetaChainProvider<ArbitrumAdapter>,

    _component_handles: ComponentHandles,

    // References to keep the processes/tasks alive
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    _nitro_docker: Docker,
    // keep the shutdown channel senders alive
    _shutdown_channels: ShutdownTx,
}

impl MetaNode {
    pub async fn new(pre_loaded: bool, mut config: MetabasedConfig) -> Result<Self> {
        // Define the addresses of the bridge and inbox contracts depedning on whether we
        // are loading in the full set of Arb contracts or not
        config.block_builder.bridge_address =
            if pre_loaded { PRELOAD_BRIDGE_ADDRESS } else { get_rollup_contract_address() };
        config.block_builder.inbox_address =
            if pre_loaded { PRELOAD_INBOX_ADDRESS } else { get_rollup_contract_address() };

        config.block_builder.sequencing_contract_address = get_rollup_contract_address();

        let (node, mchain) = start_reth(MCHAIN_ID).await?;
        config.block_builder.mchain_ipc_path = node.ipc;
        config.block_builder.mchain_auth_ipc_path = node.auth_ipc;

        // Launch mock sequencing chain and deploy contracts
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(
            &seq_provider,
            U256::from(config.block_builder.target_chain_id),
        )
        .send()
        .await?;
        let always_allowed_contract =
            AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
        mine_block(&seq_provider, 0).await?;
        let receipt = always_allowed_contract.get_receipt().await?;
        let always_allowed_module_address = match receipt.contract_address {
            Some(address) => address,
            None => {
                eprintln!("Deployment failed: No contract address found.");
                return Err(eyre!("Deployment failed: No contract address found."));
            }
        };

        // Setup the sequencing contract
        let provider_clone = seq_provider.clone();
        let sequencing_contract =
            MetabasedSequencerChain::new(get_rollup_contract_address(), provider_clone);
        _ = sequencing_contract
            .initialize(seq_provider.default_signer_address(), always_allowed_module_address)
            .send()
            .await?;
        mine_block(&seq_provider, 0).await?;

        // Launch mock settlement chain
        let (set_port, set_anvil, set_provider);
        if pre_loaded {
            // If flag is set, load the anvil state from a file
            // This is the full set of Arb contracts
            let state_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("config")
                .join("anvil.json");

            (set_port, set_anvil, set_provider) = start_anvil_with_args(
                31337,
                #[allow(clippy::unwrap_used)]
                &["--load-state", state_file.to_str().unwrap()],
            )
            .await?;
        } else {
            // If not use our mock Rollup contract for easier testing
            (set_port, set_anvil, set_provider) = start_anvil(20).await?;
            // Use the mock rollup contract for the test instead of deploying all the nitro rollup
            // contracts
            _ = Rollup::deploy_builder(
                &set_provider,
                U256::from(config.block_builder.target_chain_id),
                rollup_config(
                    config.block_builder.target_chain_id,
                    config.block_builder.owner_address,
                ),
            )
            .nonce(0)
            .send()
            .await?;
            mine_block(&set_provider, 0).await?;
        }

        // Update the RPC URLs in the config
        config.sequencing.sequencing_rpc_url = format!("http://localhost:{}", seq_port);
        config.settlement.settlement_rpc_url = format!("http://localhost:{}", set_port);
        config.sequencing.sequencing_polling_interval = Duration::from_millis(10);
        config.settlement.settlement_polling_interval = Duration::from_millis(10);

        // Create RPC clients
        let sequencing_client: Arc<dyn RPCClient> = Arc::new(
            EthClient::new(&config.sequencing.sequencing_rpc_url, Chain::Sequencing).await?,
        );
        let settlement_client: Arc<dyn RPCClient> = Arc::new(
            EthClient::new(&config.settlement.settlement_rpc_url, Chain::Settlement).await?,
        );

        // Initialize the MetaChainProvider
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = TranslatorMetrics::new(&mut metrics_state.registry);
        let mchain_provider = MetaChainProvider::start(
            &config.block_builder,
            metrics.block_builder.clone(),
            ArbitrumAdapter::new(&config.block_builder),
        )
        .await
        .map_err(|e| eyre!("Failed to initialize MetaChainProvider: {}", e))?;

        // Create shutdown channels
        let (_main_rx, shutdown_tx, shutdown_rx) = ShutdownChannels::new().split();

        // Launch components using ComponentHandles::spawn
        let component_handles = ComponentHandles::spawn(
            &config,
            None, // no known state for tests
            sequencing_client.clone(),
            settlement_client.clone(),
            metrics,
            mchain_provider.clone(),
            shutdown_rx,
        );

        // Launch the nitro rollup
        let (nitro_docker, metabased_rollup) = launch_nitro_node(
            config.block_builder.target_chain_id,
            config.block_builder.owner_address,
            node.http_port,
        )
        .await?;

        Ok(Self {
            sequencing_contract,
            sequencing_provider: seq_provider,
            sequencing_client,
            settlement_provider: set_provider,
            settlement_client,
            metabased_rollup,

            chain_id: config.block_builder.target_chain_id,

            mchain,
            mchain_provider,

            _component_handles: component_handles,

            _seq_anvil: seq_anvil,
            _set_anvil: set_anvil,
            _nitro_docker: nitro_docker,
            _shutdown_channels: shutdown_tx,
        })
    }

    pub async fn mine_seq_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.sequencing_provider, delay).await?;
        Ok(())
    }

    pub async fn mine_set_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.settlement_provider, delay).await?;
        Ok(())
    }

    pub async fn mine_both(&self, delay: u64) -> Result<()> {
        self.mine_seq_block(delay).await?;
        self.mine_set_block(delay).await?;
        Ok(())
    }
}
