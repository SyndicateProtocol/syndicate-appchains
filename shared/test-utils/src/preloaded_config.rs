//! Preloaded utils for the integration tests

use alloy::{
    primitives::{address, Address},
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use std::str::FromStr;

pub const PRELOAD_INBOX_ADDRESS_300: Address =
    address!("0x26eE2349212255614edCc046DD9472F2a5b7EF2b");
pub const PRELOAD_BRIDGE_ADDRESS_300: Address =
    address!("0xa0e810a42086da4Ebc5C49fEd626cA6A75B06437");
pub const PRELOAD_POSTER_ADDRESS_300: Address =
    address!("0x67d269191c92Caf3cD7723F116c85e6E9bf55933");

pub const PRELOAD_INBOX_ADDRESS_231: Address =
    address!("0x7e2d5FCC5E02cBF2b9f860052C0226104E23F9c7");
pub const PRELOAD_BRIDGE_ADDRESS_231: Address =
    address!("0x8dAF17A20c9DBA35f005b6324F493785D239719d");
pub const PRELOAD_POSTER_ADDRESS_231: Address =
    address!("0x09635F643e140090A9A8Dcd712eD6285858ceBef");

pub const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // address = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
pub const APPCHAIN_OWNER: Address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

pub const APPCHAIN_CHAIN_ID: u64 = 13331370;

pub fn get_default_private_key_signer() -> LocalSigner<SigningKey> {
    PrivateKeySigner::from_str(DEFAULT_PRIVATE_KEY_SIGNER)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}
