//! Chain ingestor for the integration tests

#[derive(Debug, Default)]
pub struct ChainIngestorConfig {
    pub ws_urls: Vec<String>,
    pub db_file: String,
    pub start_block: u64,
    pub port: u16,
    pub metrics_port: u16,
}

impl ChainIngestorConfig {
    pub fn cli_args(&self) -> Vec<String> {
        vec![
            "--ws-urls".to_string(),
            self.ws_urls.join(","),
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
