use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    primitives::{keccak256, Address, FixedBytes, StorageKey, U256},
    providers::{Provider, ProviderBuilder},
    rlp::Encodable,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::{SolEvent, SolValue},
};
use clap::Args;
use contract_bindings::synd::{
    block_hash_relayer::BlockHashRelayer,
    gas_aggregator::GasAggregator::{self, GasAggregatorInstance},
    gas_archive::GasArchive,
    syndicate_factory::SyndicateFactory::{self, getAppchainsAndContractsReturn},
    syndicate_sequencing_chain::SyndicateSequencingChain,
};
use shared::{parse::parse_address, types::FilledProvider};
use std::str::FromStr;
use tracing::{debug, info};

/// Arguments for updating base and ethereum block hashes
#[derive(Args, Debug)]
pub struct UpdateBaseAndEthereumBlockHashesArgs {
    /// Base chain RPC URL
    #[arg(long, env = "BASE_RPC_URL")]
    pub base_rpc_url: String,
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
pub async fn update_base_and_ethereum_block_hashes(args: &UpdateBaseAndEthereumBlockHashesArgs) {
    let provider = new_provider(&args.base_rpc_url, &args.private_key).await;
    let receipt = BlockHashRelayer::new(args.relayer_address, provider)
        .sendBlockHashes_0(args.gas_archive_address)
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

    // TODO wait until we see new block hashes on the staking appchain (?)
}

/// Arguments for submitting gas proofs to confirm epoch data hash
#[derive(Args, Debug)]
pub struct SubmitGasProofsArgs {
    /// Sequencing chain RPC URL
    #[arg(long, env = "SEQ_CHAIN_RPC_URL")]
    pub seq_chain_rpc_url: String,
    /// Ethereum RPC URL
    #[arg(long, env = "ETHEREUM_RPC_URL")]
    pub ethereum_rpc_url: String,
    /// Staking aoppchain RPC URL
    #[arg(long, env = "STAKING_APPCHAIN_RPC_URL")]
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

    let gas_archive = GasArchive::new(args.gas_archive_address, staking_provider);

    // get the known ethereum block hash from the gas archive
    let eth_block_hash = gas_archive
        .lastKnownEthereumBlockHash()
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get last known ethereum block hash: {e}"));

    // get the outbox contract address from the gas archive
    let seq_chain_id = seq_provider
        .get_chain_id()
        .await
        .unwrap_or_else(|e| panic!("failed to get sequencing chain ID: {e}"));
    let outbox_contract_addr = gas_archive
        .seqChainEthOutbox(U256::from(seq_chain_id))
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get outbox contract address: {e}"));
    let gas_aggregator_address = gas_archive
        .seqChainGasAggregatorAddresses(U256::from(seq_chain_id))
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get gas aggregator address: {e}"));
    let epoch_data_hash_storage_slot_index = gas_archive
        .seqChainEthSendRootStorageSlot(U256::from(seq_chain_id))
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get epoch data hash storage slot index: {e}"));

    //
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

    // TODO make this configurable and explain how to get it from the nitro contracts
    let roots_mapping_storage_slot_index = U256::from(3);

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

    let storage_key: StorageKey =
        keccak256((sendroot_event.outputRoot, roots_mapping_storage_slot_index).abi_encode());

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

    let receipt = gas_archive
        .confirmSequencingChainBlockHash(
            U256::from(seq_chain_id),
            sendroot_event.outputRoot,
            rlp_encoded_eth_block_header.into(),
            seq_chain_block_hash_proof.account_proof.clone(),
            seq_chain_block_hash_proof
                .storage_proof
                .first()
                .unwrap_or_else(|| panic!("no storage proof found for sequencing chain block hash"))
                .proof
                .clone(),
        )
        .send()
        .await
        .unwrap_or_else(|e| panic!("confirming sequencing chain block hash failed: {e}"))
        .get_receipt()
        .await
        .unwrap_or_else(|e| panic!("getting receipt failed: {e}"));

    assert!(
        receipt.status(),
        "failed to confirm sequencing chain block hash. receipt: {receipt:?}"
    );
    info!(
        "successfully confirmed sequencing chain block hash. tx: {}, seq_block_hash:{}",
        receipt.transaction_hash, seq_block_hash
    );

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

    let epoch_data_hash = gas_aggregator
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
            epoch,
            U256::from(seq_chain_id),
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

    info!(
        "Submitting epoch pre-image data for epoch {} on seq chain {}: {}",
        epoch, seq_chain_id, epoch_data_hash
    );

    let (appchains, tokens, emissions_receivers) =
        get_aggregated_chain_data(epoch, gas_aggregator.clone()).await;

    assert_eq!(
        epoch_data_hash,
        keccak256((appchains.clone(), tokens.clone(), emissions_receivers.clone()).abi_encode()),
        "epoch data hash doesn't match the data obtained"
    );

    let receipt = gas_archive
        .submitEpochPreImageData(
            epoch,
            U256::from(seq_chain_id),
            appchains,
            tokens,
            emissions_receivers,
        )
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
    let offchain_aggregation = gas_aggregator
        .fallbackToOffchainAggregation()
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get fallback to offchain aggregation: {e}"));
    let factory_address = gas_aggregator
        .factory()
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get factory address: {e}"));
    let factory = SyndicateFactory::new(factory_address, gas_aggregator.provider().clone());

    let getAppchainsAndContractsReturn { _chainIDs: mut appchains, _contracts: appchain_contracts } =
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
                .emissionsReceiver()
                .call()
                .await
                .unwrap_or_else(|e| panic!("failed to get emissions receiver: {e}")),
        );
    }

    if offchain_aggregation {
        // TODO SEQ-1385: need to make the allowed implementation check here (chain's seq contract
        // impl must be supported by the factory)
        let chain_count = gas_aggregator
            .maxAppchainsToQuery()
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to get max appchains to query: {e}"));

        // Create indexed tuples to sort together
        let mut sorted: Vec<(U256, U256, Address)> = (0..appchains.len())
            .map(|i| (appchains[i], tokens[i], emissions_receivers[i]))
            .collect();

        // Sort by tokens used (highest first)
        sorted.sort_by(|a, b| b.2.cmp(&a.2));

        // Select only the top {chain_count} chains
        sorted.truncate(chain_count.to());

        // Order the remaining chains by chainID (lowest first)
        sorted.sort_by(|a, b| a.1.cmp(&b.1));

        // Reconstruct the sorted arrays
        for (i, (appchain, token, receiver)) in sorted.iter().enumerate() {
            appchains[i] = *appchain;
            tokens[i] = *token;
            emissions_receivers[i] = *receiver;
        }
    }
    (appchains, tokens, emissions_receivers)
}

/// Arguments for aggregating gas data and submitting epoch pre-image data
#[derive(Args, Debug)]
pub struct AggregateGasDataTopNChainsArgs {
    /// Sequencing chain RPC URL
    #[arg(long, env = "SEQ_CHAIN_RPC_URL")]
    pub seq_chain_rpc_url: String,
    /// Private key for signing transactions
    #[arg(long, env = "PRIVATE_KEY")]
    pub private_key: String,
    /// Address of the gas aggregator contract
    #[arg(long, value_parser=parse_address)]
    pub gas_aggregator_address: Address,
    /// Epoch number (will default to the latest finalized epoch if not provided)
    #[arg(long)]
    pub epoch: Option<u64>,
}

/// Aggregates gas data from top N chains and submits epoch pre-image data
///
/// This function calls the `submitEpochPreImageData` function on the `GasArchive` contract
/// with the aggregated gas usage data from multiple appchains for a specific epoch.
pub async fn aggregate_gas_data_top_n_chains(args: &AggregateGasDataTopNChainsArgs) {
    let provider = new_provider(&args.seq_chain_rpc_url, &args.private_key).await;
    let gas_aggregator = GasAggregator::new(args.gas_aggregator_address, provider);
    let epoch = match args.epoch {
        Some(epoch) => U256::from(epoch),
        None => gas_aggregator
            .getCurrentEpoch()
            .call()
            .await
            .unwrap_or_else(|e| panic!("unable to get current epoch: {e}"))
            .saturating_sub(U256::from(1)),
    };
    let (appchains, _, _) = get_aggregated_chain_data(epoch, gas_aggregator.clone()).await;

    let receipt = gas_aggregator
        .submitOffchainTopChains(appchains)
        .send()
        .await
        .unwrap_or_else(|e| panic!("failed to submit offchain top chains: {e}"))
        .get_receipt()
        .await
        .unwrap_or_else(|e| panic!("failed to get receipt for offchain top chains: {e}"));
    assert!(receipt.status(), "failed to submit offchain top chains. receipt: {receipt:?}");
    info!("successfully submitted top chains");
}

async fn new_provider(rpc_url: &str, private_key: &str) -> FilledProvider {
    ProviderBuilder::new()
        .wallet(EthereumWallet::from(
            PrivateKeySigner::from_str(private_key)
                .unwrap_or_else(|e| panic!("invalid private key: {e}")),
        ))
        .connect(rpc_url)
        .await
        .unwrap_or_else(|e| panic!("unable to create provider: {e}"))
}
