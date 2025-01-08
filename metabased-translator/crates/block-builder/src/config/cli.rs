//! TODO not sure what the purpose of this module is
use crate::config::Configuration;
use tracing_subscriber::EnvFilter;

/// Sets global formatter subscriber, including env variables
pub fn init_tracing_subscriber() {
    #[allow(clippy::expect_used)]
    // safe to panic here, as this will execute right in the start of the program
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
}

pub fn init_config() -> Configuration {
    Configuration::parse()
}
