use std::sync::Arc;

use async_trait::async_trait;
use tendermint_rpc::client::Client;
use tendermint_rpc::Response;
use tendermint_rpc::SimpleRequest;

use crate::http_transport::HttpTransport;

/// An RPC client that delegates HTTP POSTs to a native transport callback.
///
/// This preserves all type-safe serialization from tendermint-rpc while
/// letting the native platform (URLSession on iOS, OkHttp on Android)
/// handle the actual networking with its own TLS stack.
pub struct NativeRpcClient {
    transport: Arc<dyn HttpTransport>,
    url: String,
}

impl NativeRpcClient {
    /// Create a new `NativeRpcClient`.
    ///
    /// # Parameters
    /// - `url`: The Tendermint RPC endpoint URL
    /// - `transport`: A native HTTP transport implementation
    pub fn new(url: String, transport: Arc<dyn HttpTransport>) -> Self {
        Self { transport, url }
    }
}

#[async_trait]
impl Client for NativeRpcClient {
    async fn perform<R>(&self, request: R) -> Result<R::Output, tendermint_rpc::Error>
    where
        R: SimpleRequest,
    {
        // Serialize the request to the full JSON-RPC envelope
        let body = request.into_json();

        // Delegate the HTTP POST to the native transport
        let response_bytes = self
            .transport
            .post(self.url.clone(), body.into_bytes())
            .map_err(|e| tendermint_rpc::Error::server(e.to_string()))?;

        // Deserialize using tendermint-rpc's own response parsing
        let response_str = String::from_utf8(response_bytes).map_err(|e| {
            tendermint_rpc::Error::server(format!("Invalid UTF-8 in response: {}", e))
        })?;

        let response = R::Response::from_string(&response_str)?;
        Ok(response.into())
    }
}
