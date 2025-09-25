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

// anvil defaulta ccounts

// address: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
pub const PRIVATE_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

// address: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
pub const PRIVATE_KEY2: &str = "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";

// address: 0x90F79bf6EB2c4f870365E785982E1f101E93b906
pub const PRIVATE_KEY3: &str = "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

// address: 0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65
pub const PRIVATE_KEY4: &str = "0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a";

// address:  0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc
pub const PRIVATE_KEY5: &str = "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";

// address:  0x976EA74026E726554dB657fA54763abd0C3a0aa9
pub const PRIVATE_KEY6: &str = "0x92db14e403b83dfe3df233f83dfa3a0d7096f21ca9b0d6d6b8d88b2b4ec1564e";

// address:  0x14dC79964da2C08b23698B3D3cc7Ca32193d9955
pub const PRIVATE_KEY7: &str = "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

// address:  0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f
pub const PRIVATE_KEY8: &str = "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97";

// address:  0xa0Ee7A142d267C1f36714E4a8F75612F20a79720
pub const PRIVATE_KEY9: &str = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

#[derive(Debug)]
pub struct AccountInfo {
    pub address: Address,
    pub private_key: &'static str,
    pub signer: PrivateKeySigner,
}

fn account_info_from_private_key(private_key: &'static str) -> AccountInfo {
    let signer = PrivateKeySigner::from_str(private_key)
        .unwrap_or_else(|err| panic!("Invalid private key: {err}"));
    AccountInfo { address: signer.address(), private_key, signer }
}

static TEST_ACCOUNT_1: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_2: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_3: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_4: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_5: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_6: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_7: OnceLock<AccountInfo> = OnceLock::new();
// NOTE: 8 and 9 are used as the batch posters for the source chains on E2E tests
static TEST_ACCOUNT_8: OnceLock<AccountInfo> = OnceLock::new();
static TEST_ACCOUNT_9: OnceLock<AccountInfo> = OnceLock::new();

pub fn test_account1() -> &'static AccountInfo {
    TEST_ACCOUNT_1.get_or_init(|| account_info_from_private_key(PRIVATE_KEY))
}

pub fn test_account2() -> &'static AccountInfo {
    TEST_ACCOUNT_2.get_or_init(|| account_info_from_private_key(PRIVATE_KEY2))
}

pub fn test_account3() -> &'static AccountInfo {
    TEST_ACCOUNT_3.get_or_init(|| account_info_from_private_key(PRIVATE_KEY3))
}

pub fn test_account4() -> &'static AccountInfo {
    TEST_ACCOUNT_4.get_or_init(|| account_info_from_private_key(PRIVATE_KEY4))
}

pub fn test_account5() -> &'static AccountInfo {
    TEST_ACCOUNT_5.get_or_init(|| account_info_from_private_key(PRIVATE_KEY5))
}

pub fn test_account6() -> &'static AccountInfo {
    TEST_ACCOUNT_6.get_or_init(|| account_info_from_private_key(PRIVATE_KEY6))
}

pub fn test_account7() -> &'static AccountInfo {
    TEST_ACCOUNT_7.get_or_init(|| account_info_from_private_key(PRIVATE_KEY7))
}

pub fn test_account8() -> &'static AccountInfo {
    TEST_ACCOUNT_8.get_or_init(|| account_info_from_private_key(PRIVATE_KEY8))
}

pub fn test_account9() -> &'static AccountInfo {
    TEST_ACCOUNT_9.get_or_init(|| account_info_from_private_key(PRIVATE_KEY9))
}
