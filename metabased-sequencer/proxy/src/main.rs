// Shamelessly inspired by: https://github.com/hyperium/hyper/blob/master/examples/http_proxy.rs
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use clap::Parser;
use http::StatusCode;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use reqwest::Client;

use tokio::net::TcpListener;
use tokio::task::JoinSet;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[path = "../benches/support/mod.rs"]
mod support;
use support::TokioIo;

// To try:
// cargo run --bin proxy
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("in proxy.rs");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");

    // Parse command-line arguments
    let config = Config::parse();

    // Parse the proxy_address into SocketAddr
    let proxy_addr: SocketAddr = match config.proxy_address.parse() {
        Ok(addr) => addr,
        Err(e) => {
            error!(
                "Failed to parse proxy_address '{}': {}",
                config.proxy_address, e
            );
            std::process::exit(1);
        }
    };
    // Validate the target_address as a valid URL
    let target_url = match reqwest::Url::parse(&config.target_address) {
        Ok(url) => url.to_string(),
        Err(e) => {
            error!(
                "Failed to parse target_address '{}': {}",
                config.target_address, e
            );
            std::process::exit(1);
        }
    };
    // Initialize the reqwest client
    let client = match Client::builder().build() {
        Ok(cli) => cli,
        Err(e) => {
            error!("Failed to build reqwest client: {}", e);
            std::process::exit(1);
        }
    };

    // Create an instance of Proxy with the target URL
    let proxy = Proxy::new(client, target_url);

    // Clone the proxy for each connection
    let proxy_service = service_fn(move |request| {
        let proxy = proxy.clone();
        async move { proxy.handle(request).await }
    });

    let listen_addr = &proxy_addr;
    let tcp_listener = TcpListener::bind(&listen_addr).await?;
    println!("listening on http://{listen_addr}");

    let mut join_set = JoinSet::new();
    loop {
        let (stream, addr) = match tcp_listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
                continue;
            }
        };
        let proxy_service = proxy_service.clone();

        let serve_connection = async move {
            println!("handling a request from {addr}");
            let proxy_service = proxy_service.clone();

            let io = TokioIo::new(stream);
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, proxy_service)
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });

            println!("handled a request from {addr}");
        };

        join_set.spawn(serve_connection);
    }
}

/// Simple Proxy Server Configuration
#[derive(Parser, Debug)]
#[command(
    name = "Rust Proxy Server",
    version = "1.0",
    author = "Your Name <you@example.com>",
    about = "A Rust-based HTTP proxy server using Hyper and Reqwest."
)]
struct Config {
    /// Address and port for the proxy server to listen on (e.g., 127.0.0.1:8456)
    #[arg(short, long, default_value = "127.0.0.1:8456")]
    proxy_address: String,

    /// Target address to forward incoming requests to (e.g., https://example.com/api)
    #[arg(short, long)]
    target_address: String,
}

/// Struct to hold shared client
#[derive(Clone)]
struct Proxy {
    client: Arc<Client>,
    target_uri: String, // Store the target address as a string
}

impl Proxy {
    fn new(client: Client, target_uri: String) -> Self {
        Proxy {
            client: Arc::new(client),
            target_uri,
        }
    }

    /// Handles incoming HTTP requests and forwards them to the external HTTPS endpoint
    async fn handle(
        &self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        // Log the incoming request
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        // Clone the parts we need
        // let (parts, body) = req.into_parts();
        let req_host = req.headers().get("Host").cloned();

        let body = req.into_body();
        let body_bytes = body.collect().await?.to_bytes();
        let body = String::from_utf8_lossy(body_bytes.as_ref());
        let body: serde_json::Value = serde_json::from_str(body.as_ref()).unwrap();
        let body = serde_json::to_string_pretty(&body).unwrap();

        info!(
            method = %method,
            uri = %uri,
            headers = ?headers,
            body = %body,
            "Received incoming request"
        );

        let _headers = reqwest::header::HeaderMap::from_iter(
            headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .chain(std::iter::once((
                    "X-Forwarded-Host".to_owned().try_into().unwrap(),
                    req_host.unwrap(),
                ))),
        );
        let request = self
            .client
            .post(&self.target_uri)
            .headers(reqwest::header::HeaderMap::new()) // TODO figure out which header is causing 403 Forbidden
            .body(body_bytes);
        let request = request.build().unwrap();

        info!("Forwarding the following request:");
        info!("{request:#?}");

        // Use reqwest to create a new request
        let external_response = self.client.execute(request).await;

        match external_response {
            Ok(resp) => {
                let status = resp.status();
                let resp_headers = resp.headers().clone();
                let resp_body_bytes = resp.bytes().await.unwrap_or_else(|e| {
                    error!("Failed to read response body: {}", e);
                    Bytes::new()
                });

                info!(
                    status = %status,
                    headers = ?resp_headers,
                    "Received response from external server"
                );

                let resp_body = String::from_utf8_lossy(resp_body_bytes.as_ref());
                let resp_body: serde_json::Value =
                    serde_json::from_str(resp_body.as_ref()).unwrap();
                let resp_body = serde_json::to_string_pretty(&resp_body).unwrap();
                info!(resp_body = %resp_body, "Received response body from external server");

                // Build the Hyper response
                let mut response_builder = Response::builder().status(status);

                // Copy headers from external response to Hyper response
                for (key, value) in resp_headers.iter() {
                    response_builder = response_builder.header(key, value);
                }

                let response = response_builder
                    .body(full(resp_body_bytes))
                    .unwrap_or_else(|e| {
                        error!("Failed to build response: {}", e);
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(full("Internal Server Error"))
                            .unwrap()
                    });

                Ok(response)
            }
            Err(e) => {
                error!("Error forwarding request: {}", e);
                let response = Response::builder()
                    .status(StatusCode::BAD_GATEWAY)
                    .body(full("Bad Gateway"))
                    .unwrap();
                Ok(response)
            }
        }
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
