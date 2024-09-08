// Based on https://github.com/paritytech/jsonrpsee/blob/master/examples/examples/rpc_middleware_modify_request.rs

use jsonrpsee::core::client::ClientT;
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::Server;
use jsonrpsee::types::Request;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, RpcModule};
use std::borrow::Cow as StdCow;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct ModifyRequestIf<S>(S);

impl<'a, S> RpcServiceT<'a> for ModifyRequestIf<S>
where
    S: Send + Sync + RpcServiceT<'a>,
{
    type Future = S::Future;

    fn call(&self, mut req: Request<'a>) -> Self::Future {
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

    let addr = run_server().await?;
    let url = format!("ws://{}", addr);

    let client = WsClientBuilder::default().build(&url).await?;
    let _response: String = client.request("say_hello", rpc_params![]).await?;
    let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
    let _: String = client.request("say_hello", rpc_params![]).await?;

    Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    let rpc_middleware = RpcServiceBuilder::new().layer_fn(ModifyRequestIf);
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build("127.0.0.1:0")
        .await?;
    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _, _| "lo")?;
    module.register_method("say_goodbye", |_, _, _| "goodbye")?;
    let addr = server.local_addr()?;

    let handle = server.start(module);

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}
