use figment::{
    value::{Dict, Map},
    Error, Metadata, Profile, Provider,
};
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
        Self { inner, profile: Profile::Default }
    }
}

impl<T: Provider + Debug> Provider for Logged<T> {
    fn metadata(&self) -> Metadata {
        self.inner.metadata()
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error> {
        let meta = self.inner.metadata();
        let source = meta.source.map(|s| format!(" from {s}")).unwrap_or_default();

        Ok(self
            .inner
            .data()
            .inspect(|data| log::debug!(target: "configuration", "Got {} {}{source}", data[&self.profile].len(), meta.name))
            .unwrap_or_default())
    }
}
