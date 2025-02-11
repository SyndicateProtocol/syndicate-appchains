//! Integration tests for the metabased stack
#![allow(missing_docs)]

use crate::port_manager::PortManager;
use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{address, Address, U256},
    providers::{ext::AnvilApi as _, Provider, ProviderBuilder, RootProvider, WalletProvider},
    transports::http::Http,
};
use block_builder::{
    block_builder::BlockBuilder,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::anvil::{FilledProvider, MetaChainProvider},
    metrics::BlockBuilderMetrics,
};
use common::{db::DummyStore, tracing::init_test_tracing, types::Chain};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule,
        metabasedsequencerchain::MetabasedSequencerChain::{self, MetabasedSequencerChainInstance},
    },
};
use eyre::{eyre, Result};
use ingestor::{
    config::ChainIngestorConfig,
    eth_client::{EthClient, RPCClient},
    ingestor::Ingestor,
    metrics::IngestorMetrics,
};
use metabased_translator::config::MetabasedConfig;
use metrics::metrics::MetricsState;
use prometheus_client::registry::Registry;
use reqwest::Client;
use slotter::{metrics::SlotterMetrics, Slotter};
use std::{sync::Arc, time::Duration};
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::timeout,
};
use tracing::Level;

pub const GENESIS_TIMESTAMP: u64 = 1736824187;
pub const PRELOAD_INBOX_ADDRESS: Address = address!("0xD82DEBC6B9DEebee526B4cb818b3ff2EAa136899");
pub const PRELOAD_BRIDGE_ADDRESS: Address = address!("0x199Beb469aEf45CBC2B5Fb1BE58690C9D12f45E2");

#[derive(Debug)]
struct Task(task::JoinHandle<()>);

impl Drop for Task {
    fn drop(&mut self) {
        self.0.abort();
    }
}

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

async fn start_anvil(port: u16, chain_id: u64) -> Result<(AnvilInstance, FilledProvider)> {
    let timestamp = GENESIS_TIMESTAMP.to_string();
    let args = vec![
        "--base-fee",
        "0",
        "--gas-limit",
        "30000000",
        "--timestamp",
        &timestamp,
        "--no-mining",
    ];

    let anvil = Anvil::new().port(port).chain_id(chain_id).args(args).try_spawn()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    provider.anvil_set_block_timestamp_interval(0).await?;
    Ok((anvil, provider))
}

async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    provider.anvil_set_block_timestamp_interval(delay).await?;
    provider.evm_mine(None).await?;
    Ok(())
}

async fn load_anvil(port: u16) -> Result<(AnvilInstance, FilledProvider)> {
    let state_file =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config").join("anvil.json");
    let timestamp = GENESIS_TIMESTAMP.to_string();

    #[allow(clippy::unwrap_used)]
    let args = vec![
        "--base-fee",
        "0",
        "--gas-limit",
        "30000000",
        "--no-mining",
        "--load-state",
        state_file.to_str().unwrap(),
        "--timestamp",
        &timestamp,
    ];

    let anvil = Anvil::new().port(port).chain_id(31337).args(args).try_spawn()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    provider.anvil_set_block_timestamp_interval(0).await?;
    Ok((anvil, provider))
}

pub async fn launch_nitro_node(
    mchain: &MetaChainProvider,
    port: u16,
) -> Result<(Docker, RootProvider<Http<Client>>)> {
    let nitro = Command::new("docker")
        .kill_on_drop(false) // kill via SIGTERM instead of SIGKILL
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg("offchainlabs/nitro-node:v3.4.0-d896e9c-slim")
        .arg("--parent-chain.connection.url=".to_string() + mchain.anvil.endpoint_url().as_str())
        .arg("--node.dangerous.disable-blob-reader")
        .arg("--execution.forwarding-target=null")
        .arg("--execution.parent-chain-reader.old-header-timeout=1000h")
        .arg("--node.inbox-reader.check-delay=10ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg("--chain.info-json=".to_string() + &mchain.rollup_info("test"))
        .arg("--http.addr=0.0.0.0")
        .arg("--http.port=".to_string() + &port.to_string())
        .arg("--log-level=info")
        .spawn()?;
    let rollup = ProviderBuilder::new()
        .on_http(("http://localhost:".to_string() + &port.to_string()).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((Docker(nitro), rollup))
    })
    .await?
}

#[derive(Debug)]
pub struct MetaNode {
    pub sequencing_contract: MetabasedSequencerChainInstance<Http<Client>, FilledProvider>,
    pub sequencing_provider: FilledProvider,
    pub settlement_provider: FilledProvider,
    pub metabased_rollup: RootProvider<Http<Client>>,

    pub chain_id: u64,
    pub slot_duration: u64,

    pub mchain_provider: FilledProvider,

    sequencer_ingestor_task: Task,
    settlement_ingestor_task: Task,
    block_builder_task: Task,
    slotter_task: Task,

    // References to keep the processes/tasks alive
    #[allow(dead_code)]
    seq_anvil: AnvilInstance,
    #[allow(dead_code)]
    set_anvil: AnvilInstance,
    #[allow(dead_code)]
    nitro_docker: Docker,
    #[allow(dead_code)]
    seq_ingestor_tx: tokio::sync::oneshot::Sender<()>,
    #[allow(dead_code)]
    set_ingestor_tx: tokio::sync::oneshot::Sender<()>,
    #[allow(dead_code)]
    builder_tx: tokio::sync::oneshot::Sender<()>,
    #[allow(dead_code)]
    slotter_tx: tokio::sync::oneshot::Sender<()>,
}

impl MetaNode {
    pub async fn new(pre_loaded: bool, config: MetabasedConfig) -> Result<Self> {
        // We need 4 ports to run a full meta node
        // - MChain
        // - Mock Sequencing Chain
        // - Mock Settlement Chain
        // - Nitro Rollup;
        let port_tracker = PortManager::instance();

        // Define the addresses of the bridge and inbox contracts depedning on whether we
        // are loading in the full set of Arb contracts or not
        let bridge_address =
            if pre_loaded { PRELOAD_BRIDGE_ADDRESS } else { get_rollup_contract_address() };
        let inbox_address =
            if pre_loaded { PRELOAD_INBOX_ADDRESS } else { get_rollup_contract_address() };

        let _ = init_test_tracing(Level::INFO);
        let mchain_port = port_tracker.next_port();
        let block_builder_cfg = BlockBuilderConfig {
            bridge_address,
            inbox_address,
            mchain_url: format!("http://127.0.0.1:{}", mchain_port).parse()?,
            sequencing_contract_address: get_rollup_contract_address(),
            genesis_timestamp: GENESIS_TIMESTAMP,
            ..config.block_builder
        };

        // Launch mock sequencing chain and deploy contracts
        let seq_port = port_tracker.next_port();
        let (seq_anvil, seq_provider) = start_anvil(seq_port, 15).await?;
        _ = MetabasedSequencerChain::deploy_builder(
            &seq_provider,
            U256::from(block_builder_cfg.target_chain_id),
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
        let set_port = port_tracker.next_port();
        let (set_anvil, set_provider);
        if pre_loaded {
            // If flag is set, load the anvil state from a file
            // This is the full set of Arb contracts
            (set_anvil, set_provider) = load_anvil(set_port).await?;
        } else {
            // If not use our mock Rollup contract for easier testing
            (set_anvil, set_provider) = start_anvil(set_port, 20).await?;
            // Use the mock rollup contract for the test instead of deploying all the nitro rollup
            // contracts
            _ = Rollup::deploy_builder(
                &set_provider,
                U256::from(block_builder_cfg.target_chain_id),
                MetaChainProvider::rollup_config(block_builder_cfg.target_chain_id),
            )
            .nonce(0)
            .send()
            .await?;
            mine_block(&set_provider, 0).await?;
        }

        // Launch ingestors for the sequencer and settlement chains

        let mut seq_config: ChainIngestorConfig = (&config.sequencing).into();
        let mut set_config: ChainIngestorConfig = (&config.settlement).into();
        seq_config.rpc_url = format!("http://localhost:{}", seq_port);
        seq_config.polling_interval = Duration::from_millis(10);
        set_config.rpc_url = format!("http://localhost:{}", set_port);
        set_config.polling_interval = Duration::from_millis(10);
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let sequencing_client: Arc<dyn RPCClient> =
            Arc::new(EthClient::new(&seq_config.rpc_url).await?);
        let settlement_client: Arc<dyn RPCClient> =
            Arc::new(EthClient::new(&set_config.rpc_url).await?);
        let (sequencing_ingestor, sequencer_rx) = Ingestor::new(
            Chain::Sequencing,
            sequencing_client,
            &seq_config,
            IngestorMetrics::new(&mut metrics_state.registry),
        )
        .await?;
        let (settlement_ingestor, settlement_rx) = Ingestor::new(
            Chain::Settlement,
            settlement_client,
            &set_config,
            IngestorMetrics::new(&mut metrics_state.registry),
        )
        .await?;
        let (seq_ingestor_tx, seq_ingestor_rx) = tokio::sync::oneshot::channel();
        let sequencer_ingestor_task = Task(tokio::spawn(async move {
            let _ = sequencing_ingestor.start_polling(seq_ingestor_rx).await;
        }));
        let (set_ingestor_tx, set_ingestor_rx) = tokio::sync::oneshot::channel();
        let settlement_ingestor_task = Task(tokio::spawn(async move {
            let _ = settlement_ingestor.start_polling(set_ingestor_rx).await;
        }));

        // Start slotter at the genesis timestamp
        let mut slotter_cfg = config.slotter;
        slotter_cfg.start_slot_timestamp = GENESIS_TIMESTAMP;

        // Launch the slotter, block builder, and nitro rollup
        let (slotter, slotter_rx) = Slotter::new(
            &slotter_cfg,
            None,
            Box::new(DummyStore {}),
            SlotterMetrics::new(&mut metrics_state.registry),
        );
        let (shutdown_slotter_tx, shutdown_slotter_rx) = tokio::sync::oneshot::channel();
        let slotter_task = Task(tokio::spawn(async move {
            slotter.start(sequencer_rx, settlement_rx, shutdown_slotter_rx).await;
        }));

        let block_builder = BlockBuilder::new(
            slotter_rx,
            &block_builder_cfg,
            BlockBuilderMetrics::new(&mut metrics_state.registry),
        )
        .await?;
        let mchain_provider = block_builder.mchain.provider.clone();

        let nitro_port = port_tracker.next_port();
        let (nitro_docker, metabased_rollup) =
            launch_nitro_node(&block_builder.mchain, nitro_port).await?;
        let (builder_tx, builder_rx) = tokio::sync::oneshot::channel();
        let block_builder_task = Task(tokio::spawn(async move {
            block_builder.start(None, builder_rx).await;
        }));

        Ok(Self {
            sequencing_contract,
            sequencing_provider: seq_provider,
            settlement_provider: set_provider,
            metabased_rollup,

            chain_id: block_builder_cfg.target_chain_id,
            slot_duration: slotter_cfg.slot_duration,

            mchain_provider,

            sequencer_ingestor_task,
            settlement_ingestor_task,
            block_builder_task,
            slotter_task,

            seq_anvil,
            set_anvil,

            nitro_docker,
            seq_ingestor_tx,
            set_ingestor_tx,
            builder_tx,
            slotter_tx: shutdown_slotter_tx,
        })
    }

    pub async fn mine_seq_blocks(&self, delay: u64) -> Result<()> {
        mine_block(&self.sequencing_provider, delay).await?;
        Ok(())
    }

    pub async fn mine_set_blocks(&self, delay: u64) -> Result<()> {
        mine_block(&self.settlement_provider, delay).await?;
        Ok(())
    }

    pub async fn mine_both(&self, delay: u64) -> Result<()> {
        self.mine_seq_blocks(delay).await?;
        self.mine_set_blocks(delay).await?;
        Ok(())
    }

    pub async fn mine_next_slot(&self) -> Result<()> {
        self.mine_seq_blocks(self.slot_duration).await?;
        self.mine_set_blocks(self.slot_duration).await?;

        Ok(())
    }
}

impl Drop for MetaNode {
    fn drop(&mut self) {
        self.sequencer_ingestor_task.0.abort();
        self.settlement_ingestor_task.0.abort();
        self.block_builder_task.0.abort();
        self.slotter_task.0.abort();
    }
}
