//! # Mob - Multi-platform Signing Client for XION
//!
//! Mob is a Rust-based signing client library for the XION blockchain,
//! built using Mozilla's UniFFI framework to provide bindings for multiple
//! programming languages including Kotlin, Swift, Python, and Ruby.
//!
//! ## Features
//!
//! - 🔐 **Key Management**: Mnemonic-based key derivation and private key management
//! - 📝 **Transaction Building**: Intuitive API for building and signing transactions
//! - 🌐 **RPC Client**: Full-featured client for interacting with XION nodes
//! - 🔄 **Account Abstraction**: Support for XION's account abstraction features
//! - 🦀 **Pure Rust**: Core logic written in Rust for safety and performance
//! - 🌍 **Multi-platform**: Generate bindings for multiple languages via UniFFI
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use mob::{Client, RustSigner, ChainConfig, Coin};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create chain configuration
//!     let config = ChainConfig::new(
//!         "xion-testnet-1".to_string(),
//!         "https://rpc.xion-testnet-1.burnt.com:443".to_string(),
//!         "xion".to_string()
//!     );
//!
//!     // Create RPC client
//!     let mut client = Client::new(config)?;
//!
//!     // Create signer from mnemonic
//!     let signer = RustSigner::from_mnemonic(
//!         "your mnemonic words here".to_string(),
//!         "xion".to_string(),
//!         None
//!     )?;
//!
//!     // Attach signer to client
//!     client.attach_crypto_signer(Arc::new(signer)).await?;
//!
//!     // Send tokens
//!     let response = client.send(
//!         "xion1recipient...".to_string(),
//!         vec![Coin::new("uxion".to_string(), "1000000".to_string())],
//!         Some("Test transfer".to_string())
//!     )?;
//!
//!     println!("Transaction hash: {}", response.txhash);
//!
//!     Ok(())
//! }
//! ```

pub mod account;
pub mod client;
pub mod crypto_signer;
pub mod error;
#[cfg(feature = "rust-signer")]
pub mod rust_signer;
pub mod session;
#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
pub mod session_manager;
pub mod session_signer;
pub mod transaction;
pub mod types;

// Re-export main types for convenience
pub use account::{abstraction, Account};
#[cfg(feature = "rpc-client")]
pub use client::Client;
pub use cosmrs::Any;
pub use crypto_signer::{CryptoSigner, SignerError};
pub use error::{MobError, Result};
#[cfg(feature = "rust-signer")]
pub use rust_signer::RustSigner;
pub use session::SessionMetadata;
#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
pub use session_manager::MobSessionManager;
pub use session_signer::SessionSigner;
pub use transaction::{messages, TransactionBuilder};
pub use types::{
    AccountInfo, BroadcastMode, ChainConfig, Coin, Fee, Message, SignOptions, SignerInfo,
    TxResponse,
};

// UniFFI setup - only when uniffi-bindings feature is enabled
#[cfg(feature = "uniffi-bindings")]
uniffi::setup_scaffolding!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // Test that main types are accessible
        let _coin = Coin::new("uxion", "1000");
        let _config = ChainConfig::new("test-chain", "http://localhost:26657", "xion");
    }
}
