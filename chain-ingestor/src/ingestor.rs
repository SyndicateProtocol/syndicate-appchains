//! Ingestor logic

use crate::{eth_client::EthClient, metrics::ChainIngestorMetrics, server::Context};
use alloy::{
    consensus::constants::EMPTY_RECEIPTS, eips::BlockNumberOrTag, primitives::Address,
    rpc::types::Filter,
};
use eyre::eyre;
use jsonrpsee::{core::StringError, SubscriptionMessage, SubscriptionSink};
use shared::types::{BlockRef, PartialBlock, PartialLog};
use std::{collections::HashSet, sync::Mutex};
use tracing::{error, warn};

/// run the main block fetching loop. get blocks via a websocket subscription instead of polling.
/// only poll for logs.
pub async fn run(
    ctx: &Mutex<Context>,
    provider: &EthClient,
    cache_size: u64,
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
            // if we received an old block, remove stale new ones.
            lock.db.reorg_block(block.number);
            metrics.record_reorg(next_block - block.number);
            for _ in block.number..next_block {
                lock.buffer.pop_back();
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

        // remove parent block from the db if the hash does not match
        let parent_hash = lock.db.get_block(block.number - 1).unwrap().1;
        if block.parent_hash != parent_hash {
            lock.db.reorg_block(block.number);
            metrics.record_reorg(1);
            lock.buffer.pop_back();
            // skip waiting for the next block since it already exists
            if buffer.is_none() {
                buffer = Some(block);
            }
            continue
        }

        // add block to db
        lock.db.add_block(block.timestamp as u32, block.hash);
        metrics.record_block(block.number, block.timestamp);

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
                .flat_map(|x| {
                    x.logs.into_iter().map(|x| PartialLog {
                        address: x.address,
                        topics: x.topics,
                        data: x.data,
                    })
                })
                .collect(),
        };
        let msg = SubscriptionMessage::from_json(&partial_block).unwrap();
        lock.buffer.push_back(partial_block);
        if lock.buffer.len() > cache_size as usize {
            lock.buffer.pop_front();
        }

        lock.subs.retain_mut(|sink| {
            !sink.is_closed() &&
                sink.try_send(msg.clone())
                    .inspect_err(|err| error!("try_send failed: {}", err))
                    .is_ok()
        });
    }
}

/// process a new subscription
pub async fn subscribe(
    mut sink: SubscriptionSink,
    ctx: &Mutex<Context>,
    mut start_block: u64,
    addresses: Vec<Address>,
) -> Result<(), StringError> {
    let provider;
    {
        let lock = ctx.lock().unwrap();
        if start_block <= lock.db.start_block {
            return Err(eyre!(
                "start block {} before chain ingestor start block {}",
                start_block,
                lock.db.start_block
            )
            .into());
        }
        let next_block = lock.db.next_block();
        if start_block > next_block {
            return Err(
                eyre!("start block {} after next db block {}", start_block, next_block).into()
            );
        }
        provider = lock.provider.clone();
    }

    let mut addrs: HashSet<Address> = Default::default();
    for addr in addresses.clone() {
        addrs.insert(addr);
    }

    let safe_block = provider.get_block_header(BlockNumberOrTag::Safe).await.number;
    let mut logs = None;
    if start_block <= safe_block {
        logs = Some(
            provider
                .get_logs(
                    &Filter::new()
                        .from_block(BlockNumberOrTag::Number(start_block))
                        .to_block(BlockNumberOrTag::Number(safe_block))
                        .address(addresses),
                )
                .await?,
        );
    }

    {
        let mut lock = ctx.lock().unwrap();
        let next_block = lock.db.next_block();
        if start_block > next_block {
            return Err(
                eyre!("start block {} after next db block {}", start_block, next_block).into()
            );
        }
        let buf_block = lock.buffer.front().map(|x| x.block_ref.number).unwrap_or(next_block);
        if start_block < buf_block {
            // safe block must include all blocks before the start of the buffer
            if safe_block + 1 < buf_block {
                return Err(
                    eyre!("safe block {} before start of buffer {}", safe_block, buf_block).into()
                );
            }
            let mut parent_hash = lock.db.get_block(start_block - 1).unwrap().1;
            let mut blocks = Vec::default();
            for i in start_block..buf_block {
                let (timestamp, hash) = lock.db.get_block(i).unwrap();
                blocks.push(PartialBlock {
                    block_ref: BlockRef { number: i, timestamp: timestamp as u64, hash },
                    parent_hash,
                    logs: Default::default(),
                });
                parent_hash = hash;
            }
            // TODO: assert that logIndex is increasing per log per block
            for log in logs.unwrap() {
                let block_number = log.block_number.unwrap();
                assert!(block_number >= start_block);
                if block_number >= buf_block {
                    continue;
                }
                let block = &mut blocks[(block_number - start_block) as usize];
                assert_eq!(log.block_hash.unwrap(), block.block_ref.hash);
                let address = log.address();
                assert!(addrs.contains(&address));
                let (topics, data) = log.into_inner().data.split();
                block.logs.push(PartialLog { address, topics, data });
            }
            for block in blocks {
                sink.try_send(SubscriptionMessage::from_json(&block)?)?;
            }

            start_block = buf_block;
        }
        for block in lock.buffer.range((start_block - buf_block) as usize..) {
            let mut block = block.clone();
            block.logs = block.logs.into_iter().filter(|x| addrs.contains(&x.address)).collect();
            sink.try_send(SubscriptionMessage::from_json(&block)?)?;
        }

        lock.subs.push(sink.clone());
    }

    sink.closed().await;
    Ok(())
}
