//! Ingestor logic

use crate::{eth_client::EthClient, metrics::ChainIngestorMetrics, server::Context};
use alloy::{
    consensus::constants::EMPTY_RECEIPTS,
    eips::BlockNumberOrTag,
    primitives::{Address, Log},
};
use eyre::eyre;
use jsonrpsee::{core::StringError, SubscriptionMessage, SubscriptionSink};
use shared::types::{BlockRef, PartialBlock};
use std::{collections::HashSet, sync::Mutex};
use tracing::{error, warn};

/// run the main block fetching loop. get blocks via a websocket subscription instead of polling.
/// only poll for logs.
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
        if block.number > next_block {
            buffer = Some(block);
            block = provider.get_block_header(BlockNumberOrTag::Number(next_block)).await;
        } else if block.number < next_block {
            let mut lock = ctx.lock().unwrap();
            if !lock.db.update_block(&block, &metrics) {
                continue;
            }
        }

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

        let mut lock = ctx.lock().unwrap();
        assert_eq!(block.number, lock.db.next_block());

        if lock.db.update_block(&block, &metrics) {
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
            logs: receipts.into_iter().flat_map(|x| x.logs.into_iter().map(|x| x.into())).collect(),
        };

        lock.subs.retain_mut(|(sink, addrs)| {
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

/// process a new subscription
pub fn subscribe(
    mut sink: SubscriptionSink,
    ctx: &Mutex<Context>,
    start_block: u64,
    addresses: Vec<Address>,
) -> Result<(), StringError> {
    let mut lock = ctx.lock().unwrap();
    if start_block <= lock.db.start_block {
        return Err(eyre!(
            "start block {} not after chain ingestor start block {}",
            start_block,
            lock.db.start_block
        )
        .into());
    }
    let next_block = lock.db.next_block();
    if start_block > next_block {
        return Err(eyre!("start block {} after next db block {}", start_block, next_block).into());
    }
    let mut addrs = HashSet::new();
    for addr in addresses {
        addrs.insert(addr);
    }

    //  a bit hacky - send initial block data as log data
    sink.try_send(
        SubscriptionMessage::from_json(&PartialBlock {
            logs: vec![Log::new_unchecked(
                Default::default(),
                Default::default(),
                lock.db.get_block_bytes(start_block - 1),
            )],
            ..Default::default()
        })
        .unwrap(),
    )?;

    lock.subs.push((sink, addrs));
    Ok(())
}
