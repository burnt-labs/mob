/// Error types for HTTP transport operations that cross FFI boundary
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi-bindings", uniffi(flat_error))]
pub enum TransportError {
    /// HTTP request failed (non-network error: bad URL, invalid response, etc.)
    #[error("Request failed: {0}")]
    RequestFailed(String),

    /// Network-level error (DNS, timeout, TLS, connection refused, etc.)
    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Trait for HTTP transport operations.
///
/// Implement this in native code (Swift/Kotlin/Python) to use platform-native
/// HTTP clients (URLSession, OkHttp, etc.) with their own TLS stacks.
///
/// The transport is used by `NativeRpcClient` to send JSON-RPC requests
/// to a Tendermint RPC endpoint.
#[cfg_attr(feature = "uniffi-bindings", uniffi::export(with_foreign))]
pub trait HttpTransport: Send + Sync {
    /// POST a JSON body to the given URL and return the response bytes.
    ///
    /// # Parameters
    /// - `url`: The full URL to POST to (e.g., "https://rpc.example.com:443")
    /// - `body`: The JSON-RPC request body as raw bytes
    ///
    /// # Returns
    /// The response body as raw bytes on success.
    fn post(&self, url: String, body: Vec<u8>) -> Result<Vec<u8>, TransportError>;

    /// GET a URL and return the response bytes.
    ///
    /// Used for LCD/REST API queries (e.g., account info).
    fn get(&self, url: String) -> Result<Vec<u8>, TransportError>;
}
