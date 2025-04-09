//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, ingestor::assert_no_reorg, metrics::IngestorMetrics};
use alloy::{
    consensus::{Transaction, TxEnvelope},
    primitives::Address,
    providers::Provider,
    rlp::Decodable,
    rpc::types::{BlockNumberOrTag, Filter, Header},
};
use common::types::{BlockRef, Chain, PartialBlock, PartialLogWithTxdata};
use eyre::{eyre, Error};
use shared::bounded_join_set::BoundedJoinSet;
use std::{collections::BTreeMap, sync::Arc};
use tokio::{
    select,
    sync::{
        mpsc::{self, Sender},
        oneshot, Mutex,
    },
};
use tracing::{error, info, trace};

pub async fn run_subscription(
    chain: Chain,
    config: &ChainIngestorConfig,
    addresses: Vec<Address>,
    client: Arc<dyn Provider>,
    sender: Sender<Arc<PartialBlock>>,
    metrics: IngestorMetrics,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    info!(%chain, "Starting ingestor subscription");
    let base_filter = Filter::new().address(addresses);
    let start_block = config.start_block;

    let mut new_heads_sub = client.subscribe_blocks().await?;
    info!(%chain, "Subscribed to new block headers");

    //limit parallelism to the maximum batch size
    let tasks = Arc::new(Mutex::new(BoundedJoinSet::new(config.syncing_batch_size as usize + 1))); // +1 for the task that spawns all the others
    let (result_sender, mut result_receiver) = mpsc::channel(config.syncing_batch_size as usize);
    // let chain_head = new_heads_sub.recv().await?;

    let initial_head_header = match tokio::time::timeout(config.rpc_timeout, new_heads_sub.recv())
        .await
    {
        Ok(Ok(header)) => header,
        Ok(Err(e)) => {
            error!(%chain, error = %e, "Subscription error during initial head fetch");
            panic!("Subscription for chain {} error during initial head fetch: {}", chain, e);
        }
        Err(_) => {
            info!(%chain, "Subscription timed out, fetching latest block manually");
            let latest_block = client.get_block_by_number(BlockNumberOrTag::Latest).await?;
            latest_block
                .ok_or_else(|| eyre!("Failed to fetch latest block after subscription timeout"))?
                .header
        }
    };

    info!(
        %chain,
        %start_block,
        head_block = %initial_head_header.number,
        "Initial chain head received"
    );

    let tasks_clone = tasks.clone();
    tasks.lock().await.spawn(async move {
        // ingest all blocks from `config.start_block` to the latest head
        info!(%chain, %start_block, initial_head_header = %initial_head_header.number, "Syncing - ingesting blocks");
        for number in start_block..=initial_head_header.number {
            let client_clone = client.clone();
            let base_filter_clone = base_filter.clone();
            let result_sender_clone = result_sender.clone();
            let tasks_clone = tasks_clone.clone();

            trace!(%chain, block = %number, "Syncing - spawning task for block");
            tasks_clone.lock().await.spawn(async move {
                let block = client_clone
                    .get_block_by_number(BlockNumberOrTag::Number(number))
                    .await?
                    .unwrap_or_else(|| {
                        panic!("No block found for number: {} on chain {}", number, chain)
                    });
                process_block(
                    &*client_clone,
                    block.header,
                    &base_filter_clone,
                    &result_sender_clone,
                    chain,
                )
                .await
            });
        }

        info!(%chain, "Historical sync complete, now following new blocks");
        // start ingesting new heads
        loop {
            let client_clone = client.clone();
            let base_filter_clone = base_filter.clone();
            let result_sender_clone = result_sender.clone();

            let res = new_heads_sub.recv().await;
            match res {
                Ok(header) => {
                    info!(%chain, head_block_number = %header.number, "New chain head received");
                    tasks_clone.lock().await.spawn(async move {
                        process_block(
                            &*client_clone,
                            header,
                            &base_filter_clone,
                            &result_sender_clone,
                            chain,
                        )
                        .await
                    });
                }
                Err(e) => {
                    error!(%chain, error = %e, "Error receiving block");
                }
            }
        }
    });

    let mut blocks_collection: BTreeMap<u64, Arc<PartialBlock>> = BTreeMap::new();
    let mut latest_processed_block: Option<BlockRef> = None;

    trace!(%chain, "Starting ingestor loop");
    loop {
        select! {
            biased;
            _ = &mut shutdown_rx => {
                info!(%chain, "Ingestor stopped");
                tasks.lock().await.abort_all();
                return Ok(());
            }
            received_block = result_receiver.recv() => {
                match received_block {
                    Some(block) => {
                        trace!(%chain, block = %block.number, "Received block");
                        // Handle first block or next sequential block
                        let is_valid_next_block = latest_processed_block.as_ref().map_or_else(
                            || block.number == start_block,
                            |prev| {
                                assert_no_reorg(prev, &block);
                                block.number == prev.number + 1
                            }
                        );

                        if !is_valid_next_block {
                            // Store out-of-sequence block for later sending
                            trace!(%chain, block = %block.number, "Storing out-of-sequence block for later sending");
                            blocks_collection.insert(block.number, block);
                            continue;
                        }

                        // Process the valid block
                        trace!(%chain, block = %block.number, logs = %block.logs.len(), "Sending block");
                        latest_processed_block = Some(block.clone().into());
                        sender.send(block).await?;

                        // Process any subsequent blocks we have stored
                        while let Some(ref current_block) = latest_processed_block {
                            let next_block_number = current_block.number + 1;
                            let Some((_number, next_block)) = blocks_collection.remove_entry(&next_block_number) else {
                                break;
                            };
                            trace!(%chain, block = %next_block.number, logs = %next_block.logs.len(), "sending stored block");
                            assert_no_reorg(current_block, &next_block);
                            latest_processed_block = Some(next_block.clone().into());
                            sender.send(next_block).await?;
                        }
                    }
                    None => {
                        error!(%chain, "Channel closed");
                        return Ok(());
                    }
                }
            }
        }
        tasks.lock().await.join_next().await; // clear the task that delivered the processed block
    }
}

async fn process_block(
    client: &dyn Provider,
    header: Header,
    base_filter: &Filter,
    tx: &Sender<Arc<PartialBlock>>,
    chain: Chain,
) -> Result<(), Error> {
    trace!(%chain, block = %header.number, "Processing block");
    let filter = base_filter
        .clone()
        .from_block(BlockNumberOrTag::Number(header.number))
        .to_block(header.number);
    trace!(%chain, block = %header.number, "Fetching logs for block");
    let logs = client.get_logs(&filter).await?; // TODO getReceipts instead
    let mut logs_with_tx = Vec::new();
    for log in logs {
        let Some(tx_hash) = log.transaction_hash else {
            panic!("No transaction hash found for log on chain {}", chain);
        };
        let raw_tx_res = client.get_raw_transaction_by_hash(tx_hash).await?;
        let Some(raw_tx) = raw_tx_res else {
            panic!("No transaction found for hash: {} on chain {}", tx_hash, chain);
        };
        let tx = TxEnvelope::decode(&mut raw_tx.as_ref())?;
        logs_with_tx.push(PartialLogWithTxdata::new(log, tx.input().to_owned()));
    }
    trace!(%chain, block = %header.number, logs = %logs_with_tx.len(), "Block processed");
    tx.send(Arc::new(PartialBlock::new(header, logs_with_tx))).await?;
    Ok(())
}
