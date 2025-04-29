#[derive(Debug)]
pub(super) struct MaestroConfig {
    pub port: u16,
    pub redis_url: String,
    pub chain_rpc_urls: Vec<String>,
}

impl MaestroConfig {
    pub(super) fn cli_args(&self) -> Vec<String> {
        vec![
            "--port".to_string(),
            self.port.to_string(),
            "--redis-url".to_string(),
            self.redis_url.to_string(),
            "--chain-rpc-urls".to_string(),
            self.chain_rpc_urls.join(","),
        ]
    }
}
