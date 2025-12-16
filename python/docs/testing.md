# Python Tests for Mob Library

This directory contains Python tests that verify the mob library's functionality by querying the XION testnet RPC endpoint.

## Test Coverage

The test suite includes 20+ tests organized into the following categories:

### 1. Basic RPC Queries (`TestBasicRPCQueries`)
- ✅ `test_create_client` - Verify client instantiation
- ✅ `test_get_height` - Query current block height
- ✅ `test_get_chain_id` - Verify chain ID
- ✅ `test_get_node_info` - Fetch node information

### 2. Account Queries (`TestAccountQueries`)
- ✅ `test_signer_creation` - Create signer and verify address
- ✅ `test_get_account_number_and_sequence` - Query account metadata
- ✅ `test_get_balance` - Get all balances for an account
- ✅ `test_get_specific_balance` - Query specific denomination balance

### 3. Transaction Queries (`TestTransactionQueries`)
- ✅ `test_simulate_transaction` - Simulate a send transaction for gas estimation

### 4. Signing Functionality (`TestSigningFunctionality`)
- ✅ `test_sign_arbitrary_message` - Sign raw bytes
- ✅ `test_verify_signature` - Verify signature generation
- ✅ `test_get_public_key` - Extract public key from signer

### 5. Error Handling (`TestErrorHandling`)
- ✅ `test_invalid_rpc_endpoint` - Handle connection failures
- ✅ `test_invalid_mnemonic` - Handle invalid mnemonic phrases
- ✅ `test_query_nonexistent_account` - Handle missing accounts

### 6. Concurrency (`TestConcurrency`)
- ✅ `test_concurrent_height_queries` - Multiple parallel queries
- ✅ `test_concurrent_different_queries` - Different query types in parallel

### 7. Integration Tests (`TestIntegrationSendFunds`)
- 💸 `test_send_funds_to_address` - **End-to-end fund transfer on testnet**
  - Sends real funds from test mnemonic to recipient
  - Requires funded test account
  - Marked with `@pytest.mark.integration`

## Running the Tests

### Quick Start

```bash
# From project root
./scripts/run_python_tests.sh
```

This script will:
1. Install pytest if needed
2. Build and install the mob package if not already installed
3. Run all tests with verbose output

### Manual Execution

```bash
# Install dependencies
pip install pytest pytest-asyncio

# Install the package
maturin develop

# Run all tests
python -m pytest python/tests/test_rpc_queries.py -v

# Run specific test class
python -m pytest python/tests/test_rpc_queries.py::TestBasicRPCQueries -v

# Run specific test
python -m pytest python/tests/test_rpc_queries.py::TestBasicRPCQueries::test_get_height -v

# Run with output capture disabled (see print statements)
python -m pytest python/tests/test_rpc_queries.py -v -s
```

### Running Integration Tests

Integration tests require a funded test account and are marked with `@pytest.mark.integration`:

```bash
# Run only integration tests
python -m pytest python/tests/test_rpc_queries.py -m integration -v -s

# Run specific integration test
python -m pytest python/tests/test_rpc_queries.py::TestIntegrationSendFunds::test_send_funds_to_address -v -s

# Skip integration tests (run all other tests)
python -m pytest python/tests/test_rpc_queries.py -m "not integration" -v
```

**Prerequisites for integration tests:**
- Test account must be funded on XION testnet
- Sender address: `xion1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu`
- Minimum balance: 6000 uxion (1000 to send + 5000 for fees)

### Development Mode

```bash
# Install in development mode (editable)
maturin develop

# Run tests after code changes
python -m pytest python/tests/test_rpc_queries.py -v
```

## Test Configuration

Tests use the following configuration:

```python
RPC_ENDPOINT = "https://rpc.xion-testnet-2.burnt.com:443"
CHAIN_ID = "xion-testnet-2"
ADDRESS_PREFIX = "xion"
```

### Test Mnemonic

Tests use a test mnemonic for generating addresses:

```
quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull
```

**⚠️ WARNING:** This is a test mnemonic. NEVER use it for real funds!

## Expected Test Behavior

### Successful Tests

Tests that should always pass:
- Client creation
- Height queries
- Chain ID queries
- Signer creation
- Signing operations
- Concurrent queries

### Conditional Tests

Some tests may be skipped based on blockchain state:

1. **Account Queries**: If the test account doesn't exist on testnet, these tests will be skipped:
   - `test_get_account_number_and_sequence`
   - `test_get_balance`
   - `test_get_specific_balance`
   - `test_simulate_transaction`

2. **Error Handling**: Tests that expect errors should always pass:
   - Invalid RPC endpoints
   - Invalid mnemonics
   - Non-existent account queries

## Understanding Test Output

### Verbose Output Example

```
python/tests/test_rpc_queries.py::TestBasicRPCQueries::test_get_height PASSED
Current block height: 12345

python/tests/test_rpc_queries.py::TestAccountQueries::test_signer_creation PASSED
Test address: xion1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu

python/tests/test_rpc_queries.py::TestAccountQueries::test_get_balance SKIPPED
Balance query failed (expected if account doesn't exist): Account not found
```

### Test Markers

- ✅ `PASSED` - Test succeeded
- ⏭️ `SKIPPED` - Test skipped (usually due to missing testnet account)
- ❌ `FAILED` - Test failed (indicates a bug)

## Adding New Tests

To add new tests:

1. Create a new test class or add to existing one:

```python
class TestNewFeature:
    """Test description."""

    @pytest.mark.asyncio
    async def test_new_query(self, chain_config):
        """Test a new query."""
        client = await Client.new(chain_config)
        result = await client.new_method()
        assert result is not None
```

2. Use fixtures for common setup:
   - `chain_config` - Pre-configured ChainConfig
   - `test_signer` - Test signer with known mnemonic

3. Handle expected failures gracefully:

```python
try:
    result = await client.risky_query()
    assert result
except MobError as e:
    pytest.skip(f"Expected failure: {e}")
```

## Continuous Integration

The test suite is designed to run in CI/CD environments:

```yaml
# Example GitHub Actions workflow
- name: Run Python Tests
  run: |
    pip install maturin pytest pytest-asyncio
    maturin develop
    pytest python/tests/test_rpc_queries.py -v
```

## Troubleshooting

### Package Not Found

```
ImportError: No module named 'mob'
```

**Solution**: Build and install the package first:

```bash
maturin develop
```

### RPC Connection Failures

```
MobError: RPC error: Connection refused
```

**Possible causes:**
- RPC endpoint is down
- Network connectivity issues
- Firewall blocking HTTPS traffic

**Solution**: Verify RPC endpoint is accessible:

```bash
curl https://rpc.xion-testnet-2.burnt.com:443/status
```

### Test Timeouts

If tests timeout, increase pytest timeout:

```bash
pytest python/tests/test_rpc_queries.py --timeout=60
```

## Performance Notes

- Tests make real RPC calls, so they depend on network latency
- Concurrent tests verify async functionality works correctly
- Most queries should complete in < 2 seconds

## Integration Test Details

### `test_send_funds_to_address`

This integration test performs a complete end-to-end fund transfer on XION testnet.

**What it does:**
1. Creates a signer from the test mnemonic
2. Connects to XION testnet RPC
3. Queries the sender's balance (skips if no funds)
4. Sends 1000 uxion to `xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4`
5. Waits 10 seconds for block inclusion
6. Queries the transaction by hash to verify success

**Expected output:**
```
💸 Testing fund transfer on XION testnet...

1️⃣  Sender address: xion1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu

2️⃣  Creating RPC client...
   ✅ Client connected

3️⃣  Querying sender balance...
   💰 Current uxion balance: 1000000 uxion

4️⃣  Preparing transaction...
   📤 Sending 1000 uxion to xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4
   📝 Memo: Test fund transfer from Python

5️⃣  Broadcasting transaction...
   ✅ Transaction broadcast successful!
   📝 Transaction hash: ABC123...
   📊 Code: 0
   ✅ Transaction accepted by mempool

6️⃣  Waiting for transaction to be included in a block...
   (Sleeping for 10 seconds)

7️⃣  Querying transaction by hash...
   ✅ Transaction found in block!
   📊 Final code: 0
   ⛽ Gas used: 85234
   ⛽ Gas wanted: 200000
   📏 Block height: 12345

🎉 Fund transfer test completed!
```

**Handling missing funds:**

If the test account has no funds, the test will skip gracefully:
```
⚠️  WARNING: Sender has no uxion balance!
Please fund the test account: xion1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu
Skipping transaction...
```

## Future Enhancements

Potential additions to the test suite:

1. ✅ **Transaction Broadcasting**: ~~Full end-to-end transaction tests~~ **COMPLETED**
2. **Contract Interaction**: Test MsgExecuteContract functionality
3. **Abstract Accounts**: Test abstract account authenticators
4. **WebSocket Support**: Test subscription-based queries
5. **Multi-sig**: Test multi-signature transaction creation
6. **IBC Transfers**: Test cross-chain functionality

## Related Documentation

- [Main README](../../README.md)
- [Python Installation Guide](../../INSTALL_PYTHON.md)
- [Python Bindings Documentation](../../PYTHON_BINDINGS.md)
- [Rust Tests](../../core/tests/README.md)
