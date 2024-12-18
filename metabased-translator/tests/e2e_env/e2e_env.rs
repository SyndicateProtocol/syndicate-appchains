use ethers::providers::{Http, Provider};
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub enum RollupType {
    Optimism,
    Arbitrum,
}

impl RollupType {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        match std::env::var("ROLLUP_TYPE").as_deref() {
            Ok("OP") => Ok(RollupType::Optimism),
            Ok("ARB") => Ok(RollupType::Arbitrum),
            Ok(other) => {
                Err(format!("Invalid ROLLUP_TYPE: {}. Must be 'OP' or 'ARB'", other).into())
            }
            Err(_) => Err("ROLLUP_TYPE environment variable not set".into()),
        }
    }
}

pub struct ChainConnections {
    pub settlement: Arc<Provider<Http>>,
    pub sequencing: Arc<Provider<Http>>,
}

impl ChainConnections {
    pub fn new(
        settlement_rpc: &str,
        sequencing_rpc: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            settlement: Arc::new(Provider::<Http>::try_from(settlement_rpc)?),
            sequencing: Arc::new(Provider::<Http>::try_from(sequencing_rpc)?),
        })
    }

    // Helper to check if all connections are alive
    pub async fn check_connections(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tasks = Vec::new();

        for provider in [&self.settlement, &self.sequencing, &self.metabase] {
            tasks.push(provider.get_block_number());
        }

        futures::future::try_join_all(tasks).await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct TestEnvConfig {
    pub rollup_type: RollupType,
    pub settlement_rpc: Option<String>,
    pub sequencing_rpc: Option<String>,
}

impl TestEnvConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            rollup_type: RollupType::from_env()?,
            settlement_rpc: std::env::var("SETTLEMENT_JSON_RPC_URL").ok(),
            sequencing_rpc: std::env::var("SEQUENCING_JSON_RPC_URL").ok(),
        })
    }
}

pub struct TestEnv {
    pub chains: ChainConnections,
    pub rollup_type: RollupType,
    settlement_anvil: Option<AnvilHandle>,
    sequencing_anvil: Option<AnvilHandle>,
}

impl TestEnv {
    pub async fn new(config: TestEnvConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let (settlement_url, settlement_anvil) =
            Self::get_or_create_rpc(config.settlement_rpc).await?;
        let (sequencing_url, sequencing_anvil) =
            Self::get_or_create_rpc(config.sequencing_rpc).await?;

        let chains = ChainConnections::new(&settlement_url, &sequencing_url)?;

        Ok(Self {
            chains,
            rollup_type: config.rollup_type,
            settlement_anvil,
            sequencing_anvil,
        })
    }

    async fn get_or_create_rpc(
        maybe_url: Option<String>,
    ) -> Result<(String, Option<AnvilHandle>), Box<dyn std::error::Error>> {
        match maybe_url {
            Some(url) => Ok((url, None)),
            None => {
                let anvil = AnvilHandle::new().await?;
                let url = anvil.endpoint();
                Ok((url, Some(anvil)))
            }
        }
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        if let Some(handle) = self.settlement_anvil.take() {
            handle.kill();
        }
        if let Some(handle) = self.sequencing_anvil.take() {
            handle.kill();
        }
    }
}
