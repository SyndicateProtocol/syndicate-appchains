//! The `synd-proposer` polls information from the appchain and posts to the settlement chain
//! `AssertionPoster` contract

use crate::{
    config::Config,
    metrics::ProposerMetrics,
    types::{
        AppVerifyOutput, SeqVerifyOutput, VerifyAppchainConfig, VerifyAppchainInput,
        VerifySequencingChainConfig, VerifySequencingChainInput, VerifySequencingChainOutput,
    },
};
use alloy::{
    consensus::Transaction,
    eips::BlockNumberOrTag,
    network::{Ethereum, EthereumWallet},
    primitives::{fixed_bytes, keccak256, Address, FixedBytes, B256, U256},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{Block, EIP1186AccountProofResponse, Filter},
    signers::local::PrivateKeySigner,
    sol_types::{SolCall, SolEvent, SolValue},
    transports::TransportResult,
};
use contract_bindings::synd::{
    ibridge::IBridge::MessageDelivered,
    iinbox::IInbox::InboxMessageDelivered,
    isequencerinbox::ISequencerInbox::{addSequencerL2BatchFromEigenDACall, SequencerBatchData},
    rollup::Rollup::SequencerBatchDelivered,
    syndicatesequencingchain::SyndicateSequencingChain::TransactionProcessed,
    teemodule::TeeModule::{
        submitAssertionReturn, teeTrustedInputReturn, PendingAssertion, TeeModuleInstance,
    },
};
use eyre::{eyre, OptionExt, Result};
use shared::types::FilledProvider;
use std::{
    collections::{BTreeMap, HashMap},
    default::Default,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use synd_appchain_verifier::types::{
    SequencingChainInput, SettlementChainInput, SyndicateTransactionEvent,
};
use synd_seqchain_verifier::types::{calculate_slot, ArbitrumBatch, L1ChainInput, TimeBounds};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::Mutex,
    task::JoinHandle,
};
use tracing::{debug, error, log::info, warn};
use withdrawals_shared::types::{L1IncomingMessage, L1IncomingMessageHeader};

const EIGENDA_MESSAGE_HEADER_FLAG: u8 = 0xed;
// Slot 7 (0x07) stores the batch count
const _BATCH_ACCUMULATOR_COUNT_SLOT: B256 =
    fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000007");
// Accumulator storage slot for Ethereum L1 - keccak256(0x07)
const BATCH_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688");
// Accumulator storage slot for sequencing chain - Keccak256("syndicate.accumulator")
const SYNDICATE_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0x847fe1a0bfd701c2dbb0b62670ad8712eed4c0ff4d2c6c0917f4c8d260ed0b90");

/// The `Proposer` service is responsible for checking for new trusted input and verifying it using
/// the `synd-enclave` TEE
#[derive(Debug)]
pub struct Proposer {
    appchain_provider: RootProvider,
    sequencing_provider: RootProvider,
    settlement_provider: FilledProvider,
    ethereum_provider: RootProvider,
    enclave_provider: RootProvider,
    polling_interval: Duration,
    metrics: ProposerMetrics,
    tee_module: TeeModuleInstance<(), FilledProvider>,
    last_tee_trusted_input_return_hash: Mutex<B256>,
    l1_arbitrum_bridge_address: Address, // lives on l1chain <> connects to Sequencing chain
    settlement_arbitrum_bridge_address: Address, // lives on settlement l2 <> connects to appchain
    sequencer_inbox_address: Address,
    sequencing_contract_address: Address,
    settlement_delay: u64,
    settlement_inbox_address: Address, // l1 <> settlement
    appchain_inbox_address: Address,   // settlement <> appchain
}

/// Starts the Proposer loop
pub async fn run(config: Config, metrics: ProposerMetrics) -> Result<()> {
    let appchain_provider =
        ProviderBuilder::default().connect(config.appchain_rpc_url.as_str()).await?;
    let sequencing_provider =
        ProviderBuilder::default().connect(config.sequencing_rpc_url.as_str()).await?;
    let ethereum_provider: RootProvider<Ethereum> =
        ProviderBuilder::default().connect(config.ethereum_rpc_url.as_str()).await?;
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {err}"));
    let settlement_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(config.settlement_rpc_url.as_str())
        .await?;

    let tee_module =
        TeeModuleInstance::new(config.tee_module_contract_address, settlement_provider.clone());

    let enclave_provider: RootProvider =
        ProviderBuilder::default().connect(config.enclave_rpc_url.as_str()).await?;

    let proposer = Proposer::new(
        &config,
        metrics,
        appchain_provider,
        sequencing_provider,
        ethereum_provider,
        enclave_provider,
        settlement_provider,
        tee_module,
    );
    let proposer = Arc::new(proposer);

    let polling_task: JoinHandle<Result<()>> =
        tokio::spawn(async move { proposer.polling_loop().await });

    #[allow(clippy::expect_used)]
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    #[allow(clippy::expect_used)]
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {
            info!("Received SIGINT (Ctrl+C), initiating shutdown...");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, initiating shutdown...");
        }
        _ = polling_task => {
            info!("Proposer stopped, initiating shutdown...");
        }
    }
    Ok(())
}

fn hash_tee_trusted_input(input: &teeTrustedInputReturn) -> B256 {
    // Solidity's abi.encodePacked concatenates the bytes without padding
    // For bytes32 values, they're already 32 bytes each
    let mut packed = Vec::new();

    // Append each field in the same order as `TEEModule.sol`
    packed.extend_from_slice(&input.appchainConfigHash.0);
    packed.extend_from_slice(&input.appchainStartBlockHash.0);
    packed.extend_from_slice(&input.seqConfigHash.0);
    packed.extend_from_slice(&input.seqStartBlockHash.0);
    packed.extend_from_slice(&input.setDelayedMessageAcc.0);
    packed.extend_from_slice(&input.l1StartBlockHash.0);
    packed.extend_from_slice(&input.l1EndBlockHash.0);

    keccak256(&packed)
}

/// We can either look for a desired minimum message, or a desired minimum timestamp
fn is_delayed_message_criterion_met(
    msg: &L1IncomingMessage,
    criteria: DelayedMessageCriteria,
) -> bool {
    match criteria {
        DelayedMessageCriteria::MessageIndex(desired_index) => {
            let message_index: u64 = U256::from_be_bytes(msg.header.request_id.0).to::<u64>();

            assert!(
                desired_index <= message_index,
                "desired index is greater than message's, we've iterated too far and can't succeed"
            );
            desired_index == message_index
        }
        DelayedMessageCriteria::MessageTimestamp(desired_ts) => msg.header.timestamp <= desired_ts,
    }
}

enum PreimageSource {
    Sequencing,
    Appchain,
}

#[allow(dead_code)]
enum ChainProvider {
    L1,
    Sequencing,
    Settlement,
    Appchain,
}

enum DelayedMessagesSource {
    L1,
    Settlement,
}

#[derive(Clone, Copy)]
enum DelayedMessageCriteria {
    MessageIndex(u64),
    MessageTimestamp(u64),
}

impl Proposer {
    /// Create a new [`Proposer`]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: &Config,
        metrics: ProposerMetrics,
        appchain_provider: RootProvider,
        sequencing_provider: RootProvider,
        ethereum_provider: RootProvider,
        enclave_provider: RootProvider,
        _settlement_provider: FilledProvider,
        tee_module: TeeModuleInstance<(), FilledProvider>,
    ) -> Self {
        Self {
            appchain_provider,
            sequencing_provider,
            settlement_provider: _settlement_provider,
            ethereum_provider,
            enclave_provider,
            polling_interval: config.polling_interval,
            tee_module,
            metrics,
            last_tee_trusted_input_return_hash: Mutex::new(B256::default()),
            l1_arbitrum_bridge_address: config.l1_arbitrum_bridge_address,
            settlement_arbitrum_bridge_address: config.settlement_arbitrum_bridge_address,
            sequencer_inbox_address: config.sequencer_inbox_address,
            sequencing_contract_address: config.sequencing_contract_address,
            settlement_delay: config.settlement_delay,
            settlement_inbox_address: config.settlement_inbox_address,
            appchain_inbox_address: config.appchain_inbox_address,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    async fn polling_loop(&self) -> Result<()> {
        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            interval.tick().await;
            let tee_trusted_input = match self.tee_module.teeTrustedInput().call().await {
                Ok(trusted_input) => {
                    if self.check_was_tee_input_already_processed(&trusted_input) {
                        error!("TeeTrustedInput previously processed. Skipping");
                        continue
                    }
                    trusted_input
                }
                Err(e) => {
                    error!(%e, "failed to fetch latest TeeTrustedInput");
                    continue
                }
            };

            let _submit_assertion_return = match self.tee_verify(&tee_trusted_input).await {
                Ok(value) => value,
                Err(e) => {
                    error!(%e, "failed to verify TeeTrustedInput");
                    continue
                }
            };

            //TODO - is this Proposer.main_loop() responsible for contract's
            // closeChallengeWindow()? separate loop? or done automatically by contract?
        }
    }

    /// Core processing
    async fn tee_verify(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
    ) -> Result<submitAssertionReturn> {
        let verify_seq_output = match self.verify_sequencing_chain(tee_trusted_input).await {
            Ok(output) => output,
            Err(e) => {
                error!(%e, "failed to verify sequencing chain input");
                return Err(e);
            }
        };

        let verify_appchain_output =
            match self.verify_appchain(tee_trusted_input, verify_seq_output).await {
                Ok(output) => output,
                Err(e) => {
                    error!(%e, "failed to verify appchain input");
                    return Err(e);
                }
            };

        let assertion = PendingAssertion {
            blockHash: verify_appchain_output.block_hash,
            sendRoot: verify_appchain_output.send_root,
            seqBlockHash: verify_appchain_output.seq_block_hash,
        };
        let submit_assertion_return = match self
            .tee_module
            .submitAssertion(assertion, verify_appchain_output.signature)
            .call()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                error!(%e, "failed to submit assertion");
                return Err(e.into());
            }
        };
        Ok(submit_assertion_return)
    }

    async fn verify_appchain(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
        verify_seq_output: SeqVerifyOutput,
    ) -> Result<AppVerifyOutput> {
        let seq_end_block_hash = verify_seq_output.block_hash;
        let (seq_start_block, seq_end_block) = self
            .get_start_end_block(
                ChainProvider::Sequencing,
                tee_trusted_input.seqStartBlockHash,
                seq_end_block_hash,
            )
            .await
            .ok_or_eyre("failed to get sequencing start and end blocks")?;

        let sequencing_chain_input = self
            .build_sequencing_chain_input(
                tee_trusted_input,
                &seq_start_block,
                &seq_end_block,
                seq_end_block_hash,
            )
            .await?;

        let settlement_chain_input = self
            .build_settlement_chain_input(tee_trusted_input, seq_end_block, &sequencing_chain_input)
            .await?;

        // TODO - what is appchain end block?
        // TODO pranav will update the preimage endpoint
        let appchain_end_block_hash = Default::default(); // TODO populate
        let (appchain_start_block, appchain_end_block) = self
            .get_start_end_block(
                ChainProvider::Appchain,
                tee_trusted_input.appchainStartBlockHash,
                appchain_end_block_hash,
            )
            .await
            .ok_or_eyre("failed to get appchain start and end blocks")?;

        let appchain_pre_image_data: Vec<Vec<u8>> = self
            .arb_debug_preimage_data(
                PreimageSource::Appchain,
                appchain_start_block.header.number,
                appchain_end_block.header.number,
            )
            .await?;

        let verify_appchain_input = VerifyAppchainInput {
            seq_config_hash: tee_trusted_input.seqConfigHash,
            l1_start_block_hash: tee_trusted_input.l1StartBlockHash,
            l1_end_block_hash: tee_trusted_input.l1EndBlockHash,
            appchain_config_hash: tee_trusted_input.appchainConfigHash,
            verify_appchain_config: VerifyAppchainConfig {
                sequencing_contract_address: self.sequencing_contract_address,
                settlement_delay: self.settlement_delay,
            },
            settlement_chain_input,
            sequencing_chain_input,
            appchain_pre_image_data,
            verify_sequencing_chain_output: VerifySequencingChainOutput {
                sequencing_block_hash: verify_seq_output.block_hash,
                signature: verify_seq_output.signature,
            },
            appchain_start_block_hash: tee_trusted_input.appchainStartBlockHash,
        };
        let input = serde_json::to_string(&verify_appchain_input)?;
        let app_verify_output = self.enclave_verify_appchain(input).await?;
        Ok(app_verify_output)
    }

    async fn build_settlement_chain_input(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
        seq_end_block: Block,
        sequencing_chain_input: &SequencingChainInput,
    ) -> Result<SettlementChainInput> {
        let first_syndicate_event_ts = sequencing_chain_input
            .syndicate_transaction_events
            .first()
            .ok_or_eyre("no Syndicate transaction events found")?
            .timestamp;
        let desired_ts = first_syndicate_event_ts - self.settlement_delay;

        let (delayed_messages, start_delayed_messages_accumulator) = self
            .get_delayed_messages(
                DelayedMessagesSource::Settlement,
                DelayedMessageCriteria::MessageTimestamp(desired_ts),
                seq_end_block.header.number,
            )
            .await?;

        let settlement_chain_input = SettlementChainInput {
            delayed_messages,
            start_delayed_messages_accumulator,
            end_delayed_messages_accumulator: tee_trusted_input.setDelayedMessageAcc,
        };
        Ok(settlement_chain_input)
    }

    async fn build_sequencing_chain_input(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
        seq_start_block: &Block,
        seq_end_block: &Block,
        seq_end_block_hash: B256,
    ) -> Result<SequencingChainInput> {
        let start_syndicate_accumulator_merkle_proof = self
            .get_accumulator_proof(ChainProvider::Sequencing, seq_start_block.header.number)
            .await?;

        let end_syndicate_accumulator_merkle_proof = self
            .get_accumulator_proof(ChainProvider::Sequencing, seq_end_block.header.number)
            .await?;

        let mut seq_block_headers_in_range = Vec::new();
        for block_number in [seq_start_block.header.number, seq_end_block.header.number] {
            #[allow(clippy::unwrap_used)]
            let block = self
                .sequencing_provider
                .get_block_by_number(BlockNumberOrTag::Number(block_number))
                .await?
                .unwrap();
            seq_block_headers_in_range.push(block.header);
        }

        let transaction_processed = Filter::new()
            .address(self.sequencing_contract_address) // Inherits from Syndicate Accumulator contract
            .from_block(seq_start_block.header.number)
            .to_block(seq_end_block.header.number)
            .event_signature(TransactionProcessed::SIGNATURE_HASH);
        let syndicate_transaction_logs =
            self.sequencing_provider.get_logs(&transaction_processed).await?;

        let mut syndicate_transaction_events: Vec<SyndicateTransactionEvent> = Vec::new();
        for log in syndicate_transaction_logs {
            match TransactionProcessed::decode_log(log.as_ref(), true) {
                Err(e) => {
                    panic!("TransactionProcessed decoding failed: {e}");
                }
                Ok(decoded) => {
                    #[allow(clippy::unwrap_used)]
                    let idx = (log.block_number.unwrap() - seq_start_block.header.number) as usize;
                    let timestamp = seq_block_headers_in_range[idx].timestamp;

                    let txn_processed = decoded.data;
                    let event = SyndicateTransactionEvent {
                        #[allow(clippy::unwrap_used)]
                        block_number: log.block_number.unwrap(),
                        timestamp,
                        sender: txn_processed.sender,
                        payload: txn_processed.data, // TODO check this
                    };

                    syndicate_transaction_events.push(event);
                }
            }
        }

        let sequencing_chain_input = SequencingChainInput {
            start_syndicate_accumulator_merkle_proof,
            end_syndicate_accumulator_merkle_proof,
            syndicate_transaction_events,
            block_headers: seq_block_headers_in_range,
            start_block_hash: tee_trusted_input.seqStartBlockHash,
            end_block_hash: seq_end_block_hash,
        };
        Ok(sequencing_chain_input)
    }

    async fn verify_sequencing_chain(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
    ) -> Result<SeqVerifyOutput> {
        let (l1_chain_input, seq_start_block) = self.build_l1_input(tee_trusted_input).await?;

        //TODO - this is wrong, need sequencing l1 start and end. Q - how to get seq end block?
        // Pranav will update the  preimage endpoint
        let seq_end_block_number = 0; // TODO get seq_end_block_hash once endpoint is updated
        let sequencing_pre_image_data: Vec<Vec<u8>> = self
            .arb_debug_preimage_data(
                PreimageSource::Sequencing,
                seq_start_block.header.number,
                seq_end_block_number,
            )
            .await?;

        let verify_seq_chain_input = VerifySequencingChainInput {
            seq_config_hash: tee_trusted_input.seqConfigHash,
            verify_sequencing_chain_config: VerifySequencingChainConfig {
                arbitrum_bridge_address: self.l1_arbitrum_bridge_address,
            },
            l1_chain_input,
            sequencing_start_block_hash: tee_trusted_input.seqStartBlockHash,
            sequencing_pre_image_data,
        };

        let verify_seq_chain_input_string = serde_json::to_string(&verify_seq_chain_input)?;

        let seq_verify_output =
            self.enclave_verify_sequencing_chain(verify_seq_chain_input_string).await?;

        Ok(seq_verify_output)
    }

    async fn get_start_end_block(
        &self,
        provider_type: ChainProvider,
        start_block_hash: B256,
        end_block_hash: B256,
    ) -> Option<(Block, Block)> {
        let provider = match provider_type {
            ChainProvider::L1 => &self.ethereum_provider,
            ChainProvider::Sequencing => &self.sequencing_provider,
            ChainProvider::Settlement => {
                panic!("not in use for settlement chain")
            }
            ChainProvider::Appchain => &self.appchain_provider,
        };
        let l1_start_block = self.fetch_block_by_hash(provider, start_block_hash, "start").await?;
        let l1_end_block = self.fetch_block_by_hash(provider, end_block_hash, "end").await?;

        Some((l1_start_block, l1_end_block))
    }

    async fn fetch_block_by_hash<P: Provider>(
        &self,
        provider: P,
        hash: B256,
        block_type: &str,
    ) -> Option<Block> {
        match provider.get_block_by_hash(hash).await {
            Ok(Some(block)) => Some(block),
            Ok(None) => {
                error!(%hash, "L1 {block_type} block not found");
                None
            }
            Err(e) => {
                error!(%e, %hash, "Error fetching L1 {block_type} block");
                None
            }
        }
    }

    fn check_was_tee_input_already_processed(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
    ) -> bool {
        let hashed_tee_trusted_input = hash_tee_trusted_input(tee_trusted_input);
        let mut last_hash = self.last_tee_trusted_input_return_hash.blocking_lock();
        if *last_hash == hashed_tee_trusted_input {
            return true;
        }
        // Update the hash
        *last_hash = hashed_tee_trusted_input;
        false
    }

    async fn arb_debug_preimage_data(
        &self,
        source: PreimageSource,
        start_block_num: u64,
        end_block_num: u64,
    ) -> TransportResult<Vec<Vec<u8>>> {
        let provider = match source {
            PreimageSource::Sequencing => &self.sequencing_provider,
            PreimageSource::Appchain => &self.appchain_provider,
        };

        provider
            .raw_request(
                "arbdebug_preimageData".into(),
                vec![
                    BlockNumberOrTag::Number(start_block_num),
                    BlockNumberOrTag::Number(end_block_num),
                ],
            )
            .await
    }

    async fn enclave_verify_sequencing_chain(
        &self,
        verify_seq_chain_input: String,
    ) -> TransportResult<SeqVerifyOutput> {
        self.enclave_provider
            .raw_request("enclave_verifySequencingChain".into(), verify_seq_chain_input)
            .await
    }

    async fn enclave_verify_appchain(
        &self,
        verify_appchain_input: String,
    ) -> TransportResult<AppVerifyOutput> {
        self.enclave_provider
            .raw_request("enclave_verifyAppchain".into(), verify_appchain_input)
            .await
    }

    #[allow(clippy::cognitive_complexity)]
    async fn build_l1_input(
        &self,
        tee_trusted_input: &teeTrustedInputReturn,
    ) -> Result<(L1ChainInput, Block)> {
        let (l1_start_block, l1_end_block) = self
            .get_start_end_block(
                ChainProvider::L1,
                tee_trusted_input.l1StartBlockHash,
                tee_trusted_input.l1EndBlockHash,
            )
            .await
            .ok_or_eyre("failed to get l1 start and end blocks")?;

        let seq_start_block = self
            .sequencing_provider
            .get_block_by_hash(tee_trusted_input.seqStartBlockHash)
            .await?
            .ok_or_eyre("failed to get sequencing start block")?;

        debug!(
            "build_l1_input - l1_start_block: {} l1_end_block: {} seq_start_block: {}",
            l1_start_block.header.number, l1_end_block.header.number, seq_start_block.header.number
        );

        let batches =
            self.get_batches(l1_start_block.header.number, l1_end_block.header.number).await?;
        debug!("L1 batches - count: {}", batches.len());
        debug!("L1 batches within range: {batches:?}");

        let delayed_message_count: u64 = seq_start_block.header.nonce.into();
        let (delayed_messages, _first_msg_acc) = self
            .get_delayed_messages(
                DelayedMessagesSource::L1,
                DelayedMessageCriteria::MessageIndex(delayed_message_count),
                l1_end_block.header.number,
            )
            .await?;

        let start_batch_accumulator_merkle_proof =
            self.get_accumulator_proof(ChainProvider::L1, l1_start_block.header.number).await?;

        let end_batch_accumulator_merkle_proof =
            self.get_accumulator_proof(ChainProvider::L1, l1_end_block.header.number).await?;

        debug!(
            "L1 Accumulator proof data - start batch count: {}, end batch count: {}, start accumulator: {}, end accumulator: {}",
            start_batch_accumulator_merkle_proof.storage_proof[0].value,
            end_batch_accumulator_merkle_proof.storage_proof[0].value,
            start_batch_accumulator_merkle_proof.storage_proof[1].value,
            end_batch_accumulator_merkle_proof.storage_proof[1].value
        );
        let start_block_header = l1_start_block.header.clone();
        let end_block_header = l1_end_block.header.clone();
        let input = L1ChainInput {
            start_batch_accumulator_merkle_proof,
            end_batch_accumulator_merkle_proof,
            start_block_header,
            end_block_header,
            delayed_messages,
            batches,
            start_block_hash: tee_trusted_input.l1StartBlockHash,
            end_block_hash: tee_trusted_input.l1EndBlockHash,
        };
        Ok((input, seq_start_block))
    }

    // only called once, for L1
    // appchain will use SyndicateTransactionEvents
    // previous batch num - up to end block - setDelayedMessageAcc
    async fn get_batches(&self, start_block: u64, end_block: u64) -> Result<Vec<ArbitrumBatch>> {
        let batch_filter = Filter::new()
            .address(self.sequencer_inbox_address)
            .from_block(start_block)
            .to_block(end_block)
            .event_signature(vec![
                SequencerBatchData::SIGNATURE_HASH,
                SequencerBatchDelivered::SIGNATURE_HASH,
            ]);
        let batch_event_logs = self.ethereum_provider.get_logs(&batch_filter).await?;

        let mut batch_map: BTreeMap<U256, ArbitrumBatch> = BTreeMap::new();

        for log in batch_event_logs {
            match log.topics()[0] {
                SequencerBatchData::SIGNATURE_HASH => {
                    match SequencerBatchData::decode_log(log.as_ref(), true) {
                        Ok(decoded) => {
                            let arb_batch =
                                batch_map.entry(decoded.batchSequenceNumber).or_default();
                            arb_batch.data = decoded.data.data;
                        }
                        Err(e) => {
                            panic!("SequencerBatchData decoding failed: {e}");
                        }
                    }
                }
                SequencerBatchDelivered::SIGNATURE_HASH => {
                    match SequencerBatchDelivered::decode_log(log.as_ref(), true) {
                        Ok(decoded) => {
                            // Batch data may be located on EigenDA, which is enum 4 in their
                            // version of the contract event https://github.com/Layr-Labs/nitro-contracts/blob/278fdbc39089fa86330f0c23f0a05aee61972c84/src/bridge/IBridge.sol#L25
                            // decoded.dataLocation.
                            let arb_batch =
                                batch_map.entry(decoded.batchSequenceNumber).or_default();
                            if decoded.dataLocation == 4 {
                                #[allow(clippy::unwrap_used)]
                                let txn = self
                                    .ethereum_provider
                                    .get_transaction_by_hash(log.transaction_hash.unwrap())
                                    .await?
                                    .unwrap();
                                let envelope = txn.inner.into_inner();
                                let res = addSequencerL2BatchFromEigenDACall::abi_decode(
                                    envelope.input(),
                                    true,
                                )?;
                                let data = ([EIGENDA_MESSAGE_HEADER_FLAG], res.cert.abi_encode())
                                    .abi_encode_packed();
                                arb_batch.data = data.into();
                            }

                            arb_batch.time_bounds = TimeBounds {
                                min_timestamp: decoded.timeBounds.minTimestamp,
                                max_timestamp: decoded.timeBounds.maxTimestamp,
                                min_block_number: decoded.timeBounds.minBlockNumber,
                                max_block_number: decoded.timeBounds.maxBlockNumber,
                            };
                            arb_batch.delayed_acc = decoded.delayedAcc;
                            arb_batch.after_delayed_messages_read =
                                decoded.afterDelayedMessagesRead;
                        }
                        Err(e) => {
                            panic!("decoding failed: {} {:?}", "SequencerBatchDelivered", e);
                        }
                    }
                }
                event_type => {
                    panic!("unexpected batch event type hash: {event_type}")
                }
            }
        }

        let batches: Vec<ArbitrumBatch> = batch_map.into_values().collect();
        Ok(batches)
    }

    /// Get all delayed messages between `start_criteria` and `end_block` using a pagination
    /// algorithm.
    async fn get_delayed_messages(
        &self,
        delayed_message_source: DelayedMessagesSource,
        start_criteria: DelayedMessageCriteria,
        end_block: u64,
    ) -> Result<(Vec<L1IncomingMessage>, FixedBytes<32>)> {
        let mut messages: Vec<L1IncomingMessage> = Vec::new();
        let page_diff_num_blocks = 1000; // TODO tweak me for performance. Could be config?
        let mut curr_start = end_block.saturating_sub(page_diff_num_blocks);
        let mut curr_end = end_block + 1;

        let (message_delivered_address, inbox_message_delivered_address) =
            match delayed_message_source {
                DelayedMessagesSource::L1 => {
                    (self.l1_arbitrum_bridge_address, self.settlement_inbox_address)
                }
                DelayedMessagesSource::Settlement => {
                    (self.settlement_inbox_address, self.appchain_inbox_address)
                }
            };

        // Pagination algo
        loop {
            let curr_end_minus_1 = curr_end - 1;
            let (delayed_msgs, first_msg_acc) = self
                .parse_delayed_messages(
                    curr_start,
                    curr_end_minus_1,
                    message_delivered_address,
                    inbox_message_delivered_address,
                )
                .await?;

            // push to `msgs` in reverse order, most recent to the least recent
            // reverse `msgs` at the very end to return correct order
            for msg in delayed_msgs.into_iter().rev() {
                let is_met = is_delayed_message_criterion_met(&msg, start_criteria);
                messages.push(msg);

                // Happy path
                if is_met {
                    messages.reverse();
                    return Ok((messages, first_msg_acc));
                }
            }

            // Panic if we have exhausted all blocks prior to `end_block`
            assert!(
                curr_end >= page_diff_num_blocks,
                "all block messages in range exhausted without success. No more messages to check"
            );

            curr_start = curr_start.saturating_sub(page_diff_num_blocks);
            curr_end -= page_diff_num_blocks;
        }
    }

    /// This function parses a vec of delayed messages from the range of blocks. It listens for 2
    /// events to combine the data and metadata for each message event, which are uniquely indexed
    /// by messageIndex/requestId.
    async fn parse_delayed_messages(
        &self,
        start_block: u64,
        end_block: u64,
        bridge_address: Address,
        inbox_address: Address,
    ) -> Result<(Vec<L1IncomingMessage>, FixedBytes<32>)> {
        let message_delivered_filter = Filter::new()
            .address(bridge_address)
            .from_block(start_block)
            .to_block(end_block)
            .event_signature(MessageDelivered::SIGNATURE_HASH);
        let message_delivered_logs =
            self.ethereum_provider.get_logs(&message_delivered_filter).await?;

        // Parse bridge events into a map for easy lookup
        let mut message_delivered_metadata = HashMap::new();
        for log in message_delivered_logs {
            let event = MessageDelivered::decode_log(log.as_ref(), true)?;
            message_delivered_metadata.insert(event.messageIndex, event);
        }

        // for deposits
        let inbox_filter = Filter::new()
            .address(inbox_address)
            .from_block(start_block)
            .to_block(end_block)
            .event_signature(InboxMessageDelivered::SIGNATURE_HASH);

        let inbox_message_delivered_logs = self.ethereum_provider.get_logs(&inbox_filter).await?;

        // Combine the data
        let mut delayed_messages = Vec::new();
        for log in inbox_message_delivered_logs {
            let event_signature_hash = log.topics()[0];
            if event_signature_hash != InboxMessageDelivered::SIGNATURE_HASH {
                warn!(
                    "{} {event_signature_hash}",
                    "found non-InboxMessageDelivered event that will be ignored",
                );
                continue;
            }

            let event = InboxMessageDelivered::decode_log(log.as_ref(), true)?;

            // Look up the metadata from MessageDelivered event
            let metadata = message_delivered_metadata.get(&event.messageNum).ok_or_else(|| {
                eyre!("Missing MessageDelivered event for message {}", event.messageNum)
            })?;

            let message = L1IncomingMessage {
                header: L1IncomingMessageHeader {
                    kind: metadata.kind,
                    sender: metadata.sender,
                    #[allow(clippy::unwrap_used)]
                    block_number: log.block_number.unwrap(),
                    timestamp: metadata.timestamp,
                    request_id: B256::from(event.messageNum),
                    base_fee_l1: metadata.baseFeeL1,
                },
                l2msg: event.data.data,
            };

            delayed_messages.push(message);
        }
        // Sort by message index to ensure correct order
        delayed_messages.sort_by_key(|m| U256::from_be_bytes(m.header.request_id.0));

        let first_block =
            delayed_messages.first().ok_or_eyre("no delayed messages found")?.header.request_id;
        let first_block_index = U256::from_be_bytes(first_block.0);
        let first_msg_acc = message_delivered_metadata
            .get(&first_block_index)
            .ok_or_eyre("failed to find metadata for first delayed message")?
            .beforeInboxAcc;

        Ok((delayed_messages, first_msg_acc))
    }

    /// Get proofs for a chain and block number via `eth_getProof`
    async fn get_accumulator_proof(
        &self,
        provider_type: ChainProvider,
        block_number: u64,
    ) -> Result<EIP1186AccountProofResponse> {
        match provider_type {
            ChainProvider::Settlement => {
                panic!("settlement proofs are unsupported")
            }
            ChainProvider::Appchain => {
                panic!("appchain proofs are unsupported")
            }
            ChainProvider::L1 => {
                let bridge_address = self.l1_arbitrum_bridge_address;
                let storage_slot = BATCH_ACCUMULATOR_STORAGE_SLOT;
                let provider = &self.ethereum_provider;

                // Get the sequencer inbox accumulator count (batch count)
                // Note: This needs to be done at the specific block
                let batch_count =
                    self.get_batch_count_at_block(bridge_address, block_number).await?;

                debug!("Accumulator proof - block_number: {block_number}, batch_count: {batch_count}, storage slot: {storage_slot}");

                // Calculate the slot for the accumulator at this index
                let accumulator_slot = calculate_slot(storage_slot, batch_count);

                // Get proof for both the count slot and the accumulator slot
                let proof = provider
                    .get_proof(
                        bridge_address,
                        vec![
                            storage_slot,
                            accumulator_slot, // Calculated slot: accumulator value
                        ],
                    )
                    .block_id(block_number.into())
                    .await?;

                Ok(proof)
            }
            ChainProvider::Sequencing => {
                let bridge_address = self.sequencing_contract_address;
                let storage_slot = SYNDICATE_ACCUMULATOR_STORAGE_SLOT;
                let provider = &self.sequencing_provider;

                // Nothing to calculate
                let proof = provider
                    .get_proof(bridge_address, vec![storage_slot])
                    .block_id(block_number.into())
                    .await?;

                Ok(proof)
            }
        }
    }

    async fn get_batch_count_at_block(
        &self,
        bridge_address: Address,
        block_number: u64,
    ) -> Result<U256> {
        // Get the value at storage slot 7 (batch count)
        let count = self
            .ethereum_provider
            .get_storage_at(bridge_address, BATCH_ACCUMULATOR_STORAGE_SLOT.into())
            .block_id(block_number.into())
            .await?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use crate::{metrics::ProposerMetrics, proposer::Proposer, types::SeqVerifyOutput};
    use alloy::{
        eips::BlockNumberOrTag,
        network::EthereumWallet,
        primitives::{address, Address, BlockHash, B256},
        providers::{Provider, ProviderBuilder, RootProvider},
        rpc::types::{Block, Filter},
    };
    use contract_bindings::synd::teemodule::TeeModule::{teeTrustedInputReturn, TeeModuleInstance};
    use eyre::{Error, OptionExt};
    use std::{default::Default, str::FromStr, sync::Arc, time::Duration};
    use synd_appchain_verifier::config::AppchainVerifierConfig;
    use synd_seqchain_verifier::{
        config::SeqchainVerifierConfig, types::L1ChainInput, verifier::Verifier,
    };
    use tokio::sync::Mutex;
    use url::Url;

    impl Default for Proposer {
        fn default() -> Self {
            let url_str = "localhost:8080";
            #[allow(clippy::unwrap_used)]
            let dummy_url = Url::from_str(url_str).unwrap();
            let dummy_root_provider = ProviderBuilder::default().on_http(dummy_url.clone());
            let dummy_filled_provider =
                ProviderBuilder::new().wallet(EthereumWallet::default()).on_http(dummy_url);
            let tee_module = TeeModuleInstance::new(Address::ZERO, dummy_filled_provider.clone());

            Self {
                appchain_provider: dummy_root_provider.clone(),
                sequencing_provider: dummy_root_provider.clone(),
                settlement_provider: dummy_filled_provider,
                ethereum_provider: dummy_root_provider.clone(),
                enclave_provider: dummy_root_provider,
                polling_interval: Duration::from_secs(1),
                metrics: ProposerMetrics::default(),
                tee_module,
                last_tee_trusted_input_return_hash: Mutex::new(B256::default()),
                l1_arbitrum_bridge_address: Address::ZERO,
                settlement_arbitrum_bridge_address: Address::ZERO,
                sequencer_inbox_address: Address::ZERO,
                sequencing_contract_address: Address::ZERO,
                settlement_delay: Default::default(),
                settlement_inbox_address: Address::ZERO,
                appchain_inbox_address: Default::default(),
            }
        }
    }

    const L1_ARBITRUM_BRIDGE_ADDRESS: Address =
        address!("0x27b5BA9331f20afd816C247d53BDf1EC577b04CD");
    const L1_SEQUENCER_INBOX_ADDRESS: Address =
        address!("0x47c3DEC256DB25c527d92AAceE1269C17805ce9d");

    /// This test produces a JSON file of the `L1ChainInput` that is passed into the
    /// `seq-chain-verifier` based on the most recent content in the `SEQUENCER_INBOX_ADDRESS`.
    /// It currently relies on a Chainstack Eth Holesky RPC node that doesn't support archived
    /// blocks, so the call to `eth_getProof` will only succeed within ~20min of the latest
    /// transaction to the `SEQUENCER_INBOX_ADDRESS` contract address. Therefore, this test will
    /// intermittently fail
    #[ignore]
    #[tokio::test]
    async fn test_proposer_build_l1_input() {
        shared::tracing::setup_global_logging().expect("logging setup failed");

        let ethereum_provider = ProviderBuilder::default()
            .connect(
                "https://ethereum-holesky.core.chainstack.com/71e6c446fa619c2e05cb50884da82f4a",
            )
            .await
            .unwrap();
        let sequencing_provider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();
        let proposer = Proposer {
            ethereum_provider: ethereum_provider.clone(),
            sequencing_provider: sequencing_provider.clone(),
            l1_arbitrum_bridge_address: L1_ARBITRUM_BRIDGE_ADDRESS,
            sequencer_inbox_address: L1_SEQUENCER_INBOX_ADDRESS,
            ..Default::default()
        };

        let ethereum_provider_arc: Arc<RootProvider> = Arc::new(ethereum_provider);
        let sequencing_provider_arc: Arc<RootProvider> = Arc::new(sequencing_provider);

        // SequencerInbox contract address on Holesky L1 - https://eth-holesky.blockscout.com/address/0x47c3DEC256DB25c527d92AAceE1269C17805ce9d?tab=txs
        // Input - includes 1 batch
        let (l1_latest_block, l1_latest_tx_hash) = get_latest_contract_txn_block(
            ethereum_provider_arc.clone(),
            L1_SEQUENCER_INBOX_ADDRESS,
        )
        .await
        .unwrap();

        let l1_latest_block_hash = l1_latest_block.header.hash;
        let l1_latest_block_ts = l1_latest_block.header.timestamp;
        println!("Latest L1 sequencer_inbox transaction + block:");
        println!("  Block Number: {}", l1_latest_block.header.number);
        println!("  Block Hash: {l1_latest_block_hash}");
        println!("  Timestamp: {l1_latest_block_ts}");
        println!("  Transaction Hash: {l1_latest_tx_hash}");

        let n = 1;
        let l1_latest_block_minus_n_number = l1_latest_block.header.number - n;
        let l1_latest_block_minus_n: Block = ethereum_provider_arc
            .get_block_by_number(BlockNumberOrTag::Number(l1_latest_block_minus_n_number))
            .await
            .unwrap()
            .ok_or("block {} doesn't exist")
            .unwrap();
        let l1_latest_block_minus_n_hash = l1_latest_block_minus_n.header.hash;
        let _l1_latest_block_minus_n_ts = l1_latest_block_minus_n.header.timestamp;
        println!("Latest minus n ({n}) L1 sequencer_inbox block:");
        println!("  Block Number: {}", l1_latest_block_minus_n.header.number);
        println!("  Block Hash: {:?}", l1_latest_block_minus_n.header.hash);
        println!("  Block Timestamp: {}", l1_latest_block_minus_n.header.timestamp);

        // Getting latest seq block, then stepping back until the timestamp is within the l1
        // start-end range. This range will yield 1 batch.
        let seq_latest_block_number = sequencing_provider_arc.get_block_number().await.unwrap();
        let mut curr_block_number = seq_latest_block_number;

        let mut _desired_seq_start_block_hash: Option<BlockHash> = None;
        loop {
            let seq_latest_block = sequencing_provider_arc
                .get_block_by_number(BlockNumberOrTag::Number(curr_block_number))
                .await
                .unwrap()
                .ok_or_else(|| {
                    format!(
                        "seq block {curr_block_number}, which is {n} blocks back, doesn't exist"
                    )
                })
                .unwrap();
            if seq_latest_block.header.timestamp < l1_latest_block_ts {
                println!("Latest seq block to use:");
                println!("  Block Number: {}", seq_latest_block.header.number);
                println!("  Block Hash: {:?}", seq_latest_block.header.hash);
                println!("  Block Timestamp: {}", seq_latest_block.header.timestamp);
                _desired_seq_start_block_hash = Some(seq_latest_block.header.hash);
                break;
            }
            curr_block_number -= 1;
        }
        let mock_input = teeTrustedInputReturn {
            appchainConfigHash: Default::default(),
            appchainStartBlockHash: Default::default(),
            seqConfigHash: Default::default(),
            setDelayedMessageAcc: Default::default(),

            seqStartBlockHash: _desired_seq_start_block_hash.unwrap(),
            l1StartBlockHash: l1_latest_block_minus_n_hash,
            l1EndBlockHash: l1_latest_block_hash,
        };
        println!(
            "mock_input. seqStartBlockHash: {} l1StartBlockHash: {} l1EndBlockHash: {}",
            mock_input.seqStartBlockHash, mock_input.l1StartBlockHash, mock_input.l1EndBlockHash
        );

        let (l1_chain_input, _seq_start_block) =
            proposer.build_l1_input(&mock_input).await.unwrap();

        let seq_chain_verifier = Verifier::new(&SeqchainVerifierConfig {
            arbitrum_bridge_address: L1_ARBITRUM_BRIDGE_ADDRESS,
        });
        let l1_chain_input_json = serde_json::to_string_pretty(&l1_chain_input);
        std::fs::write("./tests/l1_chain_input_eigenda.json", l1_chain_input_json.unwrap())
            .unwrap();
        let res = seq_chain_verifier.verify_and_create_output(&l1_chain_input);
        assert!(res.is_ok());
    }

    /// Same as `test_proposer_build_l1_input_from_file()` above, except this test parses the
    /// JSON artifact produced by that test and verifies it. To update the artifact, run the test
    /// above.
    #[tokio::test]
    async fn test_proposer_build_l1_input_from_file() {
        let seq_chain_verifier = Verifier::new(&SeqchainVerifierConfig {
            arbitrum_bridge_address: L1_ARBITRUM_BRIDGE_ADDRESS,
        });
        let json_content = std::fs::read_to_string("./tests/l1_chain_input_eigenda.json").unwrap();
        let input: L1ChainInput = serde_json::from_str(&json_content).unwrap();
        let res = seq_chain_verifier.verify_and_create_output(&input);
        assert!(res.is_ok());
    }

    /// Given a contract address, get the latest txn call and the block that it belongs to
    async fn get_latest_contract_txn_block(
        ethereum_provider: Arc<RootProvider>,
        sequencer_inbox: Address,
    ) -> Result<(Block, BlockHash), Error> {
        let holesky_latest_block = ethereum_provider.get_block_number().await?;
        let holesky_from_block = holesky_latest_block.saturating_sub(10000); // hardcoded cosntant
        let filter = Filter::new()
            .address(sequencer_inbox)
            .from_block(holesky_from_block)
            .to_block(holesky_latest_block);
        let logs = ethereum_provider.get_logs(&filter).await?;

        assert!(!logs.is_empty(), "No logs returned");
        let latest_log = logs.last().unwrap();
        let block_hash = latest_log.block_hash.ok_or_eyre("Block hash not found")?;
        let tx_hash = latest_log.transaction_hash.ok_or_eyre("Transaction hash not found")?;
        let block =
            ethereum_provider.get_block_by_hash(block_hash).await?.ok_or_eyre("Block not found")?;
        Ok((block, tx_hash))
    }

    ///TODO will need a `SeqVerifyOutput` to start from, either a mock or real saved JSON artifact
    /// from the `synd-enclave`. Possible that an E2E test is more useful for this
    #[tokio::test]
    async fn test_proposer_build_seq_and_sett_chain_input() {
        shared::tracing::setup_global_logging().expect("logging setup failed");

        let ethereum_provider = ProviderBuilder::default()
            .connect(
                "https://ethereum-holesky.core.chainstack.com/71e6c446fa619c2e05cb50884da82f4a",
            )
            .await
            .unwrap();
        let sequencing_provider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();

        // TODO
        let proposer = Proposer {
            ethereum_provider: ethereum_provider.clone(),
            sequencing_provider: sequencing_provider.clone(),
            l1_arbitrum_bridge_address: L1_ARBITRUM_BRIDGE_ADDRESS,
            sequencer_inbox_address: L1_SEQUENCER_INBOX_ADDRESS,
            ..Default::default()
        };

        let ethereum_provider_arc: Arc<RootProvider> = Arc::new(ethereum_provider);
        let sequencing_provider_arc: Arc<RootProvider> = Arc::new(sequencing_provider);

        // TODO
        let mock_input = teeTrustedInputReturn {
            appchainConfigHash: Default::default(),
            appchainStartBlockHash: Default::default(),
            seqConfigHash: Default::default(),
            setDelayedMessageAcc: Default::default(),
            seqStartBlockHash: Default::default(),
            l1StartBlockHash: Default::default(),
            l1EndBlockHash: Default::default(),
        };
        let seq_end_block_hash: B256 = Default::default();
        let seq_start_block: Block = Block::default();
        let seq_end_block: Block = Block::default();
        let seq_chain_input = proposer
            .build_sequencing_chain_input(
                &mock_input,
                &seq_start_block,
                &seq_end_block,
                seq_end_block_hash,
            )
            .await
            .unwrap();

        //TODO
        let seq_verify_output =
            SeqVerifyOutput { block_hash: Default::default(), signature: Default::default() };

        let sett_chain_input = proposer
            .build_settlement_chain_input(&mock_input, seq_end_block, &seq_chain_input)
            .await
            .unwrap();

        let appchain_verifier =
            synd_appchain_verifier::verifier::Verifier::new(&AppchainVerifierConfig {
                sequencing_contract_address: Default::default(),
                settlement_delay: 0,
            });

        let res = appchain_verifier.verify_and_create_output(&seq_chain_input, &sett_chain_input);
        assert!(res.is_ok());
    }

    ///TODO
    /// no-touch driver test, need external calls mocked or returning correctly. Prefer a real
    /// running `synd-enclave`.
    /// Required external calls:
    /// - `eth_getProof` from Ethereum + sequencing providers
    /// - `eth_getStorageAt` from Ethereum provider
    /// - `enclave_verifySequencingChain` + `enclave_verifyAppchain` from TEE enclave
    /// - `arbdebug_preimageData` from Sequencing + Appchain nitro nodes/providers
    /// This might be easier to do in E2E tests
    #[tokio::test]
    async fn test_proposer_tee_verify() {
        shared::tracing::setup_global_logging().expect("logging setup failed");

        let ethereum_provider = ProviderBuilder::default()
            .connect(
                "https://ethereum-holesky.core.chainstack.com/71e6c446fa619c2e05cb50884da82f4a",
            )
            .await
            .unwrap();
        let sequencing_provider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();

        // TODO
        let proposer = Proposer {
            ethereum_provider: ethereum_provider.clone(),
            sequencing_provider: sequencing_provider.clone(),
            l1_arbitrum_bridge_address: L1_ARBITRUM_BRIDGE_ADDRESS,
            sequencer_inbox_address: L1_SEQUENCER_INBOX_ADDRESS,
            ..Default::default()
        };

        let ethereum_provider_arc: Arc<RootProvider> = Arc::new(ethereum_provider);
        let sequencing_provider_arc: Arc<RootProvider> = Arc::new(sequencing_provider);

        // TODO
        let mock_input = teeTrustedInputReturn {
            appchainConfigHash: Default::default(),
            appchainStartBlockHash: Default::default(),
            seqConfigHash: Default::default(),
            setDelayedMessageAcc: Default::default(),
            seqStartBlockHash: Default::default(),
            l1StartBlockHash: Default::default(),
            l1EndBlockHash: Default::default(),
        };

        let res = proposer.tee_verify(&mock_input).await;
        assert!(res.is_ok());
    }
}
