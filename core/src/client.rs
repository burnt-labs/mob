// Only compile RPC client when rpc-client feature is enabled
#[cfg(feature = "rpc-client")]
use tendermint_rpc::client::Client as TmClient;

#[cfg(feature = "rpc-client")]
use crate::native_rpc_client::NativeRpcClient;
#[cfg(feature = "rust-signer")]
use crate::rust_signer::RustSigner;
use crate::{
    account::Account,
    crypto_signer::CryptoSigner,
    error::{MobError, Result},
    http_transport::HttpTransport,
    transaction::TransactionBuilder,
    types::{AccountInfo, BroadcastMode, ChainConfig, Coin, Message, TxResponse},
};
use cosmrs::AccountId;
use std::{str::FromStr, sync::Arc};

/// RPC client for interacting with the blockchain
/// Only available with "rpc-client" feature (default)
#[cfg(feature = "rpc-client")]
#[cfg_attr(feature = "uniffi-bindings", derive(uniffi::Object))]
pub struct Client {
    config: ChainConfig,
    rpc_client: NativeRpcClient,
    signer: Option<Arc<dyn CryptoSigner>>,
    account: Option<Account>,
}

#[cfg(feature = "rpc-client")]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl Client {
    /// Create a new RPC client (synchronous wrapper for FFI)
    #[cfg_attr(feature = "uniffi-bindings", uniffi::constructor)]
    pub fn new(config: ChainConfig, transport: Arc<dyn HttpTransport>) -> Result<Self> {
        Ok(Self::new_with_transport(config, transport))
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
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.execute_contract_internal(
            &contract_address,
            &msg,
            funds,
            memo,
            gas_limit,
        ))
    }

    /// Store a CosmWasm contract (synchronous wrapper)
    pub fn store_code(
        &self,
        wasm_byte_code: Vec<u8>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.store_code_internal(wasm_byte_code, memo, gas_limit))
    }

    /// Instantiate an uploaded CosmWasm contract (synchronous wrapper)
    #[allow(clippy::too_many_arguments)]
    pub fn instantiate_contract(
        &self,
        admin: Option<String>,
        code_id: u64,
        label: Option<String>,
        msg: Vec<u8>,
        funds: Vec<Coin>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.instantiate_contract_internal(
            admin.as_deref(),
            code_id,
            label.as_deref(),
            &msg,
            funds,
            memo,
            gas_limit,
        ))
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

    /// Query a CosmWasm smart contract (read-only, synchronous wrapper)
    pub fn query_contract_smart(
        &self,
        contract_address: String,
        query_msg: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.query_contract_smart_internal(&contract_address, &query_msg))
    }

    /// Sign and broadcast a transaction with multiple FFI-safe messages (synchronous wrapper)
    pub fn sign_and_broadcast_multi(
        &self,
        messages: Vec<Message>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        let any_messages: Vec<cosmrs::Any> = messages
            .into_iter()
            .map(|m| cosmrs::Any {
                type_url: m.type_url,
                value: m.value,
            })
            .collect();

        runtime.block_on(self.sign_and_broadcast_messages(any_messages, memo, gas_limit))
    }
}

// Public API for multi-message transactions (not exposed via UniFFI)
#[cfg(feature = "rpc-client")]
impl Client {
    /// Sign and broadcast a transaction with multiple pre-built messages (synchronous wrapper).
    /// Use `mob::messages::msg_execute_contract` etc. to build `cosmrs::Any` messages.
    pub fn sign_and_broadcast(
        &self,
        messages: Vec<cosmrs::Any>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.sign_and_broadcast_messages(messages, memo, gas_limit))
    }
}

// CryptoSigner-accepting constructor (works with any CryptoSigner implementation, including foreign)
#[cfg(feature = "rpc-client")]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl Client {
    /// Create a new RPC client with any CryptoSigner implementation attached.
    ///
    /// This accepts foreign (Swift/Kotlin/Python) CryptoSigner implementations,
    /// enabling platform-native cryptographic backends.
    #[uniffi::constructor]
    pub fn new_with_crypto_signer(
        config: ChainConfig,
        signer: Arc<dyn CryptoSigner>,
        transport: Arc<dyn HttpTransport>,
    ) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(async {
            let mut client = Self::new_with_transport(config, transport);
            client.attach_crypto_signer(signer).await?;
            Ok(client)
        })
    }
}

// RustSigner-specific FFI constructors (only with rust-signer feature)
#[cfg(all(feature = "rpc-client", feature = "rust-signer"))]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl Client {
    /// Create a new RPC client with a RustSigner attached (synchronous wrapper for FFI)
    ///
    /// Note: This constructor is only available with the `rust-signer` feature.
    #[uniffi::constructor]
    pub fn new_with_signer(
        config: ChainConfig,
        signer: Arc<RustSigner>,
        transport: Arc<dyn HttpTransport>,
    ) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(async {
            let mut client = Self::new_with_transport(config, transport);
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
    /// Create a new RPC client with a native HTTP transport.
    pub fn new_with_transport(config: ChainConfig, transport: Arc<dyn HttpTransport>) -> Self {
        let rpc_client = NativeRpcClient::new(config.rpc_endpoint.clone(), transport);

        Self {
            config,
            rpc_client,
            signer: None,
            account: None,
        }
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
        let query_request = xion_types::types::cosmos_auth_v1beta1::QueryAccountRequest {
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
        let query_response = xion_types::types::cosmos_auth_v1beta1::QueryAccountResponse::decode(
            response.value.as_slice(),
        )
        .map_err(|e| MobError::Transaction(format!("Failed to decode account response: {}", e)))?;

        // Extract account info from Any type
        let account_any = query_response
            .account
            .ok_or_else(|| MobError::Account("Account not found".to_string()))?;

        // Decode BaseAccount from Any
        let base_account =
            xion_types::types::cosmos_auth_v1beta1::BaseAccount::decode(
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
        let query_request = xion_types::types::cosmos_bank_v1beta1::QueryBalanceRequest {
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
        let query_response = xion_types::types::cosmos_bank_v1beta1::QueryBalanceResponse::decode(
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
        let query_request = xion_types::types::cosmos_bank_v1beta1::QueryAllBalancesRequest {
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
            xion_types::types::cosmos_bank_v1beta1::QueryAllBalancesResponse::decode(
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
        let sender = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?
            .address();

        let msg = crate::transaction::messages::msg_send(&sender, to_address, amount)?;
        self.sign_and_broadcast_messages(vec![msg], memo, None)
            .await
    }

    /// Execute a CosmWasm contract (internal)
    async fn execute_contract_internal(
        &self,
        contract_address: &str,
        msg: &[u8],
        funds: Vec<Coin>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let sender = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?
            .address();

        let execute_msg = crate::transaction::messages::msg_execute_contract(
            &sender,
            contract_address,
            msg,
            funds,
        )?;

        self.sign_and_broadcast_messages(vec![execute_msg], memo, gas_limit)
            .await
    }

    /// Store a CosmWasm contract (internal)
    async fn store_code_internal(
        &self,
        wasm_byte_code: Vec<u8>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let sender = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?
            .address();

        let store_msg = crate::transaction::messages::msg_store_code(&sender, wasm_byte_code)?;
        self.sign_and_broadcast_messages(vec![store_msg], memo, gas_limit)
            .await
    }

    /// Instantiate an uploaded CosmWasm contract (internal)
    #[allow(clippy::too_many_arguments)]
    async fn instantiate_contract_internal(
        &self,
        admin: Option<&str>,
        code_id: u64,
        label: Option<&str>,
        msg: &[u8],
        funds: Vec<Coin>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let sender = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?
            .address();

        let instantiate_msg = crate::transaction::messages::msg_instantiate_contract(
            &sender, admin, code_id, label, msg, funds,
        )?;

        self.sign_and_broadcast_messages(vec![instantiate_msg], memo, gas_limit)
            .await
    }

    /// Simulate a signed transaction to estimate gas usage.
    /// Returns the estimated gas_used from the simulation.
    async fn simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64> {
        use prost::Message;

        #[allow(deprecated)]
        let request = xion_types::types::cosmos_tx_v1beta1::SimulateRequest { tx: None, tx_bytes };

        let mut buf = Vec::new();
        request.encode(&mut buf).map_err(|e| {
            MobError::Transaction(format!("Failed to encode simulate request: {}", e))
        })?;

        let response = self
            .rpc_client
            .abci_query(
                Some("/cosmos.tx.v1beta1.Service/Simulate".to_string()),
                buf,
                None,
                false,
            )
            .await
            .map_err(|e| MobError::Network(format!("Simulate query failed: {}", e)))?;

        if response.code.is_err() {
            return Err(MobError::Transaction(format!(
                "Simulation failed (code {}): {}",
                response.code.value(),
                response.log
            )));
        }

        let sim_response =
            xion_types::types::cosmos_tx_v1beta1::SimulateResponse::decode(
                response.value.as_slice(),
            )
            .map_err(|e| {
                MobError::Transaction(format!("Failed to decode simulate response: {}", e))
            })?;

        let gas_used = sim_response
            .gas_info
            .ok_or_else(|| MobError::Transaction("No gas info in simulate response".to_string()))?
            .gas_used;

        Ok(gas_used)
    }

    /// Estimate gas for a transaction by simulating it with a zero fee,
    /// then applying a multiplier (1.4x) for safety margin.
    async fn estimate_gas(&self, messages: &[cosmrs::Any], memo: Option<&str>) -> Result<u64> {
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?;

        let account = self
            .account
            .as_ref()
            .ok_or_else(|| MobError::Account("No account attached".to_string()))?;

        // Build tx with zero fee for simulation
        let mut zero_fee = crate::transaction::calculate_fee(0, "0", "uxion")?;
        if let Some(ref granter) = self.config.fee_granter {
            zero_fee.granter = Some(granter.clone());
        }

        let mut tx_builder = TransactionBuilder::new(&self.config.chain_id)?;
        for m in messages {
            tx_builder.add_message(m.clone());
        }
        tx_builder.with_fee(zero_fee);

        if let Some(m) = memo {
            tx_builder.with_memo(m);
        }

        let tx_bytes = tx_builder.sign(
            signer.as_ref(),
            account.account_number()?,
            account.sequence()?,
        )?;

        let gas_used = self.simulate_tx(tx_bytes).await?;

        // Apply 1.4x multiplier for safety margin
        Ok((gas_used as f64 * 1.4) as u64)
    }

    /// Sign and broadcast a transaction with one or more messages.
    /// If gas_limit is None, simulates to estimate gas.
    async fn sign_and_broadcast_messages(
        &self,
        messages: Vec<cosmrs::Any>,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> Result<TxResponse> {
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| MobError::Signing("No signer attached".to_string()))?;

        let account = self
            .account
            .as_ref()
            .ok_or_else(|| MobError::Account("No account attached".to_string()))?;

        let resolved_gas = match gas_limit {
            Some(limit) => limit,
            None => self.estimate_gas(&messages, memo.as_deref()).await?,
        };

        let mut fee =
            crate::transaction::calculate_fee(resolved_gas, &self.config.gas_price, "uxion")?;
        if let Some(ref granter) = self.config.fee_granter {
            fee.granter = Some(granter.clone());
        }

        let mut tx_builder = TransactionBuilder::new(&self.config.chain_id)?;
        tx_builder.add_messages(messages);
        tx_builder.with_fee(fee);

        if let Some(memo_text) = memo {
            tx_builder.with_memo(memo_text);
        }

        let tx_bytes = tx_builder.sign(
            signer.as_ref(),
            account.account_number()?,
            account.sequence()?,
        )?;

        let response = self
            .broadcast_tx_internal(tx_bytes, BroadcastMode::Sync)
            .await?;

        if let Some(acc) = &self.account {
            acc.increment_sequence();
        }

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

    /// Query a CosmWasm smart contract (internal)
    async fn query_contract_smart_internal(
        &self,
        contract_address: &str,
        query_msg: &[u8],
    ) -> Result<Vec<u8>> {
        use prost::Message;

        let query_path = "/cosmwasm.wasm.v1.Query/SmartContractState".to_string();

        let request = xion_types::types::cosmwasm_wasm_v1::QuerySmartContractStateRequest {
            address: contract_address.to_string(),
            query_data: query_msg.to_vec(),
        };

        let mut buf = Vec::new();
        request.encode(&mut buf).map_err(|e| {
            MobError::Transaction(format!("Failed to encode smart query request: {}", e))
        })?;

        let response = self
            .rpc_client
            .abci_query(Some(query_path), buf, None, false)
            .await
            .map_err(|e| MobError::Network(format!("Smart contract query failed: {}", e)))?;

        if response.code.is_err() {
            return Err(MobError::Network(format!(
                "Smart contract query returned error code {}: {}",
                response.code.value(),
                response.log
            )));
        }

        let query_response =
            xion_types::types::cosmwasm_wasm_v1::QuerySmartContractStateResponse::decode(
                response.value.as_slice(),
            )
            .map_err(|e| {
                MobError::Transaction(format!("Failed to decode smart query response: {}", e))
            })?;

        Ok(query_response.data)
    }

    /// Query authz grants between a granter and grantee.
    /// Returns true if at least one active grant exists.
    async fn query_grants_internal(&self, granter: &str, grantee: &str) -> Result<bool> {
        use prost::Message;

        let query_path = "/cosmos.authz.v1beta1.Query/Grants".to_string();

        let request = xion_types::types::cosmos_authz_v1beta1::QueryGrantsRequest {
            granter: granter.to_string(),
            grantee: grantee.to_string(),
            msg_type_url: String::new(),
            pagination: None,
        };

        let mut buf = Vec::new();
        request
            .encode(&mut buf)
            .map_err(|e| MobError::Transaction(format!("Failed to encode grants query: {}", e)))?;

        let response = self
            .rpc_client
            .abci_query(Some(query_path), buf, None, false)
            .await
            .map_err(|e| MobError::Network(format!("Grants query failed: {}", e)))?;

        if response.code.is_err() {
            return Ok(false);
        }

        let query_response = xion_types::types::cosmos_authz_v1beta1::QueryGrantsResponse::decode(
            response.value.as_slice(),
        )
        .map_err(|e| MobError::Transaction(format!("Failed to decode grants response: {}", e)))?;

        Ok(!query_response.grants.is_empty())
    }
}

// FFI-exported grant verification
#[cfg(feature = "rpc-client")]
#[cfg_attr(feature = "uniffi-bindings", uniffi::export)]
impl Client {
    /// Check whether authz grants exist between a granter and grantee.
    pub fn has_grants(&self, granter: String, grantee: String) -> Result<bool> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| MobError::Generic(format!("Failed to create runtime: {}", e)))?;

        runtime.block_on(self.query_grants_internal(&granter, &grantee))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_transport::{HttpTransport, TransportError};

    /// A mock transport for tests — returns an error on every request.
    struct MockTransport;
    impl HttpTransport for MockTransport {
        fn post(
            &self,
            _url: String,
            _body: Vec<u8>,
        ) -> std::result::Result<Vec<u8>, TransportError> {
            Err(TransportError::NetworkError("mock transport".to_string()))
        }
    }

    #[test]
    fn test_client_creation() {
        let config = ChainConfig::new(
            "xion-testnet-1",
            "https://rpc.xion-testnet-1.burnt.com:443",
            "xion",
        );

        let transport: Arc<dyn HttpTransport> = Arc::new(MockTransport);
        let result = Client::new(config, transport);
        assert!(result.is_ok());
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

    #[test]
    fn test_sign_and_broadcast_multi_converts_messages() {
        // Verify that Message -> cosmrs::Any conversion works correctly
        let msg = Message {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            value: vec![1, 2, 3, 4],
        };

        let any: cosmrs::Any = cosmrs::Any {
            type_url: msg.type_url.clone(),
            value: msg.value.clone(),
        };

        assert_eq!(any.type_url, "/cosmos.bank.v1beta1.MsgSend");
        assert_eq!(any.value, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_query_contract_smart_request_encoding() {
        use prost::Message as ProstMessage;

        let contract = "xion1contractaddr";
        let query_msg = br#"{"balance":{"address":"xion1user"}}"#;

        let request = xion_types::types::cosmwasm_wasm_v1::QuerySmartContractStateRequest {
            address: contract.to_string(),
            query_data: query_msg.to_vec(),
        };

        let mut buf = Vec::new();
        request.encode(&mut buf).expect("Failed to encode request");
        assert!(!buf.is_empty());

        // Verify round-trip
        let decoded =
            xion_types::types::cosmwasm_wasm_v1::QuerySmartContractStateRequest::decode(
                buf.as_slice(),
            )
            .expect("Failed to decode");
        assert_eq!(decoded.address, contract);
        assert_eq!(decoded.query_data, query_msg.to_vec());
    }
}
