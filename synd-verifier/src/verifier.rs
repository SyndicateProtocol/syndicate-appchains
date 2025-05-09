//! Verifies a batch of blocks and creates a new mchain block.

use crate::types::{ChainVerificationInput, TypedReceipt};
use alloy::{
    consensus::{
        proofs::calculate_receipt_root, Eip658Value, Header, Receipt as AlloyReceipt,
        ReceiptWithBloom,
    },
    primitives::{Bytes, FixedBytes, Log},
    rpc::types::Block,
};
use block_builder::rollups::arbitrum::arbitrum_adapter::ArbitrumAdapter;
use eyre::{eyre, Result};
use mchain::db::{DelayedMessage, MBlock, Slot};
use shared::types::{BlockRef, PartialBlock, Receipt};

/// Verifies a batch of blocks and creates a new mchain block.
pub fn verify_and_create_batch(
    sequencing_chain_input: ChainVerificationInput,
    settlement_chain_input: ChainVerificationInput,
    sequencing_expected_hash: FixedBytes<32>,
    settlement_expected_hash: FixedBytes<32>,
    settlement_delay: u64,
) -> Result<MBlock> {
    // Validate blocks
    validate_blocks(&sequencing_chain_input.blocks, sequencing_expected_hash)?;
    validate_blocks(&settlement_chain_input.blocks, settlement_expected_hash)?;

    // Validate receipts
    validate_receipts(&sequencing_chain_input)?;
    validate_receipts(&settlement_chain_input)?;

    // Generate mchain block
    generate_mblock(
        sequencing_chain_input.blocks,
        settlement_chain_input.blocks,
        sequencing_chain_input.receipts,
        settlement_chain_input.receipts,
        settlement_delay,
    )
}

// --------------------------------------------
// Validation Functions
// --------------------------------------------

fn validate_receipts(input: &ChainVerificationInput) -> Result<()> {
    for (i, block_receipts) in input.receipts.iter().enumerate() {
        let alloy_receipts: Vec<ReceiptWithBloom<TypedReceipt>> = block_receipts
            .iter()
            .map(|r| {
                let typed = TypedReceipt {
                    ty: r.r#type,
                    receipt: AlloyReceipt {
                        status: Eip658Value::Eip658(r.status == 1),
                        cumulative_gas_used: r.cumulative_gas_used,
                        logs: r.logs.clone(),
                    },
                };

                ReceiptWithBloom { receipt: typed, logs_bloom: r.logs_bloom }
            })
            .collect();

        let receipts_root = calculate_receipt_root(&alloy_receipts);

        if receipts_root != input.blocks[i].header.receipts_root {
            return Err(eyre!(
                "Receipts root mismatch: expected {}, got {}",
                input.blocks[i].header.receipts_root,
                receipts_root
            ));
        }
    }

    Ok(())
}

fn validate_blocks(blocks: &[Block], expected_hash: FixedBytes<32>) -> Result<()> {
    let Some(last_block) = blocks.last() else {
        return Err(eyre!("No blocks provided"));
    };

    if last_block.header.hash != expected_hash {
        return Err(eyre!(
            "Final block hash mismatch: expected {}, got {}",
            expected_hash,
            last_block.header.hash
        ));
    }

    for (i, block) in blocks.iter().enumerate() {
        if i > 0 {
            let prev_hash = blocks[i - 1].header.hash;
            if block.header.parent_hash != prev_hash {
                return Err(eyre!(
                    "Invalid parent hash at block {}: expected {}, got {}",
                    block.header.number,
                    prev_hash,
                    block.header.parent_hash
                ));
            }
        }
        validate_block_hash(block)?;
    }
    Ok(())
}

fn validate_block_hash(block: &Block) -> Result<()> {
    let header: Header = block.header.clone().into();
    let computed_hash = header.hash_slow();

    if computed_hash != block.header.hash {
        return Err(eyre!(
            "Block {} has invalid hash. Expected {}, got {}",
            header.number,
            computed_hash,
            block.header.hash
        ));
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
fn convert_block_to_partial_block(block: &Block, receipts: &[Receipt]) -> PartialBlock {
    // Question: do we need to filter logs here?
    let filtered_logs: Vec<Log> =
        receipts.iter().flat_map(|receipt| receipt.logs.clone()).collect();
    PartialBlock {
        block_ref: BlockRef {
            number: block.header.number,
            hash: block.header.hash,
            timestamp: block.header.timestamp,
        },
        parent_hash: block.header.parent_hash,
        logs: filtered_logs,
    }
}

#[allow(clippy::expect_used)]
fn generate_mblock(
    seq_blocks: Vec<Block>,
    set_blocks: Vec<Block>,
    seq_receipts: Vec<Vec<Receipt>>,
    set_receipts: Vec<Vec<Receipt>>,
    settlement_delay: u64,
) -> Result<MBlock> {
    let last_seq_block = seq_blocks.last().expect("No sequence blocks provided");
    let last_set_block = set_blocks.last().expect("No settlement blocks provided");
    let first_seq_block = seq_blocks.first().expect("No sequence blocks provided");
    let first_set_block = set_blocks.first().expect("No settlement blocks provided");
    let last_seq_receipts = seq_receipts.last().expect("No sequence receipts provided");

    let arbitrum_adapter = ArbitrumAdapter::default();

    let mut mblock = MBlock { timestamp: last_seq_block.header.timestamp, ..Default::default() };
    let mut payload: (Bytes, Vec<DelayedMessage>) = (Bytes::new(), Vec::new());

    if last_set_block.header.timestamp + settlement_delay <= last_seq_block.header.timestamp ||
        first_set_block.header.timestamp + settlement_delay > first_seq_block.header.timestamp
    {
        return Err(eyre!("Missing settlement blocks"));
    }

    let mut i = 1;

    while set_blocks[i].header.timestamp + settlement_delay <= first_seq_block.header.timestamp {
        // Skip settlement blocks that are before the first sequencing block
        i += 1
    }

    while set_blocks[i].header.timestamp + settlement_delay <= mblock.timestamp {
        // Include settlement blocks that belong to current slot
        let partial_block = convert_block_to_partial_block(&set_blocks[i], &set_receipts[i]);
        let mut delayed_messages = arbitrum_adapter.process_delayed_messages(&partial_block)?;
        payload.1.append(&mut delayed_messages);
        i += 1;
    }

    let seq_partial_block = convert_block_to_partial_block(last_seq_block, last_seq_receipts);
    let (tx_count, batch) = arbitrum_adapter.build_batch(&seq_partial_block)?;
    if tx_count > 0 || !payload.1.is_empty() {
        payload.0 = batch;
        mblock.payload = Some(payload);
    }

    let slot = Slot {
        seq_block_number: last_seq_block.header.number,
        seq_block_hash: last_seq_block.header.hash,
        set_block_hash: set_blocks[i].header.hash,
        set_block_number: set_blocks[i].header.number,
    };

    mblock.slot = slot;

    Ok(mblock)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{eips::BlockNumberOrTag, providers::Provider, transports::http::reqwest::Url};
    use std::str::FromStr;

    async fn test_data(url: &str, block_number: u64) -> Result<ChainVerificationInput> {
        let url = Url::from_str(url).unwrap();
        let alloy_provider = alloy::providers::ProviderBuilder::new().on_http(url);

        let block_number = BlockNumberOrTag::Number(block_number);

        let receipt: Vec<Receipt> = alloy_provider
            .raw_request("eth_getBlockReceipts".into(), (block_number,))
            .await
            .unwrap();

        let block = alloy_provider.get_block_by_number(block_number).await?.unwrap();

        Ok(ChainVerificationInput { blocks: vec![block], receipts: vec![receipt] })
    }

    #[tokio::test]
    async fn test_validate_receipts_arbitrum() {
        let input = test_data(
            "https://syndicate-exo.g.alchemy.com/v2/FFOCYExawZ3K46YRNHqaUEo3pbqS5F1F",
            10000,
        )
        .await
        .unwrap();
        validate_receipts(&input).unwrap();
    }

    #[tokio::test]
    async fn test_validate_receipts_optimism() {
        let input = test_data(
            "https://base-sepolia.g.alchemy.com/v2/FFOCYExawZ3K46YRNHqaUEo3pbqS5F1F",
            10000,
        )
        .await
        .unwrap();
        validate_receipts(&input).unwrap();
    }

    #[tokio::test]
    async fn test_validate_receipts_ethereum() {
        let input = test_data(
            "https://eth-sepolia.g.alchemy.com/v2/FFOCYExawZ3K46YRNHqaUEo3pbqS5F1F",
            10000,
        )
        .await
        .unwrap();
        validate_receipts(&input).unwrap();
    }
}
