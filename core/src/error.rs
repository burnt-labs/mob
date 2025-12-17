use thiserror::Error;

/// Main error type for the mob library
#[derive(Debug, Error)]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi-bindings", uniffi(flat_error))]
pub enum MobError {
    /// RPC-related errors
    #[error("RPC error: {0}")]
    Rpc(String),

    /// Transaction-related errors
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Signing-related errors
    #[error("Signing error: {0}")]
    Signing(String),

    /// Key derivation errors
    #[error("Key derivation error: {0}")]
    KeyDerivation(String),

    /// Address parsing/generation errors
    #[error("Address error: {0}")]
    Address(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid input errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Account-related errors
    #[error("Account error: {0}")]
    Account(String),

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Gas estimation errors
    #[error("Gas estimation error: {0}")]
    GasEstimation(String),

    /// Insufficient funds
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    /// Timeout errors
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Generic error
    #[error("Error: {0}")]
    Generic(String),
}

// Conversions from underlying library errors
impl From<cosmrs::Error> for MobError {
    fn from(err: cosmrs::Error) -> Self {
        MobError::Transaction(err.to_string())
    }
}

impl From<cosmrs::ErrorReport> for MobError {
    fn from(err: cosmrs::ErrorReport) -> Self {
        MobError::Transaction(err.to_string())
    }
}

#[cfg(feature = "rpc-client")]
impl From<tendermint_rpc::Error> for MobError {
    fn from(err: tendermint_rpc::Error) -> Self {
        MobError::Rpc(err.to_string())
    }
}

impl From<serde_json::Error> for MobError {
    fn from(err: serde_json::Error) -> Self {
        MobError::Serialization(err.to_string())
    }
}

impl From<bip32::Error> for MobError {
    fn from(err: bip32::Error) -> Self {
        MobError::KeyDerivation(err.to_string())
    }
}

impl From<hex::FromHexError> for MobError {
    fn from(err: hex::FromHexError) -> Self {
        MobError::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for MobError {
    fn from(err: std::io::Error) -> Self {
        MobError::Network(err.to_string())
    }
}

/// Result type alias for mob operations
pub type Result<T> = std::result::Result<T, MobError>;

// UniFFI error conversion
// This allows errors to cross the FFI boundary
impl From<MobError> for String {
    fn from(err: MobError) -> Self {
        err.to_string()
    }
}
