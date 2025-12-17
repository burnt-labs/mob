# Mob TypeScript/React Native Bindings

TypeScript/JavaScript bindings for the mob library - a multi-platform signing and transaction client for XION blockchain.

## Status

🚧 **In Development** - TypeScript bindings are being generated using [uniffi-bindgen-react-native](https://github.com/jhugman/uniffi-bindgen-react-native).

## Overview

This package provides TypeScript bindings for the mob Rust library using `uniffi-bindgen-react-native`, which generates:
- **TypeScript bindings** for type-safe JavaScript/TypeScript usage
- **WASM module** for web browsers and Node.js
- **React Native Turbo Modules** for iOS and Android (future)

## Features

- 🔐 **Key Management**: Create signers from mnemonics with BIP39/BIP44 support
- 📡 **RPC Queries**: Query blockchain state (height, balances, accounts, sync status)
- 💸 **Transactions**: Send tokens with memo support
- 🔏 **Message Signing**: Sign arbitrary messages with private keys
- 🌐 **Cross-Platform**: Works in Node.js, browsers (via WASM), and React Native

## Installation

```bash
npm install @burnt/mob
# or
yarn add @burnt/mob
```

## Prerequisites

For development and building from source:

- Node.js 18+ and npm/yarn
- Rust toolchain with `wasm32-unknown-unknown` target
- `wasm-bindgen-cli` installed: `cargo install wasm-bindgen-cli`
- `uniffi-bindgen-react-native` installed: `cargo install uniffi-bindgen-react-native --git https://github.com/jhugman/uniffi-bindgen-react-native --locked`

Install WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

## Building

### Generate TypeScript Bindings and WASM

```bash
# Generate WASM crate structure
uniffi-bindgen-react-native generate wasm wasm-crate --config ubrn.config.yaml mob

# Build WASM module
npm run build:wasm

# Generate TypeScript bindings from WASM
npm run bindgen:wasm

# Or run all steps:
npm run ubrn:web
```

### Build TypeScript

```bash
npm run build
```

## Usage

### Basic Setup

```typescript
import { ChainConfig, Client, Signer } from '@burnt/mob';

// Configure chain connection
const config: ChainConfig = {
  chainId: 'xion-testnet-2',
  rpcEndpoint: 'https://rpc.xion-testnet-2.burnt.com:443',
  grpcEndpoint: null,
  addressPrefix: 'xion',
  coinType: 118,
  gasPrice: '0.025'
};

// Create a read-only client
const client = new Client(config);
```

### Query Blockchain

```typescript
// Get current height
const height = await client.getHeight();
console.log('Height:', height);

// Check balance
const balance = await client.getBalance(address, 'uxion');
console.log('Balance:', balance.amount, balance.denom);

// Get account info
const account = await client.getAccount(address);
console.log('Account number:', account.accountNumber);
```

### Create Signer and Send Transaction

```typescript
// Create signer from mnemonic
const signer = Signer.fromMnemonic(
  'your twelve or twenty-four word mnemonic here',
  'xion',
  "m/44'/118'/0'/0/0"
);

const address = signer.address();
console.log('Address:', address);

// Create client with signer
const client = Client.newWithSigner(config, signer);

// Send transaction
const amount = [{ denom: 'uxion', amount: '1000' }];
const txResponse = await client.send(
  'xion1recipient...',
  amount,
  'Payment memo'
);

console.log('TX hash:', txResponse.txhash);
console.log('Code:', txResponse.code); // 0 = success
```

### Sign Messages

```typescript
const message = new TextEncoder().encode('Hello, XION!');
const signature = signer.signBytes(message);
console.log('Signature:', signature);
```

## Project Structure

```
typescript/
├── src/
│   ├── generated/       # Generated TypeScript bindings
│   ├── index.ts         # Main entry point
│   └── *.test.ts        # Test files
├── wasm/
│   ├── Cargo.toml       # WASM crate configuration
│   ├── src/lib.rs       # WASM library entry point
│   └── build.rs         # Build script
├── examples/            # Usage examples
├── ubrn.config.yaml     # uniffi-bindgen-react-native configuration
├── package.json         # NPM configuration
├── tsconfig.json        # TypeScript configuration
└── README.md            # This file
```

## Configuration

The `ubrn.config.yaml` file configures binding generation:

```yaml
rust:
  directory: ../core
  manifestPath: ../core/Cargo.toml

bindings:
  ts: ./src/generated

web:
  manifestPath: ./wasm/Cargo.toml
  wasmCrateName: mob-wasm
  target: nodejs
  tsBindings: ./src/generated
```

## Testing

```bash
# Run unit tests
npm test

# Run integration tests (requires funded testnet account)
npm run test:integration
```

## Limitations

Current limitations of using `uniffi-bindgen-react-native`:

1. **Async Operations**: All blockchain operations are async in TypeScript (unlike Ruby/Python/Kotlin bindings which are synchronous)
2. **WASM Size**: The WASM bundle includes the entire Rust library (~2-3 MB)
3. **React Native**: iOS/Android Turbo Module support is planned but not yet generated
4. **Node.js Target**: Currently optimized for Node.js; browser support requires additional configuration

## Development

### Regenerate Bindings

When the Rust library changes:

```bash
# Regenerate WASM crate
uniffi-bindgen-react-native generate wasm wasm-crate --config ubrn.config.yaml mob

# Rebuild and regenerate bindings
npm run ubrn:web
```

### Clean Build

```bash
npm run clean
npm run ubrn:web
npm run build
```

## Troubleshooting

### WASM Module Not Found

If you see "Cannot find module" errors:

1. Ensure WASM is built: `npm run build:wasm`
2. Generate bindings: `npm run bindgen:wasm`
3. Check `src/generated/` directory exists

### Build Failures

If Rust build fails:

```bash
# Clean Rust build
cd wasm && cargo clean

# Rebuild
npm run build:wasm
```

### TypeScript Errors

If TypeScript compilation fails:

```bash
# Clean generated files
rm -rf src/generated dist

# Regenerate everything
npm run ubrn:web
npm run build
```

## Related Documentation

- [Python Bindings](../PYTHON_BINDINGS.md) - Python implementation
- [Ruby Bindings](../ruby/README.md) - Ruby implementation
- [Kotlin Bindings](../kotlin/README.md) - Kotlin/JVM implementation
- [Swift Bindings](../swift/README.md) - Swift/iOS implementation
- [uniffi-bindgen-react-native](https://jhugman.github.io/uniffi-bindgen-react-native/) - Binding generator documentation
- [UniFFI Documentation](https://mozilla.github.io/uniffi-rs/) - Core UniFFI framework

## Contributing

To improve the TypeScript bindings:

1. Update the Rust code in `../core/`
2. Regenerate bindings: `npm run ubrn:web`
3. Update tests and examples
4. Run tests: `npm test`
5. Submit a pull request

## Support

For issues or questions:
- GitHub Issues: https://github.com/burnt-labs/mob/issues
- Repository: https://github.com/burnt-labs/mob
- Discord: https://discord.gg/burnt

## License

MIT - See LICENSE file for details.
