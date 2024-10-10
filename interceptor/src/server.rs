use crate::endpoint;
use jsonrpsee::server::{RpcServiceBuilder, Server, ServerHandle};
use jsonrpsee::RpcModule;
use std::net::SocketAddr;

pub async fn run(port: u16) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let rpc_middleware = RpcServiceBuilder::new();
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(format!("127.0.0.1:{}", port))
        .await?;
    let addr = server.local_addr()?;
    let module = create_eth_module()?;
    let handle = server.start(module);

    Ok((addr, handle))
}

fn create_eth_module() -> anyhow::Result<RpcModule<()>> {
    let mut module = RpcModule::new(());
    module.register_method("eth_sendRawTransaction", endpoint::send_raw_transaction)?;
    Ok(module)
}
