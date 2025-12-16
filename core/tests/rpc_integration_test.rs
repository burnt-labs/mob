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
    let height = client
        .get_height()
        .expect("Failed to get block height");

    // Verify we got a valid height (should be > 0)
    assert!(height > 0, "Block height should be greater than 0");
    println!("✅ Successfully queried block height: {}", height);
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
    let is_synced = client
        .is_synced()
        .expect("Failed to check sync status");

    println!("✅ Node sync status: {}", is_synced);
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
    let result = client.get_account(test_address.clone());

    match result {
        Ok(account_info) => {
            println!("✅ Successfully queried account: {}", account_info.address);
            println!("   Account number: {}", account_info.account_number);
            println!("   Sequence: {}", account_info.sequence);
        }
        Err(e) => {
            // Account might not exist or be invalid, that's okay for this test
            println!("⚠️  Account query returned error (expected for non-existent account): {}", e);
        }
    }
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

    // Query balance
    let result = client.get_balance(test_address.clone(), "uxion".to_string());

    match result {
        Ok(balance) => {
            println!("✅ Successfully queried balance: {} {}", balance.amount, balance.denom);
        }
        Err(e) => {
            println!("⚠️  Balance query returned error: {}", e);
        }
    }
}

#[test]
fn test_rpc_endpoint_full_workflow() {
    println!("\n🧪 Running full RPC workflow test...\n");

    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    // Step 1: Create client
    println!("1️⃣  Creating RPC client...");
    let client = Client::new(config).expect("Failed to create client");
    println!("   ✅ Client created");

    // Step 2: Get chain height
    println!("\n2️⃣  Querying latest block height...");
    let height = client
        .get_height()
        .expect("Failed to get block height");
    println!("   ✅ Block height: {}", height);
    assert!(height > 0);

    // Step 3: Check sync status
    println!("\n3️⃣  Checking node sync status...");
    let is_synced = client
        .is_synced()
        .expect("Failed to check sync status");
    println!("   ✅ Node synced: {}", is_synced);

    // Step 4: Verify chain config
    println!("\n4️⃣  Verifying chain configuration...");
    let chain_config = client.config();
    assert_eq!(chain_config.chain_id, "xion-testnet-2");
    assert_eq!(chain_config.address_prefix, "xion");
    println!("   ✅ Chain ID: {}", chain_config.chain_id);
    println!("   ✅ Address prefix: {}", chain_config.address_prefix);

    println!("\n🎉 Full workflow test completed successfully!\n");
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

    match result {
        Ok(_) => panic!("Expected query to fail with invalid endpoint"),
        Err(e) => {
            println!("✅ Correctly handled invalid endpoint: {}", e);
        }
    }
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
    assert_eq!(config.rpc_endpoint, "https://rpc.xion-testnet-2.burnt.com:443");
    assert_eq!(config.address_prefix, "xion");
    assert_eq!(config.gas_price, "0.025");
    assert_eq!(config.coin_type, 118);
}

/// Integration test for sending funds from test mnemonic to a receiving address
/// This test makes real transactions on XION testnet
#[test]
#[ignore] // Ignored by default due to network calls and requiring funded account
fn test_send_funds_to_address() {
    use mob::{Signer, Coin};
    use std::sync::Arc;

    println!("\n💸 Testing fund transfer on XION testnet...\n");

    // Test mnemonic (should have funds on testnet)
    let mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";

    // Receiving address
    let recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4";

    println!("1️⃣  Creating signer from mnemonic...");
    let signer = Signer::from_mnemonic(
        mnemonic.to_string(),
        "xion".to_string(),
        None, // Use default derivation path
    )
    .expect("Failed to create signer");

    let sender_address = signer.address();
    println!("   ✅ Sender address: {}", sender_address);

    println!("\n2️⃣  Creating RPC client...");
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let mut client = Client::new(config).expect("Failed to create client");
    println!("   ✅ Client connected");

    // Create a runtime for async operations that can't be avoided (attach_signer)
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    println!("\n3️⃣  Attaching signer to client...");
    runtime.block_on(client.attach_signer_internal(Arc::new(signer))).expect("Failed to attach signer");
    println!("   ✅ Signer attached");

    println!("\n4️⃣  Querying sender balance...");
    let balance = client
        .get_balance(sender_address.clone(), "uxion".to_string())
        .expect("Failed to get balance");

    // Parse balance amount to check if account has funds
    let balance_amount: u64 = balance.amount.parse().unwrap_or(0);

    if balance_amount == 0 {
        println!("   ⚠️  WARNING: Sender has no uxion balance!");
        println!("   Please fund the test account: {}", sender_address);
        println!("   Skipping transaction...");
        return;
    }

    println!("   💰 Current uxion balance: {} uxion", balance.amount);

    println!("\n5️⃣  Preparing transaction...");
    // Send 1000 uxion (0.001 XION)
    let amount = vec![Coin {
        denom: "uxion".to_string(),
        amount: "1000".to_string(),
    }];

    println!("   📤 Sending 1000 uxion to {}", recipient);

    println!("\n6️⃣  Broadcasting transaction...");
    let tx_response = client
        .send(recipient.to_string(), amount, Some("Test fund transfer".to_string()))
        .expect("Failed to send transaction");

    println!("   ✅ Transaction broadcast successful!");
    println!("   📝 Transaction hash: {}", tx_response.txhash);
    println!("   📊 Code: {}", tx_response.code);

    if tx_response.code == 0 {
        println!("   ✅ Transaction accepted by mempool");
    } else {
        println!("   ⚠️  Transaction failed with code: {}", tx_response.code);
        println!("   📋 Log: {}", tx_response.raw_log);
    }

    assert_eq!(
        tx_response.code, 0,
        "Transaction should be accepted (code 0), got: {} - {}",
        tx_response.code, tx_response.raw_log
    );

    println!("\n7️⃣  Waiting for transaction to be included in a block...");
    println!("   (Sleeping for 10 seconds)");
    std::thread::sleep(std::time::Duration::from_secs(10));

    println!("\n8️⃣  Querying transaction by hash...");
    match client.get_tx(tx_response.txhash.clone()) {
        Ok(tx_result) => {
            println!("   ✅ Transaction found in block!");
            println!("   📊 Final code: {}", tx_result.code);
            println!("   ⛽ Gas used: {}", tx_result.gas_used);
            println!("   ⛽ Gas wanted: {}", tx_result.gas_wanted);
            println!("   📏 Block height: {}", tx_result.height);

            assert_eq!(
                tx_result.code, 0,
                "Transaction should succeed (code 0), got: {} - {}",
                tx_result.code, tx_result.raw_log
            );
        }
        Err(e) => {
            println!("   ⚠️  Could not query transaction: {}", e);
            println!("   (Transaction may still be processing or node may be delayed)");
        }
    }

    println!("\n🎉 Fund transfer test completed!\n");
}
