//! Server
use crate::{db::DB, eth_client::EthClient, ingestor::subscribe};
use alloy::{eips::BlockNumberOrTag, primitives::Address, rpc::types::Filter};
use jsonrpsee::{types::ErrorObjectOwned, RpcModule, SubscriptionSink};
use shared::types::{BlockRef, PartialBlock, PartialLog};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;
use tracing::{error, info};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct Context {
    pub provider: EthClient,
    pub db: DB,
    pub buffer: VecDeque<PartialBlock>,
    pub subs: Vec<SubscriptionSink>,
}

struct BlockIngestor<'a> {
    block: u64,
    end_block: u64,
    provider: &'a EthClient,
    handles: VecDeque<JoinHandle<alloy::rpc::types::Header>>,
    request_limit: u64,
}
impl<'a> BlockIngestor<'a> {
    fn new(start_block: u64, end_block: u64, request_limit: u64, provider: &'a EthClient) -> Self {
        Self { block: start_block, end_block, provider, request_limit, handles: Default::default() }
    }
    async fn next(&mut self) -> Option<alloy::rpc::types::Header> {
        while self.handles.len() < self.request_limit as usize && self.block <= self.end_block {
            let provider = self.provider.clone();
            let block = self.block;
            self.block += 1;
            self.handles.push_back(tokio::spawn(async move {
                provider.get_block_header(BlockNumberOrTag::Number(block)).await
            }));
        }
        Some(self.handles.pop_front()?.await.unwrap())
    }
}

#[allow(clippy::unwrap_used)]
#[allow(missing_docs)]
pub async fn start(
    provider: EthClient,
    rpc_url: &str,
    db_name: &str,
    start_block: u64,
    parallel_sync_requests: u64,
) -> eyre::Result<(RpcModule<Mutex<Context>>, Arc<Mutex<Context>>)> {
    let mut db = DB::open(db_name, start_block, provider.get_chain_id().await)?;

    // reorg db if necessary.
    info!("checking for reorgs");
    while db.count > 0 {
        let block_num = db.next_block() - 1;
        let hash = provider.get_block_header(BlockNumberOrTag::Number(block_num)).await.hash;
        if hash != db.get_block(block_num).unwrap().1 {
            db.reorg_block(block_num);
        } else {
            break;
        }
    }

    // sync to latest. crash if a reorg is detected (extremely unlikely)
    // repeat twice in case the first sync is slow and the latest block ends up being very outdated
    for _ in 0..2 {
        info!("syncing to latest block");
        let latest = provider.get_block_header(BlockNumberOrTag::Latest).await.number;
        let mut ingestor =
            BlockIngestor::new(db.next_block(), latest, parallel_sync_requests, &provider);
        let mut parent = (db.count > 0).then(|| db.get_block(db.next_block() - 1).unwrap().1);
        loop {
            if let Some(block) = ingestor.next().await {
                if let Some(parent_hash) = parent {
                    assert_eq!(block.parent_hash, parent_hash);
                }
                parent = Some(block.hash);
                db.add_block(block.timestamp as u32, block.hash);
                if block.number % 1000 == 0 {
                    info!(
                        "synced to block {} of {} ({} %)",
                        block.number,
                        latest,
                        block.number as f32 * 100.0 / latest as f32
                    )
                }
            } else {
                break;
            }
        }
    }

    let mut buffer: VecDeque<PartialBlock> = Default::default();
    let safe_block = provider.get_block_header(BlockNumberOrTag::Safe).await.number;

    let end_block = db.next_block();
    let mut start_block = db.start_block;
    if end_block > start_block && end_block > safe_block {
        if safe_block > start_block {
            start_block = safe_block;
        }
        info!("populating buffer with {} blocks", end_block - start_block);

        let logs = provider
            .get_logs(&Filter::new().from_block(start_block).to_block(end_block - 1))
            .await?;

        let mut after_log_block = start_block;
        if let Some(log) = logs.last() {
            after_log_block = log.block_number.unwrap() + 1;
            assert_eq!(log.block_hash.unwrap(), db.get_block(after_log_block - 1).unwrap().1);
        }
        for i in after_log_block..end_block {
            let block_hash = db.get_block(i).unwrap().1;
            let receipts = provider.get_block_receipts(i).await;
            for receipt in receipts {
                assert_eq!(receipt.block_number, i);
                assert_eq!(receipt.block_hash, block_hash);
                assert_eq!(receipt.logs.len(), 0);
            }
        }

        let block = provider.get_block_header(BlockNumberOrTag::Number(start_block)).await;
        assert_eq!(block.hash, db.get_block(start_block).unwrap().1);
        let mut parent_hash = block.parent_hash;
        for i in start_block..end_block {
            let block = db.get_block(i).unwrap();
            buffer.push_back(PartialBlock {
                block_ref: BlockRef { number: i, timestamp: block.0 as u64, hash: block.1 },
                parent_hash,
                logs: Default::default(),
            });
            parent_hash = block.1;
        }

        for log in logs {
            buffer[(log.block_number.unwrap() - start_block) as usize]
                .logs
                .push(PartialLog::new(log));
        }
    }

    let ctx = Arc::new(Mutex::new(Context { db, buffer, subs: Default::default(), provider }));

    Ok((create_module(ctx.clone(), rpc_url.to_string()), ctx))
}

fn create_module(ctx: Arc<Mutex<Context>>, rpc_url: String) -> RpcModule<Mutex<Context>> {
    let mut module = RpcModule::from_arc(ctx);

    module.register_method("url", move |_, _, _| rpc_url.clone()).unwrap();
    module
        .register_method("eth_blockNumber", move |_, ctx, _| {
            ctx.lock().unwrap().db.next_block() - 1
        })
        .unwrap();

    module
        .register_method("block", |p, ctx, _| {
            let (block_number,): (u64,) = p.parse()?;
            let data = ctx.lock().unwrap();
            if let Some(start) = data.buffer.front() {
                let start_block = start.block_ref.number;
                if block_number >= start_block &&
                    block_number < data.buffer.back().unwrap().block_ref.number
                {
                    let block =
                        data.buffer[(block_number - start_block) as usize].block_ref.clone();
                    assert_eq!(block.number, block_number);
                    return Ok::<_, ErrorObjectOwned>(Some(block))
                }
            }
            Ok(data.db.get_block(block_number).map(|(ts, hash)| BlockRef {
                number: block_number,
                timestamp: ts as u64,
                hash,
            }))
        })
        .unwrap();

    module
        .register_subscription(
            "subscribe_blocks",
            "blocks",
            "unsubscribe_blocks",
            move |p, pending, ctx, _| async move {
                let (start_block, addresses): (u64, Vec<Address>) = p.parse()?;
                subscribe(pending.accept().await?, ctx.as_ref(), start_block, addresses)
                    .await
                    .inspect_err(|e| error!("ws connection error: {:?}", e))
            },
        )
        .unwrap();

    module
}
