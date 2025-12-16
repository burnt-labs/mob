# Mob Ruby Bindings

Ruby bindings for the Mob library - a multi-platform signing client for the XION blockchain.

## Overview

Mob provides a comprehensive Ruby interface for interacting with the XION blockchain, including:

- 🔐 **Key Management** - Mnemonic-based key derivation and private key management
- 📝 **Transaction Building** - Intuitive API for building and signing transactions
- 🌐 **RPC Client** - Full-featured client for interacting with XION nodes
- 🔄 **Account Abstraction** - Support for XION's account abstraction features
- 🦀 **Pure Rust Core** - High-performance core logic with Ruby bindings via UniFFI

## Installation

### Prerequisites

- Ruby 2.6 or later
- Rust toolchain (for building from source)
- The compiled Rust library (`libmob.dylib` on macOS, `libmob.so` on Linux)

### Building from Source

```bash
# From the project root directory
./ruby/scripts/generate_bindings.sh
```

This script will:
1. Build the Rust library in release mode
2. Generate Ruby bindings using UniFFI
3. Copy the library to `ruby/lib/`
4. Fix the library path in the generated bindings

### Setup

The Ruby bindings are generated via UniFFI and located in the `ruby/lib` directory:

```bash
# From the ruby/ directory
ruby examples/basic_query.rb
```

Or add to your Ruby script:

```ruby
require_relative 'lib/mob'  # Adjust path as needed
```

## Usage

See [QUICKSTART.md](QUICKSTART.md) for quick start examples and common usage patterns.

The `examples/` directory contains working sample code:

```bash
# From the ruby/ directory
ruby examples/basic_query.rb
ruby examples/account_query.rb
ruby examples/send_transaction.rb  # ⚠️ Requires funded test account
```

## Running Tests

The test suite uses Ruby's built-in Minitest framework:

```bash
# From the ruby/ directory
ruby test/test_rpc_queries.rb

# Or with verbose output
ruby test/test_rpc_queries.rb -v
```

### Integration Tests

Integration tests interact with the live XION testnet and are skipped by default. To run them:

```bash
# Run all tests including integration tests
INTEGRATION=1 ruby test/test_rpc_queries.rb

# Run with verbose output
INTEGRATION=1 ruby test/test_rpc_queries.rb -v
```

**Note:** Integration tests require a funded test account. The test mnemonic must have sufficient uxion balance for transaction fees and token transfers.

## API Reference

### ChainConfig

Configuration for connecting to a blockchain network.

```ruby
config = Mob::ChainConfig.new(
  chain_id: "xion-testnet-2",
  rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
  grpc_endpoint: nil,
  address_prefix: "xion",
  coin_type: 118,
  gas_price: "0.025"
)
```

**Parameters:**
- `chain_id` (String) - Chain identifier (e.g., "xion-testnet-2")
- `rpc_endpoint` (String) - RPC endpoint URL
- `grpc_endpoint` (String, optional) - gRPC endpoint URL (can be nil)
- `address_prefix` (String) - Bech32 address prefix (e.g., "xion")
- `coin_type` (Integer) - BIP44 coin type (118 for Cosmos chains)
- `gas_price` (String) - Gas price (e.g., "0.025")

### Client

RPC client for blockchain interaction.

**Constructors:**

```ruby
# Create client without signer
client = Mob::Client.new(config)

# Create client with signer attached
client = Mob::Client.new_with_signer(config, signer)
```

**Methods:**

```ruby
client.get_height                        # => Integer - Latest block height
client.get_chain_id                      # => String - Chain ID
client.is_synced                         # => Boolean - Node sync status
client.get_account(address)              # => AccountInfo - Account information
client.get_balance(address, denom)       # => Coin - Balance for denom
client.get_tx(hash)                      # => TxResponse - Transaction by hash
client.send(to_address, amount, memo)    # => TxResponse - Send tokens (requires signer)
```

### Signer

Key management and signing functionality.

**Constructor:**

```ruby
# Create from mnemonic (positional arguments)
signer = Mob::Signer.from_mnemonic(
  mnemonic,        # String - 12 or 24 word mnemonic
  prefix,          # String - Address prefix (e.g., "xion")
  derivation_path  # String - BIP44 path (e.g., "m/44'/118'/0'/0/0")
)
```

**Methods:**

```ruby
signer.address                           # => String - Bech32 address
signer.public_key                        # => Array<Integer> - Public key bytes
signer.sign_bytes(message)               # => Array<Integer> - Signature bytes
```

### Types

**Coin:**
```ruby
coin = Mob::Coin.new(
  denom: "uxion",
  amount: "1000000"
)

coin.denom   # => String
coin.amount  # => String
```

**AccountInfo:**
```ruby
# Returned by client.get_account(address)
account.address         # => String
account.account_number  # => Integer
account.sequence        # => Integer
```

**TxResponse:**
```ruby
# Returned by client.send() and client.get_tx()
tx.txhash      # => String - Transaction hash
tx.code        # => Integer - Result code (0 = success)
tx.raw_log     # => String - Log output
tx.height      # => Integer - Block height
tx.gas_used    # => Integer - Gas consumed
tx.gas_wanted  # => Integer - Gas requested
```

## Project Structure

```
ruby/
├── lib/                    # Generated bindings
│   ├── mob.rb             # Ruby module (generated by UniFFI)
│   └── libmob.dylib       # Compiled Rust library
├── examples/              # Usage examples
│   ├── basic_query.rb
│   ├── account_query.rb
│   └── send_transaction.rb
├── test/                  # Test suite
│   └── test_rpc_queries.rb
├── scripts/               # Build scripts
│   └── generate_bindings.sh
├── QUICKSTART.md          # Quick start guide
└── README.md              # This file
```

## Configuration

All examples connect to the XION testnet by default:

- **Chain ID:** `xion-testnet-2`
- **RPC Endpoint:** `https://rpc.xion-testnet-2.burnt.com:443`
- **Address Prefix:** `xion`

To use a different network, modify the `ChainConfig` parameters accordingly.

## Security Notes

⚠️ **IMPORTANT SECURITY WARNINGS:**

1. **Never use production mnemonics in examples or tests**
2. **Never commit mnemonics to version control**
3. **Use environment variables for sensitive data:**

```ruby
mnemonic = ENV['TEST_MNEMONIC']
raise "TEST_MNEMONIC environment variable not set" if mnemonic.nil?
```

4. **The test mnemonic in these examples is for demonstration only**
5. **Keep your mainnet keys secure and separate from testnet keys**

## Troubleshooting

### Load Error: Cannot find library

**Cause:** The `libmob.dylib` file is not in the expected location.

**Solution:**
```bash
# Build and copy the library
cargo build --release -p mob
cp target/release/libmob.dylib ruby/lib/
# Or on Linux: cp target/release/libmob.so ruby/lib/
```

### "No such file or directory" Error

**Cause:** Running from wrong directory or incorrect require path.

**Solution:** Run examples from the `ruby/` directory or adjust the `require_relative` path.

### Network Connection Errors

**Possible causes:**
- RPC endpoint is down or unreachable
- Firewall blocking HTTPS connections

**Solution:** Verify the RPC endpoint is accessible:
```bash
curl https://rpc.xion-testnet-2.burnt.com:443/status
```

### Transaction Fails with Code != 0

**Common causes:**
- Insufficient gas
- Invalid recipient address
- Account sequence mismatch
- Insufficient balance

**Solution:** Check the `raw_log` field in the transaction response:
```ruby
if tx_response.code != 0
  puts "Error: #{tx_response.raw_log}"
end
```

## Development

### Regenerating Bindings

If you modify the Rust library, regenerate the Ruby bindings:

```bash
# From the project root
./ruby/scripts/generate_bindings.sh
```

This handles building, generating bindings, and fixing the library path automatically.

**Manual steps** (if you prefer not to use the script):

```bash
# From the project root
cargo build --release -p mob
cargo run --bin uniffi-bindgen generate \
  --library target/release/libmob.dylib \
  --language ruby \
  --out-dir ruby/lib
cp target/release/libmob.dylib ruby/lib/

# Fix library path (macOS)
sed -i '' "s/ffi_lib 'mob'/ffi_lib File.expand_path('libmob.dylib', __dir__)/" ruby/lib/mob.rb

# Fix library path (Linux)
sed -i "s/ffi_lib 'mob'/ffi_lib File.expand_path('libmob.dylib', __dir__)/" ruby/lib/mob.rb
```

## Requirements

- Ruby 2.6 or later
- Rust toolchain (for building from source)
- The compiled Rust library (libmob.dylib/so)
- Network access for RPC endpoints (in tests and examples)

## License

See the root LICENSE file in the main repository.

## Contributing

Contributions are welcome! Please see the main repository for contribution guidelines.

## Support

For issues and questions:
- Open an issue in the main repository
- Check the examples for usage patterns

## Related Projects

- [Mob Core](../core/) - Rust core library
- [Mob Python](../python/) - Python bindings
- [XION Blockchain](https://xion.burnt.com) - The XION blockchain
