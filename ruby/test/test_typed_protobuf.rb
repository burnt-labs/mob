#!/usr/bin/env ruby
# frozen_string_literal: true

require 'minitest/autorun'

# Check if google-protobuf gem is available
begin
  require 'google/protobuf'
rescue LoadError
  puts "⚠️  google-protobuf gem not installed"
  puts "Install with: gem install google-protobuf"
  exit 1
end

# Load xion-types protobuf generated files
# NOTE: For production use, xion-types should be made an installable gem
# and added as a dependency, rather than using $LOAD_PATH manipulation.
# This relative path assumes xion-types is a sibling directory to mob.
xion_types_path = File.expand_path('../../../../xion-types/ruby/types', __FILE__)
$LOAD_PATH.unshift(xion_types_path) if File.directory?(xion_types_path)

begin
  require 'cosmos/base/v1beta1/coin_pb'
  require 'cosmos/bank/v1beta1/tx_pb'
  require 'cosmos/bank/v1beta1/query_pb'
rescue LoadError => e
  puts "⚠️  Failed to load xion-types protobuf files: #{e.message}"
  puts "Ensure xion-types Ruby protobufs are generated at:"
  puts "  Expected structure: mob/../xion-types/ruby/types"
  puts "  Resolved path: #{xion_types_path}"
  exit 1
end

# Typed protobuf tests using xion-types Ruby definitions
# These tests demonstrate using strongly-typed protobuf messages with the mob library
#
# Prerequisites:
# 1. xion-types Ruby protobuf files must be generated
# 2. LOAD_PATH must include xion-types/ruby/types
#
# Run with: ruby ruby/test/test_typed_protobuf.rb
class TestTypedProtobuf < Minitest::Test
  SENDER_ADDRESS = "xion1abc123def456ghi789jkl012mno345pqr678st"
  RECIPIENT_ADDRESS = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
  TEST_DENOM = "uxion"
  TEST_AMOUNT = "1000000"

  # MARK: - Typed Coin Tests

  def test_create_typed_coin
    # Create a strongly-typed Cosmos coin using protobuf
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: TEST_AMOUNT
    )

    assert_equal TEST_DENOM, coin.denom
    assert_equal TEST_AMOUNT, coin.amount
    puts "✅ Created typed Cosmos coin: #{coin.amount} #{coin.denom}"
  end

  def test_serialize_typed_coin
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: "500000"
    )

    # Encode to protobuf bytes
    data = Cosmos::Base::V1beta1::Coin.encode(coin)
    assert data.length > 0
    puts "✅ Serialized coin to #{data.length} bytes"

    # Decode from protobuf bytes
    decoded = Cosmos::Base::V1beta1::Coin.decode(data)
    assert_equal coin.denom, decoded.denom
    assert_equal coin.amount, decoded.amount
    puts "✅ Deserialized coin: #{decoded.amount} #{decoded.denom}"
  end

  def test_coin_to_json
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: "750000"
    )

    # Encode to JSON
    json_string = Cosmos::Base::V1beta1::Coin.encode_json(coin)

    assert json_string.include?(TEST_DENOM)
    assert json_string.include?("750000")
    puts "✅ Coin as JSON: #{json_string}"

    # Decode from JSON
    from_json = Cosmos::Base::V1beta1::Coin.decode_json(json_string)
    assert_equal coin.denom, from_json.denom
    assert_equal coin.amount, from_json.amount
    puts "✅ Decoded coin from JSON"
  end

  def test_coin_protobuf_roundtrip
    original = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: TEST_AMOUNT
    )

    # Serialize and deserialize
    bytes = Cosmos::Base::V1beta1::Coin.encode(original)
    decoded = Cosmos::Base::V1beta1::Coin.decode(bytes)

    assert_equal original.denom, decoded.denom
    assert_equal original.amount, decoded.amount
    puts "✅ Protobuf roundtrip successful"
  end

  # MARK: - Typed MsgSend Tests

  def test_create_typed_msg_send
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: "1000"
    )

    # Create typed MsgSend message
    msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
      from_address: SENDER_ADDRESS,
      to_address: RECIPIENT_ADDRESS,
      amount: [coin]
    )

    assert_equal SENDER_ADDRESS, msg_send.from_address
    assert_equal RECIPIENT_ADDRESS, msg_send.to_address
    assert_equal 1, msg_send.amount.length
    assert_equal TEST_DENOM, msg_send.amount[0].denom
    assert_equal "1000", msg_send.amount[0].amount

    puts "✅ Created typed MsgSend:"
    puts "   From: #{msg_send.from_address}"
    puts "   To: #{msg_send.to_address}"
    puts "   Amount: #{msg_send.amount[0].amount} #{msg_send.amount[0].denom}"
    puts "   Type URL: /cosmos.bank.v1beta1.MsgSend"
  end

  def test_serialize_msg_send
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: "2000"
    )

    msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
      from_address: SENDER_ADDRESS,
      to_address: RECIPIENT_ADDRESS,
      amount: [coin]
    )

    # Serialize to protobuf
    data = Cosmos::Bank::V1beta1::MsgSend.encode(msg_send)
    assert data.length > 0
    puts "✅ Serialized MsgSend to #{data.length} bytes"

    # Deserialize
    decoded = Cosmos::Bank::V1beta1::MsgSend.decode(data)
    assert_equal msg_send.from_address, decoded.from_address
    assert_equal msg_send.to_address, decoded.to_address
    assert_equal msg_send.amount.length, decoded.amount.length
    puts "✅ Deserialized MsgSend successfully"
  end

  def test_msg_send_to_json
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: "5000"
    )

    msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
      from_address: SENDER_ADDRESS,
      to_address: RECIPIENT_ADDRESS,
      amount: [coin]
    )

    # Convert to JSON
    json_string = Cosmos::Bank::V1beta1::MsgSend.encode_json(msg_send)

    assert json_string.include?("fromAddress")
    assert json_string.include?("toAddress")
    assert json_string.include?(TEST_DENOM)
    puts "✅ MsgSend as JSON:"
    puts "   #{json_string}"

    # Decode from JSON
    from_json = Cosmos::Bank::V1beta1::MsgSend.decode_json(json_string)
    assert_equal msg_send.from_address, from_json.from_address
    puts "✅ Decoded MsgSend from JSON"
  end

  def test_msg_send_multiple_coins
    coin1 = Cosmos::Base::V1beta1::Coin.new(
      denom: "uxion",
      amount: "1000"
    )

    coin2 = Cosmos::Base::V1beta1::Coin.new(
      denom: "uatom",
      amount: "500"
    )

    msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
      from_address: SENDER_ADDRESS,
      to_address: RECIPIENT_ADDRESS,
      amount: [coin1, coin2]
    )

    assert_equal 2, msg_send.amount.length
    assert_equal "uxion", msg_send.amount[0].denom
    assert_equal "uatom", msg_send.amount[1].denom

    puts "✅ Created MsgSend with multiple coins:"
    msg_send.amount.each do |coin|
      puts "   #{coin.amount} #{coin.denom}"
    end
  end

  # MARK: - Typed Query Tests

  def test_create_typed_query_balance_request
    request = Cosmos::Bank::V1beta1::QueryBalanceRequest.new(
      address: SENDER_ADDRESS,
      denom: TEST_DENOM
    )

    assert_equal SENDER_ADDRESS, request.address
    assert_equal TEST_DENOM, request.denom

    puts "✅ Created typed QueryBalanceRequest:"
    puts "   Address: #{request.address}"
    puts "   Denom: #{request.denom}"
    puts "   Type URL: /cosmos.bank.v1beta1.QueryBalanceRequest"
  end

  def test_serialize_query_balance_request
    request = Cosmos::Bank::V1beta1::QueryBalanceRequest.new(
      address: SENDER_ADDRESS,
      denom: TEST_DENOM
    )

    # Serialize to protobuf
    data = Cosmos::Bank::V1beta1::QueryBalanceRequest.encode(request)
    assert data.length > 0
    puts "✅ Serialized QueryBalanceRequest to #{data.length} bytes"

    # Deserialize
    decoded = Cosmos::Bank::V1beta1::QueryBalanceRequest.decode(data)
    assert_equal request.address, decoded.address
    assert_equal request.denom, decoded.denom
    puts "✅ Deserialized QueryBalanceRequest successfully"
  end

  def test_create_typed_query_balance_response
    coin = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: TEST_AMOUNT
    )

    response = Cosmos::Bank::V1beta1::QueryBalanceResponse.new(
      balance: coin
    )

    assert_equal TEST_DENOM, response.balance.denom
    assert_equal TEST_AMOUNT, response.balance.amount

    puts "✅ Created typed QueryBalanceResponse:"
    puts "   Balance: #{response.balance.amount} #{response.balance.denom}"

    # Test serialization roundtrip
    data = Cosmos::Bank::V1beta1::QueryBalanceResponse.encode(response)
    decoded = Cosmos::Bank::V1beta1::QueryBalanceResponse.decode(data)
    assert_equal response.balance.amount, decoded.balance.amount
    puts "✅ Roundtrip serialization successful"
  end

  def test_create_typed_query_all_balances_request
    request = Cosmos::Bank::V1beta1::QueryAllBalancesRequest.new(
      address: SENDER_ADDRESS,
      resolve_denom: false
    )

    assert_equal SENDER_ADDRESS, request.address
    assert_equal false, request.resolve_denom

    puts "✅ Created typed QueryAllBalancesRequest:"
    puts "   Address: #{request.address}"
    puts "   Type URL: /cosmos.bank.v1beta1.QueryAllBalancesRequest"
  end

  # MARK: - Type Descriptor Tests

  def test_protobuf_descriptors
    # Verify type descriptors exist
    coin_descriptor = Cosmos::Base::V1beta1::Coin.descriptor
    msg_send_descriptor = Cosmos::Bank::V1beta1::MsgSend.descriptor
    query_balance_descriptor = Cosmos::Bank::V1beta1::QueryBalanceRequest.descriptor

    refute_nil coin_descriptor
    refute_nil msg_send_descriptor
    refute_nil query_balance_descriptor

    puts "✅ All protobuf type descriptors verified:"
    puts "   Coin: cosmos.base.v1beta1.Coin"
    puts "   MsgSend: cosmos.bank.v1beta1.MsgSend"
    puts "   QueryBalanceRequest: cosmos.bank.v1beta1.QueryBalanceRequest"
  end

  def test_dec_coin_creation
    # Create DecCoin (decimal coin for fractional amounts)
    dec_coin = Cosmos::Base::V1beta1::DecCoin.new(
      denom: TEST_DENOM,
      amount: "1000000.123456789"
    )

    assert_equal TEST_DENOM, dec_coin.denom
    assert_equal "1000000.123456789", dec_coin.amount
    puts "✅ Created typed DecCoin: #{dec_coin.amount} #{dec_coin.denom}"
  end

  def test_empty_message_defaults
    # Create empty message and verify defaults
    msg_send = Cosmos::Bank::V1beta1::MsgSend.new

    assert_equal "", msg_send.from_address
    assert_equal "", msg_send.to_address
    assert_equal 0, msg_send.amount.length
    puts "✅ Created empty MsgSend with default values"
  end

  def test_message_modification
    # Create initial message
    coin1 = Cosmos::Base::V1beta1::Coin.new(
      denom: TEST_DENOM,
      amount: "1000000"
    )

    msg_send = Cosmos::Bank::V1beta1::MsgSend.new(
      from_address: SENDER_ADDRESS,
      to_address: RECIPIENT_ADDRESS,
      amount: [coin1]
    )

    # Modify by adding another coin
    coin2 = Cosmos::Base::V1beta1::Coin.new(
      denom: "ustake",
      amount: "500000"
    )
    msg_send.amount << coin2

    assert_equal 2, msg_send.amount.length
    assert_equal "uxion", msg_send.amount[0].denom
    assert_equal "ustake", msg_send.amount[1].denom
    puts "✅ Modified MsgSend now has #{msg_send.amount.length} coins"
  end
end
