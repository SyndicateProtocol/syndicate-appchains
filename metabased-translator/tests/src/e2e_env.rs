//! This module contains the logic for setting up the test environment for e2e tests

use alloy::{
    hex,
    network::EthereumWallet,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    signers::{k256::SecretKey, local::LocalSigner, utils::public_key_to_address, Signer},
    sol_types::private::Bytes,
};
use block_builder::connectors::mchain::{FilledProvider, HttpProvider};
use contract_bindings::metabased::metabasedsequencerchain::MetabasedSequencerChain;
use eyre::{eyre, Error};
use reqwest::Url;
use std::str::FromStr;
use strum_macros::EnumString;

// NOTE: to run these tests, the `e2e-tests` feature must be enabled, like so:
// `cargo test --features e2e-tests`

// NOTE: all these env vars should be in place when running tests
const ENV_BOB_PRIVATE_KEY: &str = "BOB_PRIVATE_KEY";
const ENV_ALICE_PRIVATE_KEY: &str = "ALICE_PRIVATE_KEY";
const ENV_SEQUENCER_PRIVATE_KEY: &str = "SEQUENCER_PRIVATE_KEY";

// specified in .envrc
const ENV_SETTLEMENT_RPC_URL: &str = "SETTLEMENT_RPC_URL";
const ENV_SEQUENCING_RPC_URL: &str = "SEQUENCING_RPC_URL";
const ENV_METABASED_RPC_URL: &str = "METABASED_RPC_URL";
const ENV_ROLLUP_TYPE: &str = "ROLLUP_TYPE";
const ENV_CHAIN_CONTRACT_ADDRESS: &str = "SEQUENCING_CONTRACT_ADDRESS";

#[derive(Debug)]
struct TestEnvConfig {
    #[allow(unused)]
    // NOTE: the rollup_type is not used yet, but should be necessary in the future
    rollup_type: RollupType,
    settlement_rpc: Url,
    sequencing_rpc: Url,
    l3_rpc: Url,
    metabased_chain_contract_address: Address,
}

impl TestEnvConfig {
    fn from_env() -> Result<Self, Error> {
        Ok(Self {
            rollup_type: RollupType::from_env()?,
            settlement_rpc: Url::from_env_var(ENV_SETTLEMENT_RPC_URL)?,
            sequencing_rpc: Url::from_env_var(ENV_SEQUENCING_RPC_URL)?,
            l3_rpc: Url::from_env_var(ENV_METABASED_RPC_URL)?,
            metabased_chain_contract_address: Address::from_env_var(ENV_CHAIN_CONTRACT_ADDRESS)?,
        })
    }
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, thiserror::Error)]
pub enum EnvConfigError {
    #[error("Missing environment variable: '{0}'")]
    MissingEnvVar(String),
    #[error("Invalid rollup type: '{0}'. Must be 'OP' or 'ARB'")]
    InvalidRollupType(String),
    #[error("Invalid private key for {0}: '{1}'")]
    InvalidPrivateKey(String, String),
    #[error("Invalid chain id for {0}: '{1}'")]
    InvalidChainId(String, String),
    #[error("Invalid RPC URL for {0}: '{1}'")]
    InvalidRpcUrl(String, String),
    #[error("Invalid address for {0}: '{1}'")]
    InvalidAddress(String, String),
}

fn get_env_var(env_var: &str) -> Result<String, Error> {
    std::env::var(env_var).map_err(|_| eyre!(EnvConfigError::MissingEnvVar(env_var.to_string())))
}

trait FromEnvVar: Sized {
    fn from_env_var(var_name: &str) -> Result<Self, Error>;
}

impl FromEnvVar for Url {
    fn from_env_var(var_name: &str) -> Result<Self, Error> {
        Self::parse(&get_env_var(var_name)?)
            .map_err(|e| eyre!(EnvConfigError::InvalidRpcUrl(var_name.to_string(), e.to_string())))
    }
}

impl FromEnvVar for Address {
    fn from_env_var(var_name: &str) -> Result<Self, Error> {
        let addr_str = get_env_var(var_name)?;
        addr_str
            .parse::<Self>()
            .map_err(|e| eyre!(EnvConfigError::InvalidAddress(var_name.to_string(), e.to_string())))
    }
}

impl FromEnvVar for Account {
    fn from_env_var(var_name: &str) -> Result<Self, Error> {
        let private_key_str = get_env_var(var_name)?;
        let key_str = private_key_str.split("0x").last().ok_or_else(|| {
            EnvConfigError::InvalidPrivateKey(var_name.to_string(), private_key_str.clone())
        })?;
        let key_hex = hex::decode(key_str).map_err(|_| {
            EnvConfigError::InvalidPrivateKey(var_name.to_string(), private_key_str.clone())
        })?;
        let private_key = SecretKey::from_bytes((&key_hex[..]).into()).map_err(|_| {
            EnvConfigError::InvalidPrivateKey(var_name.to_string(), private_key_str.clone())
        })?;
        let address = public_key_to_address(&(private_key.public_key().into()));

        Ok(Self { private_key, address })
    }
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, EnumString, PartialEq, Eq)]
pub enum RollupType {
    #[strum(serialize = "OP")]
    Optimism,
    #[strum(serialize = "ARB")]
    Arbitrum,
}

impl RollupType {
    fn from_env() -> Result<Self, Error> {
        Self::from_str(&get_env_var(ENV_ROLLUP_TYPE)?)
            .map_err(|e| eyre!(EnvConfigError::InvalidRollupType(e.to_string())))
    }
}

/// Creates a wallet from a private key and a chain ID
pub fn wallet_from_private_key(private_key: &SecretKey, chain_id: u64) -> EthereumWallet {
    let signer = LocalSigner::from(private_key.clone()).with_chain_id(Some(chain_id));
    EthereumWallet::from(signer)
}

fn provider_with_wallet(url: &Url, private_key: &SecretKey, chain_id: u64) -> FilledProvider {
    let signer = wallet_from_private_key(private_key, chain_id);
    ProviderBuilder::new().wallet(signer).on_http(url.clone())
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug)]
pub struct Account {
    pub private_key: SecretKey,
    pub address: Address,
}

/// A collection of test accounts
#[allow(missing_docs)] // self-explanatory
#[derive(Debug)]
pub struct Accounts {
    pub bob: Account,
    pub alice: Account,
    pub sequencer: Account,
}

type SequencingContractInstance =
    MetabasedSequencerChain::MetabasedSequencerChainInstance<(), FilledProvider>;

#[derive(Debug)]
/// The test environment - contains all the providers and accounts necessary to write e2e tests
pub struct TestEnv {
    settlement_chain: HttpProvider,
    sequencing_chain: HttpProvider,
    l3_chain: HttpProvider,

    settlement_chain_id: u64,
    sequencing_chain_id: u64,
    l3_chain_id: u64,

    accounts: Accounts,

    sequencing_contract: SequencingContractInstance,
}

impl TestEnv {
    /// Creates a new E2E test environment
    pub async fn new() -> Result<Self, Error> {
        let config = TestEnvConfig::from_env()?;

        let settlement_chain = ProviderBuilder::new().on_http(config.settlement_rpc.clone());
        let sequencing_chain = ProviderBuilder::new().on_http(config.sequencing_rpc.clone());
        let l3_chain = ProviderBuilder::new().on_http(config.l3_rpc.clone());

        let settlement_chain_id = settlement_chain.get_chain_id().await?;
        let sequencing_chain_id = sequencing_chain.get_chain_id().await?;
        let l3_chain_id = l3_chain.get_chain_id().await?;

        let accounts = Accounts {
            bob: Account::from_env_var(ENV_BOB_PRIVATE_KEY)?,
            alice: Account::from_env_var(ENV_ALICE_PRIVATE_KEY)?,
            sequencer: Account::from_env_var(ENV_SEQUENCER_PRIVATE_KEY)?,
        };

        let sequencing_contract = MetabasedSequencerChain::new(
            config.metabased_chain_contract_address,
            provider_with_wallet(
                &config.sequencing_rpc,
                &accounts.sequencer.private_key,
                sequencing_chain_id,
            ),
        );

        let env = Self {
            settlement_chain,
            sequencing_chain,
            l3_chain,
            settlement_chain_id,
            sequencing_chain_id,
            l3_chain_id,
            accounts,
            sequencing_contract,
        };

        env.check_connections().await?;

        Ok(env)
    }

    async fn check_connections(&self) -> Result<(), Error> {
        for provider in [&self.settlement_chain, &self.sequencing_chain, &self.l3_chain] {
            provider.get_block_number().await?;
        }
        Ok(())
    }

    /// Submits a raw L3 transaction to be sequenced
    pub async fn sequence_tx(&self, raw_tx: Bytes) -> Result<(), Error> {
        self.sequencing_contract.processTransactionRaw(raw_tx).send().await?.watch().await?;
        Ok(())
    }

    //
    // Getters

    /// Returns a reference to the settlement chain provider
    pub const fn settlement_chain(&self) -> &HttpProvider {
        &self.settlement_chain
    }

    /// Returns a reference to the sequencing chain provider
    pub const fn sequencing_chain(&self) -> &HttpProvider {
        &self.sequencing_chain
    }

    /// Returns a reference to the L3 chain provider
    pub const fn l3_chain(&self) -> &HttpProvider {
        &self.l3_chain
    }

    /// Returns the settlement chain ID
    pub const fn settlement_chain_id(&self) -> u64 {
        self.settlement_chain_id
    }

    /// Returns the sequencing chain ID
    pub const fn sequencing_chain_id(&self) -> u64 {
        self.sequencing_chain_id
    }

    /// Returns the L3 chain ID
    pub const fn l3_chain_id(&self) -> u64 {
        self.l3_chain_id
    }

    /// Returns a reference to the test accounts
    pub const fn accounts(&self) -> &Accounts {
        &self.accounts
    }
}
