#[cfg(feature = "rust-signer")]
use crate::rust_signer::RustSigner;
use crate::{
    crypto_signer::CryptoSigner,
    error::{MobError, Result},
    session::SessionMetadata,
};
use cosmrs::{
    tendermint::chain::Id as ChainId,
    tx::{self, BodyBuilder, SignDoc},
    Any,
};
use std::sync::Arc;

/// A session signer that wraps messages in MsgExec (Authz) for session key usage
///
/// This works with any implementation of CryptoSigner, allowing language-specific
/// cryptographic implementations while automatically wrapping all messages in authz.
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Object))]
pub struct SessionSigner {
    /// The underlying session key signer (trait object for flexibility)
    session_key: Arc<dyn CryptoSigner>,
    /// Session metadata including granter and expiration
    metadata: SessionMetadata,
}

// Manual Debug implementation since trait objects don't auto-derive Debug
impl std::fmt::Debug for SessionSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionSigner")
            .field("granter", &self.metadata.granter)
            .field("grantee", &self.metadata.grantee)
            .field("expires_at", &self.metadata.expires_at)
            .finish()
    }
}

#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl SessionSigner {
    /// Get the granter address (the main account)
    pub fn granter_address(&self) -> String {
        self.metadata.granter.clone()
    }

    /// Get the grantee address (the session key address)
    pub fn grantee_address(&self) -> String {
        self.session_key.address()
    }

    /// Get session metadata
    pub fn metadata(&self) -> SessionMetadata {
        self.metadata.clone()
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        self.metadata.is_expired()
    }

    /// Get remaining session time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        self.metadata.remaining_seconds()
    }

    /// Get the session key's public key as hex
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.session_key.public_key())
    }
}

// RustSigner-specific FFI constructors (only with rust-signer feature)
#[cfg(feature = "rust-signer")]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl SessionSigner {
    /// Create a new session signer from a RustSigner and metadata
    ///
    /// # Parameters
    /// - `session_key`: RustSigner instance
    /// - `metadata`: Session metadata with expiration and granter info
    ///
    /// Note: This constructor is only available with the `rust-signer` feature for FFI.
    /// Rust code can use `with_signer()` to accept any CryptoSigner implementation.
    #[uniffi::constructor]
    pub fn new(session_key: Arc<RustSigner>, metadata: SessionMetadata) -> Result<Self> {
        // Validate session on creation
        metadata.validate()?;

        Ok(Self {
            session_key,
            metadata,
        })
    }

    /// Create a session signer from a private key with duration
    ///
    /// Note: This constructor is only available with the `rust-signer` feature.
    #[uniffi::constructor]
    pub fn from_private_key(
        private_key: Vec<u8>,
        address_prefix: String,
        granter_address: String,
        duration_seconds: u64,
    ) -> Result<Self> {
        let signer = RustSigner::from_private_key(&private_key, &address_prefix)?;
        let grantee_address = signer.address();

        let metadata =
            SessionMetadata::with_duration(granter_address, grantee_address, duration_seconds);

        Ok(Self {
            session_key: Arc::new(signer),
            metadata,
        })
    }
}

impl SessionSigner {
    /// Create a new session signer with any CryptoSigner implementation
    ///
    /// This is the primary constructor for Rust code when using custom signers.
    /// For FFI usage with RustSigner, use the `new()` constructor instead.
    ///
    /// This method is not exported to FFI since UniFFI doesn't support trait objects.
    pub fn with_signer(
        session_key: Arc<dyn CryptoSigner>,
        metadata: SessionMetadata,
    ) -> Result<Self> {
        // Validate session on creation
        metadata.validate()?;

        Ok(Self {
            session_key,
            metadata,
        })
    }

    /// Get reference to the underlying session key signer
    pub fn session_key(&self) -> &Arc<dyn CryptoSigner> {
        &self.session_key
    }

    /// Wrap messages in MsgExec for authz execution
    pub fn wrap_in_msg_exec(&self, messages: Vec<Any>) -> Result<Any> {
        use prost::Message;
        use xion_types::cosmos::authz::v1beta1::MsgExec;

        // Validate session before wrapping
        self.metadata.validate()?;

        // Convert cosmrs::Any to prost_types::Any for xion-types compatibility
        let prost_messages: Vec<prost_types::Any> = messages
            .into_iter()
            .map(|msg| prost_types::Any {
                type_url: msg.type_url,
                value: msg.value,
            })
            .collect();

        // Create MsgExec with the granter as grantee and messages to execute
        let msg_exec = MsgExec {
            grantee: self.metadata.grantee.clone(),
            msgs: prost_messages,
        };

        // Encode to protobuf bytes
        let mut buf = Vec::new();
        msg_exec
            .encode(&mut buf)
            .map_err(|e| MobError::Transaction(format!("Failed to encode MsgExec: {}", e)))?;

        // Create Any type
        Ok(Any {
            type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
            value: buf,
        })
    }

    /// Sign a transaction with session key, wrapping messages in MsgExec
    pub fn sign_transaction(
        &self,
        messages: Vec<Any>,
        fee: &crate::types::Fee,
        chain_id: &ChainId,
        account_number: u64,
        sequence: u64,
        memo: Option<String>,
    ) -> Result<Vec<u8>> {
        // Validate session before signing
        self.metadata.validate()?;

        // Wrap all messages in a single MsgExec
        let msg_exec = self.wrap_in_msg_exec(messages)?;

        // Build transaction body with the wrapped message
        let mut body_builder = BodyBuilder::new();
        body_builder.msg(msg_exec);

        if let Some(memo_text) = memo {
            body_builder.memo(&memo_text);
        }

        let body = body_builder.finish();

        // Create auth info using the session key
        let auth_info = self.create_auth_info(fee, sequence)?;

        // Create SignDoc
        let sign_doc = SignDoc::new(&body, &auth_info, chain_id, account_number)?;

        // Sign with session key using only the CryptoSigner trait
        let tx_raw = self.sign_with_trait(&sign_doc, account_number)?;

        // Serialize to bytes
        Ok(tx_raw.to_bytes()?)
    }

    /// Sign a SignDoc using only the CryptoSigner trait (works with any implementation)
    fn sign_with_trait(&self, sign_doc: &SignDoc, account_number: u64) -> Result<tx::Raw> {
        use prost::Message;

        // Encode SignDoc to protobuf bytes
        let mut sign_doc_bytes = Vec::new();
        let sign_doc_proto = xion_types::cosmos::tx::v1beta1::SignDoc {
            body_bytes: sign_doc.body_bytes.clone(),
            auth_info_bytes: sign_doc.auth_info_bytes.clone(),
            chain_id: sign_doc.chain_id.to_string(),
            account_number,
        };
        sign_doc_proto
            .encode(&mut sign_doc_bytes)
            .map_err(|e| MobError::Signing(format!("Failed to encode SignDoc: {}", e)))?;

        // Sign the bytes using the CryptoSigner trait
        let signature = self
            .session_key
            .sign_bytes(sign_doc_bytes)
            .map_err(|e| MobError::Signing(e.to_string()))?;

        // Create raw transaction using proto directly
        let tx_raw_proto = xion_types::cosmos::tx::v1beta1::TxRaw {
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

    fn create_auth_info(&self, fee: &crate::types::Fee, sequence: u64) -> Result<tx::AuthInfo> {
        // Convert fee
        let cosmos_fee = self.convert_fee(fee)?;

        // Create public key from session key
        let pub_key_bytes = self.session_key.public_key();
        let verifying_key =
            cosmrs::crypto::secp256k1::VerifyingKey::from_sec1_bytes(&pub_key_bytes)
                .map_err(|e| MobError::Transaction(format!("Invalid public key: {}", e)))?;

        let pub_key = cosmrs::crypto::PublicKey::from(verifying_key);

        let signer_info = tx::SignerInfo::single_direct(Some(pub_key), sequence);

        Ok(signer_info.auth_info(cosmos_fee))
    }

    fn convert_fee(&self, fee: &crate::types::Fee) -> Result<cosmrs::tx::Fee> {
        let mut coins = Vec::new();
        for c in &fee.amount {
            let denom = c
                .denom
                .parse()
                .map_err(|e| MobError::Transaction(format!("Invalid fee denom: {}", e)))?;
            let amount = c
                .amount
                .parse()
                .map_err(|e| MobError::Transaction(format!("Invalid fee amount: {}", e)))?;
            coins.push(cosmrs::Coin { denom, amount });
        }

        let first_coin = coins
            .first()
            .ok_or(MobError::Transaction("No fee coins".to_string()))?;
        let fee_builder = cosmrs::tx::Fee::from_amount_and_gas(first_coin.clone(), fee.gas_limit);

        Ok(fee_builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_signer_creation() {
        // Create a test session key
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let session_key = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        assert_eq!(session_signer.granter_address(), granter);
        assert!(!session_signer.is_expired());
    }

    #[test]
    fn test_session_signer_from_private_key() {
        let private_key = vec![1u8; 32]; // Dummy key for testing
        let result = SessionSigner::from_private_key(
            private_key,
            "xion".to_string(),
            "xion1granter".to_string(),
            3600,
        );

        assert!(result.is_ok());
        let signer = result.unwrap();
        assert!(!signer.is_expired());
        assert_eq!(signer.remaining_seconds(), 3600);
    }

    #[test]
    fn test_expired_session_validation() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let session_key = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let metadata = SessionMetadata::new(granter, grantee, 0); // Already expired

        let result = SessionSigner::new(Arc::new(session_key), metadata);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrap_in_msg_exec() {
        use crate::transaction::messages;
        use crate::types::Coin;

        // Create two different signers for granter and grantee
        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        // Create an actual MsgSend message using valid addresses
        let recipient_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let recipient_signer = RustSigner::from_mnemonic(
            recipient_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        let amount = vec![Coin::new("uxion", "1000")];
        let msg_send = messages::msg_send(&granter, &recipient_signer.address(), amount)
            .expect("Failed to create MsgSend");

        let result = session_signer.wrap_in_msg_exec(vec![msg_send]);
        assert!(result.is_ok());

        let wrapped = result.unwrap();
        assert_eq!(wrapped.type_url, "/cosmos.authz.v1beta1.MsgExec");

        // Verify the wrapped message can be decoded
        use prost::Message;
        use xion_types::cosmos::authz::v1beta1::MsgExec;

        let decoded = MsgExec::decode(wrapped.value.as_slice());
        assert!(decoded.is_ok());

        let msg_exec = decoded.unwrap();
        assert_eq!(msg_exec.grantee, session_signer.grantee_address());
        assert_eq!(msg_exec.msgs.len(), 1);
        assert_eq!(msg_exec.msgs[0].type_url, "/cosmos.bank.v1beta1.MsgSend");
    }

    #[test]
    fn test_sign_transaction_with_session() {
        use crate::transaction::messages;
        use crate::types::{Coin, Fee};
        use cosmrs::tendermint::chain::Id as ChainId;
        use std::str::FromStr;

        // Create granter signer
        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        // Create session key signer
        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        // Create recipient signer
        let recipient_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let recipient_signer = RustSigner::from_mnemonic(
            recipient_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        // Create a real MsgSend using valid addresses
        let amount = vec![Coin::new("uxion", "1000000")];
        let msg_send = messages::msg_send(&granter, &recipient_signer.address(), amount)
            .expect("Failed to create MsgSend");

        // Create fee
        let fee = Fee::new(vec![Coin::new("uxion", "5000")], 200_000);

        // Sign transaction
        let chain_id = ChainId::from_str("xion-testnet-1").expect("Invalid chain ID");
        let account_number = 123;
        let sequence = 5;

        let tx_bytes = session_signer
            .sign_transaction(
                vec![msg_send],
                &fee,
                &chain_id,
                account_number,
                sequence,
                Some("Test session transaction".to_string()),
            )
            .expect("Failed to sign transaction");

        // Verify we got valid transaction bytes
        assert!(!tx_bytes.is_empty());

        // Decode and verify the transaction structure
        use prost::Message;
        use xion_types::cosmos::tx::v1beta1::TxRaw;

        let tx_raw = TxRaw::decode(tx_bytes.as_slice()).expect("Failed to decode TxRaw");
        assert!(!tx_raw.body_bytes.is_empty());
        assert!(!tx_raw.auth_info_bytes.is_empty());
        assert_eq!(tx_raw.signatures.len(), 1);

        // Decode body and verify it contains MsgExec
        use xion_types::cosmos::tx::v1beta1::TxBody;
        let tx_body = TxBody::decode(tx_raw.body_bytes.as_slice()).expect("Failed to decode body");
        assert_eq!(tx_body.messages.len(), 1);
        assert_eq!(
            tx_body.messages[0].type_url,
            "/cosmos.authz.v1beta1.MsgExec"
        );
        assert_eq!(tx_body.memo, "Test session transaction");
    }

    #[test]
    fn test_expired_session_cannot_sign() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let session_key = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter000000000000000000000000000000".to_string();
        let grantee = session_key.address();

        // Create an already expired session
        let metadata = SessionMetadata::new(granter.clone(), grantee, 0);

        // Should fail to create SessionSigner with expired metadata
        let result = SessionSigner::new(Arc::new(session_key), metadata);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_with_signer_trait_object() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let signer = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = signer.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee.clone(), 3600);

        let trait_signer: Arc<dyn CryptoSigner> = Arc::new(signer);
        let session = SessionSigner::with_signer(trait_signer, metadata)
            .expect("Failed to create session signer");

        assert_eq!(session.granter_address(), granter);
        assert_eq!(session.grantee_address(), grantee);
    }

    #[test]
    fn test_with_signer_rejects_expired() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let signer = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let metadata = SessionMetadata::new(
            "xion1granter".to_string(),
            signer.address(),
            0, // already expired
        );

        let trait_signer: Arc<dyn CryptoSigner> = Arc::new(signer);
        let result = SessionSigner::with_signer(trait_signer, metadata);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_wrap_multiple_messages() {
        use crate::transaction::messages;
        use crate::types::Coin;

        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let recipient = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let amount = vec![Coin::new("uxion", "1000")];
        let msgs: Vec<cosmrs::Any> = (0..3)
            .map(|_| {
                messages::msg_send(&granter, &recipient.address(), amount.clone())
                    .expect("Failed to create MsgSend")
            })
            .collect();

        let wrapped = session_signer
            .wrap_in_msg_exec(msgs)
            .expect("Failed to wrap messages");

        use prost::Message;
        use xion_types::cosmos::authz::v1beta1::MsgExec;

        let decoded = MsgExec::decode(wrapped.value.as_slice()).expect("Failed to decode MsgExec");
        assert_eq!(decoded.msgs.len(), 3);
        for msg in &decoded.msgs {
            assert_eq!(msg.type_url, "/cosmos.bank.v1beta1.MsgSend");
        }
    }

    #[test]
    fn test_wrap_empty_messages() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter, grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let wrapped = session_signer
            .wrap_in_msg_exec(vec![])
            .expect("Failed to wrap empty messages");

        assert_eq!(wrapped.type_url, "/cosmos.authz.v1beta1.MsgExec");

        use prost::Message;
        use xion_types::cosmos::authz::v1beta1::MsgExec;

        let decoded = MsgExec::decode(wrapped.value.as_slice()).expect("Failed to decode");
        assert_eq!(decoded.msgs.len(), 0);
    }

    #[test]
    fn test_metadata_accessors() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let pub_key = session_key.public_key();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee.clone(), 7200);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        assert_eq!(session_signer.granter_address(), granter);
        assert_eq!(session_signer.grantee_address(), grantee);
        assert!(!session_signer.is_expired());
        assert!(session_signer.remaining_seconds() > 7100);
        assert!(session_signer.remaining_seconds() <= 7200);

        let hex_key = session_signer.public_key_hex();
        assert_eq!(hex_key, hex::encode(&pub_key));
        assert_eq!(hex_key.len(), 66); // 33 bytes * 2

        let meta = session_signer.metadata();
        assert_eq!(meta.granter, granter);
        assert_eq!(meta.grantee, grantee);
    }

    #[test]
    fn test_sign_transaction_no_memo() {
        use crate::transaction::messages;
        use crate::types::{Coin, Fee};
        use cosmrs::tendermint::chain::Id as ChainId;
        use std::str::FromStr;

        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let recipient = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let amount = vec![Coin::new("uxion", "1000000")];
        let msg_send = messages::msg_send(&granter, &recipient.address(), amount)
            .expect("Failed to create MsgSend");

        let fee = Fee::new(vec![Coin::new("uxion", "5000")], 200_000);
        let chain_id = ChainId::from_str("xion-testnet-1").expect("Invalid chain ID");

        let tx_bytes = session_signer
            .sign_transaction(vec![msg_send], &fee, &chain_id, 123, 0, None)
            .expect("Failed to sign transaction");

        use prost::Message;
        use xion_types::cosmos::tx::v1beta1::{TxBody, TxRaw};

        let tx_raw = TxRaw::decode(tx_bytes.as_slice()).expect("Failed to decode TxRaw");
        let tx_body = TxBody::decode(tx_raw.body_bytes.as_slice()).expect("Failed to decode body");
        assert_eq!(tx_body.memo, "");
    }

    #[test]
    fn test_sign_transaction_signature_format() {
        use crate::transaction::messages;
        use crate::types::{Coin, Fee};
        use cosmrs::tendermint::chain::Id as ChainId;
        use std::str::FromStr;

        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let recipient = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let amount = vec![Coin::new("uxion", "500")];
        let msg = messages::msg_send(&granter, &recipient.address(), amount)
            .expect("Failed to create MsgSend");

        let fee = Fee::new(vec![Coin::new("uxion", "1000")], 100_000);
        let chain_id = ChainId::from_str("xion-testnet-1").expect("Invalid chain ID");

        let tx_bytes = session_signer
            .sign_transaction(
                vec![msg],
                &fee,
                &chain_id,
                42,
                7,
                Some("sig test".to_string()),
            )
            .expect("Failed to sign transaction");

        use prost::Message;
        use xion_types::cosmos::tx::v1beta1::TxRaw;

        let tx_raw = TxRaw::decode(tx_bytes.as_slice()).expect("Failed to decode TxRaw");
        assert_eq!(tx_raw.signatures.len(), 1);
        assert_eq!(tx_raw.signatures[0].len(), 64);
    }

    #[test]
    fn test_convert_fee_no_coins() {
        use crate::transaction::messages;
        use crate::types::Fee;
        use cosmrs::tendermint::chain::Id as ChainId;
        use std::str::FromStr;

        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let recipient = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let msg = messages::msg_send(
            &granter,
            &recipient.address(),
            vec![crate::types::Coin::new("uxion", "100")],
        )
        .expect("Failed to create MsgSend");

        // Fee with empty coin vec
        let fee = Fee::new(vec![], 200_000);
        let chain_id = ChainId::from_str("xion-testnet-1").expect("Invalid chain ID");

        let result = session_signer.sign_transaction(vec![msg], &fee, &chain_id, 1, 0, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No fee coins"));
    }

    #[test]
    fn test_sign_transaction_multiple_messages() {
        use crate::transaction::messages;
        use crate::types::{Coin, Fee};
        use cosmrs::tendermint::chain::Id as ChainId;
        use std::str::FromStr;

        let granter_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let granter_signer =
            RustSigner::from_mnemonic(granter_mnemonic.to_string(), "xion".to_string(), None)
                .expect("Failed to create granter signer");

        let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/1".to_string()),
        )
        .expect("Failed to create session key");

        let recipient = RustSigner::from_mnemonic(
            session_mnemonic.to_string(),
            "xion".to_string(),
            Some("m/44'/118'/0'/0/2".to_string()),
        )
        .expect("Failed to create recipient");

        let granter = granter_signer.address();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let amount = vec![Coin::new("uxion", "1000")];
        let msg1 = messages::msg_send(&granter, &recipient.address(), amount.clone())
            .expect("Failed to create MsgSend 1");
        let msg2 = messages::msg_send(&granter, &recipient.address(), amount)
            .expect("Failed to create MsgSend 2");

        let fee = Fee::new(vec![Coin::new("uxion", "10000")], 400_000);
        let chain_id = ChainId::from_str("xion-testnet-1").expect("Invalid chain ID");

        let tx_bytes = session_signer
            .sign_transaction(vec![msg1, msg2], &fee, &chain_id, 10, 3, None)
            .expect("Failed to sign transaction");

        use prost::Message;
        use xion_types::cosmos::authz::v1beta1::MsgExec;
        use xion_types::cosmos::tx::v1beta1::{TxBody, TxRaw};

        let tx_raw = TxRaw::decode(tx_bytes.as_slice()).expect("Failed to decode TxRaw");
        let tx_body = TxBody::decode(tx_raw.body_bytes.as_slice()).expect("Failed to decode body");

        // Body should contain a single MsgExec
        assert_eq!(tx_body.messages.len(), 1);
        assert_eq!(
            tx_body.messages[0].type_url,
            "/cosmos.authz.v1beta1.MsgExec"
        );

        // The MsgExec should contain 2 inner messages
        let msg_exec = MsgExec::decode(tx_body.messages[0].value.as_slice())
            .expect("Failed to decode MsgExec");
        assert_eq!(msg_exec.msgs.len(), 2);
        assert_eq!(msg_exec.msgs[0].type_url, "/cosmos.bank.v1beta1.MsgSend");
        assert_eq!(msg_exec.msgs[1].type_url, "/cosmos.bank.v1beta1.MsgSend");
    }

    #[test]
    fn test_debug_format() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let session_key = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter.clone(), grantee, 3600);
        let expires_at = metadata.expires_at;

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        let debug_str = format!("{:?}", session_signer);
        assert!(debug_str.contains("SessionSigner"));
        assert!(debug_str.contains("xion1granter"));
        assert!(debug_str.contains(&expires_at.to_string()));
    }
}
