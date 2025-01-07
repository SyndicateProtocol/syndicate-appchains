use crate::contract_bindings::metabasedsequencerchain::MetabasedSequencerChain;
use alloy::{
    hex,
    network::{Ethereum, EthereumWallet},
    primitives::Address,
    providers::{
        fillers::{FillProvider, JoinFill, WalletFiller},
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    signers::{k256::SecretKey, utils::public_key_to_address, Signer},
    transports::http::Http,
};
use eyre::{eyre, Error};
use reqwest::{Client, Url};

// NOTE: to run these tests, the `e2e-tests` feature must be enabled, like so:
// `cargo test --features e2e-tests`

// NOTE: all these env vars should be in place when running tests
const ENV_BOB_PRIVATE_KEY: &str = "BOB_PRIVATE_KEY";
const ENV_ALICE_PRIVATE_KEY: &str = "ALICE_PRIVATE_KEY";
const ENV_SEQUENCER_PRIVATE_KEY: &str = "SEQUENCER_PRIVATE_KEY";

// specified in .envrc
const ENV_SETTLEMENT_CHAIN_RPC_URL: &str = "SETTLEMENT_CHAIN_RPC_URL";
const ENV_SEQUENCING_CHAIN_RPC_URL: &str = "SEQUENCING_CHAIN_RPC_URL";
const ENV_META_BASED_CHAIN_RPC_URL: &str = "META_BASED_CHAIN_RPC_URL";
const ENV_ROLLUP_TYPE: &str = "ROLLUP_TYPE";
const ENV_METABASED_CHAIN_CONTRACT_ADDRESS: &str = "METABASED_SEQUENCER_CHAIN_CONTRACT_ADDRESS";

#[derive(Debug)]
pub struct TestEnvConfig {
    pub rollup_type: RollupType,
    pub settlement_rpc: Url,
    pub sequencing_rpc: Url,
    pub l3_rpc: Url,
    pub metabased_chain_contract_address: Address,
}

impl TestEnvConfig {
    pub fn from_env() -> Result<Self, Error> {
        // TODO TODO (SEQ-435)
        Ok(Self {
            rollup_type: RollupType::from_env()?,
            settlement_rpc: Url::parse(&get_env_var(ENV_SETTLEMENT_CHAIN_RPC_URL)?).map_err(
                |e| {
                    EnvConfigError::InvalidRpcUrl(
                        ENV_SETTLEMENT_CHAIN_RPC_URL.to_string(),
                        e.to_string(),
                    )
                },
            )?,
            sequencing_rpc: Url::parse(&get_env_var(ENV_SEQUENCING_CHAIN_RPC_URL)?).map_err(
                |e| {
                    EnvConfigError::InvalidRpcUrl(
                        ENV_SEQUENCING_CHAIN_RPC_URL.to_string(),
                        e.to_string(),
                    )
                },
            )?,
            l3_rpc: Url::parse(&get_env_var(ENV_META_BASED_CHAIN_RPC_URL)?).map_err(|e| {
                EnvConfigError::InvalidRpcUrl(
                    ENV_META_BASED_CHAIN_RPC_URL.to_string(),
                    e.to_string(),
                )
            })?,
            metabased_chain_contract_address: get_env_var(ENV_METABASED_CHAIN_CONTRACT_ADDRESS)?
                .parse::<Address>()
                .map_err(|e| {
                    EnvConfigError::InvalidAddress(
                        ENV_METABASED_CHAIN_CONTRACT_ADDRESS.to_string(),
                        e.to_string(),
                    )
                })?,
        })
    }
}

fn get_env_var(env_var: &str) -> Result<String, Error> {
    std::env::var(env_var).map_err(|_| eyre!(EnvConfigError::MissingEnvVar(env_var.to_string())))
}

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

#[derive(Debug)]
pub enum RollupType {
    Optimism,
    Arbitrum,
}

impl RollupType {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self::try_from(&get_env_var(ENV_ROLLUP_TYPE)?)?)
    }
}

impl TryFrom<&String> for RollupType {
    type Error = EnvConfigError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "OP" => Ok(Self::Optimism),
            "ARB" => Ok(Self::Arbitrum),
            other => Err(EnvConfigError::InvalidRollupType(other.to_string())),
        }
    }
}

type ProviderWithWallet = FillProvider<
    JoinFill<Identity, WalletFiller<EthereumWallet>>,
    RootProvider<Http<reqwest::Client>>,
    Http<reqwest::Client>,
    Ethereum,
>;

pub fn wallet_from_private_key(private_key: &SecretKey, chain_id: u64) -> EthereumWallet {
    let signer =
        alloy_signer_local::LocalSigner::from(private_key.clone()).with_chain_id(Some(chain_id));
    EthereumWallet::from(signer)
}

fn provider_with_wallet(url: &Url, private_key: &SecretKey, chain_id: u64) -> ProviderWithWallet {
    let signer = wallet_from_private_key(private_key, chain_id);
    ProviderBuilder::new().wallet(signer).on_http(url.clone())
}

fn account_from_env(env_var: &str) -> Result<Account, Error> {
    let private_key_str = std::env::var(env_var)?;
    let key_str = private_key_str
        .split("0x")
        .last()
        .ok_or(EnvConfigError::InvalidPrivateKey(
            env_var.to_string(),
            private_key_str.clone(),
        ))?;
    let key_hex = hex::decode(key_str).map_err(|_| {
        EnvConfigError::InvalidPrivateKey(env_var.to_string(), private_key_str.clone())
    })?;
    let private_key = SecretKey::from_bytes((&key_hex[..]).into()).map_err(|_| {
        EnvConfigError::InvalidPrivateKey(env_var.to_string(), private_key_str.clone())
    })?;
    let address = public_key_to_address(&(private_key.public_key().into()));

    Ok(Account {
        private_key,
        address,
    })
}

#[derive(Debug)]
pub struct Account {
    pub private_key: SecretKey,
    pub address: Address,
}

#[derive(Debug)]
pub struct Accounts {
    pub bob: Account,
    pub alice: Account,
    pub sequencer: Account,
}

type SequencingContractInstance = MetabasedSequencerChain::MetabasedSequencerChainInstance<
    Http<Client>,
    FillProvider<
        JoinFill<Identity, WalletFiller<EthereumWallet>>,
        RootProvider<Http<Client>>,
        Http<Client>,
        Ethereum,
    >,
>;

type HttpProvider = RootProvider<Http<reqwest::Client>>;

#[derive(Debug)]
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
    pub async fn new() -> Result<Self, Error> {
        let config = TestEnvConfig::from_env()?;

        let settlement_chain = ProviderBuilder::new().on_http(config.settlement_rpc.clone());
        let sequencing_chain = ProviderBuilder::new().on_http(config.sequencing_rpc.clone());
        let l3_chain = ProviderBuilder::new().on_http(config.l3_rpc.clone());

        let settlement_chain_id = settlement_chain.get_chain_id().await?;
        let sequencing_chain_id = sequencing_chain.get_chain_id().await?;
        let l3_chain_id = l3_chain.get_chain_id().await?;

        let accounts = Accounts {
            bob: account_from_env(ENV_BOB_PRIVATE_KEY)?,
            alice: account_from_env(ENV_ALICE_PRIVATE_KEY)?,
            sequencer: account_from_env(ENV_SEQUENCER_PRIVATE_KEY)?,
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

    // Helper to check if all connections are alive
    async fn check_connections(&self) -> Result<(), Error> {
        for provider in [
            &self.settlement_chain,
            &self.sequencing_chain,
            &self.l3_chain,
        ] {
            provider.get_block_number().await?;
        }
        Ok(())
    }

    // raw_tx is the L3 tx to be sequenced
    pub async fn sequence_tx(&self, raw_tx: alloy::sol_types::private::Bytes) -> Result<(), Error> {
        self.sequencing_contract
            .processTransactionRaw(raw_tx)
            .send()
            .await?
            .watch()
            .await?;
        Ok(())
    }

    // Getters

    pub const fn settlement_chain(&self) -> &HttpProvider {
        &self.settlement_chain
    }

    pub const fn sequencing_chain(&self) -> &HttpProvider {
        &self.sequencing_chain
    }

    pub const fn l3_chain(&self) -> &HttpProvider {
        &self.l3_chain
    }

    pub const fn settlement_chain_id(&self) -> u64 {
        self.settlement_chain_id
    }

    pub const fn sequencing_chain_id(&self) -> u64 {
        self.sequencing_chain_id
    }

    pub const fn l3_chain_id(&self) -> u64 {
        self.l3_chain_id
    }

    pub const fn accounts(&self) -> &Accounts {
        &self.accounts
    }
}
