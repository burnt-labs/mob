# Quick Start Guide

This guide will help you get started with the Mob Ruby bindings for the XION blockchain.

## Basic RPC Query

```ruby
require_relative 'lib/mob'

# Create chain configuration
config = Mob::ChainConfig.new(
  chain_id: "xion-testnet-2",
  rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
  grpc_endpoint: nil,
  address_prefix: "xion",
  coin_type: 118,
  gas_price: "0.025"
)

# Create client
client = Mob::Client.new(config)

# Query blockchain height
height = client.get_height
puts "Current height: #{height}"

# Query chain ID
chain_id = client.get_chain_id
puts "Chain ID: #{chain_id}"

# Check sync status
is_synced = client.is_synced
puts "Node synced: #{is_synced}"
```

## Creating a Signer

```ruby
# Create from mnemonic
signer = Mob::Signer.from_mnemonic(
  "your twelve or twenty-four word mnemonic here",
  "xion",
  "m/44'/118'/0'/0/0"
)

# Get address
address = signer.address
puts "Address: #{address}"

# Get public key
pub_key = signer.public_key
puts "Public key: #{pub_key.inspect}"
```

## Querying Account Information

```ruby
# Get account info
account = client.get_account(address)
puts "Account number: #{account.account_number}"
puts "Sequence: #{account.sequence}"

# Get balance
balance = client.get_balance(address, "uxion")
puts "Balance: #{balance.amount} #{balance.denom}"
```

## Signing Messages

```ruby
# Sign arbitrary bytes
message = "Hello, XION!"
signature = signer.sign_bytes(message)
puts "Signature: #{signature.inspect}"
```

## Sending a Transaction

```ruby
# Configure client
config = Mob::ChainConfig.new(
  chain_id: "xion-testnet-2",
  rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
  grpc_endpoint: nil,
  address_prefix: "xion",
  coin_type: 118,
  gas_price: "0.025"
)

# Create signer
signer = Mob::Signer.from_mnemonic(
  "your mnemonic here",
  "xion",
  "m/44'/118'/0'/0/0"
)

# Create client with signer attached
client = Mob::Client.new_with_signer(config, signer)

# Send transaction
amount = [Mob::Coin.new(denom: "uxion", amount: "1000000")]
tx_response = client.send(
  "xion1recipient...",
  amount,
  "My first transaction"
)

puts "Transaction hash: #{tx_response.txhash}"
puts "Code: #{tx_response.code}"  # 0 = success

if tx_response.code == 0
  puts "Transaction successful!"
else
  puts "Transaction failed: #{tx_response.raw_log}"
end
```

## Querying Transaction Results

```ruby
# Query by transaction hash
tx_result = client.get_tx(tx_response.txhash)
puts "Block height: #{tx_result.height}"
puts "Gas used: #{tx_result.gas_used}"
puts "Gas wanted: #{tx_result.gas_wanted}"
```

## Complete Example

Here's a complete example that ties everything together:

```ruby
require_relative 'lib/mob'

# Configuration
config = Mob::ChainConfig.new(
  chain_id: "xion-testnet-2",
  rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
  grpc_endpoint: nil,
  address_prefix: "xion",
  coin_type: 118,
  gas_price: "0.025"
)

# Create signer from mnemonic
signer = Mob::Signer.from_mnemonic(
  ENV['TEST_MNEMONIC'] || "your test mnemonic here",
  "xion",
  "m/44'/118'/0'/0/0"
)

address = signer.address
puts "Using address: #{address}"

# Create client with signer
client = Mob::Client.new_with_signer(config, signer)

# Query current height
height = client.get_height
puts "Current height: #{height}"

# Query balance
balance = client.get_balance(address, "uxion")
puts "Balance: #{balance.amount} #{balance.denom}"

# Send tokens
recipient = "xion1recipient..."
amount = [Mob::Coin.new(denom: "uxion", amount: "1000")]

tx_response = client.send(recipient, amount, "Test transaction")

if tx_response.code == 0
  puts "Transaction successful"
  puts "Tx hash: #{tx_response.txhash}"
else
  puts "Transaction failed"
  puts "Error: #{tx_response.raw_log}"
end
```

## Next Steps

- Check out the [examples](examples/) directory for more detailed examples
- Read the [API Reference](README.md#api-reference) for complete documentation
- Run the [test suite](README.md#running-tests) to verify your setup

## Security Notes

Never use production mnemonics in code or tests.

Use environment variables for sensitive data:

```ruby
mnemonic = ENV['MNEMONIC']
raise "MNEMONIC not set" if mnemonic.nil?
```

## Troubleshooting

If you encounter issues:

1. Verify the library is in the correct location: `ruby/lib/libmob.dylib`
2. Check network connectivity to the RPC endpoint
3. Ensure your account has sufficient balance for transactions
4. Review the [Troubleshooting](README.md#troubleshooting) section in the main README
