/// Error types for signer operations that cross FFI boundary
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi-bindings", uniffi(flat_error))]
pub enum SignerError {
    /// Failed to sign data
    #[error("Signing failed: {0}")]
    SigningFailed(String),

    /// Invalid key format or data
    #[error("Invalid key: {0}")]
    InvalidKey(String),

    /// Invalid signature format
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
}

/// Trait for cryptographic signing operations
///
/// This trait defines the core signing interface that can be implemented
/// in Rust or in foreign languages (Python, Swift, Kotlin, etc.) to provide
/// custom cryptographic implementations.
///
/// # Requirements
///
/// Implementations MUST:
/// - Return compressed secp256k1 public keys (33 bytes, format: 0x02/0x03 + 32-byte X coordinate)
/// - Sign SHA256 prehashed messages
/// - Return normalized secp256k1 signatures (64 bytes, r||s in big-endian, low-S form)
/// - Be thread-safe (Send + Sync)
///
/// # Example
///
/// ```rust,no_run
/// use mob::{CryptoSigner, SignerError};
/// use std::sync::Arc;
///
/// struct MyCryptoSigner {
///     address: String,
///     pub_key: Vec<u8>,
///     prefix: String,
/// }
///
/// impl CryptoSigner for MyCryptoSigner {
///     fn address(&self) -> String {
///         self.address.clone()
///     }
///
///     fn public_key(&self) -> Vec<u8> {
///         self.pub_key.clone()
///     }
///
///     fn address_prefix(&self) -> String {
///         self.prefix.clone()
///     }
///
///     fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
///         // Custom signing implementation
///         Ok(vec![0u8; 64]) // Return 64-byte signature
///     }
/// }
/// ```
#[cfg_attr(feature = "uniffi-bindings", uniffi::export(with_foreign))]
pub trait CryptoSigner: Send + Sync {
    /// Get the signer's bech32 address
    ///
    /// Returns the address string (e.g., "xion1abc...")
    fn address(&self) -> String;

    /// Get the signer's compressed secp256k1 public key
    ///
    /// Returns 33 bytes in compressed format:
    /// - First byte: 0x02 (even Y) or 0x03 (odd Y)
    /// - Remaining 32 bytes: X coordinate
    fn public_key(&self) -> Vec<u8>;

    /// Get the address prefix
    ///
    /// Returns the bech32 prefix (e.g., "xion")
    fn address_prefix(&self) -> String;

    /// Sign arbitrary bytes with secp256k1 ECDSA
    ///
    /// # Parameters
    /// - `message`: The message bytes to sign
    ///
    /// # Returns
    /// 64-byte signature in format: r (32 bytes) || s (32 bytes)
    /// - r and s are big-endian encoded
    /// - Signature MUST be normalized to low-S form (s <= curve_order / 2)
    ///
    /// # Implementation Notes
    /// 1. Hash the message with SHA256
    /// 2. Sign the hash with secp256k1 ECDSA
    /// 3. Normalize the signature to low-S form
    /// 4. Return r||s as 64 bytes
    fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError>;
}

// Helper conversion from MobError to SignerError
impl From<crate::error::MobError> for SignerError {
    fn from(e: crate::error::MobError) -> Self {
        match e {
            crate::error::MobError::Signing(msg) => SignerError::SigningFailed(msg),
            crate::error::MobError::KeyDerivation(msg) => SignerError::InvalidKey(msg),
            _ => SignerError::SigningFailed(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    struct MockSigner {
        address: String,
        pub_key: Vec<u8>,
        prefix: String,
    }

    impl CryptoSigner for MockSigner {
        fn address(&self) -> String {
            self.address.clone()
        }

        fn public_key(&self) -> Vec<u8> {
            self.pub_key.clone()
        }

        fn address_prefix(&self) -> String {
            self.prefix.clone()
        }

        fn sign_bytes(&self, _message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
            // Return a mock 64-byte signature
            Ok(vec![0x42u8; 64])
        }
    }

    #[test]
    fn test_crypto_signer_trait() {
        let signer = MockSigner {
            address: "xion1test".to_string(),
            pub_key: vec![0x02; 33], // Compressed pubkey
            prefix: "xion".to_string(),
        };

        assert_eq!(signer.address(), "xion1test");
        assert_eq!(signer.public_key().len(), 33);
        assert_eq!(signer.address_prefix(), "xion");

        let signature = signer.sign_bytes(vec![1, 2, 3]).unwrap();
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_signer_as_trait_object() {
        use std::sync::Arc;

        let signer: Arc<dyn CryptoSigner> = Arc::new(MockSigner {
            address: "xion1test".to_string(),
            pub_key: vec![0x02; 33],
            prefix: "xion".to_string(),
        });

        assert_eq!(signer.address(), "xion1test");
        assert_eq!(signer.public_key().len(), 33);
    }
}
