//! The `synd-chain-ingestor` crate contains a function used to ingest new blocks from the chain. A
//! websocket connection is used instead of polling. Logs are fetched via a call to
//! `eth_getBlockReceipts`.

use crate::{
    db::BlockUpdateResult,
    eth_client::EthClient,
    metrics::ChainIngestorMetrics,
    server::{Context, Message},
};
use alloy::{consensus::constants::EMPTY_RECEIPTS, eips::BlockNumberOrTag};
use jsonrpsee::SubscriptionMessage;
use shared::{
    tracing::SpanKind,
    types::{BlockRef, PartialBlock},
};
use std::{cmp::Ordering, sync::Mutex};
use tracing::{error, info_span, instrument, warn};

#[allow(missing_docs)]
#[instrument(
    skip_all,
    err,
    fields(
        otel.kind = ?SpanKind::Internal,
    )
)]
pub async fn run(
    ctx: &Mutex<Context>,
    provider: &EthClient,
    metrics: &ChainIngestorMetrics,
) -> eyre::Result<()> {
    let mut sub = provider.subscribe_blocks().await;
    let mut block_count = ctx
        .lock()
        .map_err(|e| eyre::eyre!("Failed to acquire mutex lock: {}", e))?
        .db
        .as_ref()
        .ok_or_else(|| eyre::eyre!("Database not initialized"))?
        .next_block();

    loop {
        let next_block = ctx
            .lock()
            .map_err(|e| eyre::eyre!("Failed to acquire mutex lock: {}", e))?
            .db
            .as_ref()
            .ok_or_else(|| eyre::eyre!("Database not initialized"))?
            .next_block();

        // fetch next block
        let mut block = match next_block.cmp(&block_count) {
            Ordering::Less => provider.get_block_header(BlockNumberOrTag::Number(next_block)).await,
            Ordering::Equal => {
                let block = sub.recv().await?;
                if block.number >= block_count {
                    block_count = block.number + 1;
                }
                if block.number > next_block {
                    continue;
                }
                block
            }
            Ordering::Greater => panic!("next block greater than block count"),
        };

        // if the block is not added, continue
        if ctx
            .lock()
            .map_err(|e| eyre::eyre!("Failed to acquire mutex lock: {}", e))?
            .db
            .as_mut()
            .ok_or_else(|| eyre::eyre!("Database not initialized"))?
            .update_block(&block, metrics)
            != BlockUpdateResult::Added
        {
            continue;
        }

        // fetch receipts
        let mut receipts = provider.get_block_receipts(block.number).await;

        // refetch block and receipts until the block hash matches
        while match receipts.first() {
            Some(receipt) => receipt.block_hash != block.hash,
            None => block.receipts_root != EMPTY_RECEIPTS,
        } {
            warn!("mismatched block and receipt data - refetching both");
            block = provider.get_block_header(BlockNumberOrTag::Number(block.number)).await;
            receipts = provider.get_block_receipts(block.number).await;
        }

        // if the block reorgs the db to an earlier block number, continue
        if ctx
            .lock()
            .map_err(|e| eyre::eyre!("Failed to acquire mutex lock: {}", e))?
            .db
            .as_mut()
            .ok_or_else(|| eyre::eyre!("Database not initialized"))?
            .update_block(&block, metrics)
            == BlockUpdateResult::Reorged
        {
            continue;
        }

        // send block to subscribers
        {
            let _guard = info_span!("send_subscriptions").entered();

            let partial_block = PartialBlock {
                block_ref: BlockRef {
                    number: block.number,
                    timestamp: block.timestamp,
                    hash: block.hash,
                },
                parent_hash: block.parent_hash,
                logs: receipts
                    .into_iter()
                    .enumerate()
                    .flat_map(|(i, x)| {
                        assert_eq!(x.block_hash, block.hash);
                        assert_eq!(x.transaction_index, i as u64);
                        x.logs
                    })
                    .collect(),
            };

            #[allow(clippy::unwrap_used)]
            ctx.lock()
                .map_err(|e| eyre::eyre!("Failed to acquire mutex lock: {}", e))?
                .subs
                .retain_mut(|(sink, addrs)| {
                    !sink.is_closed()
                        && sink
                            .try_send(SubscriptionMessage::from(
                                serde_json::value::to_raw_value(&Message::Block(PartialBlock {
                                    logs: partial_block
                                        .logs
                                        .clone()
                                        .into_iter()
                                        .filter(|log| addrs.contains(&log.address))
                                        .collect(),
                                    block_ref: partial_block.block_ref.clone(),
                                    parent_hash: partial_block.parent_hash,
                                }))
                                .unwrap(),
                            ))
                            .inspect_err(|err| error!("try_send failed: {err}"))
                            .is_ok()
                });
        }
    }
}
