//! Preloaded utils for the integration tests

use alloy::primitives::{address, Address};

const PRELOAD_INBOX_ADDRESS_300: Address = address!("0x26eE2349212255614edCc046DD9472F2a5b7EF2b");
const PRELOAD_BRIDGE_ADDRESS_300: Address = address!("0xa0e810a42086da4Ebc5C49fEd626cA6A75B06437");
const PRELOAD_ASSERTION_POSTER_ADDRESS_300: Address =
    address!("0x67d269191c92Caf3cD7723F116c85e6E9bf55933");

const PRELOAD_INBOX_ADDRESS_231: Address = address!("0x7e2d5FCC5E02cBF2b9f860052C0226104E23F9c7");
const PRELOAD_BRIDGE_ADDRESS_231: Address = address!("0x8dAF17A20c9DBA35f005b6324F493785D239719d");
const PRELOAD_ASSERTION_POSTER_ADDRESS_231: Address =
    address!("0x09635F643e140090A9A8Dcd712eD6285858ceBef");

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum ContractVersion {
    V213,
    V300,
}

pub const fn get_anvil_file(version: &ContractVersion) -> &str {
    match version {
        ContractVersion::V300 => "anvil_300.json",
        ContractVersion::V213 => "anvil_213.json",
    }
}

pub const fn get_assertion_poster_address(version: &ContractVersion) -> Address {
    match version {
        ContractVersion::V213 => PRELOAD_ASSERTION_POSTER_ADDRESS_231,
        ContractVersion::V300 => PRELOAD_ASSERTION_POSTER_ADDRESS_300,
    }
}

pub const fn get_inbox_address(version: &ContractVersion) -> Address {
    match version {
        ContractVersion::V213 => PRELOAD_INBOX_ADDRESS_231,
        ContractVersion::V300 => PRELOAD_INBOX_ADDRESS_300,
    }
}

pub const fn get_bridge_address(version: &ContractVersion) -> Address {
    match version {
        ContractVersion::V213 => PRELOAD_BRIDGE_ADDRESS_231,
        ContractVersion::V300 => PRELOAD_BRIDGE_ADDRESS_300,
    }
}
