#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct MaestroConfig {
    pub port: u16,
    pub redis_url: String,
    pub chain_rpc_urls: String,
    pub metrics_port: u16,
}

impl MaestroConfig {
    #[allow(dead_code)]
    pub(super) fn cli_args(&self) -> Vec<String> {
        vec![
            "--port".to_string(),
            self.port.to_string(),
            "--redis-url".to_string(),
            self.redis_url.to_string(),
            "--chain-rpc-urls".to_string(),
            self.chain_rpc_urls.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
        ]
    }
}
