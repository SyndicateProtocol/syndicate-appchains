//! The `synd-proposer` polls information from the appchain and posts to the settlement chain
//! `AssertionPoster` contract

use crate::{
    config::Config,
    metrics::ProposerMetrics,
    types::{Batch, BatchWithCount, NitroBlock, SeqVerifyOutput},
};
use alloy::{
    consensus::{BlockHeader, Transaction},
    eips::{
        BlockNumberOrTag,
        BlockNumberOrTag::{Earliest, Latest},
    },
    network::{BlockResponse, Ethereum, EthereumWallet},
    primitives::{bytes, fixed_bytes, keccak256, Address, BlockNumber, B256, U256},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider as _, ProviderBuilder, RootProvider, WalletProvider as _,
    },
    rpc::types::{Block, Filter},
    signers::local::PrivateKeySigner,
    sol_types::{sol, SolCall, SolEvent, SolValue},
    transports::TransportResult,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve, Router,
};
use contract_bindings::synd::{
    assertionposter::AssertionPoster::{self, AssertionPosterInstance},
    ibridge::IBridge::{IBridgeInstance, MessageDelivered},
    iinbox::IInbox::InboxMessageDelivered,
    isequencerinbox::ISequencerInbox::{
        addSequencerL2BatchFromEigenDACall, ISequencerInboxInstance, SequencerBatchData,
    },
    rollup::Rollup::SequencerBatchDelivered,
    teemodule::TeeModule::{teeTrustedInputReturn, TeeModuleInstance},
};
use eyre::{eyre, Result};
use log::warn;
use shared::{
    parse::{parse_address, parse_url},
    tx_validation::validate_transaction,
    types::FilledProvider,
};
use std::{
    collections::{BTreeMap, HashMap},
    default::Default,
    ops::Index,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use synd_seqchain_verifier::types::{
    ArbitrumBatch, L1ChainInput, L1IncomingMessage, L1IncomingMessageHeader, TimeBounds,
};
use tokio::{net::TcpListener, sync::RwLock, task::JoinHandle};
use tracing::{error, info};
use url::Url;

const EIGENDA_MESSAGE_HEADER_FLAG: i32 = 0xed;

#[derive(Debug)]
pub struct Proposer {
    appchain_provider: RootProvider,
    sequencing_provider: RootProvider,
    settlement_provider: FilledProvider,
    ethereum_provider: RootProvider,
    polling_interval: Duration,
    assertion_poster: AssertionPosterInstance<(), FilledProvider>,
    metrics: ProposerMetrics,
    tee_module: TeeModuleInstance<(), FilledProvider>,
    last_tee_trusted_input_return_hash: Arc<RwLock<B256>>, // TODO revisit
    arbitrum_bridge_address: Address,
    sequencer_inbox_address: Address, // batches
    inbox_address: Address,           // deposits
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
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let settlement_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(config.settlement_rpc_url.as_str())
        .await?;

    let assertion_poster =
        AssertionPoster::new(config.assertion_poster_contract_address, settlement_provider.clone());

    let tee_module =
        TeeModuleInstance::new(config.tee_module_contract_address, settlement_provider.clone());

    let proposer = Proposer::new(
        &config,
        metrics,
        appchain_provider,
        sequencing_provider,
        ethereum_provider,
        settlement_provider,
        assertion_poster,
        tee_module,
    );
    let proposer = Arc::new(proposer);

    // Clone for both tasks
    let proposer_polling = Arc::clone(&proposer);
    let proposer_http = Arc::clone(&proposer);

    // Start polling loop - no more mutex needed
    let polling_task: JoinHandle<Result<()>> =
        tokio::spawn(async move { proposer_polling.main_loop().await });

    // Start HTTP server with /propose endpoint
    let app =
        Router::new().route("/propose", post(post_assertion_handler)).with_state(proposer_http);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

    let server_task = tokio::spawn(async move {
        if let Err(err) = serve(listener, app).await {
            eprintln!("HTTP server error: {:?}", err);
        }
    });

    // Wait for both tasks
    let _ = tokio::try_join!(polling_task, server_task)?;
    Ok(())
}

async fn post_assertion_handler(State(proposer): State<Arc<Proposer>>) -> Response {
    match proposer.fetch_block_and_post().await {
        Ok(_) => (StatusCode::OK, "Assertion posted successfully").into_response(),
        Err(err) => {
            error!("Handler error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to post assertion: {:?}", err))
                .into_response()
        }
    }
}

fn hash_tee_trusted_input(input: &teeTrustedInputReturn) -> B256 {
    // Solidity's abi.encodePacked concatenates the bytes without padding
    // For bytes32 values, they're already 32 bytes each
    let mut packed = Vec::new();

    // Append each field in the same order as Solidity
    packed.extend_from_slice(&input.appchainConfigHash.0);
    packed.extend_from_slice(&input.appchainStartBlockHash.0);
    packed.extend_from_slice(&input.seqConfigHash.0);
    packed.extend_from_slice(&input.seqStartBlockHash.0);
    packed.extend_from_slice(&input.setDelayedMessageAcc.0);
    packed.extend_from_slice(&input.l1StartBlockHash.0);
    packed.extend_from_slice(&input.l1EndBlockHash.0);

    // Hash the packed data
    keccak256(&packed)
}

enum PreimageSource {
    Sequencing,
    Appchain,
}

impl Default for Proposer {
    fn default() -> Self {
        let url_str = "localhost:8080";
        #[allow(clippy::unwrap_used)]
        let dummy_url = Url::from_str(url_str).unwrap();
        let dummy_root_provider = ProviderBuilder::default().on_http(dummy_url.clone());
        let dummy_filled_provider =
            ProviderBuilder::new().wallet(EthereumWallet::default()).on_http(dummy_url);

        let assertion_poster = AssertionPoster::new(Address::ZERO, dummy_filled_provider.clone());

        let tee_module = TeeModuleInstance::new(Address::ZERO, dummy_filled_provider.clone());

        Proposer {
            appchain_provider: dummy_root_provider.clone(),
            sequencing_provider: dummy_root_provider.clone(),
            settlement_provider: dummy_filled_provider.clone(),
            ethereum_provider: dummy_root_provider.clone(),
            polling_interval: Duration::from_secs(1),
            assertion_poster,
            metrics: ProposerMetrics::default(),
            tee_module,
            last_tee_trusted_input_return_hash: Arc::new(RwLock::new(B256::default())),
            arbitrum_bridge_address: Address::ZERO,
            sequencer_inbox_address: Address::ZERO,
            inbox_address: Address::ZERO,
        }
    }
}

impl Proposer {
    pub fn new(
        config: &Config,
        metrics: ProposerMetrics,
        appchain_provider: RootProvider,
        sequencing_provider: RootProvider,
        ethereum_provider: RootProvider,
        settlement_provider: FilledProvider,
        assertion_poster: AssertionPosterInstance<(), FilledProvider>,
        tee_module: TeeModuleInstance<(), FilledProvider>,
    ) -> Self {
        Proposer {
            appchain_provider,
            sequencing_provider,
            settlement_provider,
            ethereum_provider,
            polling_interval: config.polling_interval,
            assertion_poster,
            tee_module,
            metrics,
            last_tee_trusted_input_return_hash: Arc::new(RwLock::new(B256::default())),
            arbitrum_bridge_address: config.arbitrum_bridge_address,
            sequencer_inbox_address: config.sequencer_inbox_address,
            inbox_address: config.inbox_address,
        }
    }

    async fn main_loop(&self) -> Result<()> {
        let mut interval = tokio::time::interval(self.polling_interval);

        // call view func for `teeTrustedInput()`, get the data
        // check "have I submitted this teeTrustedInput before?" via state variable

        let x = self.tee_module.teeTrustedInput().call().await?;

        if self.check_was_tee_input_already_processed(&x).await {
            // skip
            return Ok(());
        }

        // this gets submitted to synd-enclave

        #[allow(clippy::unwrap_used)]
        let l1_start_block =
            self.ethereum_provider.get_block_by_hash(x.l1StartBlockHash).await?.unwrap();
        #[allow(clippy::unwrap_used)]
        let l1_end_block =
            self.ethereum_provider.get_block_by_hash(x.l1EndBlockHash).await?.unwrap();
        // let (l1_input, l1_preimages) = self.build_l1_input_with_preimages(x).await?;
        let l1_input = self.build_l1_input(x).await?;
        let l1_preimages: Vec<Vec<u8>> = self
            .arb_debug_preimage_data(
                PreimageSource::Sequencing,
                l1_start_block.header.number,
                l1_end_block.header.number,
            )
            .await?;

        // assume we get SeqVerifyOutput from synd-enclave
        let seq_verify_output =
            SeqVerifyOutput { block_hash: Default::default(), signature: Default::default() };

        // query 4 nodes
        loop {
            interval.tick().await;
            self.fetch_block_and_post().await?;
        }
    }

    async fn check_was_tee_input_already_processed(&self, x: &teeTrustedInputReturn) -> bool {
        let hashed_tee_trusted_input = hash_tee_trusted_input(&x);
        let last_hash = *self.last_tee_trusted_input_return_hash.read().await;

        if last_hash == hashed_tee_trusted_input {
            return true;
        }
        // Update the hash
        *self.last_tee_trusted_input_return_hash.write().await = hashed_tee_trusted_input;
        false
    }

    // /// Enhanced build_l1_input that also gets preimages
    // async fn build_l1_input_with_preimages(
    //     &self,
    //     x: teeTrustedInputReturn,
    // ) -> Result<(L1ChainInput, Vec<Vec<u8>>)> {
    //     // Build the L1ChainInput as before
    //     // #[allow(clippy::unwrap_used)]
    //     // let l1_start_block =
    //     //     self.ethereum_provider.get_block_by_hash(x.l1StartBlockHash).await?.unwrap();
    //     // #[allow(clippy::unwrap_used)]
    //     // let l1_end_block =
    //     //     self.ethereum_provider.get_block_by_hash(x.l1EndBlockHash).await?.unwrap();
    //     let l1_input = self.build_l1_input(x, l1_start_block.clone(),
    // l1_end_block.clone()).await?;
    //
    //     let l1_preimages: Vec<Vec<u8>> = self
    //         .arb_debug_preimage_data(
    //             PreimageSource::Sequencing,
    //             l1_start_block.header.number,
    //             l1_end_block.header.number,
    //         )
    //         .await?;
    //
    //     Ok((l1_input, l1_preimages))
    // }

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

        // TODO validation?

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

    async fn build_l1_input(&self, x: teeTrustedInputReturn) -> Result<L1ChainInput> {
        // setDelayedMessageAcc is the endpoint,
        #[allow(clippy::unwrap_used)]
        let l1_start_block =
            self.ethereum_provider.get_block_by_hash(x.l1StartBlockHash).await?.unwrap();
        #[allow(clippy::unwrap_used)]
        let l1_end_block =
            self.ethereum_provider.get_block_by_hash(x.l1EndBlockHash).await?.unwrap();

        let sequencer_inbox =
            ISequencerInboxInstance::new(self.sequencer_inbox_address, &self.ethereum_provider);

        #[allow(clippy::unwrap_used)]
        let seq_start_block =
            self.sequencing_provider.get_block_by_hash(x.seqStartBlockHash).await?.unwrap();

        println!("X_X_X_X");
        println!(
            "X_X_X_X l1_start_block {} l1_end_block {} seq_start_block {}",
            l1_start_block.header.number, l1_end_block.header.number, seq_start_block.header.number
        );

        // get batches first
        // TODO batches size 0
        let batches =
            self.get_batches(l1_start_block.header.number, l1_end_block.header.number).await?;
        println!("X_X_X_X first batch");
        // println!("{:?}", batches[0]);

        let delayed_message_count: u64 = seq_start_block.header.nonce.into();
        let delayed_messages = self
            .get_delayed_messages(
                delayed_message_count,
                l1_start_block.header.number,
                l1_end_block.header.number,
            )
            .await?;

        let start_batch_accumulator_merkle_proof = self
            .get_accumulator_proof(self.arbitrum_bridge_address, l1_start_block.header.number)
            .await?;

        let end_batch_accumulator_merkle_proof = self
            .get_accumulator_proof(self.arbitrum_bridge_address, l1_end_block.header.number)
            .await?;

        println!("X_X_X_X start + end accumulator_proof - batches");
        info!("{}", start_batch_accumulator_merkle_proof.storage_proof[0].value);
        info!("{}", end_batch_accumulator_merkle_proof.storage_proof[0].value);

        println!("X_X_X_X start + end accumulator_proof - accumulators");
        info!("{}", start_batch_accumulator_merkle_proof.storage_proof[1].value);
        info!("{}", end_batch_accumulator_merkle_proof.storage_proof[1].value);

        let input = L1ChainInput {
            start_batch_accumulator_merkle_proof,
            end_batch_accumulator_merkle_proof,
            start_block_header: l1_start_block.header,
            end_block_header: l1_end_block.header,
            delayed_messages,
            batches,
            start_block_hash: x.l1StartBlockHash,
            end_block_hash: x.l1EndBlockHash,
        };
        Ok(input)
    }

    // only called once, for L1
    // appchain will use SyndicateTransactionEvents
    // previous batch num - up to end block - setDelayedMessageAcc
    async fn get_batches(&self, start_block: u64, end_block: u64) -> Result<Vec<ArbitrumBatch>> {
        let start_block = start_block; // TODO
        let batch_filter = Filter::new()
            .address(self.sequencer_inbox_address)
            .from_block(start_block)
            .to_block(end_block)
            // .event_signature(SequencerBatchDelivered::SIGNATURE_HASH);
            .event_signature(vec![
                SequencerBatchData::SIGNATURE_HASH,
                SequencerBatchDelivered::SIGNATURE_HASH,
            ]);
        let batch_event_logs = self.ethereum_provider.get_logs(&batch_filter).await?;

        // Parse bridge events into a map for easy lookup
        let mut message_delivered_metadata = HashMap::new();
        #[allow(clippy::unwrap_used)]
        for log in batch_event_logs
            .iter()
            .filter(|log| *log.topic0().unwrap() == SequencerBatchData::SIGNATURE_HASH)
        {
            let event = MessageDelivered::decode_log(log.as_ref(), true)?;
            message_delivered_metadata.insert(event.messageIndex, event);
        }

        //

        // Step 3: Combine the data
        // let mut batches = Vec::new();
        let mut batch_map: BTreeMap<U256, ArbitrumBatch> = BTreeMap::new();

        // TODO - DEBUG code, delete me
        let mut first_seq_number = 0usize;
        let mut last_seq_number = 0usize;

        // TODO listen for the eigen version of these contracts too? so we create a submodule for
        // layr-labs contracts?
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
                            panic!("decoding failed: {} {:?}", "SequencerBatchData", e);
                        }
                    }
                }
                SequencerBatchDelivered::SIGNATURE_HASH => {
                    match SequencerBatchDelivered::decode_log(log.as_ref(), true) {
                        Ok(decoded) => {
                            if first_seq_number == 0 {
                                first_seq_number = usize::try_from(decoded.batchSequenceNumber)?;
                            }
                            last_seq_number = usize::try_from(decoded.batchSequenceNumber)?;

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
                                let data =
                                    (EIGENDA_MESSAGE_HEADER_FLAG, res.cert).abi_encode_packed();
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
                    panic!("unexpected batch event type hash: {}", event_type)
                }
            }
        }

        // skip validation

        info!(%first_seq_number, %last_seq_number);

        let batches: Vec<ArbitrumBatch> = batch_map.into_values().collect();
        info!(batch_length=%batches.len());
        Ok(batches)
    }

    // TODO this needs to change
    // previous batch num - up to end block - setDelayedMessageAcc
    async fn get_delayed_messages(
        &self,
        start_delayed_message_index: u64,
        start_block: u64,
        end_block: u64,
    ) -> Result<Vec<L1IncomingMessage>> {
        // TODO could optimize
        let filter = &Filter::new()
            .address(self.arbitrum_bridge_address)
            .from_block(start_block - 1000) // too large
            .to_block(end_block)
            .event_signature(MessageDelivered::SIGNATURE_HASH)
            .topic1(B256::from(U256::from(start_delayed_message_index)));
        let first_log = self.ethereum_provider.get_logs(filter).await?;

        match first_log.len() {
            0 => {
                let bridge =
                    IBridgeInstance::new(self.arbitrum_bridge_address, &self.ethereum_provider);
                let delayed_message_count = bridge.delayedMessageCount().call().await?;
                assert_eq!(U256::from(start_delayed_message_index), delayed_message_count._0);
                return Ok(vec![]);
            }
            1 => {}
            2.. => {
                panic!(
                    "found multiple blocks for first delayed message: {}",
                    start_delayed_message_index
                )
            }
        }
        #[allow(clippy::unwrap_used)]
        let start_block = first_log[0].block_number.unwrap();

        let message_delivered_filter = Filter::new()
            .address(self.arbitrum_bridge_address)
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
        let mut inbox_filter = Filter::new()
            .address(self.inbox_address)
            .from_block(start_block)
            .to_block(end_block)
            .event_signature(InboxMessageDelivered::SIGNATURE_HASH);

        inbox_filter = inbox_filter.event_signature(InboxMessageDelivered::SIGNATURE_HASH);

        let inbox_message_delivered_logs = self.ethereum_provider.get_logs(&inbox_filter).await?;

        // Step 3: Combine the data
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
                    block_number: log.block_number.unwrap_or_default(),
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
        Ok(delayed_messages)
    }

    async fn get_accumulator_proof(
        &self,
        bridge_address: Address,
        block_number: u64,
    ) -> Result<alloy::rpc::types::EIP1186AccountProofResponse> {
        println!("accumulator proof holesky - block number: {}", block_number);

        // Get the batch count at this block to know which accumulator slot to query
        // let bridge = IBridgeInstance::new(bridge_address, &self.ethereum_provider);

        // Get the sequencer inbox accumulator count (batch count)
        // Note: This needs to be done at the specific block
        let batch_count = self.get_batch_count_at_block(bridge_address, block_number).await?;

        info!(%batch_count, %block_number, "X_X_X_X batch count at block number");

        // Calculate the storage slot for the accumulator
        // Slot 7 (0x07) stores the batch count
        // The accumulators are stored starting at keccak256(0x07)
        const BATCH_ACCUMULATOR_STORAGE_SLOT: B256 =
            fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000007");
        const BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT: B256 =
            fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688"); // Keccak256("0x7")

        // Calculate the actual slot for the accumulator at this index
        let accumulator_slot =
            self.calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, batch_count);

        // Get proof for both the count slot and the accumulator slot
        let proof = self
            .ethereum_provider
            .get_proof(
                bridge_address,
                vec![
                    BATCH_ACCUMULATOR_STORAGE_SLOT, // Slot 7: batch count
                    accumulator_slot,               // Calculated slot: accumulator value
                ],
            )
            .block_id(block_number.into())
            .await?;

        Ok(proof)
    }

    // TODO use shared
    // This matches the calculate_slot function in types.rs
    #[allow(clippy::unwrap_used)]
    fn calculate_slot(&self, start_slot: B256, index: U256) -> B256 {
        B256::from(
            U256::from_be_bytes::<32>(start_slot.as_slice().try_into().unwrap()) + index -
                U256::from(1),
        )
    }

    async fn get_batch_count_at_block(
        &self,
        bridge_address: Address,
        block_number: u64,
    ) -> Result<U256> {
        // Get the value at storage slot 7 (batch count)
        const BATCH_COUNT_SLOT: B256 =
            fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000007");

        let count = self
            .ethereum_provider
            .get_storage_at(bridge_address, BATCH_COUNT_SLOT.into())
            .block_id(block_number.into())
            .await?;

        Ok(count)
    }

    async fn fetch_block(&self) -> Result<NitroBlock> {
        self.appchain_provider
            .raw_request("eth_getBlockByNumber".into(), ("latest", false))
            .await
            .map_err(|err| eyre!("eth_getBlockByNumber request failed: {:?}", err))
    }

    // async fn appchain_validation_inputs(&self) -> Result {
    //     self.appchain_provider
    //         .raw_request("arbdebug_validationInputsAt".into())
    // }

    async fn record_wallet_balance(&self) -> Result<()> {
        let provider = self.assertion_poster.provider();
        let wallet_address = provider.default_signer_address();

        let balance = provider.get_balance(wallet_address).await?;
        self.metrics.record_wallet_balance(balance.to());
        Ok(())
    }

    async fn post_assertion(&self, block: NitroBlock) -> Result<()> {
        // TODO make non-blocking?
        self.record_wallet_balance().await?;

        let _ = self.assertion_poster.postAssertion(block.hash, block.send_root).send().await?;
        self.metrics.record_last_block_posted(block.number.to());

        info!("Assertion submitted for block: {:?}", block);
        Ok(())
    }

    async fn fetch_block_and_post(&self) -> Result<()> {
        match self.fetch_block().await {
            Ok(block) => {
                if let Err(err) = self.post_assertion(block).await {
                    error!("Failed to post assertion: {:?}", err);
                }
            }
            Err(err) => {
                error!("Failed to fetch block: {:?}", err);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::proposer::Proposer;
    use alloy::{
        eips::BlockNumberOrTag,
        primitives::{address, fixed_bytes, Address, BlockHash},
        providers::{Provider, ProviderBuilder, RootProvider},
        rpc::types::{Block, Filter},
    };
    use contract_bindings::synd::teemodule::TeeModule::teeTrustedInputReturn;
    use eyre::{eyre, Error};
    use serde::Serialize;
    use std::{fs, str::FromStr, sync::Arc};
    use synd_seqchain_verifier::{config::SeqchainVerifierConfig, verifier::Verifier};

    #[tokio::test]
    async fn test1() {
        shared::logger::set_global_default_subscriber().unwrap();

        let arbitrum_bridge_address = address!("0x27b5BA9331f20afd816C247d53BDf1EC577b04CD");
        let sequencer_inbox_address = address!("0x47c3DEC256DB25c527d92AAceE1269C17805ce9d");

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
            arbitrum_bridge_address,
            sequencer_inbox_address,
            ..Default::default()
        };

        let ethereum_provider_arc: Arc<RootProvider> = Arc::new(
            ProviderBuilder::default()
                // .connect("https://eth-holesky.g.alchemy.com/v2/FFOCYExawZ3K46YRNHqaUEo3pbqS5F1F")
                .connect(
                    "https://ethereum-holesky.core.chainstack.com/71e6c446fa619c2e05cb50884da82f4a",
                )
                .await
                .unwrap(),
        );
        let sequencing_provider_arc: Arc<RootProvider> = Arc::new(
            ProviderBuilder::default()
                .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
                .await
                .unwrap(),
        );

        // SequencerInbox contract address on Holesky L1 - https://eth-holesky.blockscout.com/address/0x47c3DEC256DB25c527d92AAceE1269C17805ce9d?tab=txs
        let l1_sequencer_inbox =
            Address::from_str("0x47c3DEC256DB25c527d92AAceE1269C17805ce9d").unwrap();
        // FAILING below - includes batch
        let (l1_latest_block, l1_latest_tx_hash) =
            get_latest_contract_txn_block(ethereum_provider_arc.clone(), l1_sequencer_inbox)
                .await
                .unwrap();
        // WORKING below, has no batches
        // let l1_latest_block_number = ethereum_provider.get_block_number().await.unwrap();
        // let l1_latest_block =
        // ethereum_provider.get_block_by_number(BlockNumberOrTag::Number(l1_latest_block_number)).
        // await.unwrap().unwrap(); let l1_latest_tx_hash = l1_latest_block.header.hash;

        let l1_latest_block_hash = l1_latest_block.header.hash;
        let l1_latest_block_ts = l1_latest_block.header.timestamp;
        println!("Latest L1 sequencer_inbox transaction + block:");
        println!("  Block Number: {}", l1_latest_block.header.number);
        println!("  Block Hash: {:?}", l1_latest_block_hash);
        println!("  Timestamp: {}", l1_latest_block_ts);
        println!("  Transaction Hash: {:?}", l1_latest_tx_hash);

        let n = 1;
        let l1_latest_block_minus_n_number = l1_latest_block.header.number - n;
        let l1_latest_block_minus_n: Block = ethereum_provider_arc
            .get_block_by_number(BlockNumberOrTag::Number(l1_latest_block_minus_n_number))
            .await
            .unwrap()
            .ok_or("block {} doesn't exist")
            .unwrap();
        let l1_latest_block_minus_n_hash = l1_latest_block_minus_n.header.hash;
        let l1_latest_block_minus_n_ts = l1_latest_block_minus_n.header.timestamp;
        println!("Latest minus n ({}) L1 sequencer_inbox block:", n);
        println!("  Block Number: {}", l1_latest_block_minus_n.header.number);
        println!("  Block Hash: {:?}", l1_latest_block_minus_n.header.hash);
        println!("  Block Timestamp: {}", l1_latest_block_minus_n.header.timestamp);

        // Getting latest seq block, then stepping back until the timestamp is within the l1
        // start-end range
        let seq_latest_block_number = sequencing_provider_arc.get_block_number().await.unwrap();
        let mut curr_block_number = seq_latest_block_number;

        let mut desired_seq_start_block_hash: Option<BlockHash> = None;
        loop {
            let seq_latest_block = sequencing_provider_arc
                .get_block_by_number(BlockNumberOrTag::Number(curr_block_number))
                .await
                .unwrap()
                .ok_or(format!(
                    "seq block {}, which is {} blocks back, doesn't exist",
                    curr_block_number, n
                ))
                .unwrap();
            if seq_latest_block.header.timestamp < l1_latest_block_ts {
                println!("Latest seq block to use:");
                println!("  Block Number: {}", seq_latest_block.header.number);
                println!("  Block Hash: {:?}", seq_latest_block.header.hash);
                println!("  Block Timestamp: {}", seq_latest_block.header.timestamp);
                desired_seq_start_block_hash = Some(seq_latest_block.header.hash);
                break;
            } else {
                curr_block_number -= 1;
            }
        }
        let mock_input = teeTrustedInputReturn {
            appchainConfigHash: Default::default(),     // ignore
            appchainStartBlockHash: Default::default(), // ignore
            seqConfigHash: Default::default(),          // ignore

            seqStartBlockHash: desired_seq_start_block_hash.unwrap(),
            setDelayedMessageAcc: Default::default(),
            l1StartBlockHash: l1_latest_block_minus_n_hash,
            l1EndBlockHash: l1_latest_block_hash,
        };
        println!("X_X_X_X mock input");
        println!("{:?}", mock_input);

        // let mock_input = teeTrustedInputReturn {
        //     appchainConfigHash: Default::default(),     // ignore
        //     appchainStartBlockHash: Default::default(), // ignore
        //     seqConfigHash: Default::default(),          // ignore
        //
        //     // first batch from block 0 - https://syndicate-exo.explorer.alchemy.com/block/0?tab=index
        //     seqStartBlockHash: fixed_bytes!(
        //         "0xea7e440eb1d9e37d9b2fcb7883a1ca631331c2d5b54c4404ee21c0231fde5196"
        //     ),
        //     setDelayedMessageAcc: Default::default(),
        //     // https://eth-holesky.blockscout.com/block/3326232
        //     l1StartBlockHash: fixed_bytes!(
        //         "0xe0158dfd5065d86cfe060317244aec0aa7ab27f58d4bef71bf9518282ce88d15"
        //     ),
        //     // 5th contract call down - https://eth-holesky.blockscout.com/tx/0x15854153b2b9d1851cbaa98c29bba607c58286cf822ca81afc465517aab1c84e?tab=index
        //     // this block hash https://eth-holesky.blockscout.com/block/3328058
        //     l1EndBlockHash: fixed_bytes!(
        //         "0x7b10ea0111d6ea9e3a2b91010f89693f4c56378cabc2b95ae7decfe738bfa7d6"
        //     ),
        // };

        // TODO - get proof window issue fixed on Holesky
        let l1_chain_input = proposer.build_l1_input(mock_input).await.unwrap();

        let seq_chain_verifier = Verifier::new(&SeqchainVerifierConfig { arbitrum_bridge_address });
        let l1_chain_input_json = serde_json::to_string_pretty(&l1_chain_input);
        fs::write("l1_chain_input_eigenda.json", l1_chain_input_json.unwrap()).unwrap();
        let res = seq_chain_verifier.verify_and_create_output(&l1_chain_input);
        assert!(res.is_ok());
    }

    // Given a contract address, get the latest txn call and the block it is part of
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

        if logs.is_empty() {
            assert!(false, "No logs returned");
        }
        let latest_log = logs.last().unwrap();
        let block_hash = latest_log.block_hash.ok_or(eyre!("Block hash not found"))?;
        let tx_hash = latest_log.transaction_hash.ok_or(eyre!("Transaction hash not found"))?;
        let block = ethereum_provider
            .get_block_by_hash(block_hash)
            .await?
            .ok_or(eyre!("Block not found"))?;
        Ok((block, tx_hash))
    }
}
