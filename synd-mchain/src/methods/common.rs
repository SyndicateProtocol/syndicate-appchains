//! Shared functions and constants for the `synd-mchain` RPC server methods

use alloy::primitives::{address, Address, FixedBytes, U256};
use jsonrpsee::{
    server::SubscriptionSink,
    types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned},
};
use std::collections::VecDeque;

/// The chain id of the mockchain. This is the same for all appchains.
pub const MCHAIN_ID: u64 = 510;

/// The address of the Appchain contract
pub const APPCHAIN_CONTRACT: Address = address!("0x0000000000000000000000000000000000000510");

/// Helper function to create an error object
pub fn err(message: &'static str) -> ErrorObjectOwned {
    ErrorObjectOwned::borrowed(INTERNAL_ERROR_CODE, message, None)
}

/// Helper function to create a mock log object
pub fn create_log(block_num: u64, data: alloy::primitives::LogData) -> alloy::rpc::types::Log {
    alloy::rpc::types::Log {
        inner: alloy::primitives::Log { address: APPCHAIN_CONTRACT, data },
        transaction_hash: Some(FixedBytes::ZERO),
        block_number: Some(block_num),
        block_hash: Some(U256::from(block_num).into()),
        ..Default::default()
    }
}

/// Helper function to create a mock header object
pub fn create_header(
    block_num: u64,
    l1_block_num: u64,
    timestamp: u64,
) -> alloy::rpc::types::Header {
    alloy::rpc::types::Header {
        inner: alloy::consensus::Header {
            number: block_num,
            base_fee_per_gas: Some(1),
            extra_data: FixedBytes::<32>::ZERO.into(),
            #[allow(clippy::unwrap_used)]
            mix_hash: U256::from(l1_block_num)
                .checked_shl(64)
                .unwrap()
                .checked_add(U256::from(1))
                .unwrap()
                .checked_shl(64)
                .unwrap()
                .into(),
            timestamp,
            difficulty: U256::ONE,
            ..Default::default()
        },
        hash: U256::from(block_num).into(),
        ..Default::default()
    }
}

/// Return the on-chain config for an appchain with a given `chain_id`
pub fn appchain_config(chain_id: u64) -> String {
    let mut cfg = format!(
        r#"{{
            "chainId": {chain_id},
            "homesteadBlock": 0,
            "daoForkBlock": null,
            "daoForkSupport": true,
            "eip150Block": 0,
            "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "eip155Block": 0,
            "eip158Block": 0,
            "byzantiumBlock": 0,
            "constantinopleBlock": 0,
            "petersburgBlock": 0,
            "istanbulBlock": 0,
            "muirGlacierBlock": 0,
            "berlinBlock": 0,
            "londonBlock": 0,
            "clique": {{
            "period": 0,
            "epoch": 0
            }},
            "arbitrum": {{
            "EnableArbOS": true,
            "GenesisBlockNum": 0
            }}
        }}"#
    );
    cfg.retain(|c| !c.is_whitespace());
    cfg.shrink_to_fit();
    cfg
}

/// The context of the mockchain
#[derive(Debug)]
pub struct Context {
    /// The finalized block number
    pub finalized_block: u64,
    /// The pending timestamps
    pub pending_ts: VecDeque<u64>,
    /// The subscriptions
    pub subs: Vec<SubscriptionSink>,
}

#[cfg(test)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) mod test_utils {
    use std::{sync::RwLock, time::UNIX_EPOCH};

    pub(crate) struct SystemTime(RwLock<u64>);
    pub(crate) static TIME: SystemTime = SystemTime(RwLock::new(0));
    impl SystemTime {
        pub(crate) fn now() -> &'static Self {
            &TIME
        }
        pub(crate) fn duration_since(&self, from: std::time::SystemTime) -> Option<&Self> {
            assert_eq!(from, UNIX_EPOCH);
            Some(&TIME)
        }
        pub(crate) fn as_secs(&self) -> u64 {
            *self.0.read().unwrap()
        }
        pub(crate) fn set(&self, secs: u64) {
            *self.0.write().unwrap() = secs;
        }
    }
}
