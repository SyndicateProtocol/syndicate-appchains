//! Client

use alloy::primitives::Address;
use eyre::{eyre, OptionExt};
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
use std::{collections::VecDeque, pin::Pin, sync::Arc, time::Duration};
use tracing::error;

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
}

#[async_trait]
#[allow(missing_docs)]
pub trait BlockStreamT<Block> {
    async fn recv_block(&mut self) -> eyre::Result<Block>;
    async fn recv_blocks(&mut self, timestamp: u64) -> eyre::Result<Vec<Block>>;
}

#[allow(missing_docs)]
impl<
        S: Stream<Item = Result<PartialBlock, serde_json::Error>> + 'static,
        Block: GetBlockRef,
        B: BlockBuilder<Block>,
    > BlockStream<S, Block, B>
{
    fn new(stream: S, block_builder: Arc<B>, start_block: u64) -> Self {
        Self {
            stream: Box::pin(stream.ready_chunks(1024).peekable()),
            block_builder,
            buffer: Default::default(),
            index: start_block,
        }
    }
    async fn recv(&mut self, timestamp: Option<u64>) -> eyre::Result<Vec<Block>> {
        let mut blocks = vec![];
        if self.stream.as_mut().peek().now_or_never().is_some() {
            blocks = self.stream.next().await.ok_or_eyre("stream closed")?;
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
        Ok(BlockStream::new(
            self.subscribe::<_, PartialBlock>(
                "subscribe_blocks",
                (start_block, addresses),
                "unsubscribe_blocks",
            )
            .await?,
            block_builder,
            start_block,
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
                WsClientBuilder::new().request_timeout(timeout).build(url),
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
