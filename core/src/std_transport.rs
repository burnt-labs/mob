use crate::http_transport::{HttpTransport, TransportError};

/// An HTTP transport using `ureq` for Rust-only (non-mobile) usage.
///
/// This uses the platform's native TLS via `native-tls`, avoiding the
/// rustls CA bundle issues on iOS/Android. For mobile platforms, use the
/// native transport (URLSession / OkHttp) instead.
pub struct UreqTransport;

impl UreqTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UreqTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpTransport for UreqTransport {
    fn post(&self, url: String, body: Vec<u8>) -> Result<Vec<u8>, TransportError> {
        ureq::post(&url)
            .header("Content-Type", "application/json")
            .send(body)
            .and_then(|mut response| response.body_mut().read_to_vec())
            .map_err(|e| TransportError::NetworkError(e.to_string()))
    }

    fn get(&self, url: String) -> Result<Vec<u8>, TransportError> {
        ureq::get(&url)
            .call()
            .and_then(|mut response| response.body_mut().read_to_vec())
            .map_err(|e| TransportError::NetworkError(e.to_string()))
    }
}
