# Mob 🦀

A multi-platform signing client library for the XION blockchain, written in Rust using Mozilla's UniFFI framework.

## Features

- 🔐 **Key Management**: Mnemonic-based key derivation and private key management
- 📝 **Transaction Building**: Intuitive API for building and signing transactions
- 🌐 **RPC Client**: Full-featured client for interacting with XION nodes
- 🔄 **Account Abstraction**: Support for XION's account abstraction features
- 🦀 **Pure Rust**: Core logic written in Rust for safety and performance
- 🌍 **Multi-platform**: Generate bindings for Kotlin, Swift, Python, Ruby, and more via UniFFI

## Architecture

Mob is designed to replace the existing JavaScript-based signing clients (xion.js/cosm.js) with a pure Rust implementation that can be used across multiple platforms:

```
┌─────────────────────────────────────────┐
│           Application Layer             │
│  (Kotlin, Swift, Python, Ruby, etc.)    │
└─────────────────┬───────────────────────┘
                  │
                  │ UniFFI Bindings
                  │
┌─────────────────▼───────────────────────┐
│           Mob Core (Rust)               │
│  ┌──────────┬──────────┬──────────┐    │
│  │  Signer  │  Client  │ TxBuilder│    │
│  └──────────┴──────────┴──────────┘    │
│  ┌──────────┬──────────┬──────────┐    │
│  │ Account  │  Types   │  Errors  │    │
│  └──────────┴──────────┴──────────┘    │
└─────────────────┬───────────────────────┘
                  │
                  │ cosmrs / tendermint-rpc
                  │
┌─────────────────▼───────────────────────┐
│          XION Blockchain                │
└─────────────────────────────────────────┘
```

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
mob = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/burnt-labs/mob.git
cd mob

# Build the library
cargo build --release

# Run tests
cargo test

# Run examples
cargo run --example basic_usage
```

## Usage

### Rust

```rust
use mob::{ChainConfig, Client, Coin, Signer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create chain configuration
    let config = ChainConfig::new(
        "xion-testnet-1",
        "https://rpc.xion-testnet-1.burnt.com:443",
        "xion"
    );

    // Create RPC client
    let mut client = Client::new(config).await?;

    // Create signer from mnemonic
    let signer = Signer::from_mnemonic(
        "your mnemonic words here",
        "xion",
        None
    )?;

    // Attach signer to client
    client.attach_signer(signer).await?;

    // Send tokens
    let response = client.send(
        "xion1recipient...",
        vec![Coin::new("uxion", "1000000")],
        Some("Test transfer".to_string())
    ).await?;

    println!("Transaction hash: {}", response.txhash);

    Ok(())
}
```

### Generating Language Bindings

UniFFI can generate bindings for multiple languages.

#### Python (Recommended - Complete Setup Available)

We provide a complete script that handles everything:

```bash
./scripts/generate_python_bindings.sh
```

This will:
1. Build the Rust library
2. Generate Python bindings
3. Create setup.py and package files
4. Generate test scripts
5. Create documentation

After generation:
```bash
cd bindings/python
pip install .
python example_usage.py
```

📚 **See [PYTHON_BINDINGS.md](PYTHON_BINDINGS.md) for complete Python documentation**

#### Kotlin (Android)

```bash
cargo run --bin uniffi-bindgen generate \
    --library target/release/libmob.so \
    --language kotlin \
    --out-dir bindings/kotlin
```

#### Swift (iOS)

```bash
cargo run --bin uniffi-bindgen generate \
    --library target/release/libmob.dylib \
    --language swift \
    --out-dir bindings/swift
```

## Examples

The `examples/` directory contains several usage examples:

- **basic_usage.rs**: Simple client setup and account queries
- **send_tokens.rs**: Sending tokens between accounts
- **execute_contract.rs**: Executing CosmWasm contracts
- **query_balance.rs**: Querying account balances

Run examples with:

```bash
cargo run --example basic_usage
```

For examples that require credentials:

```bash
MNEMONIC="your mnemonic here" cargo run --example send_tokens
```

## Project Structure

```
mob/
├── core/                    # Core library
│   ├── src/
│   │   ├── lib.rs          # Library entry point
│   │   ├── account.rs      # Account management
│   │   ├── client.rs       # RPC client
│   │   ├── error.rs        # Error types
│   │   ├── signer.rs       # Key management and signing
│   │   ├── transaction.rs  # Transaction building
│   │   ├── types.rs        # Common types
│   │   └── mob.udl         # UniFFI interface definition
│   ├── Cargo.toml
│   └── build.rs            # Build script
├── examples/               # Usage examples
├── Cargo.toml              # Workspace configuration
└── README.md
```

## Key Components

### Signer

Manages private keys and signing operations:

```rust
let signer = Signer::from_mnemonic(
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    "xion",
    Some("m/44'/118'/0'/0/0")
)?;

let signature = signer.sign_bytes(b"message to sign")?;
```

### Client

RPC client for blockchain interaction:

```rust
let mut client = Client::new(config).await?;
client.attach_signer(signer).await?;

// Query operations
let balance = client.get_balance("xion1...", "uxion").await?;
let account_info = client.get_account("xion1...").await?;
let height = client.get_height().await?;

// Transaction operations
let tx_response = client.send(recipient, amount, memo).await?;
let tx = client.get_tx("tx_hash").await?;
```

### Transaction Builder

Build and sign complex transactions:

```rust
use mob::transaction::{TransactionBuilder, messages};

let mut builder = TransactionBuilder::new("xion-testnet-1")?;
builder.add_message(messages::msg_send(from, to, amount)?);
builder.with_fee(fee);
builder.with_memo("My transaction");

let signed_tx = builder.sign(&signer, account_number, sequence)?;
```

## Dependencies

- **xion-cosmos-sdk-proto**: Protocol definitions for XION blockchain
- **cosmrs**: Cosmos SDK for Rust
- **tendermint-rpc**: Tendermint RPC client
- **uniffi**: Multi-language bindings generator
- **tokio**: Async runtime
- **k256**: Cryptographic operations

## Development

### Prerequisites

- Rust 1.70+ (stable)
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Build with all features
cargo build --all-features
```

### Testing

```bash
# Run all unit tests
cargo test

# Run integration tests (requires network)
cargo test --test rpc_integration_test -- --ignored

# Run specific integration test with output
cargo test --test rpc_integration_test test_rpc_endpoint_full_workflow -- --ignored --nocapture

# Run tests with output
cargo test -- --nocapture
```

The integration tests make real network calls to the XION testnet-2 RPC endpoint:
```
https://rpc.xion-testnet-2.burnt.com:443
```

See `core/tests/README.md` for detailed test documentation.

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check without building
cargo check
```

## Roadmap

- [x] Core signing functionality
- [x] RPC client implementation
- [x] Transaction building and broadcasting
- [x] Account abstraction support
- [x] UniFFI bindings interface
- [ ] gRPC client implementation
- [ ] Advanced account abstraction features
- [ ] Gas estimation improvements
- [ ] Mobile SDK packages (iOS/Android)
- [ ] Python package distribution
- [ ] Comprehensive integration tests

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT

## Credits

Built by [Burnt Labs](https://burnt.com) for the XION ecosystem.

## Related Projects

- [xion.js](https://github.com/burnt-labs/xion.js) - JavaScript signing client (being replaced)
- [XION](https://github.com/burnt-labs/xion) - XION blockchain
- [UniFFI](https://github.com/mozilla/uniffi-rs) - Multi-language bindings generator
