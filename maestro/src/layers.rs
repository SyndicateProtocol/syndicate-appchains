//! The `layers` module contains code relating to the  `Maestro` service server layers

use crate::errors::Error;
use axum::http::Response;
use bytes::Bytes as HyperBytes;
use futures_util::TryFutureExt;
use jsonrpsee::core::{
    http_helpers::{Body as HttpBody, Request as HttpRequest},
    BoxError,
};
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::debug;

/// Layer to check for headers in the request
#[derive(Debug, Clone)]
pub struct HeadersLayer(Option<Arc<Vec<String>>>);

impl HeadersLayer {
    /// Creates new `HeadersLayer` with the given optional headers
    pub fn new(optional_headers: Vec<String>) -> eyre::Result<Self, Error> {
        Ok(Self(Some(Arc::new(optional_headers))))
    }

    /// Convenience function to disable the check, rather than delete the layer entirely
    pub const fn disable() -> Self {
        Self(None)
    }
}

impl<S> Layer<S> for HeadersLayer {
    type Service = HeadersService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HeadersService { inner, optional_headers: self.0.clone() }
    }
}

/// Enables header checking
#[derive(Debug, Clone)]
pub struct HeadersService<S> {
    inner: S,
    optional_headers: Option<Arc<Vec<String>>>,
}

impl<S, B> Service<HttpRequest<B>> for HeadersService<S>
where
    S: Service<HttpRequest<B>, Response = Response<HttpBody>>,
    S::Error: Into<BoxError> + 'static,
    S::Future: Send + 'static,
    B: http_body::Body<Data = HyperBytes> + Send + std::fmt::Debug + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future =
        Pin<Box<dyn Future<Output = eyre::Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<eyre::Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut request: HttpRequest<B>) -> Self::Future {
        if let Some(optional_headers) = &self.optional_headers {
            // Create a HashMap to store found header-values
            let mut header_map: HashMap<String, String> = HashMap::new();

            for optional_header in optional_headers.iter() {
                if let Some(header_value) = request.headers().get(optional_header) {
                    match header_value.to_str() {
                        Ok(valid_str_header_val) => {
                            // If the header value is valid, insert it into the HashMap
                            header_map
                                .insert(optional_header.clone(), valid_str_header_val.to_string());
                        }
                        Err(_) => {
                            debug!(
                                "Header '{}' value contains non-ASCII characters; ignoring header",
                                optional_header
                            );
                        }
                    }
                } else {
                    debug!("Header '{}' not found in request; skipping it", optional_header);
                };
            }

            if !header_map.is_empty() {
                request.extensions_mut().insert(header_map);
            }
        }

        // Pass the request to the inner service
        Box::pin(self.inner.call(request).map_err(Into::into))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http::StatusCode;
    use std::{collections::HashMap, convert::Infallible};
    use tower::service_fn;

    // Mock service that correctly returns `Response<HttpBody>`
    async fn inner_mock_service<B>(_req: HttpRequest<B>) -> Result<Response<HttpBody>, Infallible> {
        Ok(Response::builder().status(StatusCode::OK).body(HttpBody::empty()).unwrap())
    }

    #[tokio::test]
    async fn test_header_found_in_request() {
        // Create a mock inner service to inspect the modified request
        let mock_inner_service = service_fn(inner_mock_service);

        // Create the HeadersService with a required header
        let mut headers_service = HeadersService {
            inner: mock_inner_service,
            optional_headers: Some(Arc::new(vec!["x-synd-chain-id".to_string()])),
        };

        // Create a mock request with the header
        let request =
            HttpRequest::builder().header("x-synd-chain-id", "123").body(Body::empty()).unwrap();

        // Call the service
        let response = headers_service.call(request).await;

        // Ensure the response is not an error
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_header_not_found_in_request() {
        // Create a mock inner service to inspect the unmodified request
        let mock_inner_service = service_fn(|req: HttpRequest<Body>| async move {
            // Verify that the request's extensions do not contain the header map
            let header_map = req.extensions().get::<HashMap<String, String>>();
            assert!(header_map.is_none(), "Request header map should be empty");

            // Return a dummy response
            Ok::<_, BoxError>(Response::new(HttpBody::empty()))
        });

        // Create the HeadersService expecting missing headers
        let mut headers_service = HeadersService {
            inner: mock_inner_service,
            optional_headers: Some(Arc::new(vec!["X-Synd-Missing-Header".to_string()])),
        };

        // Create a mock request without the required header
        let request =
            HttpRequest::builder().header("X-Unrelated-Header", "456").body(Body::empty()).unwrap();

        // Call the service
        let response = headers_service.call(request).await;

        // Ensure the response is not an error
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_header_with_invalid_ascii() {
        // Create a mock inner service
        let mock_inner_service = service_fn(|_req: HttpRequest<Body>| async move {
            // Return a dummy response to satisfy the call.
            Ok::<_, BoxError>(Response::new(HttpBody::empty()))
        });

        // Create the HeadersService expecting a specific header.
        let mut headers_service = HeadersService {
            inner: mock_inner_service,
            optional_headers: Some(Arc::new(vec!["X-Synd-Invalid".to_string()])),
        };

        // Create a request with an invalid ASCII header value
        let request = HttpRequest::builder()
            .header("X-Synd-Invalid", b"\xff\xff\xff".as_ref()) // Invalid ASCII
            .body(Body::empty())
            .unwrap();

        // Call the service
        let response = headers_service.call(request).await.expect("Should succeed");

        // Ensure the service fails gracefully (e.g., returns an error)
        assert_eq!(response.status(), 200, "Service should return 200 OK for non-ASCII headers");
    }
}
