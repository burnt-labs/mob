# Mob Swift Bindings

Swift bindings for the Mob library - a multi-platform signing client for the XION blockchain.

## Status

✅ **Swift bindings are fully functional!** The library is packaged as an XCFramework and works with Swift Package Manager.

### Completed

- ✅ UniFFI bindings generated successfully
- ✅ All Swift APIs defined (Client, Signer, types)
- ✅ Compatible with the same API as Ruby/Python
- ✅ XCFramework created for macOS (arm64)
- ✅ Package.swift configured for SPM
- ✅ Basic test suite included

### Future Enhancements

- ⏳ Multi-architecture support (x86_64, iOS, iOS Simulator)
- ⏳ Comprehensive test suite with integration tests
- ⏳ CI/CD for automated XCFramework builds

## Overview

Mob provides a comprehensive Swift interface for interacting with the XION blockchain, including:

- 🔐 **Key Management** - Mnemonic-based key derivation and private key management
- 📝 **Transaction Building** - Intuitive API for building and signing transactions
- 🌐 **RPC Client** - Full-featured client for interacting with XION nodes
- 🔄 **Account Abstraction** - Support for XION's account abstraction features
- 🦀 **Pure Rust Core** - High-performance core logic with Swift bindings via UniFFI

## Installation

### Prerequisites

- Swift 5.9 or later
- Xcode 15.0+ (for macOS/iOS development)
- Rust toolchain (for building from source)

### Building from Source

```bash
# From the project root directory
./swift/scripts/generate_bindings.sh
```

This script will:
1. Build the Rust library for macOS arm64
2. Generate Swift bindings using UniFFI
3. Create an XCFramework bundle
4. Set up the Swift Package Manager structure

### Adding to Your Project

#### Swift Package Manager

Add the package dependency to your `Package.swift`:

```swift
dependencies: [
    .package(path: "../mob/swift")
]
```

Or clone and add as a local package in Xcode.

## Usage

See [QUICKSTART.md](QUICKSTART.md) for quick start examples and common usage patterns.

The `examples/` directory contains working sample code.

## Running Tests

The test suite includes comprehensive coverage matching Ruby and Python bindings:

**Test Coverage:**
- ✅ Client creation
- ✅ RPC queries (height, chain ID, sync status)
- ✅ Signer creation and key management
- ✅ Account queries
- ✅ Balance queries
- ✅ Message signing
- ✅ Error handling (invalid mnemonics, addresses)
- ✅ Multiple signer derivation paths
- ✅ Coin creation
- ✅ Integration test for sending funds on testnet

Run the test suite using Swift Package Manager:

```bash
# From the swift/ directory
swift test

# With verbose output
swift test --verbose
```

### Integration Tests

Integration tests interact with the live XION testnet and are skipped by default. To run them:

```bash
# Run all tests including integration tests
INTEGRATION=1 swift test

# With verbose output
INTEGRATION=1 swift test --verbose
```

**Note:** Integration tests require a funded test account. The test mnemonic must have sufficient uxion balance for transaction fees and token transfers.

## API Reference

### ChainConfig

Configuration for connecting to a blockchain network.

```swift
let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)
```

**Parameters:**
- `chainId` (String) - Chain identifier (e.g., "xion-testnet-2")
- `rpcEndpoint` (String) - RPC endpoint URL
- `grpcEndpoint` (String?, optional) - gRPC endpoint URL (can be nil)
- `addressPrefix` (String) - Bech32 address prefix (e.g., "xion")
- `coinType` (UInt32) - BIP44 coin type (118 for Cosmos chains)
- `gasPrice` (String) - Gas price (e.g., "0.025")

### Client

RPC client for blockchain interaction.

**Constructors:**

```swift
// Create client without signer
let client = try Client(config: config)

// Create client with signer attached
let client = try Client.newWithSigner(config: config, signer: signer)
```

**Methods:**

```swift
try client.getHeight()                              // -> UInt64 - Latest block height
try client.getChainId()                             // -> String - Chain ID
try client.isSynced()                               // -> Bool - Node sync status
try client.getAccount(address: address)             // -> AccountInfo - Account information
try client.getBalance(address: address, denom: denom) // -> Coin - Balance for denom
try client.getTx(hash: hash)                        // -> TxResponse - Transaction by hash
try client.send(toAddress: address, amount: amount, memo: memo) // -> TxResponse - Send tokens
```

### Signer

Key management and signing functionality.

**Constructor:**

```swift
// Create from mnemonic
let signer = try Signer.fromMnemonic(
    mnemonic: mnemonic,     // String - 12 or 24 word mnemonic
    prefix: prefix,         // String - Address prefix (e.g., "xion")
    derivationPath: path    // String - BIP44 path (e.g., "m/44'/118'/0'/0/0")
)
```

**Methods:**

```swift
signer.address()                    // -> String - Bech32 address
signer.publicKey()                  // -> [UInt8] - Public key bytes
signer.signBytes(message: message)  // -> [UInt8] - Signature bytes
```

### Types

**Coin:**
```swift
let coin = Coin(denom: "uxion", amount: "1000000")

coin.denom   // String
coin.amount  // String
```

**AccountInfo:**
```swift
// Returned by client.getAccount(address:)
account.address         // String
account.accountNumber   // UInt64
account.sequence        // UInt64
```

**TxResponse:**
```swift
// Returned by client.send() and client.getTx()
tx.txhash      // String - Transaction hash
tx.code        // UInt32 - Result code (0 = success)
tx.rawLog      // String - Log output
tx.height      // UInt64 - Block height
tx.gasUsed     // UInt64 - Gas consumed
tx.gasWanted   // UInt64 - Gas requested
```

## Project Structure

```
swift/
├── lib/                    # Generated bindings
│   ├── mob.swift          # Swift module (generated by UniFFI)
│   ├── mobFFI.h           # C header for FFI
│   ├── mobFFI.modulemap   # Module map
│   └── libmob.dylib       # Compiled Rust library
├── examples/              # Usage examples
│   ├── BasicQuery.swift
│   └── SendTransaction.swift
├── tests/                 # Test suite
│   └── MobTests.swift
├── scripts/               # Build scripts
│   └── generate_bindings.sh
├── Package.swift          # Swift Package Manager manifest
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

```swift
guard let mnemonic = ProcessInfo.processInfo.environment["TEST_MNEMONIC"] else {
    fatalError("TEST_MNEMONIC environment variable not set")
}
```

4. **The test mnemonic in these examples is for demonstration only**
5. **Keep your mainnet keys secure and separate from testnet keys**

## Troubleshooting

### Build Errors

**Cause:** Missing or outdated Rust library.

**Solution:**
```bash
# Regenerate bindings
./swift/scripts/generate_bindings.sh
```

### Library Not Found

**Cause:** The `libmob.dylib` file is not in the expected location.

**Solution:**
```bash
# Build and copy the library
cargo build --release -p mob
cp target/release/libmob.dylib swift/lib/
```

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

**Solution:** Check the `rawLog` field in the transaction response:
```swift
if txResponse.code != 0 {
    print("Error: \(txResponse.rawLog)")
}
```

## Development

### Regenerating Bindings

If you modify the Rust library, regenerate the Swift bindings:

```bash
# From the project root
./swift/scripts/generate_bindings.sh
```

This handles building, generating bindings, and copying the library automatically.

### Building the Package

```bash
cd swift
swift build
```

### Running Tests

```bash
cd swift
swift test
```

## Requirements

- Swift 5.9 or later
- macOS 13.0+ or iOS 16.0+
- Rust toolchain (for building from source)
- The compiled Rust library (libmob.dylib)
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
- [Mob Ruby](../ruby/) - Ruby bindings
- [Mob Python](../python/) - Python bindings
- [XION Blockchain](https://xion.burnt.com) - The XION blockchain
