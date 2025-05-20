//! The `synd-appchain-verifier` crate is responsible for verifying a batch of blocks and creating a
//! new `mchain` block

use crate::{
    config::AppchainVerifierConfig,
    errors::VerifierError,
    types::{
        DelayedMessageBlock, L1IncomingMessage, L1IncomingMessageHeader, SequencingChainInput,
        SettlementChainInput, TypedReceipt, VerifierOutput,
    },
};
use alloy::{
    consensus::{proofs::calculate_receipt_root, Header, Receipt as AlloyReceipt},
    primitives::{keccak256, FixedBytes, B256},
    rpc::types::Block,
};
use eyre::Result;
use shared::types::{convert_block_to_partial_block, Receipt};
use std::collections::HashSet;
use synd_block_builder::appchains::{
    arbitrum::arbitrum_adapter::ArbitrumAdapter,
    shared::sequencing_transaction_parser::SequencingTransactionParser,
};

/// The `Verifier` struct is responsible for verifying a batch of blocks and creating a new mchain
/// block.
#[derive(Default, Debug, Clone)]
pub struct Verifier {
    /// The adapter for the sequencing chain
    arbitrum_adapter: ArbitrumAdapter,
    /// Settlement delay
    settlement_delay: u64,
}

impl Verifier {
    /// Create a new `Verifier`
    pub fn new(config: &AppchainVerifierConfig) -> Self {
        Self {
            arbitrum_adapter: ArbitrumAdapter {
                transaction_parser: SequencingTransactionParser::new(
                    config.sequencing_contract_address,
                ),
                bridge_address: config.arbitrum_bridge_address,
                inbox_address: config.arbitrum_inbox_address,
                ignore_delayed_messages: config.arbitrum_ignore_delayed_messages,
                allowed_settlement_addresses: config
                    .allowed_settlement_addresses
                    .iter()
                    .copied()
                    .collect::<HashSet<_>>(),
            },
            settlement_delay: config.settlement_delay,
        }
    }

    /// Verifies blocks and receipts and creates a new mchain block
    pub fn verify_and_create_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<(Vec<VerifierOutput>, B256), VerifierError> {
        // Validate blocks
        self.validate_blocks(
            sequencing_chain_input.start_block_number,
            &sequencing_chain_input.blocks,
            sequencing_chain_input.end_block_hash,
        )?;
        self.validate_blocks(
            settlement_chain_input.start_block_number,
            &settlement_chain_input.blocks,
            settlement_chain_input.end_block_hash,
        )?;

        // Validate receipts
        self.validate_receipts(&sequencing_chain_input.receipts, &sequencing_chain_input.blocks)?;
        self.validate_receipts(&settlement_chain_input.receipts, &settlement_chain_input.blocks)?;

        // Validate delayed messages
        self.validate_delayed_messages(
            &settlement_chain_input.unused_delayed_messages,
            settlement_chain_input.unused_delayed_messages_hash,
        )?;

        // Generate output
        self.generate_output(sequencing_chain_input, settlement_chain_input)
    }

    // --------------------------------------------
    // Validation Functions
    // --------------------------------------------
    fn validate_receipts(
        &self,
        receipts: &[Vec<Receipt>],
        blocks: &[Block],
    ) -> Result<(), VerifierError> {
        for (i, block_receipts) in receipts.iter().enumerate() {
            let alloy_receipts: Vec<TypedReceipt> = block_receipts
                .iter()
                .map(|r| TypedReceipt {
                    ty: r.r#type,
                    receipt: AlloyReceipt {
                        status: r.status,
                        cumulative_gas_used: r.cumulative_gas_used,
                        logs: r.logs.clone(),
                    },
                })
                .collect();

            let receipts_root = calculate_receipt_root(&alloy_receipts);

            if receipts_root != blocks[i].header.receipts_root {
                return Err(VerifierError::ReceiptsRootMismatch {
                    index: i,
                    expected: blocks[i].header.receipts_root,
                    actual: receipts_root,
                });
            }
        }

        Ok(())
    }

    fn validate_blocks(
        &self,
        start_block_number: u64,
        blocks: &[Block],
        expected_hash: FixedBytes<32>,
    ) -> Result<(), VerifierError> {
        let Some(first_block) = blocks.first() else {
            return Err(VerifierError::NoBlocksProvided);
        };

        if first_block.header.number != start_block_number {
            return Err(VerifierError::InvalidStartBlockNumber {
                expected: start_block_number,
                actual: first_block.header.number,
            });
        }

        let Some(last_block) = blocks.last() else {
            return Err(VerifierError::NoBlocksProvided);
        };

        if last_block.header.hash != expected_hash {
            return Err(VerifierError::FinalBlockHashMismatch {
                expected: expected_hash,
                actual: last_block.header.hash,
            });
        }

        for (i, block) in blocks.iter().enumerate() {
            if i > 0 {
                let prev_hash = blocks[i - 1].header.hash;
                if block.header.parent_hash != prev_hash {
                    return Err(VerifierError::InvalidParentHash {
                        block_number: block.header.number,
                        expected: prev_hash,
                        actual: block.header.parent_hash,
                    });
                }
            }
            self.validate_block_hash(block)?;
        }
        Ok(())
    }

    fn validate_block_hash(&self, block: &Block) -> Result<(), VerifierError> {
        let header: Header = block.header.clone().into();
        let computed_hash = header.hash_slow();

        if computed_hash != block.header.hash {
            return Err(VerifierError::InvalidBlockHash {
                block_number: header.number,
                expected: computed_hash,
                actual: block.header.hash,
            });
        }
        Ok(())
    }

    #[allow(clippy::unwrap_used)]
    fn validate_delayed_messages(
        &self,
        unused_delayed_messages: &Vec<DelayedMessageBlock>,
        unused_delayed_messages_hash: B256,
    ) -> Result<(), VerifierError> {
        let encoded = serde_json::to_string(unused_delayed_messages).unwrap();
        let computed_hash = keccak256(encoded);

        if computed_hash != unused_delayed_messages_hash {
            return Err(VerifierError::InvalidDelayedMessagesHash {
                expected: unused_delayed_messages_hash,
                actual: computed_hash,
            });
        }

        Ok(())
    }

    // --------------------------------------------
    // Output Generation
    // --------------------------------------------
    #[allow(clippy::unwrap_used)]
    fn generate_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<(Vec<VerifierOutput>, B256), VerifierError> {
        let (first_seq_block, last_seq_block) =
            Self::get_first_and_last_blocks(&sequencing_chain_input.blocks)?;
        let (first_set_block, last_set_block) =
            Self::get_first_and_last_blocks(&settlement_chain_input.blocks)?;

        if !Self::has_full_settlement_coverage(
            first_set_block.header.timestamp,
            last_set_block.header.timestamp,
            first_seq_block.header.timestamp,
            last_seq_block.header.timestamp,
            self.settlement_delay,
        ) {
            return Err(VerifierError::MissingSettlementBlocks);
        }

        let mut verifier_outputs = vec![];
        let mut delayed_messages = vec![];

        let mut delayed_messages_index = 0;
        let mut set_block_index = 0;
        let mut seq_block_index = 0;
        while seq_block_index < sequencing_chain_input.blocks.len() {
            // Process unused delayed messages and add to delayed messages vector
            while delayed_messages_index < settlement_chain_input.unused_delayed_messages.len() &&
                settlement_chain_input.unused_delayed_messages[delayed_messages_index].timestamp +
                    self.settlement_delay <=
                    sequencing_chain_input.blocks[seq_block_index].header.timestamp
            {
                let mut current_delayed_messages = settlement_chain_input.unused_delayed_messages
                    [delayed_messages_index]
                    .delayed_messages
                    .clone();
                delayed_messages.append(&mut current_delayed_messages);
                delayed_messages_index += 1;
            }

            // Process settlement blocks and add to delayed messages vector
            while set_block_index < settlement_chain_input.blocks.len() &&
                settlement_chain_input.blocks[set_block_index].header.timestamp +
                    self.settlement_delay <=
                    sequencing_chain_input.blocks[seq_block_index].header.timestamp
            {
                // Include settlement blocks that belong to current slot
                let partial_block = convert_block_to_partial_block(
                    &settlement_chain_input.blocks[set_block_index],
                    &settlement_chain_input.receipts[set_block_index],
                );
                let mut current_delayed_messages =
                    self.arbitrum_adapter.process_delayed_messages(&partial_block)?;
                delayed_messages.append(&mut current_delayed_messages);
                set_block_index += 1;
            }

            let seq_partial_block = convert_block_to_partial_block(
                &sequencing_chain_input.blocks[seq_block_index],
                &sequencing_chain_input.receipts[seq_block_index],
            );
            let (seq_tx_count, seq_batch) =
                self.arbitrum_adapter.build_batch(&seq_partial_block)?;
            // If there are no sequencing transactions or settlement transactions (delayed messages)
            // add batch to the payload
            if seq_tx_count == 0 && delayed_messages.is_empty() {
                seq_block_index += 1;
                continue;
            }

            seq_block_index += 1;
            let seq_block = &sequencing_chain_input.blocks[seq_block_index];

            verifier_outputs.push(VerifierOutput {
                block_hash: sequencing_chain_input.blocks[seq_block_index].header.hash,
                min_timestamp: 0,
                max_timestamp: u64::MAX,
                min_block_number: 0,
                max_block_number: u64::MAX,
                messages: delayed_messages
                    .iter()
                    .map(|m| L1IncomingMessage {
                        header: L1IncomingMessageHeader {
                            kind: m.kind,
                            poster: m.sender,
                            block_number: seq_block.header.number,
                            timestamp: seq_block.header.timestamp,
                            request_id: B256::ZERO, // TODO: add request id
                            l1_base_fee: m.base_fee_l1,
                        },
                        l2msg: m.data.clone(),
                    })
                    .collect(),
                batch: seq_batch,
            });
        }

        // Push delayed messages that are not included to the unused delayed messages vector
        let mut unused_delayed_messages: Vec<DelayedMessageBlock> =
            settlement_chain_input.unused_delayed_messages[delayed_messages_index..].to_vec();

        while set_block_index < settlement_chain_input.blocks.len() {
            let partial_block = convert_block_to_partial_block(
                &settlement_chain_input.blocks[set_block_index],
                &settlement_chain_input.receipts[set_block_index],
            );
            let delayed_messages =
                self.arbitrum_adapter.process_delayed_messages(&partial_block)?;
            let delayed_message_block = DelayedMessageBlock {
                timestamp: settlement_chain_input.blocks[set_block_index].header.timestamp,
                delayed_messages,
            };
            unused_delayed_messages.push(delayed_message_block);
            set_block_index += 1;
        }

        let unused_delayed_messages_hash =
            keccak256(serde_json::to_string(&unused_delayed_messages).unwrap());

        Ok((verifier_outputs, unused_delayed_messages_hash))
    }

    // --------------------------------------------
    // Helper Functions
    // --------------------------------------------

    /// Returns true if the settlement blocks fully cover the sequencing slot window.
    /// We want to make sure we have settlement blocks that are before and after the slot
    /// window (sequencing start - sequencing end) to make sure we have a full slot and are not
    /// missing any blocks
    const fn has_full_settlement_coverage(
        first_settlement_ts: u64,
        last_settlement_ts: u64,
        first_sequencing_ts: u64,
        last_sequencing_ts: u64,
        settlement_delay: u64,
    ) -> bool {
        (first_settlement_ts + settlement_delay <= first_sequencing_ts) &&
            (last_settlement_ts + settlement_delay > last_sequencing_ts)
    }

    fn get_first_and_last_blocks(blocks: &[Block]) -> Result<(&Block, &Block), VerifierError> {
        let first = blocks.first().ok_or(VerifierError::NoBlocksProvided)?;
        let last = blocks.last().ok_or(VerifierError::NoBlocksProvided)?;
        Ok((first, last))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        consensus::Eip658Value,
        primitives::{address, b256, Bloom, Bytes, U256},
        rpc::types::{Block, BlockTransactions},
    };
    use shared::{logger::set_global_default_subscriber, types::Receipt};
    use std::str::FromStr;

    struct MockInput {
        blocks: Vec<Block>,
        receipts: Vec<Vec<Receipt>>,
    }

    fn mock_op_input() -> MockInput {
        MockInput {
            blocks: vec![Block {
                header: alloy::rpc::types::Header {
                    hash: b256!(
                        "0x87b5359026eaca979b0e6396cda4c55d7b3ab94e0fdb8965ec0e9db88b1d4dbc"
                    ),
                    inner: Header {
                        parent_hash: b256!(
                            "0x3cf28ef6b2242924f6a756cf7cf1e8584ee2191a283e919eef529b50b2d9e738"
                        ),
                        ommers_hash: b256!(
                            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
                        ),
                        beneficiary: address!("0x4200000000000000000000000000000000000011"),
                        state_root: b256!(
                            "0xeab1ec4c9785d34a3da530c8ea43e39d8cad9f50d84eb2be53b9ec53e17063a8"
                        ),
                        transactions_root: b256!(
                            "0x506f0c6271701a741d7802f6002a1beb9c4d8a1e67e6bad6e7f3da28b7abd1ff"
                        ),
                        receipts_root: b256!(
                            "0x29079b696c12a19999f3bb303fddb6fc12fb701f427678cca24954b91080ada3"
                        ),
                        logs_bloom: Bloom::ZERO,
                        difficulty: U256::ZERO,
                        number: 10000,
                        gas_limit: 25_000_000,
                        gas_used: 46865,
                        timestamp: 1695788288,
                        extra_data: Bytes::new(),
                        mix_hash: b256!(
                            "0x97febdc93eb00c4c778a82c598bc50dce4e771403bd3821b57d4606a244d7dc2"
                        ),
                        nonce: 0x0000000000000000u64.into(),
                        base_fee_per_gas: Some(50),
                        ..Default::default()
                    },
                    total_difficulty: None,
                    size: None,
                },
                transactions: BlockTransactions::Hashes(vec![b256!(
                    "0x17f01516ef3aee2a5c4cdff6465e1b0cb8c4de9592f27a5de290def9e0bc8f7f"
                )]),
                ..Default::default()
            }],
            receipts: vec![vec![Receipt {
                block_hash: b256!(
                    "0x87b5359026eaca979b0e6396cda4c55d7b3ab94e0fdb8965ec0e9db88b1d4dbc"
                ),
                block_number: 10000,
                from: address!("0xdeaddeaddeaddeaddeaddeaddeaddeaddead0001"),
                to: Some(address!("0x4200000000000000000000000000000000000015")),
                contract_address: None,
                logs: vec![],
                logs_bloom: Bloom::ZERO,
                status: Eip658Value::Eip658(true),
                r#type: 126,
                transaction_index: 0,
                transaction_hash: b256!(
                    "0x17f01516ef3aee2a5c4cdff6465e1b0cb8c4de9592f27a5de290def9e0bc8f7f"
                ),
                gas_used: 46865,
                cumulative_gas_used: 46865,
            }]],
        }
    }

    fn mock_arbitrum_input() -> MockInput {
        MockInput {
            blocks: vec![
                Block {
                    header: alloy::rpc::types::Header {
                        hash: b256!(
                            "0x01f14eb4ce809febf400372eaacfcc8c59d215ec84066efb5bcc24c916493e17"
                        ),
                        inner: Header {
                            parent_hash: b256!(
                            "0xca9bdb7b0ae2e164b20484c16bcb3209a76577de262714cbc99e8c1e64c68ffa"
                        ),
                            ommers_hash: b256!(
                            "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
                        ),
                            beneficiary: address!("0xa4b000000000000000000073657175656e636572"),
                            state_root: b256!(
                            "0x32d7a9623e73a4f0121666780799fbcfdf9e4f3c8d983ffe6028120e80936dc6"
                        ),
                            transactions_root: b256!(
                            "0xf1a71e4e0ae784474d3413bdc5dec113e440eb9c33e6c9d286d3eee55ab73c2a"
                        ),
                            receipts_root: b256!(
                            "0x3b5e9d084dc4a770cd470167568601342aebe33d876f98f0a981f27700234cec"
                        ),
                            logs_bloom: Bloom::ZERO,
                            difficulty: U256::from(1),
                            number: 10000,
                            gas_limit: 1125899906842624u64,
                            gas_used: 21000,
                            timestamp: 1739564230,
                            extra_data: Bytes::from(vec![0; 32]),
                            mix_hash: b256!(
                            "0x000000000000000000000000003329fb00000000000000200000000000000000"
                        ),
                            nonce: 0x0000000000000012u64.into(),
                            base_fee_per_gas: Some(100_000_000),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    transactions: BlockTransactions::Hashes(vec![
                        b256!("0x1e79a4901d2e4c3d8319160d5d46d24bccd2b7d9a4018abb8a5cf9f99fe0627c"),
                        b256!("0x42e44cde66ef6130c49e7261be2d24269ae20c3393be968615b6d344fce0683e"),
                    ]),
                    ..Default::default()
                },
                Block {
                    header: alloy::rpc::types::Header {
                        hash: b256!(
                            "0x8af94836bc338d58d7b442e4761e92557764341d47c41379a70dda495dcf688f"
                        ),
                        inner: Header {
                            parent_hash: b256!(

"0x01f14eb4ce809febf400372eaacfcc8c59d215ec84066efb5bcc24c916493e17"
),
                            ommers_hash: b256!(

"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
),
                            beneficiary: address!("0xa4b000000000000000000073657175656e636572"),
                            state_root: b256!(
"0x14b4a23fb803b2e73b67ba1e2a3df23a6ba15eeb383e47b7d866a8ac19b102ff"
),
                            transactions_root: b256!(

"0x5c0ae78c90b6eee94a5171b1016a327f9ff6a5c287a6db5f5e81786379808415"
),
                            receipts_root: b256!(

"0x3b5e9d084dc4a770cd470167568601342aebe33d876f98f0a981f27700234cec"
),
                            logs_bloom: Bloom::ZERO,
                            difficulty: U256::from(1),
                            number: 10001,
                            gas_limit: 0x4000000000000u64,
                            gas_used: 0x5208u64,
                            timestamp: 0x67afa4c9u64,
                            extra_data: Bytes::from(vec![0; 32]),
                            mix_hash: b256!(

"0x000000000000000000000000003329fb00000000000000200000000000000000"
),
                            nonce: 0x0000000000000012u64.into(),
                            base_fee_per_gas: Some(100000000),
                            ..Default::default()
                        },
                        total_difficulty: Some(U256::from(0x2712)), // 10002
                        size: None,                                 // 807
                    },
                    transactions: BlockTransactions::Hashes(vec![
                        b256!("0xe83dce65522435298054905c4b0f21e9a3f1875fb7ecf654857cd1b0124b509c"),
                        b256!("0x0afc123d5dd8ae26e878b5ceca887d42eb6ea7aa66f52d3fdbc69a97df23bb6d"),
                    ]),
                    ..Default::default()
                },
            ],
            receipts: vec![
                vec![
                    Receipt {
                        block_hash: b256!(
                            "0x01f14eb4ce809febf400372eaacfcc8c59d215ec84066efb5bcc24c916493e17"
                        ),
                        block_number: 10000,
                        from: address!("0x00000000000000000000000000000000000a4b05"),
                        to: Some(address!("0x00000000000000000000000000000000000a4b05")),
                        contract_address: None,
                        logs: vec![],
                        logs_bloom: Bloom::ZERO,
                        status: Eip658Value::Eip658(true),
                        r#type: 106,
                        transaction_index: 0,
                        transaction_hash: b256!(
                            "0x1e79a4901d2e4c3d8319160d5d46d24bccd2b7d9a4018abb8a5cf9f99fe0627c"
                        ),
                        gas_used: 0,
                        cumulative_gas_used: 0,
                    },
                    Receipt {
                        block_hash: b256!(
                            "0x01f14eb4ce809febf400372eaacfcc8c59d215ec84066efb5bcc24c916493e17"
                        ),
                        block_number: 10000,
                        from: address!("0x8ee98c0f520749611f65e9723e07abc0b93c30b3"),
                        to: Some(address!("0x0000000000000000000000000000000000000000")),
                        contract_address: None,
                        logs: vec![],
                        logs_bloom: Bloom::ZERO,
                        status: Eip658Value::Eip658(true),
                        r#type: 2,
                        transaction_index: 1,
                        transaction_hash: b256!(
                            "0x42e44cde66ef6130c49e7261be2d24269ae20c3393be968615b6d344fce0683e"
                        ),
                        gas_used: 21000,
                        cumulative_gas_used: 21000,
                    },
                ],
                vec![
                    Receipt {
                        block_hash: b256!(
                            "0x8af94836bc338d58d7b442e4761e92557764341d47c41379a70dda495dcf688f"
                        ),
                        block_number: 10001, // 0x2711
                        from: address!("0x00000000000000000000000000000000000a4b05"),
                        to: Some(address!("0x00000000000000000000000000000000000a4b05")),
                        contract_address: None,
                        logs: vec![],
                        logs_bloom: Bloom::ZERO,
                        status: Eip658Value::Eip658(true),
                        r#type: 106, // 0x6a
                        transaction_index: 0,
                        transaction_hash: b256!(
                            "0xe83dce65522435298054905c4b0f21e9a3f1875fb7ecf654857cd1b0124b509c"
                        ),
                        gas_used: 0,
                        cumulative_gas_used: 0,
                    },
                    Receipt {
                        block_hash: b256!(
                            "0x8af94836bc338d58d7b442e4761e92557764341d47c41379a70dda495dcf688f"
                        ),
                        block_number: 10001, // 0x2711
                        from: address!("0x8ee98c0f520749611f65e9723e07abc0b93c30b3"),
                        to: Some(address!("0x0000000000000000000000000000000000000000")),
                        contract_address: None,
                        logs: vec![],
                        logs_bloom: Bloom::ZERO,
                        status: Eip658Value::Eip658(true),
                        r#type: 2,
                        transaction_index: 1,
                        transaction_hash: b256!(
                            "0x0afc123d5dd8ae26e878b5ceca887d42eb6ea7aa66f52d3fdbc69a97df23bb6d"
                        ),
                        gas_used: 21000,            // 0x5208
                        cumulative_gas_used: 21000, // 0x5208
                    },
                ],
            ],
        }
    }

    fn mock_ethereum_input() -> MockInput {
        MockInput {
            blocks: vec![
                Block {
                    header: alloy::rpc::types::Header {
                        hash:
b256!("0x42bf3daa9037e46b9e07b5740a9e7eb0a50fedbe8ffb00e0df9f522500d3ae24"),
inner: Header {                             parent_hash:
b256!("0x86ebcc146f6e7c743b97cd405a1b1d0663b190f8b705042935c37a9cf3667b4f"),
ommers_hash: b256!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                            beneficiary: address!("0x2f14582947e292a2ecd20c430b46f2d27cfe213c"),
                            state_root:
b256!("0x7cf6b6cb78ad287f06ca663d1beb52287c188804c798d084f7f0a7ae6c18373c"),
transactions_root: b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                            receipts_root:
b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
logs_bloom: Bloom::ZERO,                             difficulty: U256::from(0x10522bf),
                            number: 9998,
                            gas_limit: 0x1c9c380,
                            gas_used: 0,
                            timestamp: 0x617387a0,
                            extra_data: Bytes::new(),
                            mix_hash:
b256!("0x94742dae29697464bf564dab1bc9613bbd10443947b730e870fb1b095a142765"),
nonce: 0xc7faaf72b47f63adu64.into(),                             base_fee_per_gas: Some(7),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    ..Default::default()
                },
                Block {
                    header: alloy::rpc::types::Header {
                        hash:
b256!("0x1bf8c6858ef8ad67dd7eaceb34d99a3277c7b6e4a45a3c2a9135489b2586a8db"),
inner: Header {                             parent_hash:
b256!("0x42bf3daa9037e46b9e07b5740a9e7eb0a50fedbe8ffb00e0df9f522500d3ae24"),
ommers_hash: b256!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                            beneficiary: address!("0x2f14582947e292a2ecd20c430b46f2d27cfe213c"),
                            state_root:
b256!("0x557772037c22621a005f4dd75dcc23317c286ce02bb67a9c0eb2121226f3d5dd"),
transactions_root: b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                            receipts_root:
b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
logs_bloom: Bloom::ZERO,                             difficulty: U256::from(0x1054363),
                            number: 9999,
                            gas_limit: 0x1c9c380,
                            gas_used: 0,
                            timestamp: 0x617387a1,
                            extra_data: Bytes::new(),
                            mix_hash:
b256!("0x6ab7e92d407236826a86752eba2d709174d9829f528313d176e4a71789628cc7"),
nonce: 0xc7faaf72b46add7cu64.into(),                             base_fee_per_gas: Some(7),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    ..Default::default()
                },
                Block {
                    header: alloy::rpc::types::Header {
                        hash:
b256!("0x4169aea7eaf9809d899ef45d437727bbe15c8179ecbd112da00ebdb4736e5be1"),
inner: Header {                             parent_hash:
b256!("0x1bf8c6858ef8ad67dd7eaceb34d99a3277c7b6e4a45a3c2a9135489b2586a8db"),
ommers_hash: b256!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                            beneficiary: address!("0x2f14582947e292a2ecd20c430b46f2d27cfe213c"),
                            state_root:
b256!("0x011416cbb9d766dc4fcabbb79515a1caa4676ec62213b0ea8b346e807f8bcaec"),
transactions_root: b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                            receipts_root:
b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
logs_bloom: Bloom::ZERO,                             difficulty: U256::from(17130507),
                            number: 10000,
                            gas_limit: 30_000_000,
                            gas_used: 0,
                            timestamp: 1634961314,
                            extra_data: Bytes::new(),
                            mix_hash:
b256!("0x10b8ffb683f18b3f8c66fe781bab9eccb9579ca14607784c15a9406710e00eaa"),
nonce: 0xc7faaf72b5ae7b05u64.into(),                             base_fee_per_gas: Some(7),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    ..Default::default()
                },
                Block {
                    header: alloy::rpc::types::Header {
                        hash:
b256!("0xb396b16c0be0b0470ebb7e3c77484d34944d4b83cfa8a3f2fe77aaad31151434"),
inner: Header {                             parent_hash:
b256!("0x4169aea7eaf9809d899ef45d437727bbe15c8179ecbd112da00ebdb4736e5be1"),
ommers_hash: b256!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                            beneficiary: address!("0x2f14582947e292a2ecd20c430b46f2d27cfe213c"),
                            state_root:
b256!("0x0ee9572dd35dae26befa7a3405b6c0fc2f2b7d42fc6cc2ab8d1908222fc5c0b1"),
transactions_root: b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                            receipts_root:
b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
logs_bloom: Bloom::ZERO,                             difficulty: U256::from(0x10584b7u64),
                            number: 10001,
                            gas_limit: 0x1c9c380u64,
                            gas_used: 0,
                            timestamp: 0x617387a3u64,
                            extra_data: Bytes::new(),
                            mix_hash:
b256!("0xe82f54b551deb1eacd32c5327617894a35b9a163b0a101fde4f5d92120814b69"),
nonce: 0xc7faaf72b56687c1u64.into(),                             base_fee_per_gas: Some(7),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    transactions: BlockTransactions::Hashes(vec![]),
                    ..Default::default()
                },
                Block {
                    header: alloy::rpc::types::Header {
                        hash:
b256!("0xa1f860dcda8ea9cba4d4378a1c15b555551cf6b820df006f5c42ea5aa7cd7332"),
inner: Header {                             parent_hash:
b256!("0xb396b16c0be0b0470ebb7e3c77484d34944d4b83cfa8a3f2fe77aaad31151434"),
ommers_hash: b256!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                            beneficiary: address!("0x2f14582947e292a2ecd20c430b46f2d27cfe213c"),
                            state_root:
b256!("0xc7f8f404c0db440769cb1057ef9c0dde6261743df633429a8af9abf8d2167a66"),
transactions_root: b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                            receipts_root:
b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
logs_bloom: Bloom::ZERO,                             difficulty: U256::from(0x105a567),
                            number: 10002,
                            gas_limit: 0x1c9c380,
                            gas_used: 0,
                            timestamp: 0x617387a4,
                            extra_data: Bytes::new(),
                            mix_hash:
b256!("0x5b83874ab99315e7281738f5248d8be839f1eba6ba05eb81ab2e07793c67f9d7"),
nonce: 0xc7faaf72b4afd3e6u64.into(),                             base_fee_per_gas: Some(7),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    ..Default::default()
                },
                Block {
                    header: alloy::rpc::types::Header {
                        hash:
b256!("0x32b973ac9e5450b2919903cdc335c519423f286638857487d432e0369079c248"),
inner: Header {                             parent_hash:
b256!("0xa1f860dcda8ea9cba4d4378a1c15b555551cf6b820df006f5c42ea5aa7cd7332"),
ommers_hash: b256!("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
                            beneficiary: address!("0x2f14582947e292a2ecd20c430b46f2d27cfe213c"),
                            state_root:
b256!("0x8a2eb0ffc93141455498a7626915e4e9812dcfa1a70d6c1db8e219b2f5b6f9ba"),
transactions_root: b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
                            receipts_root:
b256!("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
logs_bloom: Bloom::ZERO,                             difficulty: U256::from(0x105c61b),
                            number: 10003,
                            gas_limit: 0x1c9c380,
                            gas_used: 0,
                            timestamp: 0x617387a5,
                            extra_data: Bytes::new(),
                            mix_hash:
b256!("0x6c6c505b3f1db136f53ce71434a11d930e189652acd6afa9ef74b99c3e54b0a5"),
nonce: 0xc7faaf72b783de68u64.into(),                             base_fee_per_gas: Some(7),
                            ..Default::default()
                        },
                        total_difficulty: None,
                        size: None,
                    },
                    ..Default::default()
                },
            ],
            receipts: vec![vec![]; 6],
        }
    }

    #[tokio::test]
    async fn test_validate_receipts_arbitrum() {
        let verifier = Verifier::new(&AppchainVerifierConfig::default());
        let input = mock_arbitrum_input();
        verifier.validate_receipts(&input.receipts, &input.blocks).unwrap();
    }

    #[tokio::test]
    async fn test_validate_receipts_optimism() {
        let verifier = Verifier::new(&AppchainVerifierConfig::default());
        let input = mock_op_input();
        verifier.validate_receipts(&input.receipts, &input.blocks).unwrap();
    }

    #[tokio::test]
    async fn test_validate_receipts_ethereum() {
        let verifier = Verifier::new(&AppchainVerifierConfig::default());
        let input = mock_ethereum_input();
        verifier.validate_receipts(&input.receipts, &input.blocks).unwrap();
    }

    #[tokio::test]
    async fn test_verify_and_create_output_success() {
        if let Err(e) = set_global_default_subscriber() {
            println!("Failed to set global default subscriber: {}", e);
        }
        let mut verifier = Verifier::new(&AppchainVerifierConfig::default());
        verifier.settlement_delay = 104_602_917;
        let seq_blocks_and_receipts = mock_arbitrum_input();
        let seq_input = &SequencingChainInput {
            blocks: seq_blocks_and_receipts.blocks,
            receipts: seq_blocks_and_receipts.receipts,
            start_block_number: 10000,
            end_block_hash: FixedBytes::from_str(
                "0x8af94836bc338d58d7b442e4761e92557764341d47c41379a70dda495dcf688f",
            )
            .unwrap(),
        };

        let set_blocks_and_receipts = mock_ethereum_input();
        let set_input = &SettlementChainInput {
            blocks: set_blocks_and_receipts.blocks,
            receipts: set_blocks_and_receipts.receipts,
            start_block_number: 9998,
            end_block_hash: FixedBytes::from_str(
                "0x32b973ac9e5450b2919903cdc335c519423f286638857487d432e0369079c248",
            )
            .unwrap(),
            unused_delayed_messages: vec![],
            unused_delayed_messages_hash: FixedBytes::from_str(
                "0x518674ab2b227e5f11e9084f615d57663cde47bce1ba168b4c19c7ee22a73d70",
            )
            .unwrap(),
        };

        // Settlement delay is 104,603,916 seconds so that it gets included in the slot
        let result = verifier.verify_and_create_output(seq_input, set_input);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_and_create_output_failure() {
        let mut verifier = Verifier::new(&AppchainVerifierConfig::default());
        verifier.settlement_delay = 104_602_917;
        let seq_blocks_and_receipts = mock_arbitrum_input();
        let seq_input = &SequencingChainInput {
            blocks: seq_blocks_and_receipts.blocks,
            receipts: seq_blocks_and_receipts.receipts,
            start_block_number: 10000,
            end_block_hash: FixedBytes::from_str(
                "0x8af94836bc338d58d7b442e4761e92557764341d47c41379a70dda495dcf688f",
            )
            .unwrap(),
        };

        let mut set_blocks_and_receipts = mock_ethereum_input();
        // Remove the last block from the previous input to make it invalid
        set_blocks_and_receipts.blocks.pop();
        set_blocks_and_receipts.receipts.pop();
        let set_input = &SettlementChainInput {
            blocks: set_blocks_and_receipts.blocks,
            receipts: set_blocks_and_receipts.receipts,
            start_block_number: 9998,
            end_block_hash: FixedBytes::from_str(
                "0xa1f860dcda8ea9cba4d4378a1c15b555551cf6b820df006f5c42ea5aa7cd7332",
            )
            .unwrap(),
            unused_delayed_messages: vec![],
            unused_delayed_messages_hash: FixedBytes::from_str(
                "0x518674ab2b227e5f11e9084f615d57663cde47bce1ba168b4c19c7ee22a73d70",
            )
            .unwrap(),
        };

        // Settlement delay is 104,603,916 seconds so that it gets the first sequencing block
        let result = verifier.verify_and_create_output(seq_input, set_input);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing settlement blocks"));
    }

    #[tokio::test]
    async fn test_missing_settlement_block() {
        let verifier = Verifier::new(&AppchainVerifierConfig::default());
        let seq_blocks_and_receipts = mock_arbitrum_input();
        let seq_input = &SequencingChainInput {
            blocks: seq_blocks_and_receipts.blocks,
            receipts: seq_blocks_and_receipts.receipts,
            start_block_number: 10000,
            end_block_hash: FixedBytes::from_str(
                "0x8af94836bc338d58d7b442e4761e92557764341d47c41379a70dda495dcf688f",
            )
            .unwrap(),
        };

        let set_blocks_and_receipts = mock_ethereum_input();
        let set_input = &SettlementChainInput {
            blocks: set_blocks_and_receipts.blocks,
            receipts: set_blocks_and_receipts.receipts,
            start_block_number: 9998,
            end_block_hash: FixedBytes::from_str(
                "0x32b973ac9e5450b2919903cdc335c519423f286638857487d432e0369079c248",
            )
            .unwrap(),
            unused_delayed_messages: vec![],
            unused_delayed_messages_hash: FixedBytes::from_str(
                "0x518674ab2b227e5f11e9084f615d57663cde47bce1ba168b4c19c7ee22a73d70",
            )
            .unwrap(),
        };

        // Settlement delay is 104,603,916 seconds so that it gets the first sequencing block
        let result = verifier.verify_and_create_output(seq_input, set_input);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing settlement blocks"));
    }

    #[test]
    fn test_has_full_settlement_coverage_passes() {
        // Settlement blocks fully cover the sequencing window.
        let settlement_delay = 10;
        let first_settlement_ts = 90;
        let last_settlement_ts = 200;
        let first_sequencing_ts = 100;
        let last_sequencing_ts = 190;

        assert!(
            Verifier::has_full_settlement_coverage(
                first_settlement_ts,
                last_settlement_ts,
                first_sequencing_ts,
                last_sequencing_ts,
                settlement_delay
            ),
            "Expected settlement coverage to be valid"
        );
    }

    #[test]
    fn test_missing_settlement_before_sequencing() {
        // Settlement starts too late (after sequencing start)
        let settlement_delay = 10;
        let first_settlement_ts = 120;
        let last_settlement_ts = 200;
        let first_sequencing_ts = 120;
        let last_sequencing_ts = 190;

        assert!(
            !Verifier::has_full_settlement_coverage(
                first_settlement_ts,
                last_settlement_ts,
                first_sequencing_ts,
                last_sequencing_ts,
                settlement_delay
            ),
            "Expected failure due to missing earlier settlement block"
        );
    }

    #[test]
    fn test_missing_settlement_after_sequencing() {
        // Settlement ends too early (before sequencing end)
        let settlement_delay = 10;
        let first_settlement_ts = 80;
        let last_settlement_ts = 170;
        let first_sequencing_ts = 100;
        let last_sequencing_ts = 190;

        assert!(
            !Verifier::has_full_settlement_coverage(
                first_settlement_ts,
                last_settlement_ts,
                first_sequencing_ts,
                last_sequencing_ts,
                settlement_delay
            ),
            "Expected failure due to missing later settlement block"
        );
    }
}
