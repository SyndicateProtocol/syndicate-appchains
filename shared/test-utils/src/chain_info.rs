use crate::docker::E2EProcess;
use alloy::{node_bindings::AnvilInstance, signers::local::PrivateKeySigner};
use shared::types::FilledProvider;
use std::{future, process::ExitStatus, str::FromStr};

#[derive(Debug)]
pub struct ChainInfo {
    pub ws_url: String,
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

pub fn default_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str(PRIVATE_KEY)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}
