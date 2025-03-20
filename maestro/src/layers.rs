//! The `layers` module contains code relating to the  `Maestro` service server layers

use crate::errors::Error;
use axum::http::Response;
use bytes::Bytes as HyperBytes;
use futures_util::TryFutureExt;
use jsonrpsee::{
    core::{
        http_helpers::{Body as HttpBody, Request as HttpRequest},
        BoxError,
    },
    server::http::response::malformed,
};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::debug;

/// Layer to check for required headers in the request
#[derive(Debug, Clone)]
pub struct HeadersLayer(Option<Arc<Vec<String>>>);

impl HeadersLayer {
    /// Creates new `HeadersLayer` with the given required headers
    pub fn new(required_headers: Vec<String>) -> eyre::Result<Self, Error> {
        Ok(Self(Some(Arc::new(required_headers))))
    }

    /// Convenience function to disable the check, rather than delete the layer entirely
    pub const fn disable() -> Self {
        Self(None)
    }
}

impl<S> Layer<S> for HeadersLayer {
    type Service = HeadersService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HeadersService { inner, headers: self.0.clone() }
    }
}

/// Enables header checking and fails the request if those headers are not present
#[derive(Debug, Clone)]
pub struct HeadersService<S> {
    inner: S,
    headers: Option<Arc<Vec<String>>>,
}

impl<S, B> Service<HttpRequest<B>> for HeadersService<S>
where
    S: Service<HttpRequest<B>, Response = Response<HttpBody>>,
    S::Response: 'static,
    S::Error: Into<BoxError> + 'static,
    S::Future: Send + 'static,
    B: http_body::Body<Data = HyperBytes> + Send + std::fmt::Debug + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future =
        Pin<Box<dyn Future<Output = eyre::Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<eyre::Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: HttpRequest<B>) -> Self::Future {
        if let Some(headers) = &self.headers {
            for header in headers.iter() {
                if !request.headers().contains_key(header) {
                    debug!("Denied request: {:?}", request);
                    return Box::pin(async { Ok(malformed()) });
                }
            }
        }
        Box::pin(self.inner.call(request).map_err(Into::into))
    }
}
