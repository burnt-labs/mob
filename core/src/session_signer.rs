use crate::{
    error::{MobError, Result},
    session::SessionMetadata,
    signer::Signer,
};
use cosmrs::{
    tendermint::chain::Id as ChainId,
    tx::{self, BodyBuilder, SignDoc},
    Any,
};
use std::sync::Arc;

/// A session signer that wraps messages in MsgExec (Authz) for session key usage
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Object))]
pub struct SessionSigner {
    /// The underlying session key signer
    session_key: Arc<Signer>,
    /// Session metadata including granter and expiration
    metadata: SessionMetadata,
}

#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl SessionSigner {
    /// Create a new session signer from a session key signer and metadata
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn new(session_key: Arc<Signer>, metadata: SessionMetadata) -> Result<Self> {
        // Validate session on creation
        metadata.validate()?;

        Ok(Self {
            session_key,
            metadata,
        })
    }

    /// Create a session signer from a private key with duration
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn from_private_key(
        private_key: Vec<u8>,
        address_prefix: String,
        granter_address: String,
        duration_seconds: u64,
    ) -> Result<Self> {
        let signer = Signer::from_private_key(&private_key, &address_prefix)?;
        let grantee_address = signer.address();

        let metadata =
            SessionMetadata::with_duration(granter_address, grantee_address, duration_seconds);

        Ok(Self {
            session_key: Arc::new(signer),
            metadata,
        })
    }

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
        self.session_key.public_key_hex()
    }
}

impl SessionSigner {
    /// Get reference to the underlying session key signer
    pub fn session_key(&self) -> &Arc<Signer> {
        &self.session_key
    }

    /// Wrap messages in MsgExec for authz execution
    pub fn wrap_in_msg_exec(&self, messages: Vec<Any>) -> Result<Any> {
        use cosmos_sdk_proto::cosmos::authz::v1beta1::MsgExec;
        use prost::Message;

        // Validate session before wrapping
        self.metadata.validate()?;

        // Create MsgExec with the granter as grantee and messages to execute
        let msg_exec = MsgExec {
            grantee: self.metadata.grantee.clone(),
            msgs: messages,
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

        // Sign with session key
        let tx_raw = self.session_key.sign_direct(&sign_doc, account_number)?;

        // Serialize to bytes
        Ok(tx_raw.to_bytes()?)
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
        let session_key = Signer::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
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
        let session_key = Signer::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let metadata = SessionMetadata::new(granter, grantee, 0); // Already expired

        let result = SessionSigner::new(Arc::new(session_key), metadata);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrap_in_msg_exec() {
        let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
        let session_key = Signer::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
            .expect("Failed to create signer");

        let granter = "xion1granter".to_string();
        let grantee = session_key.address();
        let metadata = SessionMetadata::with_duration(granter, grantee, 3600);

        let session_signer = SessionSigner::new(Arc::new(session_key), metadata)
            .expect("Failed to create session signer");

        // Create a dummy message
        let dummy_msg = Any {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            value: vec![1, 2, 3],
        };

        let result = session_signer.wrap_in_msg_exec(vec![dummy_msg]);
        assert!(result.is_ok());

        let wrapped = result.unwrap();
        assert_eq!(wrapped.type_url, "/cosmos.authz.v1beta1.MsgExec");
    }
}
