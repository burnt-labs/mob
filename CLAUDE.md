# Mob - Multi-Platform Signing Client for XION

## Communication Style

Maintain a professional, technical tone in all interactions:

- Use direct, precise language without unnecessary embellishments
- Avoid emojis, exclamation marks, and enthusiastic language
- State facts and outcomes objectively
- Present information concisely without preamble or postamble
- Focus on technical accuracy over motivational language
- Report status using neutral terminology (e.g., "completed" not "done! ✓")

## Project Overview

Mob is a Rust-based signing client library for the XION blockchain that provides UniFFI bindings for multiple programming languages. The core library implements cryptographic operations, transaction building, and RPC communication.

## Architecture

```
mob/
├── core/           # Rust library with UniFFI exports
├── python/         # Python bindings and tests
├── kotlin/         # Kotlin bindings and tests
├── swift/          # Swift bindings and tests
├── ruby/           # Ruby bindings and tests
└── bindings/       # Generated output directory (gitignored)
```

## Technology Stack

**Core Library:**
- Rust (edition 2021)
- UniFFI 0.30 for cross-language bindings
- cosmos-sdk-proto (xion-cosmos-sdk-proto 0.26.1)
- cosmrs 0.22 for Cosmos SDK integration
- Optional features: rpc-client, uniffi-bindings, rust-signer

**Cryptography:**
- bip32, k256, ripemd, sha2 (optional via rust-signer feature)
- secp256k1 ECDSA signatures
- BIP39/BIP32 hierarchical deterministic key derivation

**Language Bindings:**
- Python (maturin)
- Kotlin (Gradle)
- Swift (Swift Package Manager)
- Ruby (FFI)

## Development Workflow

### Building

```bash
# Build Rust core
cargo build --release -p mob

# Generate bindings for all languages
./scripts/generate_bindings.sh

# Language-specific builds
cd python && maturin develop --release
cd kotlin && ./gradlew build
cd swift && swift build
cd ruby && ruby -I lib examples/basic_query.rb
```

### Testing

```bash
# Rust tests (lib, unit, integration, doc)
cargo test -p mob

# Language-specific tests
cd python && pytest
cd kotlin && ./gradlew test
cd swift && swift test
cd ruby && ruby test/test_rpc_queries.rb
```

### Feature Flags

- `default`: Includes rpc-client, uniffi-bindings, rust-signer
- `rpc-client`: Tokio runtime, RPC/gRPC support
- `uniffi-bindings`: Cross-language FFI generation
- `rust-signer`: Pure Rust cryptography implementation
- `wasm`: Minimal build without async runtime

## Code Organization

### Core Modules

- `crypto_signer.rs`: Trait definition for pluggable signers
- `rust_signer.rs`: Pure Rust cryptographic implementation
- `session_signer.rs`: Authz-wrapped session key support
- `client.rs`: RPC client for chain interaction
- `transaction.rs`: Transaction building and signing
- `account.rs`: Account state management

### Generated Artifacts (Do Not Commit)

All files matching these patterns are build artifacts:
- `python/mob/mob/mob.py`, `*.dylib`, `*.so`
- `kotlin/lib/libmob.dylib`, `lib/uniffi/mob/mob.kt`
- `ruby/lib/mob.rb`, `lib/libmob.dylib`
- `swift/lib/mob.swift`, `lib/mobFFI.*`, `lib/libmob.dylib`
- `kotlin/.gradle/`, `kotlin/build/`
- `swift/.build/`

## Dependencies

When updating dependencies, verify compatibility across:
1. Rust core library compilation
2. UniFFI binding generation
3. All four language binding tests
4. Example code in each language directory

## Testing Requirements

Before submitting changes:
1. All Rust tests pass (cargo test -p mob)
2. All language binding tests pass
3. Examples compile and run without errors
4. No generated files included in commits

## Documentation

Technical documentation should be:
- Factual and implementation-focused
- Free of subjective assessments or enthusiasm
- Structured with clear headings and code examples
- Concise without redundant explanations
