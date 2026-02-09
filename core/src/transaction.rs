use crate::{
    crypto_signer::CryptoSigner,
    error::{MobError, Result},
    types::{Coin, Fee},
};
use cosmrs::{
    tendermint::chain::Id as ChainId,
    tx::{self, Body, BodyBuilder, Fee as CosmosFee, Msg, SignDoc},
    Any,
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
            body_builder.timeout_height(
                cosmrs::tendermint::block::Height::try_from(self.timeout_height)
                    .map_err(|e| MobError::Transaction(format!("Invalid timeout height: {}", e)))?,
            );
        }

        Ok(body_builder.finish())
    }

    /// Build and sign the transaction
    pub fn sign(
        &self,
        signer: &dyn CryptoSigner,
        account_number: u64,
        sequence: u64,
    ) -> Result<Vec<u8>> {
        let body = self.build_body()?;

        // Get fee or use default
        let fee = self
            .fee
            .as_ref()
            .ok_or_else(|| MobError::Transaction("Transaction fee not set".to_string()))?;

        // Create auth info
        let auth_info = self.create_auth_info(signer, fee, sequence)?;

        // Create SignDoc
        let sign_doc = SignDoc::new(&body, &auth_info, &self.chain_id, account_number)?;

        // Sign the transaction using the trait method
        let tx_raw = self.sign_with_trait(signer, &sign_doc, account_number)?;

        // Serialize to bytes
        let tx_bytes = tx_raw.to_bytes()?;
        Ok(tx_bytes)
    }

    /// Sign a transaction using only CryptoSigner trait methods
    fn sign_with_trait(
        &self,
        signer: &dyn CryptoSigner,
        sign_doc: &SignDoc,
        account_number: u64,
    ) -> Result<tx::Raw> {
        use prost::Message;

        // Encode SignDoc to protobuf bytes
        let mut sign_doc_bytes = Vec::new();
        let sign_doc_proto = xion_types::cosmos::tx::v1beta1::SignDoc {
            body_bytes: sign_doc.body_bytes.clone(),
            auth_info_bytes: sign_doc.auth_info_bytes.clone(),
            chain_id: sign_doc.chain_id.to_string(),
            account_number,
        };
        sign_doc_proto
            .encode(&mut sign_doc_bytes)
            .map_err(|e| MobError::Signing(format!("Failed to encode SignDoc: {}", e)))?;

        // Sign the bytes using the trait method
        let signature = signer
            .sign_bytes(sign_doc_bytes)
            .map_err(|e| MobError::Signing(e.to_string()))?;

        // Create raw transaction
        let tx_raw_proto = xion_types::cosmos::tx::v1beta1::TxRaw {
            body_bytes: sign_doc.body_bytes.clone(),
            auth_info_bytes: sign_doc.auth_info_bytes.clone(),
            signatures: vec![signature],
        };

        // Encode and decode back to cosmrs Raw type
        let mut tx_raw_bytes = Vec::new();
        tx_raw_proto
            .encode(&mut tx_raw_bytes)
            .map_err(|e| MobError::Transaction(format!("Failed to encode tx: {}", e)))?;

        let tx_raw = tx::Raw::from_bytes(&tx_raw_bytes)
            .map_err(|e| MobError::Transaction(format!("Failed to create Raw tx: {}", e)))?;

        Ok(tx_raw)
    }

    fn create_auth_info(
        &self,
        signer: &dyn CryptoSigner,
        fee: &Fee,
        sequence: u64,
    ) -> Result<tx::AuthInfo> {
        let cosmos_fee = self.convert_fee(fee)?;

        // Create public key from bytes (33 bytes compressed format)
        let pub_key_bytes = signer.public_key();
        let verifying_key =
            cosmrs::crypto::secp256k1::VerifyingKey::from_sec1_bytes(&pub_key_bytes)
                .map_err(|e| MobError::Transaction(format!("Invalid public key: {}", e)))?;

        // Convert to cosmrs PublicKey
        let pub_key = cosmrs::crypto::PublicKey::from(verifying_key);

        let signer_info = tx::SignerInfo::single_direct(Some(pub_key), sequence);

        Ok(signer_info.auth_info(cosmos_fee))
    }

    fn convert_fee(&self, fee: &Fee) -> Result<CosmosFee> {
        let mut coins = Vec::new();
        for c in &fee.amount {
            let denom = c
                .denom
                .parse()
                .map_err(|e| MobError::Transaction(format!("Invalid fee denom: {}", e)))?;
            let amount = c
                .amount
                .parse()
                .map_err(|e| MobError::Transaction(format!("Invalid fee amount: {}", e)))?;
            coins.push(cosmrs::Coin { denom, amount });
        }

        let first_coin = coins
            .first()
            .ok_or(MobError::Transaction("No fee coins".to_string()))?;
        let fee_builder = CosmosFee::from_amount_and_gas(first_coin.clone(), fee.gas_limit);

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
    pub fn msg_send(from_address: &str, to_address: &str, amount: Vec<Coin>) -> Result<Any> {
        let from = AccountId::from_str(from_address)
            .map_err(|e| MobError::Transaction(format!("Invalid from address: {}", e)))?;

        let to = AccountId::from_str(to_address)
            .map_err(|e| MobError::Transaction(format!("Invalid to address: {}", e)))?;

        let coins: Vec<CosmosCoin> = amount.into_iter().map(|c| c.into()).collect();

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

        let coins: Vec<CosmosCoin> = funds.into_iter().map(|c| c.into()).collect();

        let msg = MsgExecuteContract {
            sender: sender_addr,
            contract: contract_addr,
            msg: msg.to_vec(),
            funds: coins,
        };

        msg.to_any().map_err(|e| {
            MobError::Transaction(format!("Failed to create MsgExecuteContract: {}", e))
        })
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
    use super::{calculate_fee, TransactionBuilder};
    use crate::types::{BroadcastMode, Coin, Fee};

    #[test]
    fn test_transaction_builder() {
        let mut builder = TransactionBuilder::new("test-chain-1").unwrap();

        let fee = Fee::new(vec![Coin::new("uxion", "1000")], 200_000);

        builder.with_fee(fee).with_memo("test transaction");

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
