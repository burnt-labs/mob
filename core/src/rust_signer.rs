//! Pure Rust cryptographic signer implementation
//!
//! This module is only available when the `rust-signer` feature is enabled.
//! It provides a complete implementation using pure Rust cryptography libraries.

use crate::crypto_signer::{CryptoSigner, SignerError};
use crate::error::{MobError, Result};
use bip32::{DerivationPath, Mnemonic, XPrv};
use cosmrs::{
    crypto::secp256k1::SigningKey,
    tx::{self, SignDoc},
    AccountId,
};
use k256::ecdsa::{signature::hazmat::PrehashSigner, Signature, SigningKey as K256SigningKey};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

/// Pure Rust implementation of the CryptoSigner trait
///
/// This signer uses Rust's k256 and bip32 libraries for all cryptographic operations.
/// It implements BIP39 mnemonic support and BIP32 hierarchical deterministic key derivation.
#[derive(Debug)]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Object))]
pub struct RustSigner {
    signing_key: Vec<u8>, // Store as bytes instead of SigningKey (not Clone)
    address: AccountId,
    pub_key: Vec<u8>,
    address_prefix: String,
}

#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl RustSigner {
    /// Create a new signer from a mnemonic phrase
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn from_mnemonic(
        mnemonic: String,
        address_prefix: String,
        derivation_path: Option<String>,
    ) -> Result<Self> {
        Self::from_mnemonic_internal(&mnemonic, &address_prefix, derivation_path.as_deref())
    }

    /// Get the signer's address
    pub fn address(&self) -> String {
        self.address.to_string()
    }

    /// Get the signer's public key as hex
    pub fn public_key_hex(&self) -> String {
        hex::encode(&self.pub_key)
    }

    /// Get the address prefix
    pub fn address_prefix(&self) -> String {
        self.address_prefix.clone()
    }

    /// Sign arbitrary bytes
    pub fn sign_bytes(&self, message: Vec<u8>) -> Result<Vec<u8>> {
        self.sign_bytes_internal(&message)
    }
}

impl RustSigner {
    /// Internal method that uses &str
    fn from_mnemonic_internal(
        mnemonic: &str,
        address_prefix: &str,
        derivation_path: Option<&str>,
    ) -> Result<Self> {
        // Convert mnemonic phrase to Mnemonic using new()
        let mnemonic = Mnemonic::new(mnemonic.trim(), bip32::Language::English)
            .map_err(|e| MobError::KeyDerivation(format!("Invalid mnemonic: {:?}", e)))?;

        // Default to Cosmos derivation path: m/44'/118'/0'/0/0
        let path = derivation_path.unwrap_or("m/44'/118'/0'/0/0");
        let derivation_path: DerivationPath = path
            .parse()
            .map_err(|e| MobError::KeyDerivation(format!("Invalid derivation path: {}", e)))?;

        // Derive the private key
        let seed = mnemonic.to_seed("");
        let child_xprv = XPrv::derive_from_path(&seed, &derivation_path)?;
        let private_key = child_xprv.to_bytes();

        Self::from_private_key(&private_key, address_prefix)
    }

    /// Create a new signer from a private key (32 bytes)
    pub fn from_private_key(private_key: &[u8], address_prefix: &str) -> Result<Self> {
        if private_key.len() != 32 {
            return Err(MobError::Signing(
                "Private key must be 32 bytes".to_string(),
            ));
        }

        let signing_key = SigningKey::from_slice(private_key)
            .map_err(|e| MobError::Signing(format!("Invalid private key: {}", e)))?;

        let pub_key = signing_key.public_key().to_bytes();

        // Generate address from public key
        let address = Self::public_key_to_address(&pub_key, address_prefix)?;

        Ok(Self {
            signing_key: private_key.to_vec(),
            address,
            pub_key: pub_key.to_vec(),
            address_prefix: address_prefix.to_string(),
        })
    }

    /// Get the signer's public key (internal use)
    pub fn public_key(&self) -> Vec<u8> {
        self.pub_key.clone()
    }

    /// Sign arbitrary bytes (internal use)
    fn sign_bytes_internal(&self, message: &[u8]) -> Result<Vec<u8>> {
        let k256_key = K256SigningKey::from_slice(&self.signing_key)
            .map_err(|e| MobError::Signing(format!("Failed to create signing key: {}", e)))?;

        // Hash the message with SHA256 first
        let hash = Sha256::digest(message);

        // Sign the hash directly (prehashed signature)
        let signature: Signature = k256_key
            .sign_prehash(&hash)
            .map_err(|e| MobError::Signing(format!("Failed to sign prehash: {}", e)))?;

        // Normalize signature to low-S form (required by Cosmos SDK)
        let normalized_sig = signature.normalize_s().unwrap_or(signature);

        Ok(normalized_sig.to_bytes().to_vec())
    }

    /// Sign a transaction SignDoc
    pub fn sign_direct(&self, sign_doc: &SignDoc, account_number: u64) -> Result<tx::Raw> {
        use prost::Message;

        // Encode SignDoc to protobuf bytes
        let mut sign_doc_bytes = Vec::new();
        let sign_doc_proto = xion_types::types::cosmos_tx_v1beta1::SignDoc {
            body_bytes: sign_doc.body_bytes.clone(),
            auth_info_bytes: sign_doc.auth_info_bytes.clone(),
            chain_id: sign_doc.chain_id.to_string(),
            account_number,
        };
        sign_doc_proto
            .encode(&mut sign_doc_bytes)
            .map_err(|e| MobError::Signing(format!("Failed to encode SignDoc: {}", e)))?;

        // Sign the bytes
        let signature = self.sign_bytes_internal(&sign_doc_bytes)?;

        // Create raw transaction using proto directly
        let tx_raw_proto = xion_types::types::cosmos_tx_v1beta1::TxRaw {
            body_bytes: sign_doc.body_bytes.clone(),
            auth_info_bytes: sign_doc.auth_info_bytes.clone(),
            signatures: vec![signature],
        };

        // Encode and decode back to cosmrs Raw type
        let mut tx_raw_bytes = Vec::new();
        tx_raw_proto
            .encode(&mut tx_raw_bytes)
            .map_err(|e| MobError::Transaction(format!("Failed to encode tx: {}", e)))?;

        let tx_raw = tx::Raw::from_bytes(&tx_raw_bytes)
            .map_err(|e| MobError::Transaction(format!("Failed to create Raw tx: {}", e)))?;

        Ok(tx_raw)
    }

    /// Convert a public key to an address
    pub fn public_key_to_address(pub_key: &[u8], prefix: &str) -> Result<AccountId> {
        // Hash the public key with SHA256
        let sha256_hash = Sha256::digest(pub_key);

        // Hash the result with RIPEMD160
        let ripemd_hash = Ripemd160::digest(sha256_hash);

        // Create address from hash
        let address = AccountId::new(prefix, &ripemd_hash)
            .map_err(|e| MobError::Address(format!("Failed to create address: {}", e)))?;

        Ok(address)
    }

    /// Verify a signature
    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<bool> {
        use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};

        let verifying_key = VerifyingKey::from_sec1_bytes(&self.pub_key)
            .map_err(|e| MobError::Signing(format!("Invalid public key: {}", e)))?;

        let signature = Signature::from_slice(signature)
            .map_err(|e| MobError::Signing(format!("Invalid signature: {}", e)))?;

        Ok(verifying_key.verify(message, &signature).is_ok())
    }
}

// Implement CryptoSigner trait for RustSigner
impl CryptoSigner for RustSigner {
    fn address(&self) -> String {
        self.address.to_string()
    }

    fn public_key(&self) -> Vec<u8> {
        self.pub_key.clone()
    }

    fn address_prefix(&self) -> String {
        self.address_prefix.clone()
    }

    fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        self.sign_bytes_internal(&message).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_signer::CryptoSigner;
    use std::sync::Arc;

    #[test]
    fn test_rust_signer_from_mnemonic() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let signer =
            RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

        assert!(!signer.address().is_empty());
        assert_eq!(signer.address_prefix(), "xion");
        assert_eq!(signer.public_key().len(), 33);
    }

    #[test]
    fn test_sign_and_verify() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let signer =
            RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

        let message = b"test message";
        let signature = signer.sign_bytes(message.to_vec()).unwrap();

        assert!(signer.verify_signature(message, &signature).unwrap());
    }

    #[test]
    fn test_invalid_mnemonic() {
        let result =
            RustSigner::from_mnemonic("invalid mnemonic".to_string(), "xion".to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_derivation_path() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let signer1 = RustSigner::from_mnemonic(
            mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/0".to_string()),
        )
        .unwrap();
        let signer2 = RustSigner::from_mnemonic(
            mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .unwrap();

        assert_ne!(signer1.address(), signer2.address());
    }

    #[test]
    fn test_crypto_signer_trait_implementation() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let signer =
            RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

        // Test as trait object
        let trait_signer: Arc<dyn CryptoSigner> = Arc::new(signer);

        assert!(!trait_signer.address().is_empty());
        assert_eq!(trait_signer.public_key().len(), 33);
        assert_eq!(trait_signer.address_prefix(), "xion");

        let message = b"test message";
        let signature = trait_signer.sign_bytes(message.to_vec()).unwrap();
        assert_eq!(signature.len(), 64);
    }
}
