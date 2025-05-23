use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct MaestroConfig {
    pub port: u16,
    pub valkey_url: String,
    pub chain_rpc_urls: String,
    pub metrics_port: u16,
    pub finalization_duration: Option<Duration>,
    pub finalization_checker_interval: Option<Duration>,
}

impl MaestroConfig {
    #[allow(dead_code)]
    pub(super) fn cli_args(&self) -> Vec<String> {
        let mut args = vec![
            "--port".to_string(),
            self.port.to_string(),
            "--valkey-url".to_string(),
            self.valkey_url.to_string(),
            "--chain-rpc-urls".to_string(),
            self.chain_rpc_urls.to_string(),
            "--metrics-port".to_string(),
            self.metrics_port.to_string(),
        ];

        if let Some(duration) = self.finalization_duration {
            args.extend(vec![
                "--finalization-duration".to_string(),
                format!("{}s", duration.as_secs()),
            ]);
        }

        if let Some(interval) = self.finalization_checker_interval {
            args.extend(vec![
                "--finalization-checker-interval".to_string(),
                format!("{}s", interval.as_secs()),
            ]);
        }
        args
    }
}
