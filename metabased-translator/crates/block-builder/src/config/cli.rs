use crate::config::Configuration;
use tracing_subscriber::EnvFilter;

// Sets global formatter subscriber, including env variables
pub fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
}

pub fn init_config() -> Configuration {
    Configuration::parse()
}
