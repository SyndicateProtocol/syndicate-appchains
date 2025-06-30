//! The server crate is used to create a `RpcModule` that handles websocket jsonrpc requests.
use crate::{
    db::{BlockUpdateResult, DB},
    eth_client::EthClient,
    metrics::ChainIngestorMetrics,
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, Bytes},
};
use eyre::eyre;
use jsonrpsee::{
    core::StringError, types::ErrorObjectOwned, RpcModule, SubscriptionMessage, SubscriptionSink,
};
use serde::{Deserialize, Serialize};
use shared::types::PartialBlock;
use std::{
    collections::{HashSet, VecDeque},
    sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;
use tracing::{error, info};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct Context {
    pub db: DB,
    pub subs: Vec<(SubscriptionSink, HashSet<Address>)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Message {
    Init(Bytes),
    Block(PartialBlock),
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
        // panic if the thread panics
        #[allow(clippy::unwrap_used)]
        Some(self.handles.pop_front()?.await.unwrap())
    }
}

#[allow(clippy::unwrap_used, clippy::cognitive_complexity)]
#[allow(missing_docs)]
#[allow(clippy::cognitive_complexity)]
pub async fn start(
    provider: &EthClient,
    ws_url: &str,
    db_name: &str,
    start_block: u64,
    parallel_sync_requests: u64,
    metrics: &ChainIngestorMetrics,
) -> eyre::Result<(RpcModule<Mutex<Context>>, Arc<Mutex<Context>>)> {
    let mut db = DB::open(db_name, start_block, provider.get_chain_id().await)?;

    // reorg db if necessary.
    info!("checking for reorgs");
    while db.count > 0 {
        let block_num = db.next_block() - 1;
        let header = provider.get_block_header(BlockNumberOrTag::Number(block_num)).await;
        if db.update_block(&header, metrics) != BlockUpdateResult::Reorged {
            break;
        }
    }

    // sync to latest, stopping early if a reorg is detected.
    // repeat twice in case the first sync is slow and the latest block ends up being very outdated
    for _ in 0..2 {
        let latest = provider.get_block_header(BlockNumberOrTag::Latest).await.number;
        let start_sync = db.next_block();
        info!("syncing from {} to latest block {}", start_sync, latest);
        let mut ingestor = BlockIngestor::new(start_sync, latest, parallel_sync_requests, provider);
        while let Some(block) = ingestor.next().await {
            match db.update_block(&block, metrics) {
                BlockUpdateResult::Reorged => {
                    error!("reorged during initial sync on block {block:?}");
                    break;
                }
                BlockUpdateResult::Added => {
                    if block.number % 10000 == 0 {
                        info!(
                            "synced to block {} of {} ({} %)",
                            block.number,
                            latest,
                            (block.number - start_sync) as f32 * 100.0 /
                                (latest + 1 - start_sync) as f32
                        )
                    }
                }
                BlockUpdateResult::Verified => panic!("unexpected update_block result: verified"),
            }
        }
    }
    info!("synced to latest block");

    let ctx = Arc::new(Mutex::new(Context { db, subs: Default::default() }));
    Ok((create_module(ctx.clone(), ws_url.to_string()), ctx))
}

#[allow(clippy::unwrap_used)]
fn handle_subscription(
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

    sink.try_send(
        SubscriptionMessage::from_json(&Message::Init(lock.db.get_block_bytes(start_block - 1)))
            .unwrap(),
    )?;

    lock.subs.push((sink, addrs));
    drop(lock);
    Ok(())
}

#[allow(clippy::unwrap_used)]
fn create_module(ctx: Arc<Mutex<Context>>, ws_url: String) -> RpcModule<Mutex<Context>> {
    let mut module = RpcModule::from_arc(ctx);

    module.register_method("url", move |_, _, _| ws_url.clone()).unwrap();
    module
        .register_method("eth_blockNumber", move |_, ctx, _| {
            ctx.lock().unwrap().db.next_block() - 1
        })
        .unwrap();

    module
        .register_method("block", |p, ctx, _| {
            let (block_number,): (u64,) = p.parse()?;
            let data = ctx.lock().unwrap();
            Ok::<_, ErrorObjectOwned>(
                data.db.in_range(block_number).then(|| data.db.get_block(block_number)),
            )
        })
        .unwrap();

    module
        .register_subscription(
            "subscribe_blocks",
            "blocks",
            "unsubscribe_blocks",
            move |p, pending, ctx, _| async move {
                let (start_block, addresses): (u64, Vec<Address>) = p.parse()?;
                let sink = pending.accept().await?;
                handle_subscription(sink.clone(), ctx.as_ref(), start_block, addresses)
                    .inspect_err(|e| error!("ws connection error: {:?}", e))?;
                sink.closed().await;
                drop(sink);
                Ok(())
            },
        )
        .unwrap();

    module
}
