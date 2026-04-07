use mob::{ChainConfig, Client, HttpTransport, UreqTransport};
use std::sync::Arc;

fn transport() -> Arc<dyn HttpTransport> {
    Arc::new(UreqTransport::new())
}

/// Integration test for RPC endpoint connectivity
/// This test makes real network calls to the XION testnet RPC endpoint
#[test]
fn test_rpc_endpoint_get_height() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config, transport()).expect("Failed to create client");

    let height = client.get_height().expect("Failed to get block height");

    assert!(height > 0, "Block height should be greater than 0");
}

#[test]
fn test_rpc_endpoint_is_synced() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config, transport()).expect("Failed to create client");

    let _is_synced = client.is_synced().expect("Failed to check sync status");
}

#[test]
fn test_rpc_endpoint_get_account() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config, transport()).expect("Failed to create client");

    let test_address = "xion1test".to_string();
    let _result = client.get_account(test_address.clone());
}

#[test]
fn test_rpc_endpoint_get_balance() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config, transport()).expect("Failed to create client");

    let test_address = "xion1test".to_string();
    let _result = client.get_balance(test_address.clone(), "uxion".to_string());
}

#[test]
fn test_rpc_endpoint_full_workflow() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config, transport()).expect("Failed to create client");

    let height = client.get_height().expect("Failed to get block height");
    assert!(height > 0);

    let _is_synced = client.is_synced().expect("Failed to check sync status");

    let chain_config = client.config();
    assert_eq!(chain_config.chain_id, "xion-testnet-2");
    assert_eq!(chain_config.address_prefix, "xion");
}

/// Test that verifies we can handle network errors gracefully
#[test]
fn test_invalid_rpc_endpoint() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://invalid-rpc-endpoint.example.com:443",
        "xion",
    );

    let client = Client::new(config, transport()).expect("Client creation should succeed");

    let result = client.get_height();

    assert!(
        result.is_err(),
        "Expected query to fail with invalid endpoint"
    );
}

/// Test chain configuration builder
#[test]
fn test_chain_config_builder() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    )
    .with_gas_price("0.025".to_string())
    .with_coin_type(118);

    assert_eq!(config.chain_id, "xion-testnet-2");
    assert_eq!(
        config.rpc_endpoint,
        "https://rpc.xion-testnet-2.burnt.com:443"
    );
    assert_eq!(config.address_prefix, "xion");
    assert_eq!(config.gas_price, "0.025");
    assert_eq!(config.coin_type, 118);
}

/// Integration test for sending funds from test mnemonic to a receiving address
/// This test makes real transactions on XION testnet
#[test]
#[ignore] // Ignored by default due to network calls and requiring funded account
fn test_send_funds_to_address() {
    use mob::{Coin, RustSigner};

    let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";

    let recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4";

    let signer = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None)
        .expect("Failed to create signer");

    let sender_address = signer.address();

    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let mut client = Client::new(config, transport()).expect("Failed to create client");

    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    runtime
        .block_on(client.attach_signer_internal(Arc::new(signer)))
        .expect("Failed to attach signer");

    let balance = client
        .get_balance(sender_address.clone(), "uxion".to_string())
        .expect("Failed to get balance");

    let balance_amount: u64 = balance.amount.parse().unwrap_or(0);

    if balance_amount == 0 {
        eprintln!("WARNING: Sender has no uxion balance");
        eprintln!("Please fund the test account: {}", sender_address);
        return;
    }

    let amount = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000".to_string(),
    }];

    let tx_response = client
        .send(
            recipient.to_string(),
            amount,
            Some("Test fund transfer".to_string()),
        )
        .expect("Failed to send transaction");

    assert_eq!(
        tx_response.code, 0,
        "Transaction should be accepted (code 0), got: {} - {}",
        tx_response.code, tx_response.raw_log
    );

    std::thread::sleep(std::time::Duration::from_secs(10));

    if let Ok(tx_result) = client.get_tx(tx_response.txhash.clone()) {
        assert_eq!(
            tx_result.code, 0,
            "Transaction should succeed (code 0), got: {} - {}",
            tx_result.code, tx_result.raw_log
        );
    }
}
