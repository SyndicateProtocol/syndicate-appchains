//! The ingestor crate contains a function used to ingest new blocks from the chain. A websocket
//! connection is used instead of polling. Logs are fetched via a call to `eth_getBlockReceipts`.

use crate::{eth_client::EthClient, metrics::ChainIngestorMetrics, server::Context};
use alloy::{consensus::constants::EMPTY_RECEIPTS, eips::BlockNumberOrTag};
use jsonrpsee::SubscriptionMessage;
use shared::types::{BlockRef, PartialBlock};
use std::{cmp::Ordering, sync::Mutex};
use tracing::{error, warn};

#[allow(missing_docs)]
#[allow(clippy::unwrap_used)]
pub async fn run(
    ctx: &Mutex<Context>,
    provider: &EthClient,
    metrics: &ChainIngestorMetrics,
) -> eyre::Result<()> {
    let mut sub = provider.subscribe_blocks().await;
    let start_block = ctx.lock().unwrap().db.start_block;

    let mut buffer = None;

    loop {
        let mut block = buffer.take().unwrap_or(sub.recv().await?);
        let next_block = ctx.lock().unwrap().db.next_block();
        assert!(block.number > start_block);

        // if we're missing blocks, get the missing blocks first
        match block.number.cmp(&next_block) {
            Ordering::Less => {
                ctx.lock().unwrap().db.update_block(&block, metrics);
                continue;
            }
            Ordering::Greater => {
                buffer = Some(block);
                block = provider.get_block_header(BlockNumberOrTag::Number(next_block)).await;
            }
            Ordering::Equal => {}
        };

        // fetch receipts
        let mut receipts = provider.get_block_receipts(block.number).await;

        // refetch block & receipts until the block hash matches
        while match receipts.first() {
            Some(receipt) => receipt.block_hash != block.hash,
            None => block.receipts_root != EMPTY_RECEIPTS,
        } {
            warn!("mismatching block & receipt data - refetching both");
            block = provider.get_block_header(BlockNumberOrTag::Number(block.number)).await;
            receipts = provider.get_block_receipts(block.number).await;
        }

        assert_eq!(block.number, ctx.lock().unwrap().db.next_block());

        if ctx.lock().unwrap().db.update_block(&block, metrics) {
            if buffer.is_none() {
                buffer = Some(block);
            }
            continue
        }

        // send block to subscribers
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

        ctx.lock().unwrap().subs.retain_mut(|(sink, addrs)| {
            !sink.is_closed() &&
                sink.try_send(
                    SubscriptionMessage::from_json(&PartialBlock {
                        logs: partial_block
                            .logs
                            .clone()
                            .into_iter()
                            .filter(|log| addrs.contains(&log.address))
                            .collect(),
                        block_ref: partial_block.block_ref.clone(),
                        parent_hash: partial_block.parent_hash,
                    })
                    .unwrap(),
                )
                .inspect_err(|err| error!("try_send failed: {}", err))
                .is_ok()
        });
    }
}
