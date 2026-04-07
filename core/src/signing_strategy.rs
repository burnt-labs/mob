use crate::{
    crypto_signer::{CryptoSigner, SignerError},
    error::Result,
    session_signer::SessionSigner,
};
use cosmrs::Any;
use std::sync::Arc;

/// Transaction-level signing policy for the client.
///
/// This sits above raw byte signing and owns the transaction semantics:
/// which address signs, which address appears as the logical sender, how
/// messages are transformed before simulation/signing, and any fee overrides.
pub trait TransactionSigner: Send + Sync {
    /// The on-chain account whose sequence/account number are used for signing.
    fn signing_address(&self) -> String;

    /// The address that should appear as the sender inside the transaction's
    /// logical messages before any higher-level transformation.
    fn logical_sender_address(&self) -> Result<String>;

    /// Transform messages prior to simulation/signing. Plain signers typically
    /// return the input unchanged, while session signers may wrap messages in
    /// authz envelopes such as MsgExec.
    fn transform_messages(&self, messages: Vec<Any>) -> Result<Vec<Any>>;

    /// Optional fee granter override applied when building auth info.
    fn fee_granter(&self) -> Option<String> {
        None
    }

    /// Optional fee payer override applied when building auth info.
    fn fee_payer(&self) -> Option<String> {
        None
    }

    /// The public key used for signing the transaction.
    fn public_key(&self) -> Vec<u8>;

    /// Sign the encoded SignDoc bytes for the transaction.
    fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError>;
}

/// Default transaction signer for non-session flows.
#[derive(Clone)]
pub struct BasicSigningStrategy {
    signer: Arc<dyn CryptoSigner>,
}

impl BasicSigningStrategy {
    pub fn new(signer: Arc<dyn CryptoSigner>) -> Self {
        Self { signer }
    }
}

impl TransactionSigner for BasicSigningStrategy {
    fn signing_address(&self) -> String {
        self.signer.address()
    }

    fn logical_sender_address(&self) -> Result<String> {
        Ok(self.signer.address())
    }

    fn transform_messages(&self, messages: Vec<Any>) -> Result<Vec<Any>> {
        Ok(messages)
    }

    fn public_key(&self) -> Vec<u8> {
        self.signer.public_key()
    }

    fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        self.signer.sign_bytes(message)
    }
}

impl TransactionSigner for SessionSigner {
    fn signing_address(&self) -> String {
        self.session_key().address()
    }

    fn logical_sender_address(&self) -> Result<String> {
        self.metadata().validate()?;
        Ok(self.granter_address())
    }

    fn transform_messages(&self, messages: Vec<Any>) -> Result<Vec<Any>> {
        Ok(vec![self.wrap_in_msg_exec(messages)?])
    }

    fn fee_granter(&self) -> Option<String> {
        self.metadata()
            .fee_granter
            .clone()
            .or_else(|| Some(self.granter_address()))
    }

    fn fee_payer(&self) -> Option<String> {
        self.metadata().fee_payer.clone()
    }

    fn public_key(&self) -> Vec<u8> {
        self.session_key().public_key()
    }

    fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        self.session_key().sign_bytes(message)
    }
}

impl<T: CryptoSigner + ?Sized> TransactionSigner for T {
    fn signing_address(&self) -> String {
        self.address()
    }

    fn logical_sender_address(&self) -> Result<String> {
        Ok(self.address())
    }

    fn transform_messages(&self, messages: Vec<Any>) -> Result<Vec<Any>> {
        Ok(messages)
    }

    fn public_key(&self) -> Vec<u8> {
        CryptoSigner::public_key(self)
    }

    fn sign_bytes(&self, message: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        CryptoSigner::sign_bytes(self, message)
    }
}
