//! Client code is run in the syndicate translator instead of the synd-chain-ingestor application

use crate::{db::ITEM_SIZE, eth_client::EthClient, server::Message};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, Bytes, B256},
    rpc::types::Filter,
};
use eyre::{eyre, OptionExt as _};
use futures_util::{
    stream::{Peekable, ReadyChunks},
    FutureExt, Stream, StreamExt as _,
};
use jsonrpsee::{
    core::{
        async_trait,
        client::{ClientT as _, SubscriptionClientT},
        traits::ToRpcParams,
        ClientError,
    },
    ws_client::{PingConfig, WsClient, WsClientBuilder},
};
use serde::de::DeserializeOwned;
use shared::{
    tracing::SpanKind,
    types::{BlockBuilder, BlockRef, GetBlockRef, PartialBlock},
};
use std::{
    collections::{HashSet, VecDeque},
    future::Future,
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use tracing::{info, instrument};

/// Uses the [`EthClient`] to fetch log data for blocks in a range and combines them with raw
/// (timestamp, block hash) data from the db to build partial blocks
#[allow(clippy::unwrap_used, clippy::cognitive_complexity)]
async fn build_partial_blocks(
    start_block: u64,
    data: &IndexedBlockData,
    client: &EthClient,
    addrs: Vec<Address>,
) -> eyre::Result<Vec<PartialBlock>> {
    let count = data.count();
    let mut blocks = Vec::default();
    if count == 0 {
        return Ok(blocks);
    }
    // The first item is always just used to get the parent hash, so we skip it
    let mut parent_hash = data.get_item(0)?.1;
    for i in start_block..start_block + count {
        let (timestamp, hash) = data.get_item(i + 1 - start_block)?;
        blocks.push(PartialBlock {
            block_ref: BlockRef { number: i, timestamp, hash },
            parent_hash,
            logs: Default::default(),
        });
        parent_hash = hash;
    }

    let end_block = start_block + count - 1;

    let mut safe_block = client.get_block_header(BlockNumberOrTag::Latest).await.number;
    if safe_block < start_block {
        safe_block = start_block - 1;
    }

    info!("fetching partial logs from blocks {} to {}", start_block, end_block);
    let mut logs = client
        .get_logs(&Filter::new().address(addrs.clone()).from_block(start_block).to_block(end_block))
        .await?;

    if let Some(log) = logs.last() {
        let block_number = log.block_number.unwrap();
        if log.block_hash.unwrap() != blocks[(block_number - start_block) as usize].block_ref.hash {
            return Err(eyre!(
                "reorg detected: block {}, {} != {}",
                log.block_number.unwrap(),
                log.block_hash.unwrap(),
                blocks[(block_number - start_block) as usize].block_ref.hash
            ));
        }
        if block_number > safe_block {
            safe_block = block_number;
        }
    }

    if safe_block < end_block {
        // Fetch all logs for unsafe blocks. This makes it more likely that a log is included which
        // contains block hash info with it.
        let first_unsafe_block = safe_block + 1;
        info!("fetching full logs from blocks {} to {}", first_unsafe_block, end_block);
        let mut unsafe_logs = client
            .get_logs(&Filter::new().from_block(first_unsafe_block).to_block(end_block))
            .await?;

        if let Some(log) = unsafe_logs.last() {
            safe_block = log.block_number.unwrap();
            if log.block_hash.unwrap() != blocks[(safe_block - start_block) as usize].block_ref.hash
            {
                return Err(eyre!(
                    "reorg detected: block {}, {} != {}",
                    safe_block,
                    log.block_hash.unwrap(),
                    blocks[(safe_block - start_block) as usize].block_ref.hash
                ));
            }
            let mut addr_set = HashSet::new();
            for addr in &addrs {
                addr_set.insert(addr);
            }
            unsafe_logs.retain(|x| addr_set.contains(&x.address()));
            logs.append(&mut unsafe_logs);
        }

        // for blocks without any logs, refetch by block hash
        // to make sure the block hash matches
        for i in first_unsafe_block..end_block + 1 {
            info!("fetching logs for block {} of {}", i, end_block);
            let mut block_logs = client
                .get_logs(
                    &Filter::new()
                        .address(addrs.clone())
                        .at_block_hash(blocks[(i - start_block) as usize].block_ref.hash),
                )
                .await?;
            logs.append(&mut block_logs);
        }
    }

    let mut block = start_block - 1;
    let mut index = 0;
    for log in logs {
        assert!(!log.removed);
        let log_block = log.block_number.unwrap();
        assert_eq!(log.block_hash, Some(blocks[(log_block - start_block) as usize].block_ref.hash));
        let log_index = log.log_index.unwrap();
        assert!(log_block > block || (log_block == block && log_index > index), "out of order log found from rpc provider: previous (block, index) = ({block} {index}), current = ({log_block}, {log_index})");
        block = log_block;
        index = log_index;
        blocks[(log.block_number.unwrap() - start_block) as usize].logs.push(log.into_inner());
    }

    Ok(blocks)
}

struct BlockStream<
    S: Stream<Item = Result<Message, serde_json::Error>>,
    Block: GetBlockRef,
    B: BlockBuilder<Block> + Sync,
> {
    stream: Pin<Box<Peekable<ReadyChunks<S>>>>,
    buffer: VecDeque<Block>,
    block_builder: Arc<B>,
    indexed_block_number: u64,
    init_data: Option<(EthClient, Vec<Address>, u64)>,
    #[allow(clippy::type_complexity)]
    init_requests: VecDeque<
        Pin<
            Box<
                dyn Future<Output = Result<Vec<Result<Message, serde_json::Error>>, eyre::Error>>
                    + Send,
            >,
        >,
    >,
}

#[allow(missing_docs)]
impl<
        S: Stream<Item = Result<Message, serde_json::Error>> + Send + 'static,
        Block: GetBlockRef + Send,
        B: BlockBuilder<Block> + Sync,
    > BlockStream<S, Block, B>
{
    fn new(
        stream: S,
        block_builder: Arc<B>,
        start_block: u64,
        init_data: (EthClient, Vec<Address>, u64),
    ) -> Self {
        Self {
            stream: Box::pin(stream.ready_chunks(1024).peekable()),
            block_builder,
            buffer: Default::default(),
            indexed_block_number: start_block,
            init_data: Some(init_data),
            init_requests: Default::default(),
        }
    }

    /// Process the init message into initial requests to be processed later
    #[allow(clippy::unwrap_used)]
    async fn process_init_message(&mut self) -> eyre::Result<(), eyre::Error> {
        if let Some((client, addrs, max_blocks_per_request)) = self.init_data.take() {
            // fetch initial blocks from the stream
            let mut init_blocks = self.stream.next().await.ok_or_eyre("stream closed")?;
            // remove the first block from the stream, which is a special init message
            init_blocks.rotate_left(1);
            let mut init: IndexedBlockData = init_blocks.pop().unwrap()?.init();

            // get start and end blocks for batching
            let mut start_block = self.indexed_block_number;

            if max_blocks_per_request == 0 {
                self.init_requests.push_back(Box::pin(async move {
                    let blocks = build_partial_blocks(start_block, &init, &client, addrs)
                        .await?
                        .into_iter()
                        .map(|x| Ok(Message::Block(x)))
                        .collect();
                    Ok(blocks)
                }));
            } else {
                while init.count() > 0 {
                    let (init_batch, remaining) = init.split_at(max_blocks_per_request)?;

                    let client_clone = client.clone();
                    let addrs_clone = addrs.clone();

                    self.init_requests.push_back(Box::pin(async move {
                        let blocks = build_partial_blocks(
                            start_block,
                            &init_batch,
                            &client_clone,
                            addrs_clone,
                        )
                        .await?
                        .into_iter()
                        .map(|x| Ok(Message::Block(x)))
                        .collect();
                        Ok(blocks)
                    }));
                    start_block += max_blocks_per_request;
                    init = remaining;
                }
            };

            // make sure we don't drop any blocks from the stream
            if !init_blocks.is_empty() {
                self.init_requests.push_back(Box::pin(async move { Ok(init_blocks) }));
            }
        }

        Ok(())
    }
}

/// `BlockStream` is a stream of blocks that automatically updates stale/reorged blocks in the
/// queue.
#[async_trait]
pub trait BlockStreamT<Block> {
    /// recv fetches the next block once a block with timestamp greater than or equal to the
    /// provided one has arrived.
    async fn recv(&mut self, timestamp: u64) -> eyre::Result<Block>;
}

#[async_trait]
impl<
        S: Stream<Item = Result<Message, serde_json::Error>> + 'static + Send,
        Block: GetBlockRef + Send,
        B: BlockBuilder<Block> + Sync,
    > BlockStreamT<Block> for BlockStream<S, Block, B>
{
    #[allow(clippy::unwrap_used)]
    async fn recv(&mut self, timestamp: u64) -> eyre::Result<Block> {
        let mut blocks = vec![];

        // If there is init data, handle the initial message
        // This happens only once, the first time this function is called
        if self.init_data.is_some() {
            self.process_init_message().await?;
        }

        if !self.init_requests.is_empty() {
            blocks = self.init_requests.pop_front().unwrap().await?;
        } else if self.stream.as_mut().peek().now_or_never().is_some() {
            // If there are no init requests, and there is data in the stream, pop it off
            // This is to try to catch any reorgs ASAP
            blocks = self.stream.next().await.ok_or_eyre("stream closed")?;
        }

        loop {
            for partial_block in blocks {
                let block = self.block_builder.build_block(&partial_block?.block())?;
                let block_number = block.block_ref().number;
                assert!(
                    block_number <= self.indexed_block_number,
                    "block number {} > index {}",
                    block_number,
                    self.indexed_block_number
                );
                if block_number == self.indexed_block_number {
                    self.indexed_block_number += 1;
                    self.buffer.push_front(block);
                } else {
                    let block_index = (self.indexed_block_number - 1 - block_number) as usize;
                    match self.buffer.get_mut(block_index) {
                        Some(existing_block) => {
                            // if the block already exists in the buffer and has been reorged,
                            // update it
                            assert_eq!(existing_block.block_ref().number, block_number);
                            *existing_block = block;
                        }
                        None => {
                            return Err(eyre!(
                                "cannot reorg block {} - block already slotted",
                                block.block_ref()
                            ));
                        }
                    }
                }
            }

            if self.buffer.front().map_or(false, |x| x.block_ref().timestamp >= timestamp) {
                return Ok(self.buffer.pop_back().unwrap());
            }

            // If there are no valid blocks in the buffer, await the next block from the stream
            blocks = self.stream.next().await.ok_or_eyre("stream closed")?
        }
    }
}

impl Message {
    fn init(self) -> IndexedBlockData {
        match self {
            Self::Init(x) => IndexedBlockData::new(x),
            x => panic!("expected init message type, found {x:?}"),
        }
    }
    fn block(self) -> PartialBlock {
        match self {
            Self::Block(x) => x,
            x => panic!("expected block message type, found {x:?}"),
        }
    }
}

/// Wrapper around a byte slice that represents a list of (timestamp, block hash) pairs.
/// The raw byte layout is as follows:
/// [`block_0_timestamp` (4 bytes), `block_0_hash` (32 bytes)]
/// [`block_1_timestamp` (4 bytes), `block_1_hash` (32 bytes)]
/// [`block_2_timestamp` (4 bytes), `block_2_hash` (32 bytes)]
/// ...
/// Because the first item in the list is just used to get the parent hash,
/// the count is `(data.len() / ITEM_SIZE) - 1`.
#[derive(Debug, Clone, Default)]
struct IndexedBlockData {
    data: Bytes,
    count: u64,
}

impl IndexedBlockData {
    /// Create a new `IndexedBlockData` from a byte slice and validate that it is a valid list of
    /// (timestamp, block hash) pairs
    fn new(data: Bytes) -> Self {
        let length = data.len() as u64;
        assert!(length.is_multiple_of(ITEM_SIZE));
        assert!(length > 0);
        let count = length / ITEM_SIZE - 1;
        Self { data, count }
    }

    /// Get the number of meaningful items in the `IndexedBlockData`
    const fn count(&self) -> u64 {
        self.count
    }

    /// Get the (timestamp, block hash) pair at the given index
    fn get_item(&self, index: u64) -> Result<(u64, B256), eyre::Error> {
        if index > self.count {
            return Err(eyre!("index out of bound"));
        }
        let offset = (index * ITEM_SIZE) as usize;
        let timestamp = u32::from_be_bytes(self.data[offset..offset + 4].try_into()?) as u64;
        let hash = B256::from_slice(self.data[offset + 4..offset + ITEM_SIZE as usize].into());
        Ok((timestamp, hash))
    }

    /// Split the `IndexedBlockData` at the given index into two `IndexedBlockData`
    fn split_at(&self, index: u64) -> Result<(Self, Self), eyre::Error> {
        if index >= self.count {
            return Ok((self.clone(), Self::default()));
        }
        let data = self.data.slice(0..((index + 1) * ITEM_SIZE) as usize);
        let remaining = self.data.slice((index * ITEM_SIZE) as usize..);
        Ok((Self::new(data), Self::new(remaining)))
    }
}

#[allow(missing_docs)]
#[async_trait]
pub trait Provider: Sync {
    async fn request<Params: ToRpcParams + Send, T: DeserializeOwned + Clone>(
        &self,
        method: &'static str,
        params: Params,
    ) -> Result<T, ClientError>;

    async fn subscribe<
        Params: ToRpcParams + Send,
        Notif: DeserializeOwned + Send + Unpin + 'static,
    >(
        &self,
        method: &'static str,
        params: Params,
        unsubscribe_method: &'static str,
    ) -> Result<impl Stream<Item = Result<Notif, serde_json::Error>> + Send + 'static, ClientError>;

    #[instrument(skip(self), err, fields(otel.kind = ?SpanKind::Client))]
    async fn get_urls(&self) -> Result<Vec<String>, ClientError> {
        self.request("urls", ((),)).await
    }

    #[instrument(skip(self), err, fields(otel.kind = ?SpanKind::Client))]
    async fn ready(&self) -> Result<bool, ClientError> {
        #[derive(serde::Deserialize, Clone)]
        struct ReadyResponse {
            ready: bool,
        }

        let response: ReadyResponse = self.request("ready", ((),)).await?;
        Ok(response.ready)
    }

    #[instrument(skip(self), err, fields(otel.kind = ?SpanKind::Client))]
    async fn get_block_number(&self) -> Result<u64, ClientError> {
        self.request("eth_blockNumber", ((),)).await
    }

    #[instrument(skip(self), err, fields(otel.kind = ?SpanKind::Client))]
    async fn get_block(&self, number: u64) -> Result<Option<BlockRef>, ClientError> {
        self.request("block", (number,)).await
    }

    // returns partial blocks instead of blocks
    #[instrument(skip(self, block_builder, client), err, fields(otel.kind = ?SpanKind::Client))]
    async fn get_blocks<Block: GetBlockRef + Send + 'static>(
        &self,
        start_block: u64,
        addresses: Vec<Address>,
        block_builder: Arc<impl BlockBuilder<Block> + Sync + 'static>,
        client: EthClient,
    ) -> Result<impl BlockStreamT<Block>, ClientError> {
        Ok(BlockStream::new(
            self.subscribe::<_, Message>(
                "subscribe_blocks",
                (start_block, addresses.clone()),
                "unsubscribe_blocks",
            )
            .await?,
            block_builder,
            start_block,
            (client, addresses, 0),
        ))
    }
}

/// Configuration for the ingestor provider
#[derive(Debug, Clone)]
pub struct IngestorProviderConfig {
    /// The timeout for the websocket connection (default: 10s)
    pub timeout: Duration,
    /// The maximum buffer capacity per subscription (default: 1024)
    pub max_buffer_capacity_per_subscription: usize,
    /// The maximum response size (default: `u32::MAX` or ~4GB)
    pub max_response_size: u32,
    /// The maximum number of blocks to fetch per request, 0 means no batching (default: 0)
    pub max_blocks_per_request: u64,
}

impl Default for IngestorProviderConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            max_buffer_capacity_per_subscription: 1024,
            max_response_size: u32::MAX,
            max_blocks_per_request: 0,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct IngestorProvider(Arc<WsClient>, u64);

#[allow(missing_docs)]
impl IngestorProvider {
    pub async fn new(url: &str, config: IngestorProviderConfig) -> Self {
        match tokio::time::timeout(
            config.timeout,
            WsClientBuilder::new()
                .max_response_size(config.max_response_size)
                .max_frame_size(config.max_response_size)
                .max_buffer_capacity_per_subscription(config.max_buffer_capacity_per_subscription)
                .request_timeout(config.timeout)
                .enable_ws_ping(PingConfig::default())
                .build(url),
        )
        .await
        {
            Err(_) => {
                panic!(
                    "timed out connecting to websocket: timeout={:?}, url={}",
                    config.timeout, url
                );
            }
            Ok(Err(err)) => panic!("failed to connect to websocket: {err}, url={url}"),
            Ok(Ok(client)) => Self(Arc::new(client), config.max_blocks_per_request),
        }
    }
}

#[async_trait]
impl Provider for IngestorProvider {
    async fn request<Params: ToRpcParams + Send, T: DeserializeOwned>(
        &self,
        method: &'static str,
        params: Params,
    ) -> Result<T, ClientError> {
        self.0.request(method, params).await
    }

    async fn subscribe<Params: ToRpcParams + Send, Notif: DeserializeOwned + Send + 'static>(
        &self,
        method: &'static str,
        params: Params,
        unsubscribe_method: &'static str,
    ) -> Result<impl Stream<Item = Result<Notif, serde_json::Error>> + Send + 'static, ClientError>
    {
        self.0.subscribe::<Notif, _>(method, params, unsubscribe_method).await
    }

    async fn get_blocks<Block: GetBlockRef + Send + 'static>(
        &self,
        start_block: u64,
        addresses: Vec<Address>,
        block_builder: Arc<impl BlockBuilder<Block> + Sync + 'static>,
        client: EthClient,
    ) -> Result<impl BlockStreamT<Block>, ClientError> {
        Ok(BlockStream::new(
            self.subscribe::<_, Message>(
                "subscribe_blocks",
                (start_block, addresses.clone()),
                "unsubscribe_blocks",
            )
            .await?,
            block_builder,
            start_block,
            (client, addresses, self.1),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonrpsee::{
        core::{async_trait, traits::ToRpcParams, ClientError},
        types::{Id, Request, SubscriptionResponse},
        MethodsError, RpcModule,
    };
    use serde::de::DeserializeOwned;
    use serde_json::value::RawValue;
    use std::{marker::PhantomData, pin::Pin, task::Poll};
    use tokio::sync::mpsc;

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    fn convert_error(e: MethodsError) -> ClientError {
        match e {
            MethodsError::Parse(e) => ClientError::ParseError(e),
            MethodsError::JsonRpc(e) => ClientError::Call(e),
            MethodsError::InvalidSubscriptionId(_) => ClientError::InvalidSubscriptionId,
        }
    }

    struct SubscriptionStream<Notif>(mpsc::Receiver<Box<RawValue>>, PhantomData<Notif>);

    impl<Notif: DeserializeOwned + Unpin> Stream for SubscriptionStream<Notif> {
        type Item = Result<Notif, serde_json::Error>;

        fn poll_next(
            mut self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            self.0.poll_recv(cx).map(|x| {
                x.map(|x| {
                    serde_json::from_str::<SubscriptionResponse<'_, Notif>>(x.get())
                        .map(|x| x.params.result)
                })
            })
        }
    }

    #[async_trait]
    impl<X: Send + Sync> Provider for RpcModule<X> {
        async fn request<Params: ToRpcParams + Send, T: DeserializeOwned + Clone>(
            &self,
            method: &'static str,
            params: Params,
        ) -> Result<T, ClientError> {
            self.call(method, params).await.map_err(convert_error)
        }

        async fn subscribe<
            Params: ToRpcParams + Send,
            Notif: DeserializeOwned + Unpin + Send + 'static,
        >(
            &self,
            method: &'static str,
            params: Params,
            _unsubscribe_method: &'static str,
        ) -> Result<
            impl Stream<Item = Result<Notif, serde_json::Error>> + Send + 'static,
            ClientError,
        > {
            let params = params.to_rpc_params()?;
            let req = serde_json::to_string(&Request::borrowed(
                method,
                params.as_ref().map(|p| p.as_ref()),
                Id::Number(0),
            ))?;
            Ok(SubscriptionStream(
                self.raw_json_request(&req, u32::MAX as usize).await?.1,
                PhantomData::<Notif>,
            ))
        }
    }
}
