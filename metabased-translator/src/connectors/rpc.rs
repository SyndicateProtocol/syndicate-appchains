use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use serde_json::{json, Value};
use std::error::Error;

// Define the Ethereum RPC client
pub struct RPCClient {
    client: ReqwestClient,
}

impl RPCClient {
    pub fn new(url: &str) -> Self {
        let client = ClientBuilder::default().http(url.parse().expect("Invalid URL"));
        Self { client }
    }

    pub async fn get_block_by_number(
        &self,
        number: &str,
        with_transactions: bool,
    ) -> Result<Value, Box<dyn Error>> {
        let params = json!([number, with_transactions]);
        let request = self.client.request("eth_getBlockByNumber", params);
        let response: Value = request.await?;
        Ok(response)
    }

    pub async fn get_block_by_hash(
        &self,
        block_hash: &str,
        with_transactions: bool,
    ) -> Result<Value, Box<dyn Error>> {
        let params = json!([block_hash, with_transactions]);
        let request = self.client.request("eth_getBlockByHash", params);
        let response: Value = request.await?;
        Ok(response)
    }
}
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_block_by_number_success() {
        // Start a mock server
        let server = MockServer::start();

        // Create a mock endpoint for `eth_getBlockByNumber`
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByNumber",
                    "params": ["0x1", false],
                    "id": 0
                }));
            then.status(200)
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "result": { "number": "0x1" },
                    "id": 0
                }));
        });

        // Create the RPC client
        let client = RPCClient::new(&server.base_url());

        // Run the async function in a runtime
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(client.get_block_by_number("0x1", false));

        // Assert the response
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["number"], "0x1");

        // Verify the mock was called
        mock.assert();
    }

    #[test]
    fn test_get_block_by_number_error() {
        // Start a mock server
        let server = MockServer::start();

        // Create a mock endpoint that simulates an error
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByNumber",
                    "params": ["0x1", false],
                    "id": 0
                }));
            then.status(500)
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32603, "message": "Internal Server Error" },
                    "id": 0
                }));
        });

        // Create the RPC client
        let client = RPCClient::new(&server.base_url());

        // Run the async function in a runtime
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(client.get_block_by_number("0x1", false));

        // Assert the response
        assert!(result.is_err());

        // Verify the mock was called
        mock.assert();
    }

    #[test]
    fn test_get_block_by_hash_success() {
        // Start a mock server
        let server = MockServer::start();

        // Create a mock endpoint for `eth_getBlockByHash`
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByHash",
                    "params": ["0xabc", true],
                    "id": 0
                }));
            then.status(200)
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "result": { "hash": "0xabc" },
                    "id": 0
                }));
        });

        // Create the RPC client
        let client = RPCClient::new(&server.base_url());

        // Run the async function in a runtime
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(client.get_block_by_hash("0xabc", true));

        // Assert the response
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["hash"], "0xabc");

        // Verify the mock was called
        mock.assert();
    }

    #[test]
    fn test_get_block_by_hash_error() {
        // Start a mock server
        let server = MockServer::start();

        // Create a mock endpoint that simulates an error
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByHash",
                    "params": ["0xabc", true],
                    "id": 0
                }));
            then.status(500)
                .header("Content-Type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32603, "message": "Internal Server Error" },
                    "id": 0
                }));
        });

        // Create the RPC client
        let client = RPCClient::new(&server.base_url());

        // Run the async function in a runtime
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(client.get_block_by_hash("0xabc", true));

        // Assert the response
        assert!(result.is_err());

        // Verify the mock was called
        mock.assert();
    }
}