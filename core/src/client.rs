// Only compile RPC client when rpc-client feature is enabled
#[cfg(feature = "rpc-client")]
use tendermint_rpc::{Client as TmClient, HttpClient};

#[cfg(feature = "rust-signer")]
use crate::rust_signer::RustSigner;
use crate::{
    account::Account,
    crypto_signer::CryptoSigner,
    error::{MobError, Result},
    transaction::TransactionBuilder,
    types::{AccountInfo, BroadcastMode, ChainConfig, Coin, TxResponse},
};
use cosmrs::AccountId;
use std::{str::FromStr, sync::Arc};

/// RPC client for interacting with the blockchain
/// Only available with "rpc-client" feature (default)
#[cfg(feature = "rpc-client")]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Object))]
pub struct Client {
    config: ChainConfig,
    rpc_client: HttpClient,
    signer: Option<Arc<dyn CryptoSigner>>,
    account: Option<Account>,
}

#[cfg(feature = "rpc-client")]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl Client {
    /// Create a new RPC client (synchronous wrapper for FFI)
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn new(config: ChainConfig) -> Result<Self> {
        // Create a runtime and block on the async operation
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(Self::new_async(config))
    }

    /// Query account information (synchronous wrapper)
    pub fn get_account(&self, address: String) -> Result<AccountInfo> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_account_internal(&address))
    }

    /// Query account balance (synchronous wrapper)
    pub fn get_balance(&self, address: String, denom: String) -> Result<Coin> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_balance_internal(&address, &denom))
    }

    /// Query all balances for an address (synchronous wrapper)
    pub fn get_all_balances(&self, address: String) -> Result<Vec<Coin>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_all_balances_internal(&address))
    }

    /// Send tokens to a recipient (synchronous wrapper)
    pub fn send(
        &self,
        to_address: String,
        amount: Vec<Coin>,
        memo: Option<String>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.send_internal(&to_address, amount, memo))
    }

    /// Execute a CosmWasm contract (synchronous wrapper)
    pub fn execute_contract(
        &self,
        contract_address: String,
        msg: Vec<u8>,
        funds: Vec<Coin>,
        memo: Option<String>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.execute_contract_internal(&contract_address, &msg, funds, memo))
    }

    /// Query transaction by hash (synchronous wrapper)
    pub fn get_tx(&self, hash: String) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_tx_internal(&hash))
    }

    /// Get the latest block height (synchronous wrapper)
    pub fn get_height(&self) -> Result<u64> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_height_internal())
    }

    /// Check if the node is synced (synchronous wrapper)
    pub fn is_synced(&self) -> Result<bool> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.is_synced_internal())
    }

    /// Get chain ID (synchronous wrapper)
    pub fn get_chain_id(&self) -> Result<String> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.get_chain_id_internal())
    }
}

// RustSigner-specific FFI constructors (only with rust-signer feature)
#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl Client {
    /// Create a new RPC client with a signer attached (synchronous wrapper for FFI)
    ///
    /// Note: This constructor is only available with the `rust-signer` feature.
    #[uniffi::constructor]
    pub fn new_with_signer(config: ChainConfig, signer: Arc<RustSigner>) -> Result<Self> {
        // Create a runtime and block on the async operation
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(async {
            let mut client = Self::new_async(config).await?;
            client.attach_signer_internal(signer).await?;
            Ok(client)
        })
    }

    /// Attach a signer to the client
    ///
    /// Note: This method is only available with the `rust-signer` feature.
    pub fn attach_signer(&self, _signer: Arc<RustSigner>) -> Result<()> {
        // This method is exported for FFI but we can't mutate through UniFFI
        // For now, return an error - this needs a different architecture
        Err(MobError::Generic(
            "attach_signer not yet implemented for FFI".to_string(),
        ))
    }
}

// Internal implementation
#[cfg(feature = "rpc-client")]
impl Client {
    /// Create a new RPC client (async version for internal use)
    /// Create a new RPC client (async version for internal use and tests)
    pub async fn new_async(config: ChainConfig) -> Result<Self> {
        let rpc_client = HttpClient::new(config.rpc_endpoint.as_str())
            .map_err(|e| MobError::Network(format!("Failed to create RPC client: {}", e)))?;

        Ok(Self {
            config,
            rpc_client,
            signer: None,
            account: None,
        })
    }

    /// Attach a signer to the client (internal)
    ///
    /// Note: This method is only available with the `rust-signer` feature.
    #[cfg(feature = "rust-signer")]
    pub async fn attach_signer_internal(&mut self, signer: Arc<RustSigner>) -> Result<()> {
        // Create account for the signer
        let address = signer.address();
        let account = Account::new(address);

        // Convert to trait object
        self.signer = Some(signer as Arc<dyn CryptoSigner>);
        self.account = Some(account);

        // Fetch account info
        self.refresh_account_info().await?;

        Ok(())
    }

    /// Attach any CryptoSigner implementation to the client
    ///
    /// This is the primary method for Rust code to attach custom signers.
    pub async fn attach_crypto_signer(&mut self, signer: Arc<dyn CryptoSigner>) -> Result<()> {
        // Create account for the signer
        let address = signer.address();
        let account = Account::new(address);

        self.signer = Some(signer);
        self.account = Some(account);

        // Fetch account info
        self.refresh_account_info().await?;

        Ok(())
    }

    /// Get the attached signer
    pub fn signer(&self) -> Option<&Arc<dyn CryptoSigner>> {
        self.signer.as_ref()
    }

    /// Get the account
    pub fn account(&self) -> Option<&Account> {
        self.account.as_ref()
    }

    /// Get chain configuration
    pub fn config(&self) -> ChainConfig {
        self.config.clone()
    }
}

// Internal implementation methods using &str for Rust ergonomics
#[cfg(feature = "rpc-client")]
impl Client {
    /// Refresh account information from the blockchain
    async fn refresh_account_info(&mut self) -> Result<()> {
        let account = self
            .account
            .as_ref()
            .ok_or_else(|| MobError::Account("No account attached".to_string()))?;

        let info = self.get_account_internal(&account.address).await?;

        if let Some(acc) = &mut self.account {
            acc.update_info(info);
        }

        Ok(())
    }

    /// Query account information (internal)
    async fn get_account_internal(&self, address: &str) -> Result<AccountInfo> {
        // Validate address
        let _account_id = AccountId::from_str(address)
            .map_err(|e| MobError::Address(format!("Invalid address: {}", e)))?;

        // Query account info using ABCI query
        let query_path = "/cosmos.auth.v1beta1.Query/Account".to_string();

        // Create the query request protobuf
        let query_request = cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest {
            address: address.to_string(),
        };

        // Encode the request
        use prost::Message;
        let mut buf = Vec::new();
        query_request
            .encode(&mut buf)
            .map_err(|e| MobError::Transaction(format!("Failed to encode account query: {}", e)))?;

        // Query via ABCI
        let response = self
            .rpc_client
            .abci_query(Some(query_path), buf, None, false)
            .await
            .map_err(|e| MobError::Network(format!("Account query failed: {}", e)))?;

        // Check for errors
        if response.code.is_err() {
            return Err(MobError::Network(format!(
                "Account query returned error code {}: {}",
                response.code.value(),
                response.log
            )));
        }

        // Decode the response
        let query_response = cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountResponse::decode(
            response.value.as_slice(),
        )
        .map_err(|e| MobError::Transaction(format!("Failed to decode account response: {}", e)))?;

        // Extract account info from Any type
        let account_any = query_response
            .account
            .ok_or_else(|| MobError::Account("Account not found".to_string()))?;

        // Decode BaseAccount from Any
        let base_account = cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount::decode(
            account_any.value.as_slice(),
        )
        .map_err(|e| MobError::Account(format!("Failed to decode base account: {}", e)))?;

        Ok(AccountInfo {
            address: address.to_string(),
            account_number: base_account.account_number,
            sequence: base_account.sequence,
            pub_key: None,
        })
    }

    /// Query account balance (internal)
    pub async fn get_balance_internal(&self, address: &str, denom: &str) -> Result<Coin> {
        // Query balance using ABCI query
        let query_path = "/cosmos.bank.v1beta1.Query/Balance".to_string();

        // Create the query request protobuf
        let query_request = cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest {
            address: address.to_string(),
            denom: denom.to_string(),
        };

        // Encode the request
        use prost::Message;
        let mut buf = Vec::new();
        query_request
            .encode(&mut buf)
            .map_err(|e| MobError::Transaction(format!("Failed to encode balance query: {}", e)))?;

        // Query via ABCI
        let response = self
            .rpc_client
            .abci_query(Some(query_path), buf, None, false)
            .await
            .map_err(|e| MobError::Network(format!("Balance query failed: {}", e)))?;

        // Check for errors
        if response.code.is_err() {
            return Err(MobError::Network(format!(
                "Balance query returned error code {}: {}",
                response.code.value(),
                response.log
            )));
        }

        // Decode the response
        let query_response = cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceResponse::decode(
            response.value.as_slice(),
        )
        .map_err(|e| MobError::Transaction(format!("Failed to decode balance response: {}", e)))?;

        // Convert to our Coin type
        match query_response.balance {
            Some(coin) => Ok(Coin::new(&coin.denom, &coin.amount)),
            None => Ok(Coin::new(denom, "0")),
        }
    }

    /// Query all balances for an address (internal)
    async fn get_all_balances_internal(&self, address: &str) -> Result<Vec<Coin>> {
        // Query all balances using ABCI query
        let query_path = "/cosmos.bank.v1beta1.Query/AllBalances".to_string();

        // Create the query request protobuf
        let query_request = cosmos_sdk_proto::cosmos::bank::v1beta1::QueryAllBalancesRequest {
            address: address.to_string(),
            pagination: None,
            resolve_denom: false,
        };

        // Encode the request
        use prost::Message;
        let mut buf = Vec::new();
        query_request.encode(&mut buf).map_err(|e| {
            MobError::Transaction(format!("Failed to encode all balances query: {}", e))
        })?;

        // Query via ABCI
        let response = self
            .rpc_client
            .abci_query(Some(query_path), buf, None, false)
            .await
            .map_err(|e| MobError::Network(format!("All balances query failed: {}", e)))?;

        // Check for errors
        if response.code.is_err() {
            return Err(MobError::Network(format!(
                "All balances query returned error code {}: {}",
                response.code.value(),
                response.log
            )));
        }

        // Decode the response
        let query_response =
            cosmos_sdk_proto::cosmos::bank::v1beta1::QueryAllBalancesResponse::decode(
                response.value.as_slice(),
            )
            .map_err(|e| {
                MobError::Transaction(format!("Failed to decode all balances response: {}", e))
            })?;

        // Convert to our Coin types
        Ok(query_response
            .balances
            .into_iter()
            .map(|coin| Coin::new(&coin.denom, &coin.amount))
            .collect())
    }

    /// Send tokens from the attached signer to a recipient (internal)
    pub async fn send_internal(
        &self,
        to_address: &str,
        amount: Vec<Coin>,
        memo: Option<String>,
    ) -> Result<TxResponse> {
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?;

        let account = self
            .account
            .as_ref()
            .ok_or_else(|| MobError::Account("No account attached".to_string()))?;

        // Build send message
        let msg =
            crate::transaction::messages::msg_send(&signer.address(), to_address, amount.clone())?;

        // Calculate fee
        let fee = crate::transaction::calculate_fee(200_000, &self.config.gas_price, "uxion")?;

        // Build transaction
        let mut tx_builder = TransactionBuilder::new(&self.config.chain_id)?;
        tx_builder.add_message(msg);
        tx_builder.with_fee(fee);

        if let Some(memo_text) = memo {
            tx_builder.with_memo(memo_text);
        }

        // Sign transaction
        let tx_bytes = tx_builder.sign(
            signer.as_ref(),
            account.account_number()?,
            account.sequence()?,
        )?;

        // Broadcast transaction
        let response = self
            .broadcast_tx_internal(tx_bytes, BroadcastMode::Sync)
            .await?;

        Ok(response)
    }

    /// Execute a CosmWasm contract (internal)
    async fn execute_contract_internal(
        &self,
        contract_address: &str,
        msg: &[u8],
        funds: Vec<Coin>,
        memo: Option<String>,
    ) -> Result<TxResponse> {
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?;

        let account = self
            .account
            .as_ref()
            .ok_or_else(|| MobError::Account("No account attached".to_string()))?;

        // Build execute contract message
        let execute_msg = crate::transaction::messages::msg_execute_contract(
            &signer.address(),
            contract_address,
            msg,
            funds,
        )?;

        // Calculate fee
        let fee = crate::transaction::calculate_fee(300_000, &self.config.gas_price, "uxion")?;

        // Build transaction
        let mut tx_builder = TransactionBuilder::new(&self.config.chain_id)?;
        tx_builder.add_message(execute_msg);
        tx_builder.with_fee(fee);

        if let Some(memo_text) = memo {
            tx_builder.with_memo(memo_text);
        }

        // Sign transaction
        let tx_bytes = tx_builder.sign(
            signer.as_ref(),
            account.account_number()?,
            account.sequence()?,
        )?;

        // Broadcast transaction
        let response = self
            .broadcast_tx_internal(tx_bytes, BroadcastMode::Sync)
            .await?;

        Ok(response)
    }

    /// Broadcast a signed transaction (internal)
    async fn broadcast_tx_internal(
        &self,
        tx_bytes: Vec<u8>,
        mode: BroadcastMode,
    ) -> Result<TxResponse> {
        // Use tendermint-rpc to broadcast
        match mode {
            BroadcastMode::Sync => {
                let result = self
                    .rpc_client
                    .broadcast_tx_sync(tx_bytes.clone())
                    .await
                    .map_err(|e| {
                        MobError::Rpc(format!("Failed to broadcast transaction: {}", e))
                    })?;

                Ok(TxResponse {
                    txhash: result.hash.to_string(),
                    code: result.code.value(),
                    raw_log: result.log.to_string(),
                    gas_wanted: 0,
                    gas_used: 0,
                    height: 0,
                })
            }
            BroadcastMode::Async => {
                let result = self
                    .rpc_client
                    .broadcast_tx_async(tx_bytes.clone())
                    .await
                    .map_err(|e| {
                        MobError::Rpc(format!("Failed to broadcast transaction: {}", e))
                    })?;

                Ok(TxResponse {
                    txhash: result.hash.to_string(),
                    code: 0, // Async doesn't return code
                    raw_log: String::new(),
                    gas_wanted: 0,
                    gas_used: 0,
                    height: 0,
                })
            }
            BroadcastMode::Block => {
                let result = self
                    .rpc_client
                    .broadcast_tx_commit(tx_bytes)
                    .await
                    .map_err(|e| {
                        MobError::Rpc(format!("Failed to broadcast transaction: {}", e))
                    })?;

                Ok(TxResponse {
                    txhash: result.hash.to_string(),
                    code: result.check_tx.code.value(),
                    raw_log: result.check_tx.log.to_string(),
                    gas_wanted: result.check_tx.gas_wanted as u64,
                    gas_used: result.check_tx.gas_used as u64,
                    height: result.height.value() as i64,
                })
            }
        }
    }

    /// Query transaction by hash (internal)
    pub async fn get_tx_internal(&self, hash: &str) -> Result<TxResponse> {
        let hash_bytes = hex::decode(hash)
            .map_err(|e| MobError::Transaction(format!("Invalid transaction hash: {}", e)))?;

        // Convert to fixed-size array for tendermint Hash
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().map_err(|_| {
            MobError::Transaction("Invalid hash length, expected 32 bytes".to_string())
        })?;

        let tx_hash = tendermint::Hash::Sha256(hash_array);

        let tx_result = self
            .rpc_client
            .tx(tx_hash, false)
            .await
            .map_err(|e| MobError::Rpc(format!("Failed to query transaction: {}", e)))?;

        Ok(TxResponse {
            txhash: tx_result.hash.to_string(),
            code: tx_result.tx_result.code.value() as u32,
            raw_log: tx_result.tx_result.log.to_string(),
            gas_wanted: tx_result.tx_result.gas_wanted as u64,
            gas_used: tx_result.tx_result.gas_used as u64,
            height: tx_result.height.value() as i64,
        })
    }

    /// Get the latest block height (internal)
    async fn get_height_internal(&self) -> Result<u64> {
        let status = self
            .rpc_client
            .status()
            .await
            .map_err(|e| MobError::Rpc(format!("Failed to get status: {}", e)))?;

        Ok(status.sync_info.latest_block_height.value())
    }

    /// Check if the node is synced (internal)
    async fn is_synced_internal(&self) -> Result<bool> {
        let status = self
            .rpc_client
            .status()
            .await
            .map_err(|e| MobError::Rpc(format!("Failed to get status: {}", e)))?;

        Ok(!status.sync_info.catching_up)
    }

    /// Get chain ID (internal)
    async fn get_chain_id_internal(&self) -> Result<String> {
        Ok(self.config.chain_id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = ChainConfig::new(
            "xion-testnet-1",
            "https://rpc.xion-testnet-1.burnt.com:443",
            "xion",
        );

        // This will fail without a real RPC endpoint, but tests the structure
        let _result = Client::new(config);
    }

    #[test]
    fn test_chain_config() {
        let config = ChainConfig::new(
            "xion-testnet-1",
            "https://rpc.xion-testnet-1.burnt.com:443",
            "xion",
        );

        assert_eq!(config.chain_id, "xion-testnet-1");
        assert_eq!(config.address_prefix, "xion");
        assert_eq!(config.coin_type, 118);
    }
}
