# Mob Kotlin Bindings

Kotlin bindings for the mob library - a multi-platform signing and transaction client for XION blockchain.

## Status

✅ **Available** - Kotlin bindings are generated via UniFFI and ready to use with Gradle.

## Features

- 🔐 **Key Management**: Create signers from mnemonics with BIP39/BIP44 support
- 📡 **RPC Queries**: Query blockchain state (height, balances, accounts, sync status)
- 💸 **Transactions**: Send tokens with memo support
- 🔏 **Message Signing**: Sign arbitrary messages with private keys
- ⚡ **Synchronous API**: Simple blocking calls, no coroutine complexity

## Installation

### Prerequisites

- JDK 11 or higher
- Gradle 7.0+
- Kotlin 1.9+

### Building the Library

1. **Build the Rust library**:
```bash
cd /Users/mv/Development/burnt/mob
cargo build --release -p mob
```

2. **Generate Kotlin bindings**:
```bash
cargo run --bin uniffi-bindgen generate \
  --library target/release/libmob.dylib \
  --language kotlin \
  --out-dir kotlin/lib

cp target/release/libmob.dylib kotlin/lib/
```

Or use the automated script:
```bash
./kotlin/scripts/generate_bindings.sh
```

3. **Build the Kotlin project**:
```bash
cd kotlin
./gradlew build
```

## Running Tests

### Unit Tests

Run the full test suite (excludes integration tests):
```bash
cd kotlin
./gradlew test
```

### Integration Tests

Integration tests perform real transactions on XION testnet-2 and require:
- Network connectivity
- Funded test account

Run with the `INTEGRATION=1` environment variable:
```bash
INTEGRATION=1 ./gradlew test
```

The test account needs at least 6000 uxion (1000 to send + ~5000 for gas).

## Quick Start

See [QUICKSTART.md](QUICKSTART.md) for usage examples.

## API Overview

### Core Types

- **`ChainConfig`**: Blockchain configuration (chain ID, RPC endpoint, etc.)
- **`Signer`**: Key management and signing operations
- **`Client`**: RPC client for queries and transactions
- **`Coin`**: Token amount representation

### Creating a Client

```kotlin
val config = ChainConfig(
    chainId = "xion-testnet-2",
    rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint = null,
    addressPrefix = "xion",
    coinType = 118u,
    gasPrice = "0.025"
)

val client = Client(config)
```

### Creating a Signer

```kotlin
val signer = Signer.fromMnemonic(
    mnemonic = "your twelve or twenty-four word mnemonic here",
    addressPrefix = "xion",
    derivationPath = "m/44'/118'/0'/0/0"
)

val address = signer.address()
```

### Sending Transactions

```kotlin
val client = Client.newWithSigner(config, signer)

val amount = listOf(Coin(denom = "uxion", amount = "1000"))
val txResponse = client.send(
    toAddress = "xion1recipient...",
    amount = amount,
    memo = "Test transaction"
)

println("Transaction hash: ${txResponse.txhash}")
```

## Project Structure

```
kotlin/
├── lib/
│   ├── uniffi/mob/mob.kt    # Generated Kotlin bindings
│   └── libmob.dylib          # Native library
├── src/
│   ├── main/kotlin/          # (Empty - using generated code)
│   └── test/kotlin/com/burnt/mob/
│       ├── MobTest.kt        # Unit tests
│       └── IntegrationTest.kt # Integration tests
├── examples/                 # Usage examples
├── build.gradle.kts          # Gradle build configuration
├── settings.gradle.kts       # Gradle settings
└── README.md                 # This file
```

## Gradle Configuration

The `build.gradle.kts` file is configured to:
- Use Kotlin JVM plugin (1.9.22)
- Include JNA (Java Native Access) for native library loading
- Use JUnit 5 for testing
- Set `java.library.path` to find the native library
- Exclude integration tests by default (run with `INTEGRATION=1`)

Add to your own project:
```kotlin
dependencies {
    implementation("net.java.dev.jna:jna:5.14.0")
}

sourceSets {
    main {
        kotlin { srcDir("path/to/kotlin/lib/uniffi") }
        resources { srcDir("path/to/kotlin/lib") }
    }
}
```

## Troubleshooting

### Library Loading Issues

If you see `java.lang.UnsatisfiedLinkError: Unable to load library 'mob'`:

1. Verify the library exists:
```bash
ls -la kotlin/lib/libmob.dylib
```

2. Check the library is valid:
```bash
file kotlin/lib/libmob.dylib
otool -L kotlin/lib/libmob.dylib
```

3. Ensure Gradle is configured with the correct library path:
```kotlin
tasks.test {
    systemProperty("java.library.path", "${projectDir}/lib")
}
```

### Integration Test Failures

If integration tests fail:

1. Check testnet connectivity:
```bash
curl https://rpc.xion-testnet-2.burnt.com:443/status
```

2. Verify the test account has funds:
```kotlin
val balance = client.getBalance(address, "uxion")
println("Balance: ${balance.amount} uxion")
```

3. Fund the test account if needed (address from test mnemonic):
```
xion1sxu85s77uf6r0rydud7jx6xvygn8cdu3gns84q
```

### Build Issues

If `./gradlew build` fails:

1. Ensure the Rust library is built:
```bash
cd /Users/mv/Development/burnt/mob
cargo build --release -p mob
```

2. Regenerate bindings:
```bash
./kotlin/scripts/generate_bindings.sh
```

3. Clean and rebuild:
```bash
cd kotlin
./gradlew clean build
```

## Examples

See the `examples/` directory for complete working examples:
- `BasicQuery.kt` - Simple blockchain queries
- `SendTransaction.kt` - Complete transaction flow

## Documentation

- [Quick Start Guide](QUICKSTART.md) - Usage examples and common patterns
- [Python Bindings](../PYTHON_BINDINGS.md) - Python-specific documentation
- [Ruby Documentation](../ruby/README.md) - Ruby bindings
- [Swift Documentation](../swift/README.md) - Swift bindings

## License

MIT - See LICENSE file for details.

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/burnt-labs/mob/issues
- Repository: https://github.com/burnt-labs/mob
