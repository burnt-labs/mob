# TypeScript Bindings Status

## Current State

✅ **Project structure created**
✅ **Configuration files completed**
✅ **TypeScript bindings generated** (`src/generated/mob.ts`)
❌ **WASM compilation blocked** (see limitations below)

## What Works

1. **uniffi-bindgen-react-native** is installed and configured
2. TypeScript bindings have been generated from the Rust library
3. Project structure is complete with:
   - `package.json` with build scripts
   - `ubrn.config.yaml` for uniffi-bindgen-react-native
   - `tsconfig.json` and `jest.config.js` for TypeScript/testing
   - Generated TypeScript types in `src/generated/mob.ts`

## Blocking Issue: WASM Compilation

The WASM compilation fails because the `mob` library has dependencies that are **not compatible with `wasm32-unknown-unknown` target**:

### Root Cause

The mob library uses:
- **tokio** async runtime (for async blockchain operations)
- **mio** (tokio's dependency for OS-level I/O)
- **tendermint-rpc** with HTTP client (for RPC calls)

These crates depend on platform-specific features (file descriptors, sockets, threads) that don't exist in WASM.

### Specific Error

```
error[E0433]: failed to resolve: could not find `sys` in `net`
 --> mio-1.1.1/src/net/udp.rs:7:17
  |
7 | use crate::net::sys;
  |                 ^^^ could not find `sys` in `net`
```

This occurs because `mio` expects platform-specific system calls that don't exist in WASM.

## Solution Options

### Option 1: Refactor mob Core Library (Recommended)

Create a WASM-compatible version of the mob library:

**Required Changes:**
1. **Feature flag for WASM**: Add a `wasm` feature that disables tokio
2. **Async-only API for WASM**: Remove `block_on()` wrappers, make all methods truly async
3. **WASM-compatible HTTP**: Use `reqwest` with `wasm` feature or `gloo-net`
4. **No blocking operations**: All RPC calls must be async

**Example Cargo.toml changes:**
```toml
[features]
default = ["tokio"]
wasm = ["reqwest/wasm"]

[dependencies]
tokio = { version = "1", optional = true }
reqwest = { version = "0.11", features = ["json"], default-features = false }
```

**Benefits:**
- Clean separation of native and WASM targets
- Proper async/await API for JavaScript
- Full feature parity

**Effort:** Medium (1-2 days)

### Option 2: Create a Thin WASM Wrapper

Create a separate `mob-wasm` crate that re-implements only the essential functionality:

1. Key generation and signing (works in WASM)
2. Transaction building (works in WASM)
3. RPC calls delegated to JavaScript `fetch()`

**Benefits:**
- Smaller WASM bundle
- Faster to implement
- Leverages browser's native HTTP

**Drawbacks:**
- Duplicate code
- Limited functionality
- Maintenance burden

**Effort:** Low (1 day)

### Option 3: Use Python/Ruby Bindings via Bridge

For Node.js only, call Python or Ruby bindings via child_process:

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

async function getBalance(address: string): Promise<string> {
  const script = `
    from mob import Client, ChainConfig
    config = ChainConfig(...)
    client = Client(config)
    balance = client.get_balance("${address}", "uxion")
    print(balance.amount)
  `;
  const { stdout } = await execAsync(`python3 -c "${script}"`);
  return stdout.trim();
}
```

**Benefits:**
- Works immediately
- No Rust changes needed

**Drawbacks:**
- Node.js only (no browser support)
- Requires Python/Ruby installed
- Performance overhead
- Complex error handling

**Effort:** Very Low (few hours)

### Option 4: React Native Only (No WASM)

Focus on React Native Turbo Modules for iOS/Android only:

```bash
uniffi-bindgen-react-native build ios --and-generate
uniffi-bindgen-react-native build android --and-generate
```

**Benefits:**
- Works with current mob implementation
- Native performance
- Full feature support

**Drawbacks:**
- Mobile-only (no web/Node.js)
- Requires iOS/Android development setup

**Effort:** Medium (requires mobile setup)

## Recommended Path Forward

**For Web/Node.js**: Implement **Option 1** (Refactor for WASM)

This provides the best long-term solution with full feature parity across all platforms.

**Immediate workaround**: Use **Option 3** (Python/Ruby bridge) for Node.js development until Option 1 is complete.

## Files Generated

```
typescript/
├── package.json                    # NPM configuration ✅
├── ubrn.config.yaml               # uniffi-bindgen-react-native config ✅
├── tsconfig.json                  # TypeScript config ✅
├── jest.config.js                 # Jest test config ✅
├── src/
│   └── generated/
│       └── mob.ts                 # Generated TypeScript bindings ✅
├── wasm/
│   ├── Cargo.toml                 # WASM crate config ✅
│   ├── build.rs                   # Build script ✅
│   └── src/
│       ├── lib.rs                 # WASM entry point ✅
│       └── mob_module.rs          # UniFFI module ✅
├── cpp/                           # C++ bindings (for React Native) ✅
└── README.md                      # Documentation ✅
```

## Next Steps

To complete the TypeScript bindings, choose one of the options above and implement it.

For **Option 1** (recommended), the mob core library needs these changes:

1. Add WASM feature flag to `core/Cargo.toml`
2. Make RPC client WASM-compatible (use `reqwest` with `wasm` feature)
3. Remove `tokio::runtime::block_on()` for WASM target
4. Create async-only API for WASM
5. Update `uniffi` scaffolding to support async methods

## Contact

For questions about implementing WASM support:
- GitHub Issues: https://github.com/burnt-labs/mob/issues
- uniffi-bindgen-react-native: https://github.com/jhugman/uniffi-bindgen-react-native
