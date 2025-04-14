//! Components for the integration tests

use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    eips::{BlockNumberOrTag, Encodable2718},
    node_bindings::AnvilInstance,
    primitives::{address, Address, U256},
    providers::{ext::AnvilApi as _, Provider, RootProvider, WalletProvider},
    rpc::types::anvil::MineOptions,
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule, arbconfigmanager::ArbConfigManager,
        metabasedsequencerchain::MetabasedSequencerChain,
    },
};
use eyre::Result;
use mchain::mchain::{rollup_config, MProvider};
use std::{
    env,
    future::Future,
    io::{stderr, Write},
    time::{Duration, SystemTime},
};
use test_utils::{
    anvil::{mine_block, start_anvil, start_anvil_with_args, FilledProvider},
    docker::{launch_nitro_node, start_component, start_mchain, Docker},
    port_manager::PortManager,
    preloaded_config::{
        PRELOAD_BRIDGE_ADDRESS_231, PRELOAD_BRIDGE_ADDRESS_300, PRELOAD_INBOX_ADDRESS_231,
        PRELOAD_INBOX_ADDRESS_300, PRELOAD_POSTER_ADDRESS_231, PRELOAD_POSTER_ADDRESS_300,
    },
};
use tracing::info;

// needs to be different from the regular private key to prevent nonce collisions
// needs to match the owner of the poster contract
// anvil account 0
const POSTER_SEQUENCER_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

#[derive(Debug)]
struct ComponentHandles {
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    mchain: Docker,
    nitro_docker: Docker,
    translator: Docker,
    poster: Option<Docker>,
    sequencer: Docker,
}

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Components {
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

    /// Appchain
    pub appchain_provider: RootProvider,

    /// Mchain
    pub mchain_provider: MProvider,

    pub poster_url: String,
}

#[derive(Debug)]
struct TestTimer(SystemTime, Duration);

impl Drop for TestTimer {
    fn drop(&mut self) {
        let thread = std::thread::current();
        let elapsed = format!(
            "---- SYN ---- Test {:?} took: {:.2?} (setup: {:.2?})\n",
            thread.name().unwrap_or_default(),
            self.0.elapsed().unwrap_or_default(),
            self.1,
        );
        // Write directly to stderr (bypasses test harness output capture)
        let _ = stderr().write_all(elapsed.as_bytes());
    }
}

/// nitro contract version on the settlement chain used for testing
#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum ContractVersion {
    V213,
    V300,
}

#[derive(Debug, Default)]
struct TranslatorConfig {
    arbitrum_bridge_address: Option<Address>,
    arbitrum_inbox_address: Option<Address>,
    sequencing_contract_address: Option<Address>,
    arbitrum_ignore_delayed_messages: Option<bool>,
    config_manager_address: Option<Address>,
    appchain_chain_id: Option<u64>,
    mchain_rpc_url: String,
    sequencing_rpc_url: Option<String>,
    settlement_rpc_url: String,
    metrics_port: u16,
    sequencing_start_block: Option<u64>,
    settlement_start_block: Option<u64>,
    settlement_delay: Option<u64>,
}

impl TranslatorConfig {
    fn cli_args(&self) -> Vec<String> {
        let mut args = vec![
            "--mchain-rpc-url".to_string(),
            self.mchain_rpc_url.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
        ];

        if let Some(url) = &self.sequencing_rpc_url {
            args.extend(vec!["--sequencing-rpc-url".to_string(), url.to_string()]);
        }

        if let Some(addr) = self.arbitrum_bridge_address {
            args.extend(vec!["--arbitrum-bridge-address".to_string(), addr.to_string()]);
        }

        if let Some(addr) = self.arbitrum_inbox_address {
            args.extend(vec!["--arbitrum-inbox-address".to_string(), addr.to_string()]);
        }

        if let Some(block) = self.sequencing_start_block {
            args.extend(vec!["--sequencing-start-block".to_string(), block.to_string()]);
        }

        if let Some(block) = self.settlement_start_block {
            args.extend(vec!["--settlement-start-block".to_string(), block.to_string()]);
        }

        if let Some(delay) = self.settlement_delay {
            args.extend(vec!["--settlement-delay".to_string(), delay.to_string()]);
        }

        if let Some(ignore) = self.arbitrum_ignore_delayed_messages {
            args.extend(vec!["--arbitrum-ignore-delayed-messages".to_string(), ignore.to_string()]);
        }

        if let Some(addr) = self.sequencing_contract_address {
            args.extend(vec!["--sequencing-contract-address".to_string(), addr.to_string()]);
        }

        if let Some(addr) = self.config_manager_address {
            args.extend(vec!["--config-manager-address".to_string(), addr.to_string()]);
        }

        if let Some(chain_id) = self.appchain_chain_id {
            args.extend(vec!["--appchain-chain-id".to_string(), chain_id.to_string()]);
        }

        args.extend(vec![
            "--sequencing-polling-interval".to_string(),
            "50ms".to_string(),
            "--settlement-polling-interval".to_string(),
            "50ms".to_string(),
        ]);

        args
    }
}

#[derive(Debug)]
struct PosterConfig {
    assertion_poster_contract_address: Address,
    settlement_rpc_url: String,
    appchain_rpc_url: String,
    metrics_port: u16,
    port: u16,
}

#[derive(Debug)]
struct SequencerConfig {
    sequencing_contract_address: Address,
    sequencing_rpc_url: String,
    sequencer_port: u16,
    metrics_port: u16,
}

#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct ConfigurationOptions {
    pub pre_loaded: Option<ContractVersion>,
    pub sequencing_start_block: u64,
    pub settlement_start_block: u64,
    pub settlement_delay: u64,
    pub rollup_owner: Address,
    pub appchain_chain_id: u64,
    pub finality_delay: u64,
}

impl Default for ConfigurationOptions {
    fn default() -> Self {
        Self {
            pre_loaded: None,
            // skip the genesis block
            sequencing_start_block: 1,
            // skip the genesis block and the 2 blocks used to deploy the config manager
            settlement_start_block: 1 + 2,
            settlement_delay: 0,
            rollup_owner: Address::ZERO,
            appchain_chain_id: 13331370,
            finality_delay: 60,
        }
    }
}

impl Components {
    #[allow(clippy::unwrap_used)]
    pub(crate) async fn run<Fut: Future<Output = Result<()>> + Send>(
        options: &ConfigurationOptions,
        test: impl FnOnce(Self) -> Fut + Send,
    ) -> Result<()> {
        let (components, mut handles) = Self::new(options).await?;
        let poster = handles.poster.take();
        tokio::select! {
            biased;
            e = handles.mchain.wait() => panic!("mchain died: {:#?}", e),
            e = handles.nitro_docker.wait() => panic!("nitro died: {:#?}", e),
            e = handles.translator.wait() => panic!("translator died: {:#?}", e),
            e = async {poster.unwrap().wait().await}, if poster.is_some() => panic!("poster died: {:#?}", e),
            e = handles.sequencer.wait() => panic!("sequencer died: {:#?}", e),
            r = test(components) => r
        }
    }

    #[allow(clippy::unwrap_used)]
    async fn new(options: &ConfigurationOptions) -> Result<(Self, ComponentHandles)> {
        let mut options = options.clone();
        let start_time = SystemTime::now();

        // Launch mock sequencing chain and deploy contracts
        info!("Starting sequencing chain...");
        let (seq_port, seq_anvil, seq_provider) = start_anvil(15).await?;
        _ = MetabasedSequencerChain::deploy_builder(
            &seq_provider,
            U256::from(options.appchain_chain_id),
        )
        .send()
        .await?;
        let sequencing_contract_address = seq_provider.default_signer_address().create(0);
        _ = AlwaysAllowedModule::deploy_builder(&seq_provider).send().await?;
        let always_allowed_module_address = seq_provider.default_signer_address().create(1);

        // Setup the sequencing contract
        let provider_clone = seq_provider.clone();
        let sequencing_contract =
            MetabasedSequencerChain::new(sequencing_contract_address, provider_clone);
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
                rollup_config(options.appchain_chain_id, options.rollup_owner),
            )
            .nonce(0)
            .send()
            .await?;

            mine_block(&set_provider, 0).await?;
        }

        let arbitrum_bridge_address = options
            .pre_loaded
            .as_ref()
            .map_or_else(
                || Some(set_provider.default_signer_address().create(0)),
                |version| match version {
                    ContractVersion::V300 => Some(PRELOAD_BRIDGE_ADDRESS_300),
                    ContractVersion::V213 => Some(PRELOAD_BRIDGE_ADDRESS_231),
                },
            )
            .unwrap();
        let arbitrum_inbox_address = options
            .pre_loaded
            .as_ref()
            .map_or_else(
                || Some(set_provider.default_signer_address().create(0)),
                |version| match version {
                    ContractVersion::V300 => Some(PRELOAD_INBOX_ADDRESS_300),
                    ContractVersion::V213 => Some(PRELOAD_INBOX_ADDRESS_231),
                },
            )
            .unwrap();

        let sequencing_rpc_url = format!("http://localhost:{}", seq_port);
        let settlement_rpc_url = format!("http://localhost:{}", set_port);

        // overwrite the rollup owner in case it's not set (cannot be empty in config manager)
        if options.rollup_owner == Address::ZERO {
            options.rollup_owner = address!("0x0000000000000000000000000000000000000064");
        }

        info!("Starting components...");
        info!("Starting mchain...");
        let (mchain_rpc_url, mchain, mchain_provider) =
            start_mchain(options.appchain_chain_id, options.rollup_owner, options.finality_delay)
                .await?;

        // Launch components
        info!("Starting sequencer...");
        let sequencer_config = SequencerConfig {
            sequencing_contract_address,
            sequencing_rpc_url: sequencing_rpc_url.clone(),
            sequencer_port: PortManager::instance().next_port(),
            metrics_port: PortManager::instance().next_port(),
        };
        let sequencer = start_component(
            "metabased-sequencer",
            sequencer_config.metrics_port,
            sequencer_config.cli_args(),
            Default::default(),
        )
        .await?;

        // Setup config manager and get chain config address
        let config_manager_address = setup_config_manager(
            &set_provider,
            &options,
            sequencing_contract_address,
            arbitrum_bridge_address,
            arbitrum_inbox_address,
            &sequencing_rpc_url,
        )
        .await?;

        info!("Starting translator...");
        // only set the settlement rpc URL, config_manager address and appchain_chain_id - the
        // translator will use the on-chain configuration
        let translator_config = TranslatorConfig {
            settlement_rpc_url: settlement_rpc_url.clone(),
            config_manager_address: Some(config_manager_address),
            appchain_chain_id: Some(options.appchain_chain_id),
            mchain_rpc_url: mchain_rpc_url.clone(),
            metrics_port: PortManager::instance().next_port(),
            arbitrum_bridge_address: None,
            arbitrum_inbox_address: None,
            sequencing_contract_address: None,
            arbitrum_ignore_delayed_messages: None,
            sequencing_rpc_url: None,
            sequencing_start_block: None,
            settlement_start_block: None,
            settlement_delay: None,
        };

        let translator = start_component(
            "metabased-translator",
            translator_config.metrics_port,
            translator_config.cli_args(),
            Default::default(),
        )
        .await?;

        // Launch the nitro rollup
        info!("Starting nitro node...");
        let (nitro_docker, appchain_provider, nitro_url) = launch_nitro_node(
            options.appchain_chain_id,
            options.rollup_owner,
            &mchain_rpc_url,
            Some(sequencer_config.sequencer_port),
        )
        .await?;

        let (poster, poster_url) = if options.pre_loaded.is_some() {
            info!("Starting poster...");
            let poster_config = PosterConfig {
                assertion_poster_contract_address: options.pre_loaded.as_ref().map_or(
                    Address::ZERO,
                    |version| match version {
                        ContractVersion::V300 => PRELOAD_POSTER_ADDRESS_300,
                        ContractVersion::V213 => PRELOAD_POSTER_ADDRESS_231,
                    },
                ),
                settlement_rpc_url: settlement_rpc_url.clone(),
                metrics_port: PortManager::instance().next_port(),
                port: PortManager::instance().next_port(),
                appchain_rpc_url: nitro_url,
            };
            (
                Some(
                    start_component(
                        "metabased-poster",
                        poster_config.metrics_port,
                        poster_config.cli_args(),
                        Default::default(),
                    )
                    .await?,
                ),
                format!("http://localhost:{}", poster_config.port),
            )
        } else {
            (None, Default::default())
        };

        // Launch sequencer
        Ok((
            Self {
                #[allow(clippy::unwrap_used)]
                _timer: TestTimer(SystemTime::now(), start_time.elapsed().unwrap()),

                sequencing_provider: seq_provider,

                settlement_provider: set_provider,
                appchain_provider,
                chain_id: options.appchain_chain_id,
                bridge_address: arbitrum_bridge_address,
                inbox_address: arbitrum_inbox_address,
                mchain_provider,
                poster_url,
            },
            ComponentHandles {
                _seq_anvil: seq_anvil,
                _set_anvil: set_anvil,
                mchain,
                nitro_docker,
                translator,
                poster,
                sequencer,
            },
        ))
    }

    pub(crate) async fn mine_seq_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.sequencing_provider, delay).await?;
        Ok(())
    }

    pub(crate) async fn send_tx_and_mine_block(
        &self,
        tx: &EthereumTxEnvelope<TxEip4844Variant>,
        delay: u64,
    ) -> Result<()> {
        self.sequencing_provider.anvil_set_block_timestamp_interval(delay).await?;
        self.sequencing_provider.anvil_set_auto_mine(true).await?;
        _ = self.appchain_provider.send_raw_transaction(&tx.encoded_2718()).await?;
        self.sequencing_provider.anvil_set_auto_mine(false).await?;
        self.sequencing_provider.anvil_remove_block_timestamp_interval().await?;
        Ok(())
    }

    pub(crate) async fn mine_set_block(&self, delay: u64) -> Result<()> {
        mine_block(&self.settlement_provider, delay).await?;
        Ok(())
    }

    pub(crate) async fn mine_both(&self, delay: u64) -> Result<()> {
        self.mine_seq_block(delay).await?;
        self.mine_set_block(delay).await?;
        Ok(())
    }
}

impl PosterConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            POSTER_SEQUENCER_PRIVATE_KEY.to_string(),
            "--appchain-rpc-url".to_string(),
            self.appchain_rpc_url.to_string(),
            "--assertion-poster-contract-address".to_string(),
            self.assertion_poster_contract_address.to_string(),
            "--settlement-rpc-url".to_string(),
            self.settlement_rpc_url.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
            "--polling-interval".to_string(),
            "1h".to_string(),
            "--port".to_string(),
            self.port.to_string(),
        ]
    }
}

impl SequencerConfig {
    fn cli_args(&self) -> Vec<String> {
        vec![
            "--private-key".to_string(),
            POSTER_SEQUENCER_PRIVATE_KEY.to_string(),
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

/// Sets up the config manager and creates the chain configuration
#[allow(clippy::unwrap_used)]
async fn setup_config_manager(
    set_provider: &FilledProvider,
    options: &ConfigurationOptions,
    sequencing_contract_address: Address,
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_rpc_url: &str,
) -> Result<Address> {
    // Deploy config manager
    let config_manager_owner = set_provider.default_signer_address();
    let config_manager_tx =
        ArbConfigManager::deploy_builder(set_provider, config_manager_owner).send().await?;
    mine_block(set_provider, 0).await?;
    let config_manager_address = config_manager_tx.get_receipt().await?.contract_address.unwrap();
    let config_manager = ArbConfigManager::new(config_manager_address, set_provider.clone());

    let options_clone = options.clone();
    let sequencing_rpc_url_clone = sequencing_rpc_url.to_string();

    let create_chain_config_tx = config_manager
        .createArbChainConfig(
            options_clone.appchain_chain_id.try_into().unwrap(),
            false,
            arbitrum_bridge_address,
            arbitrum_inbox_address,
            false,
            options_clone.settlement_delay.try_into().unwrap(),
            options_clone.settlement_start_block.try_into().unwrap(),
            sequencing_contract_address,
            options_clone.sequencing_start_block.try_into().unwrap(),
            options_clone.rollup_owner,
            sequencing_rpc_url_clone,
        )
        .send()
        .await?;
    mine_block(set_provider, 0).await?;

    assert!(create_chain_config_tx.get_receipt().await?.status());

    Ok(config_manager.address().to_owned())
}
