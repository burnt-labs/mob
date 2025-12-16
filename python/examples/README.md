# Mob Python Examples

This directory contains example scripts demonstrating how to use the mob Python library to interact with the XION blockchain.

## Prerequisites

Before running any examples, make sure you have:

1. Built the Python package:
   ```bash
   cd python/
   maturin develop
   ```

2. Installed required dependencies:
   ```bash
   pip install asyncio
   ```

## Examples

### 1. Basic Query (`basic_query.py`)

Demonstrates basic RPC queries to the XION testnet:
- Creating a client connection
- Querying block height
- Checking node sync status
- Getting chain ID

**Usage:**
```bash
python examples/basic_query.py
```

**Expected Output:**
```
🔗 Connecting to XION testnet...

📊 Querying blockchain information...
✅ Current block height: 1234567
✅ Synced
⛓️  Chain ID: xion-testnet-2

✨ Query complete!
```

### 2. Account Query (`account_query.py`)

Shows how to work with accounts:
- Creating a signer from a mnemonic
- Getting account addresses
- Querying account information
- Checking account balances

**Usage:**
```bash
python examples/account_query.py
```

**Note:** Update the mnemonic in the script to use your own test account.

**Expected Output:**
```
🔑 Creating signer from mnemonic...
📍 Address: xion1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu

🔗 Connecting to XION testnet...

📊 Querying account information...
✅ Account found!
   Account number: 12345
   Sequence: 10

💰 Querying balance...
✅ Balance: 1000000 uxion (1.000000 XION)

✨ Query complete!
```

### 3. Send Transaction (`send_transaction.py`)

Complete end-to-end example of sending tokens:
- Creating and configuring a client
- Checking account balance
- Building and sending a transaction
- Waiting for confirmation
- Querying transaction results

**⚠️ WARNING:** This example sends real tokens on the testnet!

**Usage:**
```bash
python examples/send_transaction.py
```

**Requirements:**
- A funded test account (minimum 6000 uxion for transaction + gas)
- Update the `MNEMONIC` constant with your test account mnemonic

**Expected Output:**
```
============================================================
🚀 XION Transaction Example
============================================================

🔑 Step 1: Creating signer from mnemonic...
   Sender address: xion1qypqxpq9qcrsszg2pvxq6rs0zqg3yyc5lzv7xu

🔗 Step 2: Connecting to XION testnet...

💰 Step 3: Checking balance...
   Current balance: 1000000 uxion (1.000000 XION)

📤 Step 4: Sending transaction...
   Recipient: xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4
   Amount: 1000 uxion

✅ Transaction broadcast successful!
   Transaction hash: ABC123...
   Code: 0 (0 = success)

⏳ Step 5: Waiting for transaction confirmation (10 seconds)...

🔍 Step 6: Querying transaction result...
   ✅ Transaction confirmed!
   Height: 1234567
   Gas used: 75000
   Gas wanted: 100000

💰 Step 7: Checking new balance...
   New balance: 994000 uxion (0.994000 XION)

============================================================
✨ Transaction example complete!
============================================================
```

## Configuration

All examples connect to the XION testnet by default:

- **Chain ID:** `xion-testnet-2`
- **RPC Endpoint:** `https://rpc.xion-testnet-2.burnt.com:443`
- **Address Prefix:** `xion`

To use a different network, modify the `ChainConfig` in each example:

```python
config = ChainConfig(
    chain_id="your-chain-id",
    rpc_endpoint="https://your-rpc-endpoint:443",
    bech32_prefix="your-prefix"
)
```

## Security Notes

⚠️ **IMPORTANT SECURITY WARNINGS:**

1. **Never use production mnemonics in examples or tests**
2. **Never commit mnemonics to version control**
3. **Use environment variables for sensitive data:**

```python
import os

mnemonic = os.environ.get("TEST_MNEMONIC")
if not mnemonic:
    raise ValueError("TEST_MNEMONIC environment variable not set")
```

4. **The test mnemonic in these examples is for demonstration only**
5. **Keep your mainnet keys secure and separate from testnet keys**

## Troubleshooting

### Import Error: "No module named 'mob'"

**Solution:** Build the package first:
```bash
cd python/
maturin develop
```

### "Failed to create client" or Connection Errors

**Possible causes:**
- Network connectivity issues
- RPC endpoint is down or unreachable
- Firewall blocking HTTPS connections

**Solution:** Verify the RPC endpoint is accessible:
```bash
curl https://rpc.xion-testnet-2.burnt.com:443/status
```

### "Insufficient funds" Error

**Cause:** The test account doesn't have enough balance.

**Solution:** Fund your test account via:
- XION testnet faucet (if available)
- Ask in the XION Discord for testnet tokens
- Transfer from another funded test account

### Transaction Fails with Code != 0

**Common causes:**
- Insufficient gas
- Invalid recipient address
- Account sequence mismatch
- Insufficient balance

**Solution:** Check the `raw_log` field in the transaction response for details:
```python
if tx_response.code != 0:
    print(f"Error: {tx_response.raw_log}")
```

## Further Learning

- **[Python Package README](../README.md)** - Full API reference
- **[Testing Guide](../docs/testing.md)** - Comprehensive testing documentation
- **[Mob Core](../../core/)** - Rust implementation details

## Contributing

Have a useful example to share? Contributions are welcome!

1. Create a new example file
2. Add clear documentation and comments
3. Include error handling
4. Update this README
5. Submit a pull request
