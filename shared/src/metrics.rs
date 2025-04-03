use alloy::primitives::Address;
use prometheus_client::encoding::EncodeLabelSet;

/// Labels for the wallet balance metric
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct WalletBalanceLabels {
    /// The wallet address being monitored
    wallet_address: String,
}

impl WalletBalanceLabels {
    /// Creates a new WalletBalanceLabels instance from an Address
    pub fn new(wallet_address: Address) -> Self {
        Self { wallet_address: format!("{:#x}", wallet_address) }
    }
}
