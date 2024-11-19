use http::header::CONTENT_TYPE;
use http::{HeaderValue, Response};
use http_body_util::BodyExt;
use jsonrpsee::server::{HttpRequest, HttpResponse};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;

/// Applies the inversion of JSON string escaping to responses via the supplied inner service.
///
/// See [`UnescapeJson`] for an example.
#[derive(Debug, Clone)]
pub struct UnescapeJsonLayer<F> {
    filter: F,
}

impl<F: Fn(&'_ HttpRequest) -> bool> UnescapeJsonLayer<F> {
    /// Creates the layer for JSON unescape middleware.
    ///
    /// See [`UnescapeJson::new`] for an explanation of the middleware creation.
    pub const fn new(filter: F) -> Self {
        Self { filter }
    }
}

impl<S, F: Clone> Layer<S> for UnescapeJsonLayer<F> {
    type Service = UnescapeJson<S, F>;

    fn layer(&self, service: S) -> Self::Service {
        UnescapeJson::new(service, self.filter.clone())
    }
}

/// Applies the inversion of JSON string escaping.
///
/// # Example
/// Before:
/// ```text
/// "foo\nbar"
/// ```
/// After:
/// ```text
/// foo
/// bar
/// ```
#[derive(Debug, Clone)]
pub struct UnescapeJson<T, F> {
    inner: T,
    filter: F,
}

impl<T, F> UnescapeJson<T, F> {
    /// Creates a new [`UnescapeJson`] that only affects requests selected by the `filter` closure.
    ///
    /// Passes through any other request unchanged.
    pub const fn new(inner: T, filter: F) -> Self {
        Self { inner, filter }
    }
}

impl<S, F> Service<HttpRequest> for UnescapeJson<S, F>
where
    S: Service<
        HttpRequest,
        Response = HttpResponse,
        Error = Box<(dyn Error + Send + Sync + 'static)>,
        Future: Send + 'static,
    >,
    S::Error: Into<Box<dyn Error + Send + Sync + 'static>>,
    F: Fn(&'_ HttpRequest) -> bool,
{
    type Response = HttpResponse;
    type Error = Box<dyn Error + Send + Sync + 'static>;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
        }
    }

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        let modify = (self.filter)(&request);
        let response = self.inner.call(request);

        let res_fut = async move {
            let rp = response.await?;

            if !modify {
                return Ok(rp);
            }

            let mut body = http_body_util::BodyStream::new(rp.into_body());
            let mut bytes = Vec::new();

            while let Some(frame) = body.frame().await {
                let data = frame?.into_data().map_err(|e| format!("{e:?}"))?;
                bytes.extend(data);
            }
            let string = String::from_utf8(bytes).unwrap();
            let string: String = serde_json::from_str(string.as_str()).unwrap();

            let mut response = Response::new(string.into());

            response.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("text/plain; version=0.0.4; charset=utf-8"),
            );

            Ok(response)
        };

        Box::pin(res_fut)
    }
}
