//! Module to store unique Valkey key formats in order to improve visibility and avoid collisions

/// Valkey key to retrieve Wallet Nonce values
pub mod wallet_nonce {
    use alloy::primitives::{Address, ChainId};

    /// Unique prefix of Wallet Nonce key for Valkey String retrieval
    pub const WALLET_NONCE_KEY_PREFIX: &str = "maestro:wallet-nonce";

    /// Generates a Valkey String key for a specific chain and wallet address. Note that such a
    /// combination is unique per chain
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to create the key for
    /// * `wallet_address` - The wallet address to create the key for
    ///
    /// # Returns
    /// A string in the format `synd-maestro:wallet-nonce:{chain_id}_{wallet_address}`
    pub fn chain_wallet_nonce_key(
        chain_id: ChainId,
        wallet_address: Address,
    ) -> ChainWalletNonceKey {
        let chain_wallet_composite_key = format!("{chain_id}_{wallet_address}");
        format!("{WALLET_NONCE_KEY_PREFIX}:{chain_wallet_composite_key}")
    }

    /// Key produced by `chain_wallet_nonce_key()` above
    pub type ChainWalletNonceKey = String;
}

/// Valkey keys to retrieve Waiting Transaction values
pub mod waiting_txn {
    use alloy::primitives::{Address, ChainId};

    /// Unique prefix of Waiting Gap Txns key for Valkey String retrieval
    pub const WAITING_GAP_KEY_PREFIX: &str = "maestro:waiting-gap-txns";

    /// Generates a Valkey String key for a specific chain, wallet, and nonce, thereby corresponding
    /// to a unique transaciton.
    ///
    /// # Arguments
    /// * `chain_id` - The chain identifier to create the key for
    /// * `wallet_address` - The wallet address to create the key for
    /// * `nonce` - The transaction nonce
    ///
    /// # Returns
    /// A string in the format `synd-maestro:waiting-gap-txns:{chain_id}_{wallet_address}_{nonce}`
    pub fn waiting_gap_txns_key(chain_id: ChainId, wallet_address: Address, nonce: u64) -> String {
        let waiting_gap_composite_key = format!("{chain_id}_{wallet_address}_{nonce}");
        format!("{WAITING_GAP_KEY_PREFIX}:{waiting_gap_composite_key}")
    }
}
