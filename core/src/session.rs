use crate::error::{MobError, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Session key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Record))]
pub struct SessionMetadata {
    /// The address that granted this session (the main account)
    pub granter: String,
    /// The address of the session key (grantee)
    pub grantee: String,
    /// Optional fee granter for transactions signed under this session.
    /// Defaults to the granter when omitted.
    pub fee_granter: Option<String>,
    /// Optional fee payer for transactions signed under this session.
    pub fee_payer: Option<String>,
    /// When the session was created (Unix timestamp in seconds)
    pub created_at: u64,
    /// When the session expires (Unix timestamp in seconds)
    pub expires_at: u64,
    /// Optional description of the session
    pub description: Option<String>,
}

impl SessionMetadata {
    /// Create new session metadata
    pub fn new(granter: String, grantee: String, expires_at: u64) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            granter,
            grantee,
            fee_granter: None,
            fee_payer: None,
            created_at,
            expires_at,
            description: None,
        }
    }

    /// Create session metadata with a duration from now
    pub fn with_duration(granter: String, grantee: String, duration_seconds: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            granter,
            grantee,
            fee_granter: None,
            fee_payer: None,
            created_at: now,
            expires_at: now + duration_seconds,
            description: None,
        }
    }

    /// Override the fee granter for this session.
    pub fn with_fee_granter(mut self, fee_granter: String) -> Self {
        self.fee_granter = Some(fee_granter);
        self
    }

    /// Override the fee payer for this session.
    pub fn with_fee_payer(mut self, fee_payer: String) -> Self {
        self.fee_payer = Some(fee_payer);
        self
    }

    /// Add a description to the session
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now >= self.expires_at
    }

    /// Get remaining time in seconds
    pub fn remaining_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.expires_at.saturating_sub(now)
    }

    /// Validate the session is not expired
    pub fn validate(&self) -> Result<()> {
        if self.is_expired() {
            return Err(MobError::SessionExpired(format!(
                "Session expired at {}",
                self.expires_at
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_metadata_creation() {
        let granter = "xion1granter".to_string();
        let grantee = "xion1grantee".to_string();
        let duration = 3600; // 1 hour

        let session = SessionMetadata::with_duration(granter.clone(), grantee.clone(), duration);

        assert_eq!(session.granter, granter);
        assert_eq!(session.grantee, grantee);
        assert!(!session.is_expired());
        assert!(session.remaining_seconds() > 0);
    }

    #[test]
    fn test_session_expiration() {
        let granter = "xion1granter".to_string();
        let grantee = "xion1grantee".to_string();
        let expires_at = 0; // Already expired

        let session = SessionMetadata::new(granter, grantee, expires_at);

        assert!(session.is_expired());
        assert_eq!(session.remaining_seconds(), 0);
        assert!(session.validate().is_err());
    }

    #[test]
    fn test_session_with_description() {
        let session = SessionMetadata::with_duration(
            "xion1granter".to_string(),
            "xion1grantee".to_string(),
            3600,
        )
        .with_description("Test session".to_string());

        assert_eq!(session.description, Some("Test session".to_string()));
    }

    #[test]
    fn test_session_fee_overrides() {
        let session = SessionMetadata::with_duration(
            "xion1granter".to_string(),
            "xion1grantee".to_string(),
            3600,
        )
        .with_fee_granter("xion1treasury".to_string())
        .with_fee_payer("xion1payer".to_string());

        assert_eq!(session.fee_granter, Some("xion1treasury".to_string()));
        assert_eq!(session.fee_payer, Some("xion1payer".to_string()));
    }
}
