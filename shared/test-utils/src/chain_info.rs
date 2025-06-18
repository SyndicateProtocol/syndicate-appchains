use crate::docker::E2EProcess;
use alloy::{node_bindings::AnvilInstance, signers::local::PrivateKeySigner};
use shared::types::FilledProvider;
use std::{future, process::ExitStatus, str::FromStr};

#[derive(Debug)]
pub struct ChainInfo {
    pub ws_url: String,
    pub http_url: String,
    pub instance: ProcessInstance,
    pub provider: FilledProvider,
}

#[derive(Debug)]
pub enum ProcessInstance {
    Anvil(AnvilInstance),
    Docker(E2EProcess),
}

impl ProcessInstance {
    pub async fn wait(&mut self) -> std::io::Result<ExitStatus> {
        match self {
            ProcessInstance::Docker(docker) => docker.wait().await,
            ProcessInstance::Anvil(_) => {
                // We don't have a way to wait for Anvil to exit, so we pend forever.
                // If Anvil exits, the test will fail on a subsequent provider call.
                future::pending().await
            }
        }
    }
}

// anvil defaultaccount 1 - used on all chain providers (including nitro)
pub const PRIVATE_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
// anvil defaultaccount 2
pub const PRIVATE_KEY2: &str = "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
// anvil defaultaccount 2
pub const PRIVATE_KEY3: &str = "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

pub fn default_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str(PRIVATE_KEY)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}

pub fn default_signer2() -> PrivateKeySigner {
    PrivateKeySigner::from_str(PRIVATE_KEY2)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}

pub fn default_signer3() -> PrivateKeySigner {
    PrivateKeySigner::from_str(PRIVATE_KEY3)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}
