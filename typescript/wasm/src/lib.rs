// WASM wrapper for mob library (cryptography only)
// This provides key management and signing without RPC client

use mob::*;
use wasm_bindgen::prelude::*;

// Export Signer for key management and signing
#[wasm_bindgen]
pub struct WasmSigner {
    inner: Signer,
}

#[wasm_bindgen]
impl WasmSigner {
    /// Create a signer from mnemonic
    #[wasm_bindgen(constructor)]
    pub fn new(mnemonic: String, address_prefix: String, derivation_path: String) -> std::result::Result<WasmSigner, String> {
        let signer = Signer::from_mnemonic(mnemonic, address_prefix, Some(derivation_path))
            .map_err(|e| format!("{:?}", e))?;

        Ok(WasmSigner { inner: signer })
    }

    /// Get the signer's address
    #[wasm_bindgen(js_name = address)]
    pub fn address(&self) -> String {
        self.inner.address()
    }

    /// Get the signer's public key as bytes
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> Vec<u8> {
        self.inner.public_key()
    }

    /// Get the signer's public key as hex string
    #[wasm_bindgen(js_name = publicKeyHex)]
    pub fn public_key_hex(&self) -> String {
        self.inner.public_key_hex()
    }

    /// Get the address prefix
    #[wasm_bindgen(js_name = addressPrefix)]
    pub fn address_prefix(&self) -> String {
        self.inner.address_prefix()
    }

    /// Sign arbitrary bytes
    #[wasm_bindgen(js_name = signBytes)]
    pub fn sign_bytes(&self, message: &[u8]) -> std::result::Result<Vec<u8>, String> {
        self.inner.sign_bytes(message.to_vec())
            .map_err(|e| format!("{:?}", e))
    }
}

// Re-export types as JS objects
#[wasm_bindgen]
pub struct WasmCoin {
    denom: String,
    amount: String,
}

#[wasm_bindgen]
impl WasmCoin {
    #[wasm_bindgen(constructor)]
    pub fn new(denom: String, amount: String) -> WasmCoin {
        WasmCoin { denom, amount }
    }

    #[wasm_bindgen(getter)]
    pub fn denom(&self) -> String {
        self.denom.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> String {
        self.amount.clone()
    }
}

impl From<WasmCoin> for Coin {
    fn from(coin: WasmCoin) -> Self {
        Coin::new(&coin.denom, &coin.amount)
    }
}

#[wasm_bindgen]
pub struct WasmChainConfig {
    chain_id: String,
    rpc_endpoint: String,
    address_prefix: String,
}

#[wasm_bindgen]
impl WasmChainConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(chain_id: String, rpc_endpoint: String, address_prefix: String) -> WasmChainConfig {
        WasmChainConfig {
            chain_id,
            rpc_endpoint,
            address_prefix,
        }
    }

    #[wasm_bindgen(getter, js_name = chainId)]
    pub fn chain_id(&self) -> String {
        self.chain_id.clone()
    }

    #[wasm_bindgen(getter, js_name = rpcEndpoint)]
    pub fn rpc_endpoint(&self) -> String {
        self.rpc_endpoint.clone()
    }

    #[wasm_bindgen(getter, js_name = addressPrefix)]
    pub fn address_prefix(&self) -> String {
        self.address_prefix.clone()
    }
}

impl From<WasmChainConfig> for ChainConfig {
    fn from(config: WasmChainConfig) -> Self {
        ChainConfig::new(&config.chain_id, &config.rpc_endpoint, &config.address_prefix)
    }
}

// Note: RPC client functionality (Client) is not available in WASM
// JavaScript applications should use native fetch() for RPC calls
// and only use this library for cryptographic operations (signing)
