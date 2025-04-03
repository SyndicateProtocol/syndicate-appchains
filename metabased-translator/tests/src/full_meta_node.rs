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
use metabased_translator::{config::MetabasedConfig, spawn::run};
use metrics::metrics::{MetricsState, TranslatorMetrics};
use prometheus_client::registry::Registry;
use shared::test_utils::test_path;
use std::{sync::Arc, time::Duration};
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    task,
    time::timeout,
};

const PRELOAD_INBOX_ADDRESS_300: Address = address!("0x26eE2349212255614edCc046DD9472F2a5b7EF2b");
const PRELOAD_BRIDGE_ADDRESS_300: Address = address!("0xa0e810a42086da4Ebc5C49fEd626cA6A75B06437");

const PRELOAD_INBOX_ADDRESS_271: Address = address!("0x7e2d5FCC5E02cBF2b9f860052C0226104E23F9c7");
const PRELOAD_BRIDGE_ADDRESS_271: Address = address!("0x8dAF17A20c9DBA35f005b6324F493785D239719d");

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

pub async fn start_mchain(chain_id: u64, chain_owner: Address) -> Result<(u16, Docker)> {
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
            ])
            .spawn()?,
    );
    let mchain: RootProvider =
        ProviderBuilder::default().on_http(format!("http://localhost:{}", port).parse()?);
    timeout(Duration::from_secs(120), async {
        while mchain.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((port, docker))
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
    let nitro = Docker(
        Command::new("docker")
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
            .arg("--log-level=debug")
            .spawn()?,
    );
    let rollup = ProviderBuilder::default().on_http(format!("http://localhost:{}", port).parse()?);
    // give it two minutes to launch (in case it needs to download the image)
    timeout(Duration::from_secs(120), async {
        while rollup.get_chain_id().await.is_err() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, eyre::Error>((nitro, rollup))
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
    pub bridge_address: Address,
    pub inbox_address: Address,

    pub mchain: Docker,
    pub mchain_provider: MetaChainProvider<ArbitrumAdapter>,

    // References to keep the processes/tasks alive
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    _nitro_docker: Docker,
}

/// nitro contract version on the settlement chain used for testing
#[derive(Debug)]
pub enum ContractVersion {
    V213,
    V300,
}

impl MetaNode {
    pub async fn new(
        pre_loaded: Option<ContractVersion>,
        rollup_owner_address: Address,
        mut config: MetabasedConfig,
    ) -> Result<Self> {
        let port_manager = PortManager::instance();
        config.metrics.metrics_port = port_manager.next_port();

        // Define the addresses of the bridge and inbox contracts depedning on whether we
        // are loading in the full set of Arb contracts or not
        config.block_builder.arbitrum_bridge_address =
            pre_loaded.as_ref().map_or_else(get_rollup_contract_address, |version| match version {
                ContractVersion::V300 => PRELOAD_BRIDGE_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_BRIDGE_ADDRESS_271,
            });
        config.block_builder.arbitrum_inbox_address =
            pre_loaded.as_ref().map_or_else(get_rollup_contract_address, |version| match version {
                ContractVersion::V300 => PRELOAD_INBOX_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_INBOX_ADDRESS_271,
            });

        config.block_builder.sequencing_contract_address = get_rollup_contract_address();

        const TARGET_CHAIN_ID: u64 = 13331370;
        let (mchain_port, mchain) = start_mchain(TARGET_CHAIN_ID, rollup_owner_address).await?;
        config.block_builder.mchain_rpc_url = format!("http://localhost:{mchain_port}");

        // Launch mock sequencing chain and deploy contracts
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(&seq_provider, U256::from(TARGET_CHAIN_ID))
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
        if let Some(version) = pre_loaded {
            let file = match version {
                ContractVersion::V300 => "anvil_300.json",
                ContractVersion::V213 => "anvil_213.json",
            };

            // If flag is set, load the anvil state from a file
            // This is the full set of Arb contracts
            let state_file =
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config").join(file);

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
                U256::from(TARGET_CHAIN_ID),
                rollup_config(TARGET_CHAIN_ID, rollup_owner_address),
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

        // Launch the translator
        let cfg_clone = config.clone();
        tokio::spawn(async move {
            let err = run(&cfg_clone, ArbitrumAdapter::new(&cfg_clone.block_builder)).await;
            if let Err(e) = err {
                panic!("Translator error: {}", e);
            }
        });

        // Launch the nitro rollup
        let (nitro_docker, metabased_rollup) =
            launch_nitro_node(TARGET_CHAIN_ID, rollup_owner_address, mchain_port).await?;

        Ok(Self {
            sequencing_contract,
            sequencing_provider: seq_provider,
            sequencing_client,
            settlement_provider: set_provider,
            settlement_client,
            metabased_rollup,

            chain_id: TARGET_CHAIN_ID,
            bridge_address: config.block_builder.arbitrum_bridge_address,
            inbox_address: config.block_builder.arbitrum_inbox_address,

            mchain,
            mchain_provider,

            _seq_anvil: seq_anvil,
            _set_anvil: set_anvil,
            _nitro_docker: nitro_docker,
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
