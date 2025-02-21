//! Integration tests for the metabased stack
#![allow(missing_docs)]

use crate::port_manager::PortManager;
use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{address, Address, U256},
    providers::{ext::AnvilApi as _, Provider, ProviderBuilder, RootProvider, WalletProvider},
    rpc::types::anvil::MineOptions,
    transports::http::Http,
};
use block_builder::{
    block_builder::BlockBuilder,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::mchain::{FilledProvider, MetaChainProvider},
    metrics::BlockBuilderMetrics,
};
use common::{db::RocksDbStore, tracing::init_test_tracing, types::Chain};
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
use test_utils::test_path;
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::timeout,
};
use tracing::Level;

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

fn chain_config(chain_id: u64) -> String {
    r#"{"config": {
    "ethash": {},
    "chainId": "#
        .to_string() +
        &chain_id.to_string() +
        r#",
    "homesteadBlock": 0,
    "daoForkSupport": true,
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
  "stateRoot": "0x5eb6e371a698b8d68f665192350ffcecbbbf322916f4b51bd79bb6887da3f494",
  "alloc": {
    "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x70997970C51812dc3A010C7d01b50e0d17dc79C8": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x90F79bf6EB2c4f870365E785982E1f101E93b906": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x976EA74026E726554dB657fA54763abd0C3a0aa9": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x14dC79964da2C08b23698B3D3cc7Ca32193d9955": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xBcd4042DE499D14e55001CcbB24a551F3b954096": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x71bE63f3384f5fb98995898A86B02Fb2426c5788": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xFABB0ac9d68B0B445fB7357272Ff202C5651694a": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x1CBd3b2770909D4e10f157cABC84C7264073C9Ec": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xdF3e18d64BC6A983f673Ab319CCaE4f1a57C7097": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xcd3B766CCDd6AE721141F452C550Ca635964ce71": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x2546BcD3c84621e976D8185a91A922aE77ECEc30": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xbDA5747bFD65F08deb54cb465eB87D40e51B197E": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0xdD2FD4581271e230360230F9337D5c0430Bf44C0": {
      "balance": "0xD3C21BCECCEDA1000000"
    },
    "0x8626f6940E2eb28930eFb4CeF49B2d1F2C9C1199": {
      "balance": "0xD3C21BCECCEDA1000000"
    }
  },
  "number": "0x0",
  "gasUsed": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000"
}"#
}

pub async fn start_reth(port: u16, chain_id: u64) -> Result<Docker> {
    let reth = Command::new("docker")
        .kill_on_drop(false) // kill via SIGTERM instead of SIGKILL
        .arg("run")
        .arg("--init")
        .arg("--rm")
        .arg("--net=host")
        .arg("ghcr.io/syndicateprotocol/reth")
        .arg("node")
        .arg("--dev")
        .arg("--http")
        .arg("--http.port=".to_string() + &port.to_string())
        .arg("--http.api=eth,anvil")
        .arg("--txpool.minimal-protocol-fee=0")
        .arg("--chain=".to_string() + &chain_config(chain_id))
        .spawn()?;
    let rollup = ProviderBuilder::new()
        .on_http(("http://localhost:".to_string() + &port.to_string()).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>(Docker(reth))
    })
    .await?
}

pub async fn start_anvil(port: u16, chain_id: u64) -> Result<(AnvilInstance, FilledProvider)> {
    start_anvil_with_args(port, chain_id, Default::default()).await
}

pub async fn start_anvil_with_args(
    port: u16,
    chain_id: u64,
    args: &[&str],
) -> Result<(AnvilInstance, FilledProvider)> {
    let mut cmd =
        vec!["--base-fee", "0", "--gas-limit", "30000000", "--timestamp", "0", "--no-mining"];
    cmd.extend_from_slice(args);
    let anvil = Anvil::new().port(port).chain_id(chain_id).args(cmd).try_spawn()?;

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .on_http(anvil.endpoint_url());
    Ok((anvil, provider))
}

pub async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    let block: common::types::Block = provider
        .raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Latest, true))
        .await?;
    provider.evm_mine(Some(MineOptions::Timestamp(Some(block.timestamp + delay)))).await?;
    Ok(())
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
        .arg("--parent-chain.connection.url=".to_string() + mchain.mchain_url.as_str())
        .arg("--node.dangerous.disable-blob-reader")
        .arg("--execution.forwarding-target=null")
        .arg("--execution.parent-chain-reader.old-header-timeout=1000h")
        .arg("--node.inbox-reader.check-delay=10ms")
        .arg("--node.staker.enable=false")
        .arg("--ensure-rollup-deployment=false")
        .arg("--chain.info-json=".to_string() + &mchain.rollup_info("test"))
        .arg("--http.addr=0.0.0.0")
        .arg("--http.port=".to_string() + &port.to_string())
        .arg("--log-level=debug")
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

    pub mchain: Docker,
    pub mchain_provider: FilledProvider,
    pub rollup: Rollup::RollupInstance<Http<Client>, FilledProvider>,

    #[allow(dead_code)]
    sequencer_ingestor_task: Task,
    #[allow(dead_code)]
    settlement_ingestor_task: Task,
    #[allow(dead_code)]
    block_builder_task: Task,
    #[allow(dead_code)]
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
    #[allow(clippy::unwrap_used)] // test utility
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
            let state_file = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("config")
                .join("anvil.json");

            (set_anvil, set_provider) = start_anvil_with_args(
                set_port,
                31337,
                #[allow(clippy::unwrap_used)]
                &["--load-state", state_file.to_str().unwrap()],
            )
            .await?;
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
            Arc::new(EthClient::new(&seq_config.rpc_url, Chain::Sequencing).await?);
        let settlement_client: Arc<dyn RPCClient> =
            Arc::new(EthClient::new(&set_config.rpc_url, Chain::Settlement).await?);
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

        // new DB
        let db_path = test_path("db");
        let store = Arc::new(RocksDbStore::new(db_path.as_str()).unwrap());

        // Launch the slotter, block builder, and nitro rollup
        let (slotter, slotter_rx) = Slotter::new(
            &config.slotter,
            None,
            store.clone(),
            SlotterMetrics::new(&mut metrics_state.registry),
        );
        let (shutdown_slotter_tx, shutdown_slotter_rx) = tokio::sync::oneshot::channel();
        let slotter_task = Task(tokio::spawn(async move {
            slotter.start(sequencer_rx, settlement_rx, shutdown_slotter_rx).await;
        }));

        let mchain = start_reth(
            #[allow(clippy::unwrap_used)]
            block_builder_cfg.mchain_url.port().unwrap(),
            block_builder::connectors::mchain::MCHAIN_ID,
        )
        .await?;

        let block_builder = BlockBuilder::new(
            slotter_rx,
            &block_builder_cfg,
            config.slotter.slot_duration,
            store,
            BlockBuilderMetrics::new(&mut metrics_state.registry),
        )
        .await?;
        let mchain_provider = block_builder.mchain.provider.clone();
        let rollup = block_builder.mchain.rollup.clone();

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
            slot_duration: config.slotter.slot_duration,

            mchain,
            mchain_provider,
            rollup,

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
