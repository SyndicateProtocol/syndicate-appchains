use figment::value::{Dict, Map};
use figment::{Error, Metadata, Profile, Provider};
use std::fmt::Debug;
use tracing::log;

/// A [`Provider`] that wraps `inner` provider and [`log`]s [`metadata`] when getting [`data`],
/// ignoring errors.
///
/// [`metadata`]: Provider::metadata
/// [`data`]: Provider::data
pub struct Logged<T> {
    inner: T,
    profile: Profile,
}

impl<T> Logged<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            profile: Profile::Default,
        }
    }
}

impl<T: Provider + Debug> Provider for Logged<T> {
    fn metadata(&self) -> Metadata {
        self.inner.metadata()
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let meta = self.inner.metadata();
        let source = meta
            .source
            .map(|s| format!(" from {s}"))
            .unwrap_or_default();

        let data = self.inner.data()?;
        let dict = &data[&self.profile];

        // Convert key-value pairs to strings and join them
        let values = dict
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        log::debug!(
            target: "configuration",
            "Got {} {}{}: {}", dict.len(), meta.name, source, values
        );

        Ok(data)
    }
}
