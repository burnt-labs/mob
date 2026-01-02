# Typed Protobuf Tests for Ruby

This document explains how to use the typed protobuf tests with xion-types in the Ruby client.

## Overview

The `test_typed_protobuf.rb` file demonstrates using strongly-typed protobuf messages from xion-types instead of untyped hashes or string manipulation. This provides:

- **Type Safety**: Runtime verification of message structure
- **IDE Support**: Autocomplete for message fields
- **Protobuf Validation**: Automatic field validation
- **Cosmos SDK Compatibility**: Guaranteed compatibility with Cosmos SDK message formats

## Prerequisites

### Install google-protobuf Gem

```bash
gem install google-protobuf
```

Or add to your Gemfile:

```ruby
gem 'google-protobuf'
```

### Ensure xion-types Ruby Files Exist

The xion-types protobuf files should be generated at:

```
/path/to/xion-types/ruby/types/
```

If not generated, run from the xion-types repository:

```bash
make proto-gen-ruby
```

## Running the Tests

```bash
# Run typed tests
ruby test/test_typed_protobuf.rb

# Run all Ruby tests
ruby test/test_rpc_queries.rb
ruby test/test_typed_protobuf.rb
```

## Example Usage Patterns

### Creating a Typed Coin

```ruby
require 'cosmos/base/v1beta1/coin_pb'

coin = Cosmos::Base::V1beta1::Coin.new(
  denom: "uxion",
  amount: "1000000"
)
```

### Creating a Typed MsgSend

```ruby
require 'cosmos/bank/v1beta1/tx_pb'
require 'cosmos/base/v1beta1/coin_pb'

coin = Cosmos::Base::V1beta1::Coin.new(
  denom: "uxion",
  amount: "1000000"
)

msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
  from_address: "xion1...",
  to_address: "xion1...",
  amount: [coin]
)
```

### Adding Multiple Coins

```ruby
coin1 = Cosmos::Base::V1beta1::Coin.new(
  denom: "uxion",
  amount: "1000000"
)

coin2 = Cosmos::Base::V1beta1::Coin.new(
  denom: "ustake",
  amount: "500000"
)

msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
  from_address: "xion1...",
  to_address: "xion1...",
  amount: [coin1, coin2]
)
```

### Creating Query Requests

```ruby
require 'cosmos/bank/v1beta1/query_pb'

balance_request = Cosmos::Bank::V1beta1::QueryBalanceRequest.new(
  address: "xion1...",
  denom: "uxion"
)

all_balances_request = Cosmos::Bank::V1beta1::QueryAllBalancesRequest.new(
  address: "xion1..."
)
```

### Serialization and Deserialization

```ruby
# Serialize to protobuf bytes
bytes = Cosmos::Bank::V1beta1::MsgSend.encode(msg_send)

# Deserialize from protobuf bytes
decoded = Cosmos::Bank::V1beta1::MsgSend.decode(bytes)

# Convert to JSON
json_string = Cosmos::Bank::V1beta1::MsgSend.encode_json(msg_send)

# Parse from JSON
parsed = Cosmos::Bank::V1beta1::MsgSend.decode_json(json_string)
```

### Modifying Messages

```ruby
# Create initial message
msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
  from_address: "xion1...",
  to_address: "xion1...",
  amount: []
)

# Add coins after creation
coin = Cosmos::Base::V1beta1::Coin.new(
  denom: "uxion",
  amount: "1000000"
)
msg_send.amount << coin
```

## Test Coverage

The typed tests cover:

1. **Coin Operations**
   - Creating typed coins
   - Serialization/deserialization
   - JSON encoding/decoding
   - DecCoin (fractional amounts)
   - Protobuf roundtrip

2. **MsgSend Operations**
   - Creating typed MsgSend messages
   - Single and multiple coins
   - Message serialization
   - JSON encoding
   - Message modification

3. **Query Operations**
   - QueryBalanceRequest
   - QueryBalanceResponse
   - QueryAllBalancesRequest
   - Request/response serialization

4. **Type Verification**
   - Descriptor verification
   - Empty message defaults
   - Field access patterns

## Benefits of Typed Messages

### Before (Untyped)

```ruby
# Hash-based message construction - error-prone
msg_json = {
  "from_address" => from_address,
  "to_address" => to_address,
  "amount" => [{"denom" => "uxion", "amount" => "1000000"}]
}.to_json
```

### After (Typed)

```ruby
# Strongly-typed construction - runtime safe
coin = Cosmos::Base::V1beta1::Coin.new(
  denom: "uxion",
  amount: "1000000"
)

msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
  from_address: from_address,
  to_address: to_address,
  amount: [coin]
)
```

## Type URLs

Common Cosmos SDK type URLs used in transactions:

| Message Type | Type URL |
|--------------|----------|
| MsgSend | /cosmos.bank.v1beta1.MsgSend |
| Coin | /cosmos.base.v1beta1.Coin |
| DecCoin | /cosmos.base.v1beta1.DecCoin |
| QueryBalanceRequest | /cosmos.bank.v1beta1.QueryBalanceRequest |

## Integration with Mob Client

The typed messages can be serialized and potentially used with the mob client:

```ruby
require_relative 'lib/mob'

# Create typed message
coin = Cosmos::Base::V1beta1::Coin.new(
  denom: "uxion",
  amount: "1000000"
)

msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
  from_address: signer.address,
  to_address: recipient_address,
  amount: [coin]
)

# Serialize to bytes for signing
msg_bytes = Cosmos::Bank::V1beta1::MsgSend.encode(msg_send)

# Use with mob client (when supported)
# client.broadcast_tx(msg_bytes)
```

## Loading Path Configuration

If xion-types is in a different location, update the load path in your test file:

```ruby
# Add xion-types to load path
$LOAD_PATH.unshift('/path/to/xion-types/ruby/types')

# Then require the protobuf files
require 'cosmos/base/v1beta1/coin_pb'
require 'cosmos/bank/v1beta1/tx_pb'
require 'cosmos/bank/v1beta1/query_pb'
```

## Additional Resources

- [Google Protocol Buffers Ruby Documentation](https://protobuf.dev/reference/ruby/ruby-generated/)
- [xion-types Repository](https://github.com/burnt-labs/xion-types)
- [Cosmos SDK Proto Definitions](https://github.com/cosmos/cosmos-sdk/tree/main/proto)
