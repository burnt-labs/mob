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
//! use mob::{Client, Signer, ChainConfig, Coin};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create chain configuration
//!     let config = ChainConfig::new(
//!         "xion-testnet-1",
//!         "https://rpc.xion-testnet-1.burnt.com:443",
//!         "xion"
//!     );
//!
//!     // Create RPC client
//!     let mut client = Client::new(config).await?;
//!
//!     // Create signer from mnemonic
//!     let signer = Signer::from_mnemonic(
//!         "your mnemonic words here",
//!         "xion",
//!         None
//!     )?;
//!
//!     // Attach signer to client
//!     client.attach_signer(signer).await?;
//!
//!     // Send tokens
//!     let response = client.send(
//!         "xion1recipient...",
//!         vec![Coin::new("uxion", "1000000")],
//!         Some("Test transfer".to_string())
//!     ).await?;
//!
//!     println!("Transaction hash: {}", response.txhash);
//!
//!     Ok(())
//! }
//! ```

pub mod account;
pub mod client;
pub mod error;
pub mod session;
pub mod session_signer;
pub mod signer;
pub mod transaction;
pub mod types;

// Re-export main types for convenience
pub use account::{abstraction, Account};
#[cfg(feature = "rpc-client")]
pub use client::Client;
pub use error::{MobError, Result};
pub use session::SessionMetadata;
pub use session_signer::SessionSigner;
pub use signer::Signer;
pub use transaction::{messages, TransactionBuilder};
pub use types::{
    AccountInfo, BroadcastMode, ChainConfig, Coin, Fee, Message, SignOptions, TxResponse,
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
