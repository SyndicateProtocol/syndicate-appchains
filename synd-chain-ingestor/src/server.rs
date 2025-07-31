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
    core::SubscriptionError, types::ErrorObjectOwned, RpcModule, SubscriptionMessage,
    SubscriptionSink,
};
use serde::{Deserialize, Serialize};
use serde_json;
use shared::{tracing::SpanKind, types::PartialBlock};
use std::{
    collections::{HashSet, VecDeque},
    io::Error,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use tokio::task::JoinHandle;
use tracing::{error, info, instrument};

#[derive(Debug)]
#[allow(missing_docs)]
pub struct Context {
    pub db: Option<DB>,
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

/// Syncs the database to the latest block.
#[allow(clippy::cognitive_complexity)]
#[instrument(skip(provider, metrics), fields(otel.kind = ?SpanKind::Internal))]
pub async fn sync_db(
    provider: &EthClient,
    db_file: &str,
    start_block: u64,
    chain_id: u64,
    parallel_sync_requests: u64,
    metrics: &ChainIngestorMetrics,
) -> Result<DB, Error> {
    let mut db = DB::open(db_file, start_block, chain_id)?;

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

    Ok(db)
}

#[allow(clippy::unwrap_used)]
fn handle_subscription(
    mut sink: SubscriptionSink,
    ctx: &Mutex<Context>,
    start_block: u64,
    addresses: Vec<Address>,
) -> Result<(), SubscriptionError> {
    let mut lock = ctx.lock()?;

    // Check if DB is ready
    let db = match &lock.db {
        Some(db) => db,
        None => {
            return Err(eyre!("Chain ingestor is still syncing").into());
        }
    };

    if start_block <= db.start_block {
        return Err(eyre!(
            "start block {} not after chain ingestor start block {}",
            start_block,
            db.start_block
        )
        .into());
    }

    let next_block = db.next_block();
    if start_block > next_block {
        return Err(eyre!("start block {} after next db block {}", start_block, next_block).into());
    }

    let mut addrs = HashSet::new();
    for addr in addresses {
        addrs.insert(addr);
    }

    let message = Message::Init(db.get_block_bytes(start_block - 1));
    sink.try_send(SubscriptionMessage::from(serde_json::value::to_raw_value(&message).unwrap()))?;

    lock.subs.push((sink, addrs));
    drop(lock);
    Ok(())
}

/// The code for the "resource-unavailable" JSON-RPC error.
pub const RESOURCE_UNAVAILABLE_CODE: i32 = -32002;

/// Returns an error object indicating that the chain ingestor is still syncing.
pub fn error_still_syncing() -> ErrorObjectOwned {
    ErrorObjectOwned::owned(
        RESOURCE_UNAVAILABLE_CODE,
        "Chain ingestor is still syncing",
        None::<()>,
    )
}

/// Creates a new `RpcModule` that handles JSON-RPC requests.
#[allow(clippy::unwrap_used, clippy::option_if_let_else)]
pub fn create_module(
    ctx: Arc<Mutex<Context>>,
    ws_urls: Vec<String>,
    is_ready: Arc<AtomicBool>,
) -> RpcModule<Mutex<Context>> {
    let mut module = RpcModule::from_arc(ctx);

    module
        .register_method("health", |_, _, _| {
            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "status": "ok"
            }))
        })
        .unwrap();

    module
        .register_method("ready", move |_, _, _| {
            let ready = is_ready.load(std::sync::atomic::Ordering::SeqCst);
            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "ready": ready,
            }))
        })
        .unwrap();

    module.register_method("urls", move |_, _, _| ws_urls.clone()).unwrap();

    module
        .register_method("eth_blockNumber", move |_, ctx, _| {
            let lock = ctx.lock().unwrap();
            match &lock.db {
                Some(db) => Ok(db.next_block() - 1),
                None => Err(error_still_syncing()),
            }
        })
        .unwrap();

    module
        .register_method("block", |p, ctx, _| {
            let (block_number,): (u64,) = p.parse()?;
            let data = ctx.lock().unwrap();
            match &data.db {
                Some(db) => Ok(db.in_range(block_number).then(|| db.get_block(block_number))),
                None => Err(error_still_syncing()),
            }
        })
        .unwrap();

    module
        .register_subscription(
            "subscribe_blocks",
            "blocks",
            "unsubscribe_blocks",
            move |p, pending, ctx, _| async move {
                let (start_block, addresses): (u64, Vec<Address>) = p.parse()?;

                // Check if we're ready before accepting the subscription
                {
                    let lock = ctx.lock().unwrap();
                    if lock.db.is_none() {
                        return Err(SubscriptionError::from(eyre!(
                            "Chain ingestor is still syncing"
                        )));
                    }
                }

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
