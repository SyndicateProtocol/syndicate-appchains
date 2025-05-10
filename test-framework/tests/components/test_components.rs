//! Components for the integration tests

use crate::components::{
    batch_sequencer::BatchSequencerConfig,
    configuration::{setup_config_manager, ConfigurationOptions, ContractVersion},
    maestro::MaestroConfig,
    poster::PosterConfig,
    sequencer::SequencerConfig,
    timer::TestTimer,
    translator::TranslatorConfig,
};
use alloy::{
    consensus::{EthereumTxEnvelope, TxEip4844Variant},
    eips::{BlockNumberOrTag, Encodable2718},
    node_bindings::AnvilInstance,
    primitives::{address, hex, Address, TxHash, U256},
    providers::{ext::AnvilApi as _, Provider, RootProvider, WalletProvider},
    rpc::types::anvil::MineOptions,
};
use contract_bindings::{
    arbitrum::rollup::Rollup,
    metabased::{
        alwaysallowedmodule::AlwaysAllowedModule,
        metabasedsequencerchain::MetabasedSequencerChain::{self, MetabasedSequencerChainInstance},
        walletpoolwrappermodule::WalletPoolWrapperModule,
    },
};
use eyre::Result;
use maestro::server::HEADER_CHAIN_ID;
use mchain::{client::MProvider, server::rollup_config};
use serde_json::{json, Value};
use shared::types::FilledProvider;
use std::{env, future::Future, str::FromStr, time::SystemTime};
use test_utils::{
    anvil::{mine_block, start_anvil, start_anvil_with_args, PRIVATE_KEY},
    docker::{launch_nitro_node, start_component, start_mchain, start_redis, Docker},
    port_manager::PortManager,
    preloaded_config::{
        PRELOAD_BRIDGE_ADDRESS_231, PRELOAD_BRIDGE_ADDRESS_300, PRELOAD_INBOX_ADDRESS_231,
        PRELOAD_INBOX_ADDRESS_300, PRELOAD_POSTER_ADDRESS_231, PRELOAD_POSTER_ADDRESS_300,
    },
};
use tracing::info;

#[derive(Debug)]
struct ComponentHandles {
    _seq_anvil: AnvilInstance,
    _set_anvil: AnvilInstance,
    mchain: Docker,
    nitro_docker: Docker,
    translator: Docker,
    poster: Option<Docker>,
    sequencer: Docker,

    // Write loop
    batch_sequencer: Option<Docker>,
    redis: Option<Docker>,
    maestro: Option<Docker>,
}

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub struct TestComponents {
    /// Timer for latency measurement
    /// Keep this on top - the top element gets destroyed first
    _timer: TestTimer,

    /// Sequencing
    pub sequencing_provider: FilledProvider,
    pub sequencing_rpc_url: String,
    pub sequencing_contract: MetabasedSequencerChainInstance<(), FilledProvider>,

    /// Settlement
    pub settlement_provider: FilledProvider,
    pub settlement_rpc_url: String,
    pub chain_id: u64,
    pub bridge_address: Address,
    pub inbox_address: Address,

    /// Appchain
    pub appchain_provider: RootProvider,

    /// Mchain
    pub mchain_provider: MProvider,

    pub poster_url: String,

    pub maestro_url: String,

    #[allow(dead_code)]
    pub appchain_block_explorer_url: String,
}

impl TestComponents {
    #[allow(clippy::unwrap_used)]
    pub async fn run<Fut: Future<Output = Result<()>> + Send>(
        options: &ConfigurationOptions,
        test: impl FnOnce(Self) -> Fut + Send,
    ) -> Result<()> {
        let (components, mut handles) = Self::new(options).await?;
        let poster = handles.poster.take();
        let maestro = handles.maestro.take();
        let batch_sequencer = handles.batch_sequencer.take();
        let redis = handles.redis.take();
        tokio::select! {
            biased;
            e = handles.mchain.wait() => panic!("mchain died: {:#?}", e),
            e = handles.nitro_docker.wait() => panic!("nitro died: {:#?}", e),
            e = handles.translator.wait() => panic!("translator died: {:#?}", e),
            e = async {poster.unwrap().wait().await}, if poster.is_some() => panic!("poster died: {:#?}", e),
            e = handles.sequencer.wait() => panic!("sequencer died: {:#?}", e),
            e = async {maestro.unwrap().wait().await}, if maestro.is_some() => panic!("maestro died: {:#?}", e),
            e = async {batch_sequencer.unwrap().wait().await}, if batch_sequencer.is_some() => panic!("synd-batch-sequencer died: {:#?}", e),
            e = async {redis.unwrap().wait().await}, if redis.is_some() => panic!("redis died: {:#?}", e),
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

        // Deploy wallet pool
        let admin = seq_provider.default_signer_address();
        _ = WalletPoolWrapperModule::deploy_builder(&seq_provider, admin).send().await?;
        let wallet_pool_address = seq_provider.default_signer_address().create(2);

        // Setup the sequencing contract
        let provider_clone = seq_provider.clone();
        let sequencing_contract =
            MetabasedSequencerChain::new(sequencing_contract_address, provider_clone);
        _ = sequencing_contract
            .initialize(seq_provider.default_signer_address(), always_allowed_module_address)
            .send()
            .await?;

        // Set up allowlist
        let provider_clone = seq_provider.clone();
        let wallet_pool_contract =
            WalletPoolWrapperModule::new(wallet_pool_address, provider_clone);
        _ = wallet_pool_contract.addToAllowlist(admin).send().await?;

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

            (set_port, set_anvil, set_provider) =
                start_anvil_with_args(31337, &["--load-state", state_file.to_str().unwrap()])
                    .await?;

            // Sync the tips of the sequencing and settlement chains
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

        info!("sequencing_rpc_url: {}", sequencing_rpc_url);
        info!("settlement_rpc_url: {}", settlement_rpc_url);

        // overwrite the rollup owner in case it's not set (cannot be empty in config manager)
        if options.rollup_owner == Address::ZERO {
            options.rollup_owner = address!("0x0000000000000000000000000000000000000064");
        }

        // Setup config manager and get chain config address
        let appchain_block_explorer_url = "https://example.com/explorer".to_string();
        let config_manager_address = setup_config_manager(
            &set_provider,
            &options,
            sequencing_contract_address,
            arbitrum_bridge_address,
            arbitrum_inbox_address,
            &sequencing_rpc_url,
            &appchain_block_explorer_url,
        )
        .await?;

        info!("Starting components...");
        info!("Starting mchain...");
        let (mchain_rpc_url, mchain, mchain_provider) = start_mchain(
            options.appchain_chain_id,
            None,
            Some(config_manager_address),
            Some(&settlement_rpc_url),
            options.finality_delay,
        )
        .await?;

        info!("Starting sequencer...");
        let sequencer_config = SequencerConfig {
            sequencing_contract_address,
            sequencing_rpc_url: sequencing_rpc_url.clone(),
            sequencer_port: PortManager::instance().next_port().await,
            metrics_port: PortManager::instance().next_port().await,
        };
        let sequencer = start_component(
            "metabased-sequencer",
            // `/health` is proxied to RPC method
            sequencer_config.sequencer_port,
            sequencer_config.cli_args(),
            Default::default(),
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
            metrics_port: PortManager::instance().next_port().await,
            arbitrum_bridge_address: None,
            arbitrum_inbox_address: None,
            sequencing_contract_address: None,
            arbitrum_ignore_delayed_messages: None,
            sequencing_rpc_url: None,
            appchain_block_explorer_url: Some(appchain_block_explorer_url.clone()),
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
        info!("Nitro URL: {}", nitro_url);

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
                metrics_port: PortManager::instance().next_port().await,
                port: PortManager::instance().next_port().await,
                appchain_rpc_url: nitro_url.clone(),
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

        let (mut redis, mut maestro, mut batch_sequencer) = (None, None, None);
        let mut maestro_url = Default::default();
        if options.use_write_loop {
            info!("Starting Write Loop components...");
            info!("Starting redis...");
            let (redis_instance, redis_url) = start_redis().await?;
            redis = Some(redis_instance);
            info!("Starting maestro...");
            let maestro_config = MaestroConfig {
                port: PortManager::instance().next_port().await,
                redis_url: redis_url.clone(),
                chain_rpc_urls: format!("{{\"{}\":\"{}\"}}", options.appchain_chain_id, nitro_url),
                metrics_port: PortManager::instance().next_port().await,
            };
            let maestro_instance = start_component(
                "maestro",
                // `/health` is proxied to RPC method
                maestro_config.port,
                maestro_config.cli_args(),
                Default::default(),
            )
            .await?;
            maestro = Some(maestro_instance);
            maestro_url = format!("http://localhost:{}", maestro_config.port);
            info!("Starting batch sequencer...");
            let batch_sequencer_config = BatchSequencerConfig {
                chain_id: options.appchain_chain_id,
                redis_url: redis_url.clone(),
                private_key: PRIVATE_KEY.to_string(),
                wallet_pool_address,
                sequencing_address: sequencing_contract_address,
                sequencing_rpc_url: sequencing_rpc_url.clone(),
                metrics_port: PortManager::instance().next_port().await,
            };
            let batch_sequencer_instance = start_component(
                "synd-batch-sequencer",
                batch_sequencer_config.metrics_port,
                batch_sequencer_config.cli_args(),
                Default::default(),
            )
            .await?;
            batch_sequencer = Some(batch_sequencer_instance);
        }

        Ok((
            Self {
                _timer: TestTimer(SystemTime::now(), start_time.elapsed().unwrap()),

                sequencing_provider: seq_provider,
                sequencing_rpc_url: sequencing_rpc_url.clone(),
                sequencing_contract,

                settlement_provider: set_provider,
                settlement_rpc_url: settlement_rpc_url.clone(),
                appchain_provider,
                chain_id: options.appchain_chain_id,
                bridge_address: arbitrum_bridge_address,
                inbox_address: arbitrum_inbox_address,

                mchain_provider,
                poster_url,
                maestro_url,
                appchain_block_explorer_url,
            },
            ComponentHandles {
                _seq_anvil: seq_anvil,
                _set_anvil: set_anvil,
                mchain,
                nitro_docker,
                translator,
                poster,
                sequencer,
                batch_sequencer,
                redis,
                maestro,
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

    /// Use this if you intend for the txn to succeed
    /// Returns [`TxHash`]
    #[allow(clippy::unwrap_used)]
    pub async fn send_maestro_tx_successful(
        &self,
        tx: &EthereumTxEnvelope<TxEip4844Variant>,
    ) -> Result<TxHash> {
        let client = reqwest::Client::new();
        let encoded_tx = tx.encoded_2718();
        let tx_hex = hex::encode_prefixed(encoded_tx);
        info!("tx_hex: {}", tx_hex);
        let response = client
            .post(self.maestro_url.clone())
            .header(HEADER_CHAIN_ID, self.chain_id.to_string())
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": [tx_hex]
            }))
            .send()
            .await?;
        assert!(response.status().is_success(), "EIP-1559 transaction request failed");
        let json_resp: Value = response.json().await?;

        assert!(
            json_resp.get("result").is_some(),
            "Transaction response missing 'result' field: {}",
            json_resp
        );
        let result = json_resp.get("result").unwrap().as_str().unwrap();
        let tx_hash = TxHash::from_str(result).unwrap();

        Ok(tx_hash)
    }

    /// Use this instead of `send_maestro_tx_successful()` to inspect the JSON `error` field
    #[allow(clippy::unwrap_used)]
    pub async fn send_maestro_tx_could_be_unsuccessful(
        &self,
        tx: &EthereumTxEnvelope<TxEip4844Variant>,
    ) -> Result<Value> {
        let client = reqwest::Client::new();
        let encoded_tx = tx.encoded_2718();
        let tx_hex = hex::encode_prefixed(encoded_tx);
        info!("tx_hex: {}", tx_hex);
        let response = client
            .post(self.maestro_url.clone())
            .header(HEADER_CHAIN_ID, self.chain_id.to_string())
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_sendRawTransaction",
                "params": [tx_hex]
            }))
            .send()
            .await?;
        // assert!(response.status().is_success(), "EIP-1559 transaction request failed");
        let json_resp: Value = response.json().await?;
        Ok(json_resp)
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
