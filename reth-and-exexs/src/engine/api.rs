use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use again::RetryPolicy;
use eyre::Result;
use futures::prelude::*;
use futures_timer::TryFutureExt;
use reqwest::{header, Client};
use reth_tracing::tracing::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::engine::DEFAULT_AUTH_PORT;

use super::{
    Engine, ExecutionPayload, ForkChoiceUpdate, ForkchoiceState, JwtSecret, PayloadAttributes,
    PayloadId, PayloadStatus, ENGINE_FORKCHOICE_UPDATED_V2, ENGINE_GET_PAYLOAD_V2,
    ENGINE_NEW_PAYLOAD_V2,
};

use super::{JSONRPC_VERSION, STATIC_ID};

/// An external op-geth engine api client
#[derive(Debug, Clone)]
pub struct EngineApi {
    /// Base request url
    pub base_url: String,
    /// The url port
    pub port: u16,
    /// HTTP Client
    pub client: Option<Client>,
    /// A [crate::engine::JwtSecret] used to authenticate with the engine api
    secret: JwtSecret,
}

impl EngineApi {
    /// Creates a new [`EngineApi`] with a base url and secret.
    pub fn new(base_url: &str, secret_str: &str) -> Self {
        let secret = JwtSecret::from_hex(secret_str).unwrap();

        // Gracefully parse the port from the base url
        let parts: Vec<&str> = base_url.split(':').collect();
        let port = parts[parts.len() - 1]
            .parse::<u16>()
            .unwrap_or(DEFAULT_AUTH_PORT);
        let base_url = if parts.len() <= 2 {
            parts[0].to_string()
        } else {
            parts.join(":")
        };

        let client = reqwest::Client::builder()
            .default_headers({
                header::HeaderMap::from_iter([(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static("application/json"),
                )])
            })
            .timeout(Duration::from_secs(5))
            .build()
            .expect("reqwest::Client could not be built, TLS backend could not be initialized");

        Self {
            base_url,
            port,
            client: Some(client),
            secret,
        }
    }

    /// Constructs the base engine api url for the given address
    pub fn auth_url_from_addr(addr: &str, port: Option<u16>) -> String {
        let stripped = addr.strip_prefix("http://").unwrap_or(addr);
        let stripped = addr.strip_prefix("https://").unwrap_or(stripped);
        let port = port.unwrap_or(DEFAULT_AUTH_PORT);
        format!("http://{stripped}:{port}")
    }

    /// Returns if the provided secret matches the secret used to authenticate with the engine api.
    pub fn check_secret(&self, secret: &str) -> bool {
        self.secret.equal(secret)
    }

    /// Creates an engine api from environment variables
    pub fn from_env() -> Self {
        // TODO [SEQ-47]: Confirm the environment variables are correct
        let base_url = std::env::var("ENGINE_API_URL").unwrap_or_else(|_| {
            panic!(
                "ENGINE_API_URL environment variable not set. \
                Please set this to the base url of the engine api"
            )
        });
        let secret_key = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            panic!(
                "JWT_SECRET environment variable not set. \
                Please set this to the 256 bit hex-encoded secret key used to authenticate with the engine api. \
                This should be the same as set in the `--auth.secret` flag when executing op-reth."
            )
        });
        let base_url = EngineApi::auth_url_from_addr(&base_url, None);
        Self::new(&base_url, &secret_key)
    }

    /// Construct base body
    pub fn base_body(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "jsonrpc".to_string(),
            Value::String(JSONRPC_VERSION.to_string()),
        );
        map.insert("id".to_string(), Value::Number(STATIC_ID.into()));
        map
    }

    /// Helper to construct a post request through the client
    async fn post<P>(&self, method: &str, params: Vec<Value>) -> Result<P>
    where
        P: DeserializeOwned,
    {
        // Construct the request params
        let mut body = self.base_body();
        body.insert("method".to_string(), Value::String(method.to_string()));
        body.insert("params".to_string(), Value::Array(params));

        debug!("Sending request to url: {:?}", self.base_url);
        debug!("Sending request: {:?}", serde_json::to_string(&body));

        // Send the client request
        let client = self
            .client
            .as_ref()
            .ok_or(eyre::eyre!("Driver missing http client"))?;

        // Clone the secret so we can use it in the retry policy.
        let secret_clone = self.secret.clone();

        let policy = RetryPolicy::fixed(Duration::ZERO).with_max_retries(5);

        // Send the request
        let res = policy
            .retry(|| async {
                // Construct the JWT Authorization Token
                let claims = secret_clone.generate_claims(Some(SystemTime::now()));
                let jwt = secret_clone
                    .encode(&claims)
                    .map_err(|_| eyre::eyre!("EngineApi failed to encode jwt with claims!"))?;

                // Send the request
                client
                    .post(&self.base_url)
                    .header(header::AUTHORIZATION, format!("Bearer {}", jwt))
                    .json(&body)
                    .send()
                    .map_err(|e| eyre::eyre!(e))
                    .timeout(Duration::from_secs(2))
                    .await?
                    .json::<EngineApiResponse<P>>()
                    .map_err(|e| eyre::eyre!(e))
                    .timeout(Duration::from_secs(2))
                    .map_err(|e| eyre::eyre!(e))
                    .await
            })
            .await?;

        if let Some(res) = res.result {
            return Ok(res);
        }

        if let Some(err) = res.error {
            eyre::bail!("Engine API POST error: {}", err.message);
        }

        // This scenario shouldn't occur as the response should always have either data or an error
        eyre::bail!("Failed to parse Engine API response")
    }

    /// Calls the engine to verify it's available to receive requests
    pub async fn is_available(&self) -> bool {
        self.post::<Value>("eth_chainId", vec![]).await.is_ok()
    }
}

/// Generic Engine API response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineApiResponse<P> {
    /// JSON RPC version
    jsonrpc: String,
    /// Request ID
    id: u64,
    /// JSON RPC payload
    result: Option<P>,
    /// JSON RPC error payload
    error: Option<EngineApiErrorPayload>,
}

/// Engine API error payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineApiErrorPayload {
    /// The error code
    pub code: i64,
    /// The error message
    pub message: String,
    /// Optional additional error data
    pub data: Option<Value>,
}

#[async_trait::async_trait]
impl Engine for EngineApi {
    /// Sends an `engine_forkchoiceUpdatedV2` (V3 post Ecotone) message to the engine.
    async fn forkchoice_updated(
        &self,
        forkchoice_state: ForkchoiceState,
        payload_attributes: Option<PayloadAttributes>,
    ) -> Result<ForkChoiceUpdate> {
        let payload_attributes_param = match payload_attributes {
            Some(payload_attributes) => serde_json::to_value(payload_attributes)?,
            None => Value::Null,
        };
        let forkchoice_state_param = serde_json::to_value(forkchoice_state)?;
        let params = vec![forkchoice_state_param, payload_attributes_param];
        let res = self.post(ENGINE_FORKCHOICE_UPDATED_V2, params).await?;
        Ok(res)
    }

    /// Sends an `engine_newPayloadV2` (V3 post Ecotone) message to the engine.
    async fn new_payload(&self, execution_payload: ExecutionPayload) -> Result<PayloadStatus> {
        let params = vec![serde_json::to_value(execution_payload)?];
        let res = self.post(ENGINE_NEW_PAYLOAD_V2, params).await?;
        Ok(res)
    }

    /// Sends an `engine_getPayloadV2` (V3 post Ecotone) message to the engine.
    async fn get_payload(&self, payload_id: PayloadId) -> Result<ExecutionPayload> {
        let encoded = format!("{:x}", payload_id);
        let padded = format!("0x{:0>16}", encoded);
        let params = vec![Value::String(padded)];
        let res = self
            .post::<GetPayloadResponse>(ENGINE_GET_PAYLOAD_V2, params)
            .await?;
        Ok(res.execution_payload)
    }
}

/// Wrapper around an [ExecutionPayload]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct GetPayloadResponse {
    /// The execution payload returned by the engine via `engine_getPayloadV2` (`engine_getPayloadV3` post Ecotone)
    execution_payload: ExecutionPayload,
}

#[cfg(test)]
mod tests {
    use super::*;

    const AUTH_ADDR: &str = "0.0.0.0";
    const SECRET: &str = "1a81b8d6100c07b9a5ab1c9c0a469661f262067ba002649b22c9621585bf502a";

    #[tokio::test]
    async fn test_engine_get_payload() {
        // Construct the engine api client
        let base_url = EngineApi::auth_url_from_addr(AUTH_ADDR, Some(8551));
        assert_eq!(base_url, "http://0.0.0.0:8551");
        let engine_api = EngineApi::new(&base_url, SECRET);
        assert_eq!(engine_api.base_url, "http://0.0.0.0:8551");
        assert_eq!(engine_api.port, 8551);
    }
}
