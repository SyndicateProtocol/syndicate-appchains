//! Components for the integration tests

use alloy::{
    node_bindings::AnvilInstance,
    primitives::{Address, U256},
    providers::{ProviderBuilder, RootProvider, WalletProvider},
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule,
        metabasedsequencerchain::MetabasedSequencerChain::{self, MetabasedSequencerChainInstance},
    },
};
use eyre::{eyre, Result};
use reqwest::Client;
use std::{env, time::Duration};
use test_utils::{
    anvil::{mine_block, start_anvil, start_anvil_with_args, FilledProvider},
    docker::{launch_nitro_node, start_reth, Docker},
    port_manager::PortManager,
    preloaded_config::{
        APPCHAIN_OWNER, DEFAULT_PRIVATE_KEY_SIGNER, PRELOAD_BRIDGE_ADDRESS_231,
        PRELOAD_BRIDGE_ADDRESS_300, PRELOAD_INBOX_ADDRESS_231, PRELOAD_INBOX_ADDRESS_300,
        PRELOAD_POSTER_ADDRESS_231, PRELOAD_POSTER_ADDRESS_300,
    },
    rollup::{get_rollup_contract_address, rollup_config, MCHAIN_ID},
};
use tokio::{process::Command, time::sleep};
use tracing::info;

const APPCHAIN_CHAIN_ID: u64 = 13331370;

#[derive(Debug)]
pub struct Components {
    /// Sequencing
    pub sequencing_contract: MetabasedSequencerChainInstance<(), FilledProvider>,
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
    pub mchain_provider: RootProvider,

    // References to keep the processes/tasks alive
    _mchain: (Docker, Option<(Docker, Docker, Docker, Docker)>),
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    _nitro_docker: Docker,
    _translator: Docker,
    _poster: Option<Docker>,
    _sequencer: Docker,
}

/// nitro contract version on the settlement chain used for testing
#[derive(Debug)]
pub enum ContractVersion {
    V213,
    V300,
}

#[derive(Debug, Default)]
struct TranslatorConfig {
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_contract_address: Address,
    mchain_ipc_path: String,
    mchain_auth_ipc_path: String,
    sequencing_rpc_url: String,
    settlement_rpc_url: String,
    metrics_port: u16,
    sequencing_start_block: u64,
    settlement_start_block: u64,
    settlement_delay: u64,
}
#[derive(Debug, Default)]
struct PosterConfig {
    assertion_poster_contract_address: Address,
    settlement_rpc_url: String,
    appchain_rpc_url: String,
    polling_interval: Duration,
    metrics_port: u16,
}
#[derive(Debug, Default)]
struct SequencerConfig {
    sequencing_contract_address: Address,
    settlement_rpc_url: String,
    sequencer_port: u16,
    metrics_port: u16,
}
#[allow(missing_docs)]
#[derive(Debug)]
pub struct ImageTags {
    /// External components
    reth_tag: String,
    nitro_tag: String,

    /// Syndicate components
    poster_tag: Option<String>,
    translator_tag: Option<String>,
    sequencer_tag: Option<String>,
}

impl Default for ImageTags {
    fn default() -> Self {
        Self {
            reth_tag: "2.0.0".to_string(),
            nitro_tag: "v3.4.0-d896e9c-slim".to_string(), // or some other sensible default
            poster_tag: None,
            translator_tag: None,
            sequencer_tag: None,
        }
    }
}
#[derive(Debug, Default, Clone, Copy)]
pub struct ConfigurationOptions {
    pub sequencing_start_block: u64,
    pub settlement_start_block: u64,
    pub settlement_delay: u64,
}

impl Components {
    pub async fn new(
        pre_loaded: Option<ContractVersion>,
        options: Option<ConfigurationOptions>,
    ) -> Result<Self> {
        info!("Setting tags...");
        let mut tags = ImageTags::default();
        if let Ok(tag) = env::var("NITRO_TAG") {
            tags.nitro_tag = tag;
        }
        if let Ok(tag) = env::var("RETH_TAG") {
            tags.reth_tag = tag;
        }
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
        let mut translator_config: TranslatorConfig = Default::default();
        let mut poster_config: PosterConfig = Default::default();
        let mut sequencer_config: SequencerConfig = Default::default();

        if let Some(opt) = options {
            translator_config.sequencing_start_block = opt.sequencing_start_block;
            translator_config.settlement_start_block = opt.settlement_start_block;
            translator_config.settlement_delay = opt.settlement_delay;
        }
        // Define the addresses of the bridge and inbox contracts depedning on whether we
        // are loading in the full set of Arb contracts or not
        translator_config.arbitrum_bridge_address =
            pre_loaded.as_ref().map_or_else(get_rollup_contract_address, |version| match version {
                ContractVersion::V300 => PRELOAD_BRIDGE_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_BRIDGE_ADDRESS_231,
            });
        translator_config.arbitrum_inbox_address =
            pre_loaded.as_ref().map_or_else(get_rollup_contract_address, |version| match version {
                ContractVersion::V300 => PRELOAD_INBOX_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_INBOX_ADDRESS_231,
            });

        poster_config.assertion_poster_contract_address = pre_loaded.as_ref().map_or_else(
            || Address::ZERO,
            |version| match version {
                ContractVersion::V300 => PRELOAD_POSTER_ADDRESS_300,
                ContractVersion::V213 => PRELOAD_POSTER_ADDRESS_231,
            },
        );

        translator_config.sequencing_contract_address = get_rollup_contract_address();
        sequencer_config.sequencing_contract_address = get_rollup_contract_address();

        info!("Starting reth...");
        let (node, _mchain, mchain_port) = start_reth(MCHAIN_ID, &tags.reth_tag).await?;
        let mchain_provider: RootProvider = ProviderBuilder::default()
            .on_http(format!("http://localhost:{}", mchain_port).parse()?);
        translator_config.mchain_ipc_path = node.ipc;
        translator_config.mchain_auth_ipc_path = node.auth_ipc;

        // Launch mock sequencing chain and deploy contracts
        info!("Starting sequencing chain...");
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(&seq_provider, U256::from(APPCHAIN_CHAIN_ID))
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
        if let Some(version) = &pre_loaded {
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
                U256::from(APPCHAIN_CHAIN_ID),
                rollup_config(APPCHAIN_CHAIN_ID, APPCHAIN_OWNER),
            )
            .nonce(0)
            .send()
            .await?;

            mine_block(&set_provider, 0).await?;
        }

        // Update the RPC URLs in the config
        translator_config.sequencing_rpc_url = format!("http://localhost:{}", seq_port);
        translator_config.settlement_rpc_url = format!("http://localhost:{}", set_port);
        sequencer_config.settlement_rpc_url = format!("http://localhost:{}", set_port);
        poster_config.settlement_rpc_url = format!("http://localhost:{}", set_port);

        // Set up ports
        sequencer_config.sequencer_port = PortManager::instance().next_port();
        sequencer_config.metrics_port = PortManager::instance().next_port();
        translator_config.metrics_port = PortManager::instance().next_port();
        poster_config.metrics_port = PortManager::instance().next_port();

        // Launch components
        info!("Starting sequencer...");
        let sequencer = start_component(
            "metabased-sequencer",
            tags.sequencer_tag,
            get_sequencer_cli_args(&sequencer_config),
        )
        .await?;
        wait_for_service(sequencer_config.metrics_port).await?;
        info!("Starting translator...");
        let translator = start_component(
            "metabased-translator",
            tags.translator_tag,
            get_translator_cli_args(&translator_config),
        )
        .await?;
        wait_for_service(translator_config.metrics_port).await?;
        // Launch the nitro rollup
        info!("Starting nitro node...");
        let (nitro_docker, appchain_provider, nitro_url) = launch_nitro_node(
            APPCHAIN_CHAIN_ID,
            APPCHAIN_OWNER,
            node.http_port,
            Some(sequencer_config.sequencer_port),
            &tags.nitro_tag,
        )
        .await?;
        poster_config.appchain_rpc_url = nitro_url;
        poster_config.polling_interval = Duration::from_secs(1);

        let mut poster = None;
        if pre_loaded.is_some() {
            info!("Starting poster...");
            poster = Some(
                start_component(
                    "metabased-poster",
                    tags.poster_tag,
                    get_poster_cli_args(&poster_config),
                )
                .await?,
            );
            wait_for_service(poster_config.metrics_port).await?;
        }

        // Launch sequencer
        Ok(Self {
            sequencing_contract,
            sequencing_provider: seq_provider,
            settlement_provider: set_provider,
            appchain_provider,
            chain_id: APPCHAIN_CHAIN_ID,
            bridge_address: translator_config.arbitrum_bridge_address,
            inbox_address: translator_config.arbitrum_inbox_address,
            assertion_poster_contract_address: poster_config.assertion_poster_contract_address,
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

fn get_translator_cli_args(config: &TranslatorConfig) -> Vec<String> {
    vec![
        "--mchain-ipc-path".to_string(),
        config.mchain_ipc_path.to_string(),
        "--mchain-auth-ipc-path".to_string(),
        config.mchain_auth_ipc_path.to_string(),
        "--sequencing-contract-address".to_string(),
        config.sequencing_contract_address.to_string(),
        "--arbitrum-bridge-address".to_string(),
        config.arbitrum_bridge_address.to_string(),
        "--arbitrum-inbox-address".to_string(),
        config.arbitrum_inbox_address.to_string(),
        "--sequencing-rpc-url".to_string(),
        config.sequencing_rpc_url.to_string(),
        "--settlement-rpc-url".to_string(),
        config.settlement_rpc_url.to_string(),
        "--metrics-port".to_string(),
        config.metrics_port.to_string(),
        "--target-chain-id".to_string(),
        APPCHAIN_CHAIN_ID.to_string(),
        "--rollup-owner-address".to_string(),
        APPCHAIN_OWNER.to_string(),
        "--sequencing-start-block".to_string(),
        config.sequencing_start_block.to_string(),
        "--settlement-start-block".to_string(),
        config.settlement_start_block.to_string(),
        "--settlement-delay".to_string(),
        config.settlement_delay.to_string(),
    ]
}

fn get_poster_cli_args(config: &PosterConfig) -> Vec<String> {
    vec![
        "--private-key".to_string(),
        DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
        "--appchain-rpc-url".to_string(),
        config.appchain_rpc_url.to_string(),
        "--assertion-poster-contract-address".to_string(),
        config.assertion_poster_contract_address.to_string(),
        "--settlement-rpc-url".to_string(),
        config.settlement_rpc_url.to_string(),
        "--metrics-port".to_string(),
        config.metrics_port.to_string(),
        "--polling-interval".to_string(),
        config.polling_interval.as_secs().to_string() + "s",
    ]
}

fn get_sequencer_cli_args(config: &SequencerConfig) -> Vec<String> {
    vec![
        "--private-key".to_string(),
        DEFAULT_PRIVATE_KEY_SIGNER.to_string(),
        "--chain-contract-address".to_string(),
        config.sequencing_contract_address.to_string(),
        "--chain-rpc-url".to_string(),
        config.settlement_rpc_url.to_string(),
        "--metrics-port".to_string(),
        config.metrics_port.to_string(),
        "--port".to_string(),
        config.sequencer_port.to_string(),
    ]
}
