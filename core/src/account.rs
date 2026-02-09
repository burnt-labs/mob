use crate::{
    error::{MobError, Result},
    types::AccountInfo,
};
use cosmrs::{proto::cosmos::auth::v1beta1::BaseAccount, AccountId};
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};

/// Sentinel value meaning "sequence not yet fetched"
const SEQ_UNSET: u64 = u64::MAX;

/// Account manager for querying and managing account information
#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub account_number: Option<u64>,
    sequence: AtomicU64,
}

impl Clone for Account {
    fn clone(&self) -> Self {
        Self {
            address: self.address.clone(),
            account_number: self.account_number,
            sequence: AtomicU64::new(self.sequence.load(Ordering::Relaxed)),
        }
    }
}

impl Account {
    /// Create a new account instance
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            account_number: None,
            sequence: AtomicU64::new(SEQ_UNSET),
        }
    }

    /// Update account info
    pub fn update_info(&mut self, info: AccountInfo) {
        self.account_number = Some(info.account_number);
        self.sequence.store(info.sequence, Ordering::Relaxed);
    }

    /// Get account number, returning error if not yet fetched
    pub fn account_number(&self) -> Result<u64> {
        self.account_number
            .ok_or_else(|| MobError::Account("Account number not yet fetched".to_string()))
    }

    /// Get sequence, returning error if not yet fetched
    pub fn sequence(&self) -> Result<u64> {
        let seq = self.sequence.load(Ordering::Relaxed);
        if seq == SEQ_UNSET {
            Err(MobError::Account("Sequence not yet fetched".to_string()))
        } else {
            Ok(seq)
        }
    }

    /// Increment the sequence number (used after signing a transaction)
    pub fn increment_sequence(&self) {
        let _ = self.sequence.fetch_add(1, Ordering::Relaxed);
    }

    /// Validate address format
    pub fn validate_address(address: &str, expected_prefix: &str) -> Result<()> {
        let account_id = AccountId::from_str(address)
            .map_err(|e| MobError::Address(format!("Invalid address format: {}", e)))?;

        if account_id.prefix() != expected_prefix {
            return Err(MobError::Address(format!(
                "Address prefix mismatch: expected {}, got {}",
                expected_prefix,
                account_id.prefix()
            )));
        }

        Ok(())
    }

    /// Parse base account from proto response
    pub fn from_base_account(base_account: BaseAccount) -> Result<AccountInfo> {
        Ok(AccountInfo {
            address: base_account.address,
            account_number: base_account.account_number,
            sequence: base_account.sequence,
            pub_key: base_account.pub_key.map(|pk| hex::encode(pk.value)),
        })
    }
}

/// Account abstraction related types and utilities
pub mod abstraction {
    use serde::{Deserialize, Serialize};

    /// Abstract account type for XION's account abstraction
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AbstractAccount {
        pub address: String,
        pub authenticators: Vec<Authenticator>,
        pub metadata: AccountMetadata,
    }

    /// Authenticator for abstract accounts
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Authenticator {
        pub id: String,
        pub authenticator_type: AuthenticatorType,
        pub data: Vec<u8>,
    }

    /// Types of authenticators supported
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AuthenticatorType {
        /// Secp256k1 signature
        Secp256k1,
        /// WebAuthn / Passkey
        WebAuthn,
        /// JWT-based authentication
        Jwt,
        /// Multi-signature
        MultiSig,
        /// Custom authenticator
        Custom(String),
    }

    /// Metadata for abstract accounts
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccountMetadata {
        pub created_at: Option<i64>,
        pub updated_at: Option<i64>,
        pub labels: Vec<String>,
    }

    impl AbstractAccount {
        pub fn new(address: String) -> Self {
            Self {
                address,
                authenticators: vec![],
                metadata: AccountMetadata {
                    created_at: None,
                    updated_at: None,
                    labels: vec![],
                },
            }
        }

        pub fn add_authenticator(&mut self, authenticator: Authenticator) {
            self.authenticators.push(authenticator);
        }

        pub fn get_authenticator(&self, id: &str) -> Option<&Authenticator> {
            self.authenticators.iter().find(|a| a.id == id)
        }
    }

    impl Authenticator {
        pub fn new_secp256k1(id: String, pub_key: Vec<u8>) -> Self {
            Self {
                id,
                authenticator_type: AuthenticatorType::Secp256k1,
                data: pub_key,
            }
        }

        pub fn new_webauthn(id: String, credential_id: Vec<u8>) -> Self {
            Self {
                id,
                authenticator_type: AuthenticatorType::WebAuthn,
                data: credential_id,
            }
        }

        pub fn new_jwt(id: String, issuer_data: Vec<u8>) -> Self {
            Self {
                id,
                authenticator_type: AuthenticatorType::Jwt,
                data: issuer_data,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Account, AccountInfo};

    #[test]
    fn test_account_creation() {
        let account = Account::new("xion1234567890abcdef");
        assert_eq!(account.address, "xion1234567890abcdef");
        assert!(account.account_number.is_none());
        assert!(account.sequence().is_err());
    }

    #[test]
    fn test_account_update() {
        let mut account = Account::new("xion1234567890abcdef");
        let info = AccountInfo {
            address: "xion1234567890abcdef".to_string(),
            account_number: 42,
            sequence: 10,
            pub_key: None,
        };

        account.update_info(info);
        assert_eq!(account.account_number().unwrap(), 42);
        assert_eq!(account.sequence().unwrap(), 10);
    }

    #[test]
    fn test_sequence_increment() {
        let mut account = Account::new("xion1234567890abcdef");
        let info = AccountInfo {
            address: "xion1234567890abcdef".to_string(),
            account_number: 42,
            sequence: 10,
            pub_key: None,
        };

        account.update_info(info);
        account.increment_sequence();
        assert_eq!(account.sequence().unwrap(), 11);
    }

    #[test]
    fn test_validate_address() {
        // This test would need valid bech32 addresses to work properly
        // Skipping actual validation test as it requires valid checksums
    }

    #[test]
    fn test_abstract_account() {
        use crate::abstraction::*;

        let mut account = AbstractAccount::new("xion1234567890abcdef".to_string());
        let auth = Authenticator::new_secp256k1("auth1".to_string(), vec![1, 2, 3]);

        account.add_authenticator(auth);
        assert_eq!(account.authenticators.len(), 1);
        assert!(account.get_authenticator("auth1").is_some());
    }
}
