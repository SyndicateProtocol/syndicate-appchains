//! The `proofs` module contains the functions for submitting proofs to confirm epoch data on the
//! staking appchain.

use alloy::{
    eips::BlockNumberOrTag,
    primitives::{keccak256, Address, FixedBytes, StorageKey, U256},
    providers::Provider,
    rlp::Encodable,
    sol,
    sol_types::{SolEvent, SolValue},
};
use clap::Args;
use contract_bindings::synd::{
    block_hash_relayer::BlockHashRelayer,
    gas_aggregator::GasAggregator::{self, GasAggregatorInstance},
    gas_archive::GasArchive::{self, GasArchiveInstance},
    syndicate_factory::SyndicateFactory::{self, getAppchainsAndContractsReturn},
    syndicate_sequencing_chain::SyndicateSequencingChain,
};
use shared::{
    parse::{parse_address, parse_url},
    types::new_provider,
};
use tracing::{debug, info};

/// Arguments for updating base and ethereum block hashes
#[derive(Args, Debug)]
pub struct UpdateBaseAndEthereumBlockHashesArgs {
    /// Base chain RPC URL
    #[arg(long, env = "BASE_RPC_URL", value_parser = parse_url)]
    pub base_rpc_url: String,
    /// Staking appchain RPC URL (will be used to wait for new block hashes)
    #[arg(long, env = "STAKING_APPCHAIN_RPC_URL", value_parser = parse_url)]
    pub staking_appchain_rpc_url: String,
    /// Private key for signing transactions
    #[arg(long, env = "PRIVATE_KEY")]
    pub private_key: String,
    /// Address of the block hash relayer contract
    #[arg(long, value_parser=parse_address)]
    pub relayer_address: Address,
    /// Address of the gas archive contract
    #[arg(long, value_parser=parse_address)]
    pub gas_archive_address: Address,
}

/// Updates base and ethereum block hashes on the staking appchain
///
/// This function calls the `sendBlockHashes` function on the `BlockHashRelayer` contract
/// to update the known block hashes from Ethereum and the settlement chain.
// TODO (ENG-2112): Refactor use of `unwrap_or_else`
#[allow(clippy::cognitive_complexity)]
pub async fn update_base_and_ethereum_block_hashes(args: &UpdateBaseAndEthereumBlockHashesArgs) {
    let settlement_provider = new_provider(&args.base_rpc_url, &args.private_key).await;
    let staking_appchain_provider =
        new_provider(&args.staking_appchain_rpc_url, &args.private_key).await;
    let initial_appchain_block_number = staking_appchain_provider
        .get_block_number()
        .await
        .unwrap_or_else(|e| panic!("failed to get appchain block number: {e}"));
    let gas_limit = U256::from(100_000);
    let max_fee_per_gas = U256::from(100_000_000);
    let receipt = BlockHashRelayer::new(args.relayer_address, &settlement_provider)
        .sendBlockHashes(args.gas_archive_address, gas_limit, max_fee_per_gas)
        .send()
        .await
        .unwrap_or_else(|e| panic!("sending block hashes failed: {e}"))
        .get_receipt()
        .await
        .unwrap_or_else(|e| panic!("getting receipt failed: {e}"));

    assert!(
        receipt.status(),
        "failed to update base and ethereum block hashes. receipt: {receipt:?}"
    );

    info!("successfully updated base and ethereum block hashes");
    debug!("receipt: {receipt:?}");

    let expected_set_block_number =
        receipt.block_number.unwrap_or_else(|| panic!("no block number in receipt")) - 1;
    let expected_set_block_hash = settlement_provider
        .get_block_by_number(BlockNumberOrTag::Number(expected_set_block_number))
        .await
        .unwrap_or_else(|e| panic!("failed to get block by number: {e}"))
        .unwrap_or_else(|| panic!("block not found for number: {expected_set_block_number}"))
        .hash();
    let gas_archive = GasArchive::new(args.gas_archive_address, staking_appchain_provider);
    wait_for_block_hashes_updated(
        gas_archive,
        initial_appchain_block_number,
        expected_set_block_hash,
    )
    .await;
}

async fn wait_for_block_hashes_updated<P: Provider>(
    gas_archive: GasArchiveInstance<P>,
    initial_appchain_block_number: u64,
    expected_set_block_hash: FixedBytes<32>,
) {
    info!("waiting until new block hashes are seen on the staking appchain");
    let filter = gas_archive.KnownBlockHash_filter().from_block(initial_appchain_block_number + 1);
    loop {
        let logs = filter.query().await.unwrap_or_else(|e| panic!("failed to get logs: {e}"));
        for (log, _) in logs {
            if log.setBlockHash == expected_set_block_hash {
                info!("block hashes successfully updated on staking appchain");
                return;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

/// Arguments for submitting gas proofs to confirm epoch data hash
#[derive(Args, Debug)]
pub struct SubmitGasProofsArgs {
    /// Sequencing chain RPC URL
    #[arg(long, env = "SEQ_CHAIN_RPC_URL", value_parser = parse_url)]
    pub seq_chain_rpc_url: String,
    /// Ethereum RPC URL
    #[arg(long, env = "ETHEREUM_RPC_URL", value_parser = parse_url)]
    pub ethereum_rpc_url: String,
    /// Staking aoppchain RPC URL
    #[arg(long, env = "STAKING_APPCHAIN_RPC_URL", value_parser = parse_url)]
    pub staking_appchain_rpc_url: String,
    /// Private key for signing transactions
    #[arg(long, env = "PRIVATE_KEY")]
    pub private_key: String,
    /// Address of the gas archive contract
    #[arg(long, value_parser=parse_address)]
    pub gas_archive_address: Address,
    /// Epoch number (will default to the latest finalized epoch if not provided)
    #[arg(long)]
    pub epoch: Option<u64>,
}

/// Submits gas proofs to confirm epoch data hash on the `GasArchive` contract
///
/// This function calls the `confirmEpochDataHash` function on the `GasArchive` contract
/// with the provided Merkle-Patricia proofs to validate epoch data from a sequencing chain.
#[allow(clippy::cognitive_complexity)]
pub async fn submit_gas_proofs(args: &SubmitGasProofsArgs) {
    let seq_provider = new_provider(&args.seq_chain_rpc_url, &args.private_key).await;
    let eth_provider = new_provider(&args.ethereum_rpc_url, &args.private_key).await;
    let staking_provider = new_provider(&args.staking_appchain_rpc_url, &args.private_key).await;

    let seq_chain_id = seq_provider
        .get_chain_id()
        .await
        .unwrap_or_else(|e| panic!("failed to get sequencing chain ID: {e}"));
    let gas_archive = GasArchive::new(args.gas_archive_address, staking_provider);
    let gas_aggregator_address = gas_archive
        .seqChainGasAggregator(U256::from(seq_chain_id))
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get gas aggregator address: {e}"));
    let gas_aggregator = GasAggregator::new(gas_aggregator_address, seq_provider.clone());

    let epoch = match args.epoch {
        Some(epoch) => U256::from(epoch),
        None => gas_aggregator
            .getCurrentEpoch()
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to get current epoch: {e}"))
            .saturating_sub(U256::from(1)),
    };

    let mut epoch_data_hash = gas_archive
        .epochVerifiedDataHash(epoch, U256::from(seq_chain_id))
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get has chain submitted for epoch: {e}"));

    let already_submited = epoch_data_hash != FixedBytes::ZERO;

    if !already_submited {
        info!("epoch data hash not yet submitted for epoch {epoch} on seq chain {seq_chain_id}. Submitting...");
        // get the latest known ethereum block hash from the gas archive by querying KnownBlockHash
        // events
        let filter = gas_archive.KnownBlockHash_filter();
        let logs = filter
            .query()
            .await
            .unwrap_or_else(|e| panic!("failed to get KnownBlockHash events: {e}"));

        let eth_block_hash = if let Some((log, _)) = logs.last() {
            log.ethBlockHash
        } else {
            panic!("no KnownBlockHash events found - no ethereum block hashes have been submitted to the gas archive");
        };

        // get the outbox contract address from the gas archive

        let outbox_contract_addr = gas_archive
            .seqChainOutbox(U256::from(seq_chain_id))
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to get outbox contract address: {e}"));

        let epoch_data_hash_storage_slot_index =
            gas_archive.AGGREGATED_EPOCH_DATA_HASH_SLOT().call().await.unwrap_or_else(|e| {
                panic!("failed to get epoch data hash storage slot index: {e}")
            });

        // submit proof for the sequencing chain Hash that was settled on ethereum
        let eth_block = eth_provider
            .get_block_by_hash(eth_block_hash)
            .await
            .unwrap_or_else(|e| panic!("failed to get ethereum block by hash: {e}"))
            .unwrap_or_else(|| panic!("ethereum block not found for hash: {eth_block_hash}"));
        let mut rlp_encoded_eth_block_header = vec![];
        eth_block.header.encode(&mut rlp_encoded_eth_block_header);
        let eth_block_number = eth_block.number();

        info!("latest eth block hash known to the gas archive: {eth_block_hash}");

        sol! {
            event SendRootUpdated(bytes32 indexed outputRoot, bytes32 indexed l2BlockHash);
        }

        // search the 1000 previous blocks for the SendRootUpdated event
        let filter = alloy::rpc::types::Filter::new()
            .address(outbox_contract_addr)
            .event_signature(SendRootUpdated::SIGNATURE_HASH)
            .from_block(BlockNumberOrTag::Number(eth_block_number - 1000))
            .to_block(BlockNumberOrTag::Number(eth_block_number));
        let logs = eth_provider
            .get_logs(&filter)
            .await
            .unwrap_or_else(|e| panic!("failed to get logs from ethereum provider: {e}"));
        let last_log =
            logs.last().unwrap_or_else(|| panic!("No events found that update the send root"));
        let sendroot_event = SendRootUpdated::decode_log_data(last_log.data())
            .unwrap_or_else(|e| panic!("failed to decode SendRootUpdated event: {e}"));

        let send_root_storage_slot = gas_archive
            .SEND_ROOT_STORAGE_SLOT()
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to get send root storage slot: {e}"));
        let storage_key: StorageKey =
            keccak256((sendroot_event.outputRoot, send_root_storage_slot).abi_encode());

        let seq_chain_block_hash_proof = eth_provider
            .get_proof(outbox_contract_addr, vec![storage_key])
            .block_id(eth_block_hash.into())
            .await
            .unwrap_or_else(|e| panic!("failed to get sequencing chain block hash proof: {e}"));

        let seq_block_hash: FixedBytes<32> = seq_chain_block_hash_proof
            .storage_proof
            .first()
            .unwrap_or_else(|| panic!("no storage proof found for sequencing chain block hash"))
            .value
            .into();
        assert_eq!(seq_block_hash, sendroot_event.l2BlockHash); //sanity check

        info!("Submitting gas proofs for epoch {epoch}");

        let seq_block = seq_provider
            .get_block_by_hash(seq_block_hash)
            .await
            .unwrap_or_else(|e| panic!("failed to get sequencing block by hash: {e}"))
            .unwrap_or_else(|| panic!("sequencing block not found for hash: {seq_block_hash}"));
        let mut rlp_encoded_seq_block_header = vec![];
        seq_block.header.encode(&mut rlp_encoded_seq_block_header);

        let epoch_data_hash_storage_key: StorageKey =
            keccak256((epoch, epoch_data_hash_storage_slot_index).abi_encode());

        let epoch_data_hash_proof = seq_provider
            .get_proof(gas_aggregator_address, vec![epoch_data_hash_storage_key])
            .block_id(seq_block_hash.into())
            .await
            .unwrap_or_else(|e| panic!("failed to get epoch data hash proof: {e}"));

        epoch_data_hash = gas_aggregator
            .aggregatedEpochDataHash(epoch)
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to get aggregated epoch data hash: {e}"));
        assert_eq!(
            epoch_data_hash,
            Into::<FixedBytes<32>>::into(
                epoch_data_hash_proof
                    .storage_proof
                    .first()
                    .unwrap_or_else(|| panic!("no storage proof found for epoch data hash"))
                    .value
            )
        ); // sanity check

        let receipt = gas_archive
            .confirmEpochDataHash(
                U256::from(seq_chain_id),
                sendroot_event.outputRoot,
                rlp_encoded_eth_block_header.into(),
                seq_chain_block_hash_proof.account_proof.clone(),
                seq_chain_block_hash_proof
                    .storage_proof
                    .first()
                    .unwrap_or_else(|| {
                        panic!("no storage proof found for sequencing chain block hash")
                    })
                    .proof
                    .clone(),
                rlp_encoded_seq_block_header.into(),
                epoch_data_hash_proof.account_proof.clone(),
                epoch_data_hash_proof
                    .storage_proof
                    .first()
                    .unwrap_or_else(|| panic!("no storage proof found for epoch data hash"))
                    .proof
                    .clone(),
            )
            .send()
            .await
            .unwrap_or_else(|e| panic!("confirming epoch data hash failed: {e}"))
            .get_receipt()
            .await
            .unwrap_or_else(|e| panic!("getting receipt failed: {e}"));

        assert!(receipt.status(), "failed to confirm epoch data hash. receipt: {receipt:?}");

        info!("successfully confirmed epoch data hash");
        debug!("receipt: {receipt:?}");
    }
    info!(
        "Submitting epoch pre-image data for epoch {} on seq chain {}: {}",
        epoch, seq_chain_id, epoch_data_hash
    );

    let (appchains, tokens, emissions_receivers) =
        get_aggregated_chain_data(epoch, gas_aggregator.clone()).await;

    info!("appchains: {appchains:?}");
    info!("tokens: {tokens:?}");
    info!("emissions_receivers: {emissions_receivers:?}");

    let abi_encoded_data =
        &(appchains.clone(), tokens.clone(), emissions_receivers.clone()).abi_encode()[32..];
    info!("abi_encoded_data: {abi_encoded_data:?}");

    assert_eq!(
        epoch_data_hash,
        keccak256(abi_encoded_data),
        "epoch data hash doesn't match the data obtained"
    );

    let receipt = gas_archive
        .submitEpochPreImageData(U256::from(seq_chain_id), appchains, tokens)
        .send()
        .await
        .unwrap_or_else(|e| panic!("submitting epoch pre-image data failed: {e}"))
        .get_receipt()
        .await
        .unwrap_or_else(|e| panic!("getting receipt failed: {e}"));

    info!("successfully submitted epoch pre-image data");
    debug!("receipt: {receipt:?}");
}

async fn get_aggregated_chain_data<P: Provider + Clone>(
    epoch: U256,
    gas_aggregator: GasAggregatorInstance<P>,
) -> (Vec<U256>, Vec<U256>, Vec<Address>) {
    let factory_address = gas_aggregator
        .factory()
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get factory address: {e}"));
    let factory = SyndicateFactory::new(factory_address, gas_aggregator.provider().clone());

    let getAppchainsAndContractsReturn { _chainIDs: appchains, _contracts: appchain_contracts } =
        factory
            .getAppchainsAndContracts()
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to get appchains and contracts: {e}"));
    let (mut tokens, mut emissions_receivers) = (vec![], vec![]);

    for contract in appchain_contracts {
        let appchain = SyndicateSequencingChain::new(contract, gas_aggregator.provider().clone());
        tokens.push(
            appchain
                .getTokensForEpoch(epoch)
                .call()
                .await
                .unwrap_or_else(|e| panic!("failed to get tokens for epoch {epoch}: {e}")),
        );
        emissions_receivers.push(
            appchain
                .getEmissionsReceiver()
                .call()
                .await
                .unwrap_or_else(|e| panic!("failed to get emissions receiver: {e}")),
        );
    }
    (appchains, tokens, emissions_receivers)
}

/// Arguments for running both `update_base_and_ethereum_block_hashes` and `submit_gas_proofs`
/// sequentially
#[derive(Args, Debug)]
pub struct UpdateAndSubmitProofsArgs {
    /// Base chain RPC URL
    #[arg(long, env = "BASE_RPC_URL", value_parser = parse_url)]
    pub base_rpc_url: String,
    /// Sequencing chain RPC URL
    #[arg(long, env = "SEQ_CHAIN_RPC_URL", value_parser = parse_url)]
    pub seq_chain_rpc_url: String,
    /// Ethereum RPC URL
    #[arg(long, env = "ETHEREUM_RPC_URL", value_parser = parse_url)]
    pub ethereum_rpc_url: String,
    /// Staking appchain RPC URL
    #[arg(long, env = "STAKING_APPCHAIN_RPC_URL", value_parser = parse_url)]
    pub staking_appchain_rpc_url: String,
    /// Private key for signing transactions
    #[arg(long, env = "PRIVATE_KEY")]
    pub private_key: String,
    /// Address of the block hash relayer contract
    #[arg(long, value_parser=parse_address)]
    pub relayer_address: Address,
    /// Address of the gas archive contract
    #[arg(long, value_parser=parse_address)]
    pub gas_archive_address: Address,
    /// Epoch number (will default to the latest finalized epoch if not provided)
    #[arg(long)]
    pub epoch: Option<u64>,
}

/// Updates base and ethereum block hashes, then submits gas proofs to confirm epoch data hash
///
/// This function first calls `update_base_and_ethereum_block_hashes` to update the known block
/// hashes from Ethereum and the settlement chain, then calls `submit_gas_proofs` to submit gas
/// proofs to confirm epoch data hash on the `GasArchive` contract.
#[allow(clippy::cognitive_complexity)]
pub async fn update_and_submit_proofs(args: &UpdateAndSubmitProofsArgs) {
    info!("Starting update and submit proofs workflow");

    // First, update base and ethereum block hashes
    let update_args = UpdateBaseAndEthereumBlockHashesArgs {
        base_rpc_url: args.base_rpc_url.clone(),
        private_key: args.private_key.clone(),
        relayer_address: args.relayer_address,
        gas_archive_address: args.gas_archive_address,
        staking_appchain_rpc_url: args.staking_appchain_rpc_url.clone(),
    };

    info!("Step 1: Updating base and ethereum block hashes");
    update_base_and_ethereum_block_hashes(&update_args).await;

    // Then, submit gas proofs
    let submit_args = SubmitGasProofsArgs {
        seq_chain_rpc_url: args.seq_chain_rpc_url.clone(),
        ethereum_rpc_url: args.ethereum_rpc_url.clone(),
        staking_appchain_rpc_url: args.staking_appchain_rpc_url.clone(),
        private_key: args.private_key.clone(),
        gas_archive_address: args.gas_archive_address,
        epoch: args.epoch,
    };

    info!("Step 2: Submitting gas proofs");
    submit_gas_proofs(&submit_args).await;

    info!("Successfully completed update and submit proofs workflow");
}
