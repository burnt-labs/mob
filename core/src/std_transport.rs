use std::io::Read;

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
        let response = ureq::post(&url)
            .set("Content-Type", "application/json")
            .send_bytes(&body)
            .map_err(|e| TransportError::NetworkError(e.to_string()))?;

        let mut buf = Vec::new();
        response
            .into_reader()
            .read_to_end(&mut buf)
            .map_err(|e| TransportError::NetworkError(e.to_string()))?;
        Ok(buf)
    }
}
