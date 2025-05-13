//! Chain ingestor for the integration tests

#[derive(Debug, Default)]
pub(super) struct ChainIngestorConfig {
    pub(crate) rpc_url: String,
    pub(crate) db_file: String,
    pub(crate) start_block: u64,
    pub(crate) port: u16,
    pub(crate) metrics_port: u16,
}

impl ChainIngestorConfig {
    pub(crate) fn cli_args(&self) -> Vec<String> {
        vec![
            "--rpc-url".to_string(),
            self.rpc_url.to_string(),
            "--db-file".to_string(),
            self.db_file.to_string(),
            "--start-block".to_string(),
            self.start_block.to_string(),
            "--port".to_string(),
            self.port.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
        ]
    }
}
