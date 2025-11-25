//! contains batch parsing logic used by the slotter

use crate::slotter::SlotterError;
use alloy::primitives::{Bytes, TxHash};
use shared::types::{BlockRef, PartialBlock};
use synd_block_builder::appchains::shared::RollupAdapter;
use tracing::{info, warn};

/// builds a batch from a sequencing block, rollup adapter and l1 block ref
pub fn build_batch(
    seq_block: &PartialBlock,
    rollup_adapter: &impl RollupAdapter,
    l1_block: &BlockRef,
) -> Result<(u64, Bytes), SlotterError> {
    let mb_transactions = parse_block_to_txs(seq_block, rollup_adapter);

    if mb_transactions.is_empty() {
        return Ok((0, Default::default()));
    }

    info!(
        slot = seq_block.block_ref.number,
        "Processing sequencer transactions: {:?}",
        mb_transactions.iter().map(|x| x.1).collect::<Vec<_>>()
    );
    Ok((
        mb_transactions.len() as u64,
        rollup_adapter
            .build_batch_bytes(
                mb_transactions.into_iter().map(|x| x.0).collect(),
                l1_block.number,
                l1_block.timestamp,
            )
            .map_err(|e| SlotterError::BuildBatchError(e.to_string()))?,
    ))
}

/// Parses a sequencing chain block into a batch.
///
/// extracts transactions from the block logs
fn parse_block_to_txs(
    seq_block: &PartialBlock,
    rollup_adapter: &impl RollupAdapter,
) -> Vec<(Bytes, TxHash)> {
    // TODO txHash return value is completely unused, should be removed
    seq_block
        .logs
        .iter()
        .filter_map(|log| match rollup_adapter.get_event_transactions(log) {
            Ok(txs) => Some(txs),
            Err(e) => {
                warn!("Failed to get event transactions from log: {:?}, error: {:?}", log, e);
                None
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy::{
        eips::Encodable2718,
        network::{EthereumWallet, TransactionBuilder as _},
        primitives::{address, Address, Log},
        rpc::types::TransactionRequest,
        signers::local::PrivateKeySigner,
        sol_types::SolEvent,
    };
    use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::TransactionProcessed;
    use synd_block_builder::appchains::{
        arbitrum::arbitrum_adapter::ArbitrumAdapter,
        shared::sequencing_transaction_parser::L2MessageKind,
    };

    #[tokio::test]
    async fn test_parse_tx() {
        let sequencing_contract_address = address!("0x0000000000000000000000000000000000000123");
        let tx = TransactionRequest::default()
            .with_to(Address::ZERO)
            .with_nonce(0)
            .with_gas_limit(0)
            .with_max_fee_per_gas(0)
            .with_max_priority_fee_per_gas(0)
            .build(&EthereumWallet::from(PrivateKeySigner::random()))
            .await
            .unwrap();
        let mut encoded_tx = tx.encoded_2718();
        encoded_tx.splice(0..0, vec![L2MessageKind::SignedTx as u8]);
        let block = PartialBlock {
            logs: vec![
                // empty tx
                Log {
                    address: sequencing_contract_address,
                    data: TransactionProcessed {
                        sender: Default::default(),
                        data: Default::default(),
                    }
                    .encode_log_data(),
                },
                // invalid txs
                Log {
                    address: sequencing_contract_address,
                    data: TransactionProcessed {
                        sender: Default::default(),
                        data: vec![L2MessageKind::SignedTx as u8].into(),
                    }
                    .encode_log_data(),
                },
                Log {
                    address: sequencing_contract_address,
                    data: TransactionProcessed {
                        sender: Default::default(),
                        data: vec![L2MessageKind::SignedTx as u8, 0].into(),
                    }
                    .encode_log_data(),
                },
                // valid tx
                Log {
                    address: sequencing_contract_address,
                    data: TransactionProcessed {
                        sender: Default::default(),
                        data: encoded_tx.clone().into(),
                    }
                    .encode_log_data(),
                },
            ],
            ..Default::default()
        };
        // parse mbtxs
        let rollup_adapter =
            ArbitrumAdapter::new(&synd_block_builder::config::BlockBuilderConfig {
                mchain_ws_url: String::new(),
                sequencing_contract_address: Some(sequencing_contract_address),
                arbitrum_bridge_address: Some(Address::ZERO),
                arbitrum_inbox_address: Some(Address::ZERO),
            });
        let txs = parse_block_to_txs(&block, &rollup_adapter);
        assert_eq!(txs, vec![(encoded_tx.into(), *tx.hash())])
    }
}
