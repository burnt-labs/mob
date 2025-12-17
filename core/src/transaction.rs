use crate::{
    error::{MobError, Result},
    signer::Signer,
    types::{Coin, Fee},
};
use cosmrs::{
    tendermint::chain::Id as ChainId,
    tx::{self, AccountNumber, Body, BodyBuilder, Fee as CosmosFee, Msg, SequenceNumber, SignDoc},
    Any, Gas,
};
use std::str::FromStr;

/// Transaction builder for creating and signing transactions
pub struct TransactionBuilder {
    chain_id: ChainId,
    messages: Vec<Any>,
    fee: Option<Fee>,
    memo: String,
    timeout_height: u64,
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new(chain_id: impl Into<String>) -> Result<Self> {
        let chain_id = ChainId::from_str(&chain_id.into())
            .map_err(|e| MobError::Transaction(format!("Invalid chain ID: {}", e)))?;

        Ok(Self {
            chain_id,
            messages: vec![],
            fee: None,
            memo: String::new(),
            timeout_height: 0,
        })
    }

    /// Add a message to the transaction
    pub fn add_message(&mut self, message: Any) -> &mut Self {
        self.messages.push(message);
        self
    }

    /// Add multiple messages to the transaction
    pub fn add_messages(&mut self, messages: Vec<Any>) -> &mut Self {
        self.messages.extend(messages);
        self
    }

    /// Set the transaction fee
    pub fn with_fee(&mut self, fee: Fee) -> &mut Self {
        self.fee = Some(fee);
        self
    }

    /// Set the transaction memo
    pub fn with_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.memo = memo.into();
        self
    }

    /// Set the timeout height
    pub fn with_timeout_height(&mut self, height: u64) -> &mut Self {
        self.timeout_height = height;
        self
    }

    /// Build the transaction body
    pub fn build_body(&self) -> Result<Body> {
        if self.messages.is_empty() {
            return Err(MobError::Transaction(
                "Transaction must have at least one message".to_string(),
            ));
        }

        let mut body_builder = BodyBuilder::new();

        for msg in &self.messages {
            body_builder.msg(msg.clone());
        }

        if !self.memo.is_empty() {
            body_builder.memo(&self.memo);
        }

        if self.timeout_height > 0 {
            body_builder.timeout_height(cosmrs::tendermint::block::Height::try_from(self.timeout_height)
                .map_err(|e| MobError::Transaction(format!("Invalid timeout height: {}", e)))?);
        }

        Ok(body_builder.finish())
    }

    /// Build and sign the transaction
    pub fn sign(
        &self,
        signer: &Signer,
        account_number: u64,
        sequence: u64,
    ) -> Result<Vec<u8>> {
        let body = self.build_body()?;

        // Get fee or use default
        let fee = self.fee.as_ref().ok_or_else(|| {
            MobError::Transaction("Transaction fee not set".to_string())
        })?;

        // Create auth info
        let auth_info = self.create_auth_info(signer, fee, sequence)?;

        // Create SignDoc
        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &self.chain_id,
            account_number,
        )?;

        // Sign the transaction
        let tx_raw = signer.sign_direct(&sign_doc, account_number)?;

        // Serialize to bytes
        let tx_bytes = tx_raw.to_bytes()?;
        Ok(tx_bytes)
    }

    fn create_auth_info(&self, signer: &Signer, fee: &Fee, sequence: u64) -> Result<tx::AuthInfo> {
        let cosmos_fee = self.convert_fee(fee)?;

        // Create public key from bytes (33 bytes compressed format)
        let pub_key_bytes = signer.public_key();
        let verifying_key = cosmrs::crypto::secp256k1::VerifyingKey::from_sec1_bytes(&pub_key_bytes)
            .map_err(|e| MobError::Transaction(format!("Invalid public key: {}", e)))?;

        // Convert to cosmrs PublicKey
        let pub_key = cosmrs::crypto::PublicKey::from(verifying_key);

        let signer_info = tx::SignerInfo::single_direct(
            Some(pub_key),
            sequence,
        );

        Ok(signer_info.auth_info(cosmos_fee))
    }

    fn convert_fee(&self, fee: &Fee) -> Result<CosmosFee> {
        let mut coins = Vec::new();
        for c in &fee.amount {
            let denom = c.denom.parse()
                .map_err(|e| MobError::Transaction(format!("Invalid fee denom: {}", e)))?;
            let amount = c.amount.parse()
                .map_err(|e| MobError::Transaction(format!("Invalid fee amount: {}", e)))?;
            coins.push(cosmrs::Coin { denom, amount });
        }

        let first_coin = coins.first().ok_or(MobError::Transaction("No fee coins".to_string()))?;
        let fee_builder = CosmosFee::from_amount_and_gas(
            first_coin.clone(),
            fee.gas_limit,
        );

        Ok(fee_builder)
    }

    /// Get the chain ID
    pub fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    /// Get the number of messages
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }
}

/// Common transaction message builders
pub mod messages {
    use super::*;
    use cosmrs::{bank::MsgSend, cosmwasm::MsgExecuteContract, AccountId, Coin as CosmosCoin};

    /// Build a MsgSend for token transfer
    pub fn msg_send(
        from_address: &str,
        to_address: &str,
        amount: Vec<Coin>,
    ) -> Result<Any> {
        let from = AccountId::from_str(from_address)
            .map_err(|e| MobError::Transaction(format!("Invalid from address: {}", e)))?;

        let to = AccountId::from_str(to_address)
            .map_err(|e| MobError::Transaction(format!("Invalid to address: {}", e)))?;

        let coins: Vec<CosmosCoin> = amount
            .into_iter()
            .map(|c| c.into())
            .collect();

        let msg = MsgSend {
            from_address: from,
            to_address: to,
            amount: coins,
        };

        msg.to_any()
            .map_err(|e| MobError::Transaction(format!("Failed to create MsgSend: {}", e)))
    }

    /// Build a MsgExecuteContract for executing a CosmWasm contract
    pub fn msg_execute_contract(
        sender: &str,
        contract: &str,
        msg: &[u8],
        funds: Vec<Coin>,
    ) -> Result<Any> {
        let sender_addr = AccountId::from_str(sender)
            .map_err(|e| MobError::Transaction(format!("Invalid sender address: {}", e)))?;

        let contract_addr = AccountId::from_str(contract)
            .map_err(|e| MobError::Transaction(format!("Invalid contract address: {}", e)))?;

        let coins: Vec<CosmosCoin> = funds
            .into_iter()
            .map(|c| c.into())
            .collect();

        let msg = MsgExecuteContract {
            sender: sender_addr,
            contract: contract_addr,
            msg: msg.to_vec(),
            funds: coins,
        };

        msg.to_any()
            .map_err(|e| MobError::Transaction(format!("Failed to create MsgExecuteContract: {}", e)))
    }
}

/// Transaction response parser
pub mod response {
    use crate::types::TxResponse;
    use cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse as ProtoTxResponse;

    /// Parse a transaction response from proto
    pub fn parse_tx_response(proto_response: ProtoTxResponse) -> TxResponse {
        TxResponse {
            txhash: proto_response.txhash,
            code: proto_response.code,
            raw_log: proto_response.raw_log,
            gas_wanted: proto_response.gas_wanted as u64,
            gas_used: proto_response.gas_used as u64,
            height: proto_response.height,
        }
    }

    /// Check if a transaction was successful
    pub fn is_successful(response: &TxResponse) -> bool {
        response.code == 0
    }
}

/// Simulate a transaction to estimate gas
/// Only available with "rpc-client" feature (default)
#[cfg(feature = "rpc-client")]
pub async fn simulate_transaction(
    grpc_endpoint: &str,
    tx_bytes: Vec<u8>,
) -> Result<u64> {
    use cosmos_sdk_proto::cosmos::tx::v1beta1::{
        service_client::ServiceClient, SimulateRequest, TxRaw,
    };
    use prost::Message;
    use tonic::transport::Channel;

    // Parse tx_bytes into TxRaw proto (unused but validates the input)
    let _tx_raw = TxRaw::decode(tx_bytes.as_slice())
        .map_err(|e| MobError::Transaction(format!("Failed to decode transaction: {}", e)))?;

    let request = SimulateRequest {
        tx_bytes: tx_bytes.clone(),
        ..Default::default()
    };

    // Connect to gRPC endpoint
    let channel = Channel::from_shared(grpc_endpoint.to_string())
        .map_err(|e| MobError::Network(format!("Invalid gRPC endpoint: {}", e)))?
        .connect()
        .await
        .map_err(|e| MobError::Network(format!("Failed to connect to gRPC: {}", e)))?;

    let mut client = ServiceClient::new(channel);

    // Simulate the transaction
    let response = client
        .simulate(request)
        .await
        .map_err(|e| MobError::Transaction(format!("Simulation failed: {}", e)))?;

    let gas_info = response
        .into_inner()
        .gas_info
        .ok_or_else(|| MobError::Transaction("No gas info in simulation response".to_string()))?;

    Ok(gas_info.gas_used)
}

/// Simulate a transaction to estimate gas (WASM-compatible stub)
/// Returns a conservative default estimate when RPC client is not available
#[cfg(not(feature = "rpc-client"))]
pub fn simulate_transaction(_grpc_endpoint: &str, _tx_bytes: Vec<u8>) -> Result<u64> {
    // Conservative default gas estimate for WASM environments
    // gRPC simulation is not available without the rpc-client feature
    Ok(200_000)
}

/// Calculate fee from gas limit and gas price
pub fn calculate_fee(gas_limit: u64, gas_price: &str, denom: &str) -> Result<Fee> {
    let price: f64 = gas_price
        .parse()
        .map_err(|e| MobError::Transaction(format!("Invalid gas price: {}", e)))?;

    let amount = (gas_limit as f64 * price).ceil() as u64;

    Ok(Fee::new(
        vec![Coin::new(denom.to_string(), amount.to_string())],
        gas_limit,
    ))
}

#[cfg(test)]
mod tests {
    use super::{TransactionBuilder, calculate_fee};
    use crate::types::{BroadcastMode, Coin, Fee};

    #[test]
    fn test_transaction_builder() {
        let mut builder = TransactionBuilder::new("test-chain-1").unwrap();

        let fee = Fee::new(
            vec![Coin::new("uxion", "1000")],
            200_000,
        );

        builder
            .with_fee(fee)
            .with_memo("test transaction");

        assert_eq!(builder.message_count(), 0);
        assert_eq!(builder.memo, "test transaction");
    }

    #[test]
    fn test_calculate_fee() {
        let fee = calculate_fee(200_000, "0.025", "uxion").unwrap();
        assert_eq!(fee.gas_limit, 200_000);
        assert_eq!(fee.amount[0].denom, "uxion");
        assert_eq!(fee.amount[0].amount, "5000");
    }

    #[test]
    fn test_broadcast_mode_conversion() {
        // Test that our broadcast mode enum exists
        let _mode = BroadcastMode::Sync;
        let _mode = BroadcastMode::Async;
        let _mode = BroadcastMode::Block;
    }
}
