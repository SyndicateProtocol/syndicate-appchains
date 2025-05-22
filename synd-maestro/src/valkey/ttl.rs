//! Module for all default Valkey cache time-to-live (TTL) values for different Valkey String keys.
//! These are used when initializing Maestro config in `config.rs`

/// Time-to-live (TTL) for wallet nonce keys
pub mod wallet_nonce {

    /// Default time-to-live (TTL) for Wallet Nonce keys, in seconds
    pub const WALLET_NONCE_TTL: &str = "3s";
}

/// Time-to-live (TTL) for waiting transaction keys
pub mod waiting_txn {

    /// Default time-to-live (TTL) for Waiting Transaction keys
    pub const WAITING_TXN_TTL: &str = "15m"; // 15min
}
