use std::sync::{Arc, RwLock};

#[cfg(feature = "rpc-client")]
use crate::client::Client;
use crate::error::{MobError, Result};
use crate::http_transport::HttpTransport;
#[cfg(feature = "rust-signer")]
use crate::rust_signer::RustSigner;
use crate::session::SessionMetadata;
#[cfg(feature = "rpc-client")]
use crate::types::ChainConfig;
use crate::types::SignerInfo;

/// Serialization version for the export format
const EXPORT_VERSION: u8 = 1;

struct SessionManagerState {
    private_key: Option<Vec<u8>>,
    #[cfg(feature = "rust-signer")]
    signer: Option<Arc<RustSigner>>,
    metadata: Option<SessionMetadata>,
    #[cfg(feature = "rpc-client")]
    client: Option<Arc<Client>>,
}

/// Manages session key lifecycle: generation, activation, export/restore.
///
/// This object is designed to be the single point of session management across
/// all language bindings. The host is responsible only for persisting the opaque
/// bytes returned by `export()` and passing them back to `restore()`.
///
/// Typical flow:
/// 1. `MobSessionManager::new("xion")` — create a manager
/// 2. `generate_session_key()` — creates a random signer, returns address + pubkey
/// 3. (host opens dashboard auth, gets granter address + expiry)
/// 4. `activate(metadata, config)` — pairs the signer with a client
/// 5. `export()` — host persists the returned bytes
/// 6. Later: `MobSessionManager::restore(bytes, config)` — recreates everything
#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Object))]
pub struct MobSessionManager {
    address_prefix: String,
    state: RwLock<SessionManagerState>,
}

#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl MobSessionManager {
    /// Create a new session manager for the given address prefix.
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn new(address_prefix: String) -> Self {
        Self {
            address_prefix,
            state: RwLock::new(SessionManagerState {
                private_key: None,
                signer: None,
                metadata: None,
                client: None,
            }),
        }
    }

    /// Generate a random session key. Returns the address and public key hex.
    ///
    /// The private key is held in memory and included in `export()` output.
    /// Call this before opening dashboard auth — pass the returned address
    /// as the grantee.
    pub fn generate_session_key(&self) -> Result<SignerInfo> {
        let mut key_bytes = [0u8; 32];
        getrandom::getrandom(&mut key_bytes)
            .map_err(|e| MobError::Signing(format!("Failed to generate random key: {}", e)))?;

        let signer = RustSigner::from_private_key(&key_bytes, &self.address_prefix)?;
        let info = SignerInfo {
            address: signer.address(),
            public_key_hex: signer.public_key_hex(),
        };

        let mut state = self
            .state
            .write()
            .map_err(|_| MobError::Generic("Session manager lock poisoned".to_string()))?;
        state.private_key = Some(key_bytes.to_vec());
        state.signer = Some(Arc::new(signer));
        // Clear any previous session
        state.metadata = None;
        state.client = None;

        Ok(info)
    }

    /// Activate the session after dashboard auth returns the grant details.
    ///
    /// Creates a `Client` with the session signer attached. After this call,
    /// `client()` returns a usable signing client.
    pub fn activate(
        &self,
        metadata: SessionMetadata,
        config: ChainConfig,
        transport: Arc<dyn HttpTransport>,
    ) -> Result<()> {
        metadata.validate()?;

        let mut state = self
            .state
            .write()
            .map_err(|_| MobError::Generic("Session manager lock poisoned".to_string()))?;

        let signer = state
            .signer
            .as_ref()
            .ok_or_else(|| {
                MobError::Generic(
                    "No session key generated — call generate_session_key() first".to_string(),
                )
            })?
            .clone();

        let client = Client::new_with_session(config, signer, metadata.clone(), transport)?;
        state.metadata = Some(metadata);
        state.client = Some(Arc::new(client));

        Ok(())
    }

    /// Serialize the session state (private key + metadata) to an opaque byte blob.
    ///
    /// The host should persist these bytes (AsyncStorage, Keychain, etc.) and pass
    /// them to `restore()` on the next app launch.
    pub fn export_session(&self) -> Result<Vec<u8>> {
        let state = self
            .state
            .read()
            .map_err(|_| MobError::Generic("Session manager lock poisoned".to_string()))?;

        let private_key = state
            .private_key
            .as_ref()
            .ok_or_else(|| MobError::Generic("No session key to export".to_string()))?;

        let metadata = state
            .metadata
            .as_ref()
            .ok_or_else(|| MobError::Generic("No session metadata to export".to_string()))?;

        let metadata_json = serde_json::to_vec(metadata)?;

        // Format: [version: 1 byte] [private_key: 32 bytes] [metadata_json: rest]
        let mut buf = Vec::with_capacity(1 + 32 + metadata_json.len());
        buf.push(EXPORT_VERSION);
        buf.extend_from_slice(private_key);
        buf.extend_from_slice(&metadata_json);

        Ok(buf)
    }

    /// Restore a session from previously exported bytes.
    ///
    /// Recreates the signer and client. Returns an error if the session is expired
    /// or the data is malformed.
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn restore(
        data: Vec<u8>,
        config: ChainConfig,
        transport: Arc<dyn HttpTransport>,
    ) -> Result<Self> {
        if data.len() < 34 {
            return Err(MobError::InvalidInput("Export data too short".to_string()));
        }

        let version = data[0];
        if version != EXPORT_VERSION {
            return Err(MobError::InvalidInput(format!(
                "Unsupported export version: {}",
                version
            )));
        }

        let private_key = &data[1..33];
        let metadata_json = &data[33..];

        let metadata: SessionMetadata = serde_json::from_slice(metadata_json)
            .map_err(|e| MobError::Serialization(format!("Invalid session metadata: {}", e)))?;

        metadata.validate()?;

        let signer = RustSigner::from_private_key(private_key, &config.address_prefix)?;
        let signer_arc = Arc::new(signer);

        let client = Client::new_with_session(
            config.clone(),
            signer_arc.clone(),
            metadata.clone(),
            transport,
        )?;

        Ok(Self {
            address_prefix: config.address_prefix,
            state: RwLock::new(SessionManagerState {
                private_key: Some(private_key.to_vec()),
                signer: Some(signer_arc),
                metadata: Some(metadata),
                client: Some(Arc::new(client)),
            }),
        })
    }

    /// Deactivate the current session, releasing the client and metadata.
    ///
    /// The signer and private key are also cleared. After this call,
    /// `is_active()` returns false and `client()` returns an error.
    pub fn deactivate(&self) {
        if let Ok(mut state) = self.state.write() {
            state.private_key = None;
            state.signer = None;
            state.metadata = None;
            state.client = None;
        }
    }

    /// Whether a session is active (key generated, metadata set, not expired).
    pub fn is_active(&self) -> bool {
        let state = match self.state.read() {
            Ok(s) => s,
            Err(_) => return false,
        };
        match &state.metadata {
            Some(m) => !m.is_expired(),
            None => false,
        }
    }

    /// The granter (main account) address, if a session is active.
    pub fn granter_address(&self) -> Option<String> {
        self.state
            .read()
            .ok()
            .and_then(|s| s.metadata.as_ref().map(|m| m.granter.clone()))
    }

    /// The grantee (session key) address.
    pub fn grantee_address(&self) -> Option<String> {
        self.state
            .read()
            .ok()
            .and_then(|s| s.signer.as_ref().map(|sig| sig.address()))
    }

    /// The session key's public key as hex.
    pub fn public_key_hex(&self) -> Option<String> {
        self.state
            .read()
            .ok()
            .and_then(|s| s.signer.as_ref().map(|sig| sig.public_key_hex()))
    }

    /// Get the session metadata, if active.
    pub fn metadata(&self) -> Option<SessionMetadata> {
        self.state.read().ok().and_then(|s| s.metadata.clone())
    }

    /// Sign arbitrary bytes with the session key.
    ///
    /// Useful for ADR-036 signArb or any off-chain signing that doesn't
    /// require a full transaction broadcast.
    pub fn sign_bytes(&self, message: Vec<u8>) -> Result<Vec<u8>> {
        let state = self
            .state
            .read()
            .map_err(|_| MobError::Generic("Session manager lock poisoned".to_string()))?;

        let signer = state.signer.as_ref().ok_or_else(|| {
            MobError::Generic(
                "No session key generated — call generate_session_key() first".to_string(),
            )
        })?;

        signer.sign_bytes(message)
    }

    /// Get the signing client. Returns an error if no session is active.
    pub fn client(&self) -> Result<Arc<Client>> {
        let state = self
            .state
            .read()
            .map_err(|_| MobError::Generic("Session manager lock poisoned".to_string()))?;

        state.client.as_ref().cloned().ok_or_else(|| {
            MobError::Generic("No active session — call activate() or restore() first".to_string())
        })
    }
}

#[cfg(test)]
#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
mod tests {
    use super::*;
    use crate::http_transport::{HttpTransport, TransportError};

    struct MockTransport;
    impl HttpTransport for MockTransport {
        fn post(
            &self,
            _url: String,
            _body: Vec<u8>,
        ) -> std::result::Result<Vec<u8>, TransportError> {
            Err(TransportError::NetworkError("mock transport".to_string()))
        }
        fn get(&self, _url: String) -> std::result::Result<Vec<u8>, TransportError> {
            Err(TransportError::NetworkError("mock transport".to_string()))
        }
    }

    fn mock_transport() -> Arc<dyn HttpTransport> {
        Arc::new(MockTransport)
    }

    #[test]
    fn test_session_manager_creation() {
        let mgr = MobSessionManager::new("xion".to_string());
        assert!(!mgr.is_active());
        assert!(mgr.granter_address().is_none());
        assert!(mgr.grantee_address().is_none());
        assert!(mgr.client().is_err());
    }

    #[test]
    fn test_generate_session_key() {
        let mgr = MobSessionManager::new("xion".to_string());
        let info = mgr.generate_session_key().expect("Failed to generate key");

        assert!(info.address.starts_with("xion"));
        assert_eq!(info.public_key_hex.len(), 66); // 33 bytes * 2
        assert!(mgr.grantee_address().is_some());
        assert_eq!(mgr.grantee_address().unwrap(), info.address);
        // Still not active — no metadata yet
        assert!(!mgr.is_active());
    }

    #[test]
    fn test_generate_key_uniqueness() {
        let mgr = MobSessionManager::new("xion".to_string());
        let info1 = mgr.generate_session_key().expect("gen 1");
        let info2 = mgr.generate_session_key().expect("gen 2");
        // Second call replaces the first
        assert_ne!(info1.address, info2.address);
        assert_eq!(mgr.grantee_address().unwrap(), info2.address);
    }

    #[test]
    fn test_activate_without_key_fails() {
        let mgr = MobSessionManager::new("xion".to_string());
        let metadata = SessionMetadata::with_duration(
            "xion1granter".to_string(),
            "xion1grantee".to_string(),
            3600,
        );
        let config = ChainConfig::new(
            "xion-testnet-1",
            "https://rpc.xion-testnet-1.burnt.com:443",
            "xion",
        );
        let result = mgr.activate(metadata, config, mock_transport());
        assert!(result.is_err());
    }

    #[test]
    fn test_export_without_metadata_fails() {
        let mgr = MobSessionManager::new("xion".to_string());
        let _info = mgr.generate_session_key().expect("gen");
        let result = mgr.export_session();
        assert!(result.is_err());
    }

    #[test]
    fn test_export_without_key_fails() {
        let mgr = MobSessionManager::new("xion".to_string());
        let result = mgr.export_session();
        assert!(result.is_err());
    }

    #[test]
    fn test_export_format() {
        let mgr = MobSessionManager::new("xion".to_string());
        let info = mgr.generate_session_key().expect("gen");

        // Manually set metadata via activate — but we need a real RPC endpoint
        // for Client creation. Instead, test the export serialization directly.
        {
            let mut state = mgr.state.write().unwrap();
            state.metadata = Some(SessionMetadata::with_duration(
                "xion1granter".to_string(),
                info.address.clone(),
                3600,
            ));
        }

        let exported = mgr.export_session().expect("export");

        // Version byte
        assert_eq!(exported[0], EXPORT_VERSION);
        // Private key is 32 bytes at offset 1..33
        assert!(exported.len() > 33);
        // Remaining bytes are valid JSON
        let metadata_json = &exported[33..];
        let metadata: SessionMetadata = serde_json::from_slice(metadata_json).expect("valid json");
        assert_eq!(metadata.granter, "xion1granter");
        assert_eq!(metadata.grantee, info.address);
    }

    #[test]
    fn test_restore_invalid_data() {
        let config = ChainConfig::new(
            "xion-testnet-1",
            "https://rpc.xion-testnet-1.burnt.com:443",
            "xion",
        );

        // Too short
        assert!(
            MobSessionManager::restore(vec![1, 2, 3], config.clone(), mock_transport()).is_err()
        );

        // Wrong version
        let bad = vec![0xFF; 34];
        assert!(MobSessionManager::restore(bad, config.clone(), mock_transport()).is_err());

        // Correct version but invalid JSON
        let mut bad2 = vec![EXPORT_VERSION];
        bad2.extend_from_slice(&[1u8; 32]); // 32-byte "key"
        bad2.push(b'{'); // broken JSON
        assert!(MobSessionManager::restore(bad2, config, mock_transport()).is_err());
    }

    #[test]
    fn test_deactivate() {
        let mgr = MobSessionManager::new("xion".to_string());
        let _info = mgr.generate_session_key().expect("gen");
        assert!(mgr.grantee_address().is_some());

        mgr.deactivate();
        assert!(mgr.grantee_address().is_none());
        assert!(!mgr.is_active());
    }

    #[test]
    fn test_export_restore_roundtrip() {
        // This test validates the full export → restore cycle without a live RPC.
        // We manually set state to avoid needing a real node.
        let mgr = MobSessionManager::new("xion".to_string());
        let info = mgr.generate_session_key().expect("gen");

        let metadata =
            SessionMetadata::with_duration("xion1granter".to_string(), info.address.clone(), 3600);

        // Set metadata without creating a client (no RPC needed)
        {
            let mut state = mgr.state.write().unwrap();
            state.metadata = Some(metadata.clone());
        }

        let exported = mgr.export_session().expect("export");

        // Verify the exported bytes can be deserialized
        assert_eq!(exported[0], EXPORT_VERSION);
        let key_bytes = &exported[1..33];
        let meta_bytes = &exported[33..];

        // Key should produce the same address
        let restored_signer =
            RustSigner::from_private_key(key_bytes, "xion").expect("restore signer");
        assert_eq!(restored_signer.address(), info.address);

        // Metadata should round-trip
        let restored_meta: SessionMetadata =
            serde_json::from_slice(meta_bytes).expect("restore meta");
        assert_eq!(restored_meta.granter, "xion1granter");
        assert_eq!(restored_meta.grantee, info.address);
        assert!(!restored_meta.is_expired());
    }
}
