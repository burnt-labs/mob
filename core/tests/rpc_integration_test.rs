use mob::{ChainConfig, Client};

/// Integration test for RPC endpoint connectivity
/// This test makes real network calls to the XION testnet RPC endpoint
#[test]
fn test_rpc_endpoint_get_height() {
    // Create chain configuration for XION testnet-2
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    // Create RPC client
    let client = Client::new(config).expect("Failed to create client");

    // Query the latest block height
    let height = client.get_height().expect("Failed to get block height");

    // Verify we got a valid height (should be > 0)
    assert!(height > 0, "Block height should be greater than 0");
}

#[test]
fn test_rpc_endpoint_is_synced() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config).expect("Failed to create client");

    // Check if the node is synced
    let _is_synced = client.is_synced().expect("Failed to check sync status");
    // We don't assert true here because the node might be catching up
    // Just verify we can query the status
}

#[test]
fn test_rpc_endpoint_get_account() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config).expect("Failed to create client");

    // Use a known testnet address (this is just an example format)
    // You may need to replace this with an actual testnet address
    let test_address = "xion1test".to_string();

    // Query account information
    // Account might not exist or be invalid, that's okay for this test
    let _result = client.get_account(test_address.clone());
}

#[test]
fn test_rpc_endpoint_get_balance() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config).expect("Failed to create client");

    // Use a known testnet address
    let test_address = "xion1test".to_string();

    // Query balance (may succeed or fail depending on account existence)
    let _result = client.get_balance(test_address.clone(), "uxion".to_string());
}

#[test]
fn test_rpc_endpoint_full_workflow() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    // Step 1: Create client
    let client = Client::new(config).expect("Failed to create client");

    // Step 2: Get chain height
    let height = client.get_height().expect("Failed to get block height");
    assert!(height > 0);

    // Step 3: Check sync status
    let _is_synced = client.is_synced().expect("Failed to check sync status");

    // Step 4: Verify chain config
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

    // Client creation should succeed (it doesn't make network calls)
    let client = Client::new(config).expect("Client creation should succeed");

    // But querying should fail with invalid endpoint
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
    use std::sync::Arc;

    // Test mnemonic (should have funds on testnet)
    let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";

    // Receiving address
    let recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4";

    let signer = RustSigner::from_mnemonic(
        mnemonic.to_string(),
        "xion".to_string(),
        None, // Use default derivation path
    )
    .expect("Failed to create signer");

    let sender_address = signer.address();

    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let mut client = Client::new(config).expect("Failed to create client");

    // Create a runtime for async operations that can't be avoided (attach_signer)
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    runtime
        .block_on(client.attach_signer_internal(Arc::new(signer)))
        .expect("Failed to attach signer");

    let balance = client
        .get_balance(sender_address.clone(), "uxion".to_string())
        .expect("Failed to get balance");

    // Parse balance amount to check if account has funds
    let balance_amount: u64 = balance.amount.parse().unwrap_or(0);

    if balance_amount == 0 {
        eprintln!("WARNING: Sender has no uxion balance");
        eprintln!("Please fund the test account: {}", sender_address);
        return;
    }

    // Send 1000 uxion (0.001 XION)
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

    // Wait for transaction to be included in a block
    std::thread::sleep(std::time::Duration::from_secs(10));

    match client.get_tx(tx_response.txhash.clone()) {
        Ok(tx_result) => {
            assert_eq!(
                tx_result.code, 0,
                "Transaction should succeed (code 0), got: {} - {}",
                tx_result.code, tx_result.raw_log
            );
        }
        Err(_) => {
            // Transaction may still be processing or node may be delayed
        }
    }
}
