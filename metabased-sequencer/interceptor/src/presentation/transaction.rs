use alloy_primitives::U256;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct TransactionFeeTooHigh {
    pub fee_gwei: U256,
    pub cap_gwei: U256,
}

impl fmt::Display for TransactionFeeTooHigh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
                f,
                "tx fee ({} gwei) exceeds the configured cap ({} gwei)",
            self.fee_gwei, self.cap_gwei
        )
    }
}

/// Ported from op-geth/ethapi/api.go
/// Check if the transaction fee is reasonable (under the cap).
///
/// # Arguments
///
/// * `gas_price` - The gas price in wei as U256.
/// * `gas` - The gas amount as U256.
/// * `cap_in_wei` - The fee cap in wei as U256.
///
/// # Returns
///
/// * `Ok(())` if the fee is under the cap, or if there is no cap.
/// * `TransactionFeeTooHigh` with an error message if the fee exceeds the cap.
pub fn check_tx_fee(gas_price: U256, gas: U256, cap_in_wei: U256) -> Result<(), TransactionFeeTooHigh> {
    // Short circuit if there is no cap for transaction fee at all.
    if cap_in_wei.is_zero() {
        return Ok(());
    }

    let fee_wei = gas_price
        .saturating_mul(gas);

    if fee_wei > cap_in_wei {
        let gwei = U256::from(1_000_000_000u64); // 1 Gwei = 10^9 Wei
        let fee_gwei = fee_wei / gwei;
        let cap_gwei = cap_in_wei / gwei;
        Err(TransactionFeeTooHigh { fee_gwei, cap_gwei })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GWEI: U256 = U256::from_limbs([1_000_000_000, 0, 0, 0]);
    const ETHER: U256 = U256::from_limbs([1_000_000_000_000_000_000, 0, 0, 0]);

    #[test]
    fn test_no_cap() {
        assert!(check_tx_fee(
            U256::from(1_000_000_000u64),
            U256::from(21000u64),
            U256::ZERO
        )
            .is_ok());
    }

    #[test]
    fn test_under_cap() {
        assert!(check_tx_fee(U256::from(20_000_000_000u64), U256::from(21000u64), ETHER).is_ok());
    }

    #[test]
    fn test_exact_cap() {
        let gas_price = U256::from(47619047619u64); // 47.619047619 Gwei
        let gas = U256::from(21000u64);
        let cap = gas_price * gas;
        assert!(check_tx_fee(gas_price, gas, cap).is_ok());
    }

    #[test]
    fn test_over_cap() {
        let gas_price = U256::from(130u64);
        let gas = U256::from(20_123_000_123u64);
        let cap = GWEI * U256::from(1000u64); // 1000 Gwei
        let result = check_tx_fee(gas_price, gas, cap);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("tx fee (2615 gwei) exceeds the configured cap (1000 gwei)"));
    }

    #[test]
    fn test_large_values() {
        let gas_price = U256::MAX;
        let gas = U256::from(u64::MAX);
        let cap = U256::MAX;
        let result = check_tx_fee(gas_price, gas, cap);
        assert!(result.is_ok()); // This will not error due to saturation in fee calculation
    }

    #[test]
    fn test_small_values() {
        assert!(check_tx_fee(U256::from(1u64), U256::from(1u64), U256::from(2u64)).is_ok());
    }

    #[test]
    fn test_zero_gas_price() {
        assert!(check_tx_fee(U256::ZERO, U256::from(21000u64), ETHER).is_ok());
    }

    #[test]
    fn test_zero_gas() {
        assert!(check_tx_fee(U256::from(20_000_000_000u64), U256::ZERO, ETHER).is_ok());
    }

    #[test]
    fn test_precision() {
        let gas_price = U256::from(20_000_000_000u64); // 20 Gwei
        let gas = U256::from(21_000u64);
        let fee = gas_price * gas;
        let cap_slightly_over = fee + U256::from(1u64);
        let cap_slightly_under = fee - U256::from(1u64);

        assert!(check_tx_fee(gas_price, gas, cap_slightly_over).is_ok());
        assert!(check_tx_fee(gas_price, gas, cap_slightly_under).is_err());
    }

    #[test]
    fn test_very_small_cap() {
        assert!(check_tx_fee(U256::from(1u64), U256::from(1u64), U256::from(1u64)).is_ok());
    }

    #[test]
    fn test_very_large_cap() {
        // This test now uses large but not maximum values to avoid overflow
        let gas_price = U256::from(u64::MAX);
        let gas = U256::from(u64::MAX);
        let cap = U256::MAX;
        assert!(check_tx_fee(gas_price, gas, cap).is_ok()); // Will not fail
    }

    #[test]
    fn test_error_message_formatting() {
        let gas_price = U256::from(100u64);
        let gas = U256::from(1_600_000_000u64);
        let cap = GWEI * U256::from(150u64); // 150 Gwei
        let result = check_tx_fee(gas_price, gas, cap);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("tx fee (160 gwei) exceeds the configured cap (150 gwei)"));
    }
}