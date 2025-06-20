use crate::docker::E2EProcess;
use alloy::{node_bindings::AnvilInstance, primitives::Address, signers::local::PrivateKeySigner};
use shared::types::FilledProvider;
use std::{future, process::ExitStatus, str::FromStr, sync::OnceLock};

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

// anvil defaultaccount 1 - all chain providers on E2E tests come with this account signer
// address: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
pub const PRIVATE_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

// anvil defaultaccount 2
// address: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
pub const PRIVATE_KEY2: &str = "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";

// anvil defaultaccount 3
// address: 0x90F79bf6EB2c4f870365E785982E1f101E93b906
pub const PRIVATE_KEY3: &str = "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

#[derive(Debug)]
pub struct AccountInfo {
    pub address: Address,
    pub private_key: &'static str,
    pub signer: PrivateKeySigner,
}

fn account_info_from_private_key(private_key: &'static str) -> AccountInfo {
    let signer = PrivateKeySigner::from_str(private_key)
        .unwrap_or_else(|err| panic!("Invalid private key: {}", err));
    AccountInfo { address: signer.address(), private_key, signer }
}

static TEST_ACCOUNT_1: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_2: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_3: OnceLock<AccountInfo> = OnceLock::new();

pub fn test_account1() -> &'static AccountInfo {
    TEST_ACCOUNT_1.get_or_init(|| account_info_from_private_key(PRIVATE_KEY))
}

pub fn test_account2() -> &'static AccountInfo {
    TEST_ACCOUNT_2.get_or_init(|| account_info_from_private_key(PRIVATE_KEY2))
}

pub fn test_account3() -> &'static AccountInfo {
    TEST_ACCOUNT_3.get_or_init(|| account_info_from_private_key(PRIVATE_KEY3))
}
