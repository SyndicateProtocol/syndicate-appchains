//! Client

use crate::{db::ITEM_SIZE, eth_client::EthClient};
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
    ws_client::{WsClient, WsClientBuilder},
};
use serde::de::DeserializeOwned;
use shared::types::{BlockBuilder, BlockRef, GetBlockRef, PartialBlock};
use std::{
    collections::{HashSet, VecDeque},
    pin::Pin,
    sync::Arc,
    time::Duration,
};
use tracing::{error, info};

/// handles reorgs & builds blocks
#[derive(Debug)]
pub struct BlockStream<
    S: Stream<Item = Result<PartialBlock, serde_json::Error>>,
    Block: GetBlockRef,
    B: BlockBuilder<Block>,
> {
    stream: Pin<Box<Peekable<ReadyChunks<S>>>>,
    buffer: VecDeque<Block>,
    block_builder: Arc<B>,
    index: u64,
    init_data: Option<(String, Vec<Address>)>,
}

#[async_trait]
#[allow(missing_docs)]
pub trait BlockStreamT<Block> {
    async fn recv_block(&mut self) -> eyre::Result<Block>;
    async fn recv_blocks(&mut self, timestamp: u64) -> eyre::Result<Vec<Block>>;
}

async fn build_partial_blocks(
    start_block: u64,
    data: &Bytes,
    ws_url: &str,
    addrs: Vec<Address>,
) -> eyre::Result<Vec<PartialBlock>> {
    assert!(data.len() as u64 % ITEM_SIZE == 0 && data.len() > 0);
    let count = data.len() as u64 / ITEM_SIZE - 1;
    let mut blocks = Vec::default();
    if count == 0 {
        return Ok(blocks);
    }
    let mut parent_hash = B256::from_slice(data[4..ITEM_SIZE as usize].into());
    for i in start_block..start_block + count {
        let offset = ((i + 1 - start_block) * ITEM_SIZE) as usize;
        let hash = B256::from_slice(data[offset + 4..offset + ITEM_SIZE as usize].into());
        blocks.push(PartialBlock {
            block_ref: BlockRef {
                number: i,
                timestamp: u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()) as u64,
                hash,
            },
            parent_hash,
            logs: Default::default(),
        });
        parent_hash = hash;
    }
    let client =
        EthClient::new(ws_url, Duration::from_secs(10), Duration::from_secs(300), 1024).await;

    let end_block = start_block + count - 1;

    let mut safe_block = client.get_block_header(BlockNumberOrTag::Safe).await.number;
    if safe_block < start_block {
        safe_block = start_block - 1;
    }

    info!("fetching partial logs from {} to {}", start_block, end_block);
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
        // fetch all logs for unsafe blocks -> makes it more likely that a log is included which
        // contains block hash info with it.
        info!("fetching full logs from {} to {}", safe_block + 1, end_block);
        let mut unsafe_logs =
            client.get_logs(&Filter::new().from_block(safe_block + 1).to_block(end_block)).await?;

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
        for i in safe_block + 1..end_block + 1 {
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

    for log in logs {
        assert_eq!(log.removed, false);
        assert_eq!(
            log.block_hash,
            Some(blocks[(log.block_number.unwrap() - start_block) as usize].block_ref.hash)
        );
        blocks[(log.block_number.unwrap() - start_block) as usize].logs.push(log.into_inner());
    }

    Ok(blocks)
}

#[allow(missing_docs)]
impl<
        S: Stream<Item = Result<PartialBlock, serde_json::Error>> + 'static,
        Block: GetBlockRef,
        B: BlockBuilder<Block>,
    > BlockStream<S, Block, B>
{
    fn new(
        stream: S,
        block_builder: Arc<B>,
        start_block: u64,
        init_data: (String, Vec<Address>),
    ) -> Self {
        Self {
            stream: Box::pin(stream.ready_chunks(1024).peekable()),
            block_builder,
            buffer: Default::default(),
            index: start_block,
            init_data: Some(init_data),
        }
    }
    async fn recv(&mut self, timestamp: Option<u64>) -> eyre::Result<Vec<Block>> {
        let mut blocks = vec![];
        let init_data = self.init_data.take();
        if init_data.is_some() || self.stream.as_mut().peek().now_or_never().is_some() {
            blocks = self.stream.next().await.ok_or_eyre("stream closed")?;
            if let Some((ws_url, addrs)) = init_data {
                blocks.rotate_left(1);
                let init = blocks.pop().unwrap()?;
                let mut partials: Vec<_> =
                    build_partial_blocks(self.index, &init.logs[0].data.data, &ws_url, addrs)
                        .await?
                        .into_iter()
                        .map(Ok)
                        .collect();
                partials.append(&mut blocks);
                blocks = partials;
            }
        }
        loop {
            for partial_block in blocks {
                let block = self.block_builder.build_block(&partial_block?)?;
                let block_number = block.block_ref().number;
                assert!(
                    block_number <= self.index,
                    "block number {} > index {}",
                    block_number,
                    self.index
                );
                if block_number == self.index {
                    self.index += 1;
                    self.buffer.push_front(block);
                } else if let Some(old_block) =
                    self.buffer.get_mut((self.index - 1 - block_number) as usize)
                {
                    assert_eq!(old_block.block_ref().number, block_number);
                    *old_block = block;
                } else {
                    return Err(eyre!(
                        "cannot reorg block {}, block already slotted",
                        block.block_ref()
                    ));
                }
            }
            match timestamp {
                Some(timestamp) => {
                    if let Some(block) = self.buffer.front() {
                        if block.block_ref().timestamp > timestamp {
                            let mut blocks = Vec::default();
                            loop {
                                blocks.push(self.buffer.pop_back().unwrap());
                                if blocks.last().unwrap().block_ref().timestamp > timestamp {
                                    return Ok(blocks)
                                }
                            }
                        }
                    }
                }
                None => {
                    if let Some(block) = self.buffer.pop_back() {
                        return Ok(vec![block]);
                    }
                }
            }
            blocks = self.stream.next().await.ok_or_eyre("stream closed")?
        }
    }
}

#[async_trait]
impl<
        S: Stream<Item = Result<PartialBlock, serde_json::Error>> + 'static + Send,
        Block: GetBlockRef + Send,
        B: BlockBuilder<Block> + Sync,
    > BlockStreamT<Block> for BlockStream<S, Block, B>
{
    async fn recv_block(&mut self) -> eyre::Result<Block> {
        self.recv(None).await.map(|mut x| x.pop().unwrap())
    }

    async fn recv_blocks(&mut self, timestamp: u64) -> eyre::Result<Vec<Block>> {
        self.recv(Some(timestamp)).await
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
    ) -> Result<impl Stream<Item = Result<Notif, serde_json::Error>> + 'static, ClientError>;

    async fn get_url(&self) -> Result<String, ClientError> {
        self.request("url", ((),)).await
    }

    async fn get_block_number(&self) -> Result<u64, ClientError> {
        self.request("eth_blockNumber", ((),)).await
    }

    async fn get_block(&self, number: u64) -> Result<Option<BlockRef>, ClientError> {
        self.request("block", (number,)).await
    }

    // returns partial blocks instead of blocks
    async fn get_blocks<Block: GetBlockRef + 'static>(
        &self,
        start_block: u64,
        addresses: Vec<Address>,
        block_builder: Arc<impl BlockBuilder<Block> + Sync + Send + 'static>,
    ) -> Result<
        BlockStream<
            impl Stream<Item = Result<PartialBlock, serde_json::Error>> + 'static,
            Block,
            impl BlockBuilder<Block> + 'static,
        >,
        ClientError,
    > {
        let ws_url = self.get_url().await?;
        Ok(BlockStream::new(
            self.subscribe::<_, PartialBlock>(
                "subscribe_blocks",
                (start_block, addresses.clone()),
                "unsubscribe_blocks",
            )
            .await?,
            block_builder,
            start_block,
            (ws_url, addresses),
        ))
    }
}

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct IngestorProvider(Arc<WsClient>);

#[allow(missing_docs)]
impl IngestorProvider {
    pub async fn new(url: &str, timeout: Duration) -> Self {
        loop {
            match tokio::time::timeout(
                timeout,
                WsClientBuilder::new()
                    .max_response_size(u32::MAX)
                    .max_buffer_capacity_per_subscription(1024)
                    .request_timeout(timeout)
                    .build(url),
            )
            .await
            {
                Err(_) => error!("timed out connecting to websocket"),
                Ok(Err(err)) => panic!("failed to connect to websocket: {}, url={}", err, url),
                Ok(Ok(client)) => return Self(Arc::new(client)),
            }
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
    ) -> Result<impl Stream<Item = Result<Notif, serde_json::Error>> + 'static, ClientError> {
        self.0.subscribe::<Notif, _>(method, params, unsubscribe_method).await
    }
}

#[cfg(test)]
mod tests {
    use super::{Provider, Stream};
    use jsonrpsee::{
        core::{async_trait, traits::ToRpcParams, ClientError},
        types::{Id, Request, SubscriptionResponse},
        MethodsError, RpcModule,
    };
    use serde::de::DeserializeOwned;
    use std::{marker::PhantomData, pin::Pin, task::Poll};
    use tokio::sync::mpsc;

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    fn convert_error(e: MethodsError) -> ClientError {
        match e {
            MethodsError::Parse(e) => ClientError::ParseError(e),
            MethodsError::JsonRpc(e) => ClientError::Call(e),
            MethodsError::InvalidSubscriptionId(_) => ClientError::InvalidSubscriptionId,
        }
    }

    struct SubscriptionStream<Notif>(mpsc::Receiver<String>, PhantomData<Notif>);

    impl<Notif: DeserializeOwned + Unpin> Stream for SubscriptionStream<Notif> {
        type Item = Result<Notif, serde_json::Error>;

        fn poll_next(
            mut self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            self.0.poll_recv(cx).map(|x| {
                x.map(|x| {
                    serde_json::from_str::<SubscriptionResponse<'_, Notif>>(&x)
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
            Notif: DeserializeOwned + Unpin + 'static,
        >(
            &self,
            method: &'static str,
            params: Params,
            _unsubscribe_method: &'static str,
        ) -> Result<impl Stream<Item = Result<Notif, serde_json::Error>> + 'static, ClientError>
        {
            let params = params.to_rpc_params()?;
            let req = serde_json::to_string(&Request::new(
                method.into(),
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
