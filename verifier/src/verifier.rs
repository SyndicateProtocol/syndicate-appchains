//! Verifies a batch of blocks and creates a new mchain block

use alloy::{consensus::Header, primitives::FixedBytes, rpc::types::Block};
use eyre::{eyre, Result};
use mchain::db::MBlock;

/// Verifies a batch of blocks and creates a new mchain block
pub fn verify_and_create_batch(
    seq_blocks: Vec<Block>,
    set_blocks: Vec<Block>,
    expected_seq_hash: FixedBytes<32>,
    expected_set_hash: FixedBytes<32>,
) -> Result<MBlock> {
    validate_blocks(&seq_blocks, expected_seq_hash)?;
    validate_blocks(&set_blocks, expected_set_hash)?;

    generate_mblock(seq_blocks, set_blocks)
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

fn generate_mblock(_seq_blocks: Vec<Block>, _set_blocks: Vec<Block>) -> Result<MBlock> {
    // TODO: implement
    let mblock = MBlock::default();

    Ok(mblock)
}
