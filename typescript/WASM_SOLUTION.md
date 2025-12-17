# WASM Support Solution

## Problem Summary

The mob library can't compile to WASM because:
- It uses `tokio::runtime::block_on()` to wrap async operations
- Tokio requires OS threads and I/O that don't exist in WASM
- The RPC client (`tendermint-rpc`) expects tokio runtime

## Solution: Conditional Async API

Make the mob library support both:
1. **Synchronous FFI** (for Python, Ruby, Kotlin, Swift) - using `block_on()`
2. **Async API** (for TypeScript/WASM) - returning Futures directly

UniFFI v0.30 supports async methods! We can use `#[uniffi::export(async_runtime = "tokio")]` for native targets and remove the `block_on()` wrappers for WASM.

## Implementation Steps

### Step 1: Add WASM feature flag

**`core/Cargo.toml`:**
```toml
[features]
default = ["sync-api"]
sync-api = ["tokio"]  # For Python, Ruby, Kotlin, Swift
wasm = []             # For TypeScript/WASM

[dependencies]
tokio = { version = "1", features = ["full"], optional = true }
# ... rest of dependencies
```

### Step 2: Create conditional API in client.rs

**`core/src/client.rs`:**
```rust
#[uniffi::export]
impl Client {
    // For native targets: synchronous API with block_on()
    #[cfg(feature = "sync-api")]
    #[uniffi::constructor]
    pub fn new(config: ChainConfig) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(Self::new_async(config))
    }

    // For WASM: async API
    #[cfg(feature = "wasm")]
    #[uniffi::constructor]
    pub async fn new(config: ChainConfig) -> Result<Self> {
        Self::new_async(config).await
    }

    // Similar pattern for all other methods...

    #[cfg(feature = "sync-api")]
    pub fn get_account(&self, address: String) -> Result<AccountInfo> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_account_internal(&address))
    }

    #[cfg(feature = "wasm")]
    pub async fn get_account(&self, address: String) -> Result<AccountInfo> {
        self.get_account_internal(&address).await
    }
}
```

### Step 3: Make RPC client WASM-compatible

The `tendermint-rpc` crate with `http-client` feature doesn't work in WASM. We need to:

**Option A: Use `reqwest` with WASM feature**
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"], default-features = false }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
reqwest = { version = "0.11", features = ["json", "native-tls"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
reqwest = { version = "0.11", features = ["json"], default-features = false }
```

**Option B: Create custom HTTP client abstraction**
```rust
#[cfg(not(target_arch = "wasm32"))]
async fn http_post(url: &str, body: &str) -> Result<String> {
    // Use tendermint-rpc HttpClient
}

#[cfg(target_arch = "wasm32")]
async fn http_post(url: &str, body: &str) -> Result<String> {
    // Use reqwest with wasm feature or gloo-net
}
```

### Step 4: Update WASM Cargo.toml

**`typescript/wasm/Cargo.toml`:**
```toml
[dependencies]
mob = { path = "../../core", features = ["wasm"], default-features = false }
uniffi-runtime-javascript = { version = "=0.29.3-1", features = ["wasm32"] }
wasm-bindgen = "*"
wasm-bindgen-futures = "0.4"  # For async support
```

### Step 5: Update TypeScript usage (async)

The generated TypeScript will have async methods:

```typescript
// All methods return Promises
const client = await Client.new(config);
const height = await client.getHeight();
const balance = await client.getBalance(address, 'uxion');
```

## Benefits

✅ **No code duplication** - same Rust codebase for all targets
✅ **Proper async in TypeScript** - natural JavaScript/TypeScript API
✅ **Sync API maintained** - Python, Ruby, Kotlin, Swift keep working
✅ **WASM compatible** - no tokio runtime needed in browser

## Alternative: Simpler Approach

If modifying the core library is not desired, create a thin WASM-specific wrapper:

**`typescript/wasm/src/lib.rs`:**
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmClient {
    // Minimal state - just config
}

#[wasm_bindgen]
impl WasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new(chain_id: String, rpc_endpoint: String) -> Self {
        // Initialize without mob::Client
    }

    #[wasm_bindgen]
    pub async fn get_height(&self) -> Result<u64, JsValue> {
        // Direct HTTP call using reqwest/gloo-net
        // Implement only essential functionality
    }
}
```

This avoids modifying the core library but requires implementing RPC calls twice.

## Recommended Approach

**Use Step 1-5** to properly support WASM with feature flags. This is the cleanest long-term solution that maintains code in one place while supporting all targets.

The key insight: **Don't force synchronous API on WASM**. JavaScript/TypeScript naturally expects async, so let the API be async for WASM targets.

## Next Steps

1. Add `wasm` and `sync-api` features to `core/Cargo.toml`
2. Update `client.rs` with conditional compilation
3. Replace `tendermint-rpc` HTTP client with WASM-compatible alternative
4. Test WASM compilation: `cd typescript/wasm && cargo build --target wasm32-unknown-unknown --release`
5. Generate TypeScript bindings and verify async API works

## References

- [UniFFI Async Support](https://mozilla.github.io/uniffi-rs/latest/internals/async-overview.html)
- [reqwest WASM support](https://docs.rs/reqwest/latest/reqwest/#wasm)
- [wasm-bindgen futures](https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html)
