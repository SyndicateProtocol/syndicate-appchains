//! Components for the integration tests

use alloy::{
    eips::BlockNumberOrTag,
    node_bindings::AnvilInstance,
    primitives::{Address, U256},
    providers::{ext::AnvilApi as _, Provider as _, RootProvider, WalletProvider},
    rpc::types::anvil::MineOptions,
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule, metabasedsequencerchain::MetabasedSequencerChain,
    },
};
use eyre::{eyre, Result};
use mchain::mchain::{rollup_config, MProvider};
use reqwest::Client;
use std::{
    env,
    io::{stderr, Write},
    time::{Duration, SystemTime},
};
use test_utils::{
    anvil::{mine_block, start_anvil, start_anvil_with_args, FilledProvider},
    docker::{launch_nitro_node, start_mchain, Docker},
    logger::init_test_tracing,
    port_manager::PortManager,
    preloaded_config::{
        APPCHAIN_OWNER, DEFAULT_PRIVATE_KEY_SIGNER, PRELOAD_BRIDGE_ADDRESS_231,
        PRELOAD_BRIDGE_ADDRESS_300, PRELOAD_INBOX_ADDRESS_231, PRELOAD_INBOX_ADDRESS_300,
        PRELOAD_POSTER_ADDRESS_231, PRELOAD_POSTER_ADDRESS_300,
    },
    rollup::get_rollup_contract_address,
};
use tokio::{process::Command, time::sleep};
use tracing::{info, Level};

const APPCHAIN_CHAIN_ID: u64 = 13331370;

#[derive(Debug)]
pub struct Components {
    /// Timer for latency measurement
    /// Keep this on top - the top element gets destroyed first
    _timer: TestTimer,

    /// Sequencing
    pub sequencing_provider: FilledProvider,

    /// Settlement
    pub settlement_provider: FilledProvider,
    pub chain_id: u64,
    pub bridge_address: Address,
    pub inbox_address: Address,
    pub assertion_poster_contract_address: Address,

    /// Appchain
    pub appchain_provider: RootProvider,

    /// Mchain
    pub mchain_provider: MProvider,

    /// References to keep the processes/tasks alive
    _mchain: Docker,
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    _nitro_docker: Docker,
    _translator: Docker,
    _poster: Option<Docker>,
    _sequencer: Docker,
}

#[derive(Debug)]
pub struct TestTimer(SystemTime);

impl Drop for TestTimer {
    fn drop(&mut self) {
        let thread = std::thread::current();
        let elapsed = format!(
            "---- SYN ---- Test {:?} took: {:.2?}\n",
            thread.name().unwrap_or_default(),
            self.0.elapsed().unwrap_or_default()
        );
        // Write directly to stderr (bypasses test harness output capture)
        let _ = stderr().write_all(elapsed.as_bytes());
    }
}

/// nitro contract version on the settlement chain used for testing
#[derive(Debug, Clone)]
pub enum ContractVersion {
    V213,
    V300,
}

#[derive(Debug)]
struct TranslatorConfig {
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_contract_address: Address,
    mchain_rpc_url: String,
    sequencing_rpc_url: String,
    settlement_rpc_url: String,
    metrics_port: u16,
    sequencing_start_block: u64,
    settlement_start_block: u64,
    settlement_delay: u64,
}
#[derive(Debug)]
struct PosterConfig {
    assertion_poster_contract_address: Address,
    settlement_rpc_url: String,
    appchain_rpc_url: String,
    polling_interval: Duration,
    metrics_port: u16,
}
#[derive(Debug)]
struct SequencerConfig {
    sequencing_contract_address: Address,
    sequencing_rpc_url: String,
    sequencer_port: u16,
    metrics_port: u16,
}
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct ImageTags {
    poster_tag: Option<String>,
    translator_tag: Option<String>,
    sequencer_tag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationOptions {
    pub pre_loaded: Option<ContractVersion>,
    pub sequencing_start_block: u64,
    pub settlement_start_block: u64,
    pub settlement_delay: u64,
    pub appchain_owner: Address,
    pub appchain_chain_id: u64,
    pub finality_delay: u64,
}

impl Default for ConfigurationOptions {
    fn default() -> Self {
        Self {
            pre_loaded: None,
            sequencing_start_block: 3,
            settlement_start_block: 1,
            settlement_delay: 0,
            appchain_owner: APPCHAIN_OWNER,
            appchain_chain_id: APPCHAIN_CHAIN_ID,
            finality_delay: 60,
        }
    }
}

impl Components {
    pub async fn new(options: &ConfigurationOptions) -> Result<Self> {
        // This can be made per-test once we switch to nextest.
        // Until then, the tracer from the first test persists for all the tests
        // and additional calls to init_test_tracing() fail and have no effect.
        _ = init_test_tracing(Level::INFO);

        info!("Setting tags...");
        let mut tags = ImageTags::default();
        if let Ok(tag) = env::var("POSTER_TAG") {
            tags.poster_tag = Some(tag);
        }
        if let Ok(tag) = env::var("TRANSLATOR_TAG") {
            tags.translator_tag = Some(tag);
        }
        if let Ok(tag) = env::var("SEQUENCER_TAG") {
            tags.sequencer_tag = Some(tag);
        }

        info!("Starting components...");

        info!("Starting mchain...");
        let (mchain_rpc_url, _mchain, mchain_provider) =
            start_mchain(options.appchain_chain_id, options.appchain_owner, options.finality_delay)
                .await?;

        // Launch mock sequencing chain and deploy contracts
        info!("Starting sequencing chain...");
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(
            &seq_provider,
            U256::from(options.appchain_chain_id),
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
        info!("Starting settlement chain...");
        let (set_port, set_anvil, set_provider);
        if let Some(version) = &options.pre_loaded {
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

            // Sync the tips of the sequencing and settlement chains
            #[allow(clippy::unwrap_used)]
            let block = set_provider.get_block_by_number(BlockNumberOrTag::Latest).await?.unwrap();
            seq_provider
                .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp))))
                .await?;
        } else {
            // If not use our mock Rollup contract for easier testing
            (set_port, set_anvil, set_provider) = start_anvil(20).await?;
            // Use the mock rollup contract for the test instead of deploying all the nitro rollup
            // contracts
            _ = Rollup::deploy_builder(
                &set_provider,
                U256::from(options.appchain_chain_id),
                rollup_config(options.appchain_chain_id, options.appchain_owner),
            )
            .nonce(0)
            .send()
            .await?;

            mine_block(&set_provider, 0).await?;
        }
        let sequencing_rpc_url = format!("http://localhost:{}", seq_port);
        let settlement_rpc_url = format!("http://localhost:{}", set_port);

        // Launch components
        info!("Starting sequencer...");
        let sequencer_config = SequencerConfig {
            sequencing_contract_address: get_rollup_contract_address(),
            sequencing_rpc_url: sequencing_rpc_url.clone(),
            sequencer_port: PortManager::instance().next_port(),
            metrics_port: PortManager::instance().next_port(),
        };
        let sequencer =
            start_component("metabased-sequencer", tags.sequencer_tag, sequencer_config.cli_args())
                .await?;
        wait_for_service(sequencer_config.metrics_port).await?;

        info!("Starting translator...");
        let translator_config = TranslatorConfig {
            sequencing_start_block: options.sequencing_start_block,
            settlement_start_block: options.settlement_start_block,
            settlement_delay: options.settlement_delay,
            arbitrum_bridge_address: options.pre_loaded.as_ref().map_or_else(
                get_rollup_contract_address,
                |version| match version {
                    ContractVersion::V300 => PRELOAD_BRIDGE_ADDRESS_300,
                    ContractVersion::V213 => PRELOAD_BRIDGE_ADDRESS_231,
                },
            ),
            arbitrum_inbox_address: options.pre_loaded.as_ref().map_or_else(
                get_rollup_contract_address,
                |version| match version {
                    ContractVersion::V300 => PRELOAD_INBOX_ADDRESS_300,
                    ContractVersion::V213 => PRELOAD_INBOX_ADDRESS_231,
                },
            ),
            sequencing_contract_address: get_rollup_contract_address(),
            mchain_rpc_url: mchain_rpc_url.clone(),
            metrics_port: PortManager::instance().next_port(),
            sequencing_rpc_url,
            settlement_rpc_url: settlement_rpc_url.clone(),
        };
        let translator = start_component(
            "metabased-translator",
            tags.translator_tag,
            translator_config.cli_args(),
        )
        .await?;
        wait_for_service(translator_config.metrics_port).await?;

        // Launch the nitro rollup
        info!("Starting nitro node...");
        let (nitro_docker, appchain_provider, nitro_url) = launch_nitro_node(
            options.appchain_chain_id,
            options.appchain_owner,
            &mchain_rpc_url,
            Some(sequencer_config.sequencer_port),
        )
        .await?;

        let assertion_poster_contract_address = options.pre_loaded.as_ref().map_or_else(
            || Address::ZERO,
            |version| match version {
                ContractVersion::V300 => PRELOAD_POSTER_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_POSTER_ADDRESS_231,
            },
        );
        let mut poster = None;
        if options.pre_loaded.is_some() {
            info!("Starting poster...");
            let poster_config = PosterConfig {
                assertion_poster_contract_address,
                settlement_rpc_url: settlement_rpc_url.clone(),
                metrics_port: PortManager::instance().next_port(),
                appchain_rpc_url: nitro_url,
                polling_interval: Duration::from_secs(1),
            };
            poster = Some(
                start_component("metabased-poster", tags.poster_tag, poster_config.cli_args())
                    .await?,
            );
            wait_for_service(poster_config.metrics_port).await?;
        }

        // Launch sequencer
        Ok(Self {
            _timer: TestTimer(SystemTime::now()),

            sequencing_provider: seq_provider,
            settlement_provider: set_provider,
            appchain_provider,
            chain_id: options.appchain_chain_id,
            bridge_address: translator_config.arbitrum_bridge_address,
            inbox_address: translator_config.arbitrum_inbox_address,
            assertion_poster_contract_address,
            mchain_provider,

            _mchain,
            _seq_anvil: seq_anvil,
            _set_anvil: set_anvil,
            _nitro_docker: nitro_docker,
            _translator: translator,
            _poster: poster,
            _sequencer: sequencer,
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

async fn start_component(
    executable_name: &str,
    tag: Option<String>,
    args: Vec<String>,
) -> Result<Docker> {
    if let Some(tag) = tag {
        Ok(Docker(
            Command::new("docker")
                .arg("run")
                .arg("--init")
                .arg("--rm")
                .arg("--net=host")
                .arg(format!("ghcr.io/syndicateprotocol/{executable_name}:{tag}"))
                .args(args)
                .spawn()?,
        ))
    } else {
        Ok(Docker(
            Command::new("cargo")
                .current_dir("../")
                .arg("run")
                .arg("--bin")
                .arg(executable_name)
                .arg("--")
                .args(args)
                .spawn()?,
        ))
    }
}

async fn wait_for_service(port: u16) -> Result<()> {
    let client = Client::new();
    loop {
        match client.get(format!("http://localhost:{port}/metrics")).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Metabased-translator is now running.");
                return Ok(());
            }
            _ => {
                println!("Waiting for metabased-translator to start...");
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

impl TranslatorConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--mchain-rpc-url".to_string(),
            self.mchain_rpc_url.to_string(),
            "--sequencing-contract-address".to_string(),
            self.sequencing_contract_address.to_string(),
            "--arbitrum-bridge-address".to_string(),
            self.arbitrum_bridge_address.to_string(),
            "--arbitrum-inbox-address".to_string(),
            self.arbitrum_inbox_address.to_string(),
            "--sequencing-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--sequencing-start-block".to_string(),
            self.sequencing_start_block.to_string(),
            "--settlement-start-block".to_string(),
            self.settlement_start_block.to_string(),
            "--settlement-delay".to_string(),
            self.settlement_delay.to_string(),
            "--sequencing-polling-interval".to_string(),
            "100ms".to_string(),
            "--settlement-polling-interval".to_string(),
            "100ms".to_string(),
        ]
    }
}

impl PosterConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
            "--appchain-rpc-url".to_string(),
            self.appchain_rpc_url.to_string(),
            "--assertion-poster-contract-address".to_string(),
            self.assertion_poster_contract_address.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--polling-interval".to_string(),
            self.polling_interval.as_secs().to_string() + "s",
        ]
    }
}

impl SequencerConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
            "--chain-contract-address".to_string(),
            self.sequencing_contract_address.to_string(),
            "--chain-rpc-url".to_string(),
            self.sequencing_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--port".to_string(),
            self.sequencer_port.to_string(),
        ]
    }
}
