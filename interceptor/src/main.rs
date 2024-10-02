// Based on https://github.com/paritytech/jsonrpsee/blob/master/examples/examples/rpc_middleware_modify_request.rs

use jsonrpsee::core::async_trait;
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::{Server, ServerHandle};
use jsonrpsee::types::Request;
use jsonrpsee::RpcModule;
use std::borrow::Cow as StdCow;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct ModifyRequestIf<S>(S);

#[async_trait]
impl<'a, S> RpcServiceT<'a> for ModifyRequestIf<S>
where
    S: Send + Sync + RpcServiceT<'a>,
{
    type Future = S::Future;

    fn call(&self, mut req: Request<'a>) -> Self::Future {
        info!(
            method = %req.method,
            params = %req.params.as_ref().map(|r| r.get()).unwrap_or("None"),
            "Received request"
        );

        // Example how to modify the params in the call.
        // TODO: Change this to intercept eth_sendRawTransaction
        if req.method == "say_hello" {
            // It's a bit awkward to create new params in the request
            // but this shows how to do it.
            let raw_value = serde_json::value::to_raw_value("myparams").unwrap();
            req.params = Some(StdCow::Owned(raw_value));
        }
        // Re-direct all calls that isn't `say_hello` to `say_goodbye`
        // TODO: Change this to redirect all calls to the underlying RPC
        // (optionally, via an env variable if you want to forward)
        // If the node operator does not want to forward
        // non-eth_sendRawTransaction calls, then reject all other calls
        else if req.method != "say_hello" {
            req.method = "say_goodbye".into();
        }

        self.0.call(req)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");

    let port = 8456;
    let (addr, handle) = run_server(port).await?;
    println!("RPC Server started on {}", addr);

    // Keep the server running
    handle.stopped().await;

    Ok(())
}

async fn run_server(port: u16) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let rpc_middleware = RpcServiceBuilder::new().layer_fn(ModifyRequestIf);
    // .layer_fn(LoggingMiddleware);
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build(format!("127.0.0.1:{}", port))
        .await?;
    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _, _| "lo")?;
    module.register_method("say_goodbye", |_, _, _| "goodbye")?;
    let addr = server.local_addr()?;

    let handle = server.start(module);

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    // tokio::spawn(handle.stopped());

    Ok((addr, handle))
}
