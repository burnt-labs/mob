# Integration Tests

This directory contains integration tests that make real network calls to the XION testnet.

## Running Tests

### Run All Tests (Excluding Network Tests)

```bash
cargo test
```

### Run Integration Tests with Network Calls

The RPC integration tests are marked with `#[ignore]` by default to prevent making network calls during regular test runs.

To run them:

```bash
# Run all ignored tests
cargo test --test rpc_integration_test -- --ignored

# Run specific test
cargo test --test rpc_integration_test test_rpc_endpoint_get_height -- --ignored

# Run with verbose output
cargo test --test rpc_integration_test -- --ignored --nocapture
```

### Run the Full Workflow Test

```bash
cargo test --test rpc_integration_test test_rpc_endpoint_full_workflow -- --ignored --nocapture
```

This will:
1. Create an RPC client connection
2. Query the latest block height
3. Check node sync status
4. Verify chain configuration

## Test Descriptions

### `test_rpc_endpoint_get_height`
Tests basic connectivity by querying the current block height from the testnet.

**Expected Result:** Returns a block height > 0

### `test_rpc_endpoint_is_synced`
Checks if the RPC node is fully synced with the network.

**Expected Result:** Returns sync status (true/false)

### `test_rpc_endpoint_get_account`
Attempts to query account information for a test address.

**Expected Result:** Returns account info or error (depending on whether account exists)

### `test_rpc_endpoint_get_balance`
Queries the balance of a test address.

**Expected Result:** Returns balance or error

### `test_rpc_endpoint_full_workflow`
Comprehensive test that runs through a complete workflow of client operations.

**Expected Result:** All operations succeed and return valid data

### `test_invalid_rpc_endpoint`
Tests error handling with an invalid RPC endpoint.

**Expected Result:** Client creation fails gracefully

### `test_chain_config_builder`
Unit test for the ChainConfig builder (no network call).

**Expected Result:** Config built correctly with all parameters

## RPC Endpoint

All tests use the XION testnet-2 RPC endpoint:
```
https://rpc.xion-testnet-2.burnt.com:443
```

## Troubleshooting

### Tests Fail with Network Error

The RPC endpoint might be temporarily unavailable. Check:
1. Network connectivity
2. RPC endpoint status
3. Firewall settings

### Tests Timeout

Some operations may take time depending on network conditions. You can increase the timeout:

```rust
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_with_timeout() {
    // Your test code
}
```

### Account/Balance Queries Return Errors

This is expected if the test address doesn't exist on the testnet. Replace with a real testnet address:

```rust
let test_address = "xion1youraddresshere".to_string();
```

## Adding New Tests

When adding new integration tests:

1. Mark them with `#[ignore]` if they make network calls
2. Use `#[tokio::test]` for async tests
3. Add descriptive assertions and error messages
4. Document expected behavior in comments

Example:

```rust
#[tokio::test]
#[ignore]
async fn test_new_rpc_feature() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    let client = Client::new(config).await.unwrap();

    // Test your feature here
    let result = client.some_new_method().await;

    assert!(result.is_ok(), "Should succeed");
}
```

## CI/CD Integration

For CI/CD pipelines, you may want to:

1. Run regular tests on every commit:
   ```bash
   cargo test
   ```

2. Run integration tests on a schedule or before releases:
   ```bash
   cargo test -- --ignored
   ```

3. Set environment variables for test configuration:
   ```bash
   export XION_RPC_ENDPOINT="https://rpc.xion-testnet-2.burnt.com:443"
   cargo test -- --ignored
   ```
