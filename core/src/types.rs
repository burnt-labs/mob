use serde::{Deserialize, Serialize};

/// Represents a coin amount with denomination
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, uniffi::Record)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

impl Coin {
    pub fn new(denom: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            denom: denom.into(),
            amount: amount.into(),
        }
    }
}

impl From<Coin> for cosmrs::Coin {
    fn from(coin: Coin) -> Self {
        cosmrs::Coin {
            denom: coin.denom.parse().expect("invalid denom"),
            amount: coin.amount.parse().expect("invalid amount"),
        }
    }
}

impl From<cosmrs::Coin> for Coin {
    fn from(coin: cosmrs::Coin) -> Self {
        Coin {
            denom: coin.denom.to_string(),
            amount: coin.amount.to_string(),
        }
    }
}

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct AccountInfo {
    pub address: String,
    pub account_number: u64,
    pub sequence: u64,
    pub pub_key: Option<String>,
}

/// Transaction fee information
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct Fee {
    pub amount: Vec<Coin>,
    pub gas_limit: u64,
    pub payer: Option<String>,
    pub granter: Option<String>,
}

impl Fee {
    pub fn new(amount: Vec<Coin>, gas_limit: u64) -> Self {
        Self {
            amount,
            gas_limit,
            payer: None,
            granter: None,
        }
    }

    pub fn with_payer(mut self, payer: String) -> Self {
        self.payer = Some(payer);
        self
    }

    pub fn with_granter(mut self, granter: String) -> Self {
        self.granter = Some(granter);
        self
    }
}

/// Transaction response
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct TxResponse {
    pub txhash: String,
    pub code: u32,
    pub raw_log: String,
    pub gas_wanted: u64,
    pub gas_used: u64,
    pub height: i64,
}

/// Broadcast mode for transactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, uniffi::Enum)]
pub enum BroadcastMode {
    /// Synchronous - returns after CheckTx
    Sync,
    /// Asynchronous - returns immediately
    Async,
    /// Block - waits until transaction is included in a block
    Block,
}

/// Chain configuration
#[derive(Debug, Clone, Serialize, Deserialize, uniffi::Record)]
pub struct ChainConfig {
    pub chain_id: String,
    pub rpc_endpoint: String,
    pub grpc_endpoint: Option<String>,
    pub address_prefix: String,
    pub coin_type: u32,
    pub gas_price: String,
}

impl ChainConfig {
    pub fn new(
        chain_id: impl Into<String>,
        rpc_endpoint: impl Into<String>,
        address_prefix: impl Into<String>,
    ) -> Self {
        Self {
            chain_id: chain_id.into(),
            rpc_endpoint: rpc_endpoint.into(),
            grpc_endpoint: None,
            address_prefix: address_prefix.into(),
            coin_type: 118, // Default Cosmos coin type
            gas_price: "0.025".to_string(),
        }
    }

    pub fn with_grpc(mut self, grpc_endpoint: String) -> Self {
        self.grpc_endpoint = Some(grpc_endpoint);
        self
    }

    pub fn with_coin_type(mut self, coin_type: u32) -> Self {
        self.coin_type = coin_type;
        self
    }

    pub fn with_gas_price(mut self, gas_price: String) -> Self {
        self.gas_price = gas_price;
        self
    }
}

/// Message type for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub type_url: String,
    pub value: Vec<u8>,
}

/// Signature information
#[derive(Debug, Clone)]
pub struct SignatureInfo {
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

/// Signing options
#[derive(Debug, Clone, Default)]
pub struct SignOptions {
    pub memo: Option<String>,
    pub timeout_height: Option<u64>,
    pub extension_options: Vec<Message>,
    pub non_critical_extension_options: Vec<Message>,
}

impl SignOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn with_timeout_height(mut self, height: u64) -> Self {
        self.timeout_height = Some(height);
        self
    }
}
