//! The `types` module handles types for the metabased poster.

use alloy::primitives::{B256, U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct NitroBlock {
    pub number: U256,
    pub l1_block_number: U256,
    pub timestamp: U256,
    pub hash: B256,
    pub send_root: B256,
}
