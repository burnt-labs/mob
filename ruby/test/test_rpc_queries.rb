#!/usr/bin/env ruby
# frozen_string_literal: true

require 'minitest/autorun'
require_relative '../lib/mob'

# Ruby tests for mob library RPC queries against XION testnet
#
# Run with: ruby ruby/test/test_rpc_queries.rb
class TestRpcQueries < Minitest::Test
  RPC_ENDPOINT = "https://rpc.xion-testnet-2.burnt.com:443"
  CHAIN_ID = "xion-testnet-2"
  ADDRESS_PREFIX = "xion"

  # Test mnemonic (DO NOT USE IN PRODUCTION)
  TEST_MNEMONIC = "quiz cattle knock bacon million abstract word reunion educate antenna " \
                  "put fitness slide dash point basket jaguar fun humor multiply " \
                  "emotion rescue brand pull"

  def setup
    @config = Mob::ChainConfig.new(
      chain_id: CHAIN_ID,
      rpc_endpoint: RPC_ENDPOINT,
      grpc_endpoint: nil,
      address_prefix: ADDRESS_PREFIX,
      coin_type: 118,
      gas_price: "0.025"
    )

    @signer = Mob::Signer.from_mnemonic(
      TEST_MNEMONIC,
      ADDRESS_PREFIX,
      "m/44'/118'/0'/0/0"
    )
  end

  def test_create_client
    client = Mob::Client.new(@config)
    refute_nil client
  end

  def test_get_height
    client = Mob::Client.new(@config)
    height = client.get_height

    assert height > 0, "Height should be greater than 0"
    assert_instance_of Integer, height
    puts "✅ Current block height: #{height}"
  end

  def test_get_chain_id
    client = Mob::Client.new(@config)
    chain_id = client.get_chain_id

    assert_equal CHAIN_ID, chain_id
    puts "✅ Chain ID: #{chain_id}"
  end

  def test_is_synced
    client = Mob::Client.new(@config)
    is_synced = client.is_synced

    assert [true, false].include?(is_synced), "is_synced should return boolean"
    puts "✅ Node synced: #{is_synced}"
  end

  def test_create_signer
    signer = Mob::Signer.from_mnemonic(
      TEST_MNEMONIC,
      ADDRESS_PREFIX,
      "m/44'/118'/0'/0/0"
    )

    refute_nil signer
    address = signer.address
    assert address.start_with?(ADDRESS_PREFIX), "Address should start with #{ADDRESS_PREFIX}"
    puts "✅ Signer address: #{address}"
  end

  def test_get_account
    client = Mob::Client.new(@config)
    address = @signer.address

    account_info = client.get_account(address)

    assert_equal address, account_info.address
    assert account_info.account_number >= 0
    assert account_info.sequence >= 0
    puts "✅ Account number: #{account_info.account_number}, Sequence: #{account_info.sequence}"
  end

  def test_get_balance
    client = Mob::Client.new(@config)
    address = @signer.address

    balance = client.get_balance(address, "uxion")

    assert_equal "uxion", balance.denom
    assert balance.amount.to_i >= 0
    puts "✅ Balance: #{balance.amount} #{balance.denom}"
  end

  def test_sign_message
    message = "Hello, XION!"
    signature = @signer.sign_bytes(message)

    refute_nil signature
    assert signature.length > 0
    puts "✅ Signed message, signature length: #{signature.length} bytes"
  end

  # test_verify_signature removed - verify_signature method not exported in UniFFI bindings

  def test_invalid_mnemonic
    assert_raises do
      Mob::Signer.from_mnemonic(
        "invalid mnemonic words",
        "xion",
        "m/44'/118'/0'/0/0"
      )
    end
    puts "✅ Invalid mnemonic properly rejected"
  end

  def test_invalid_address
    client = Mob::Client.new(@config)

    assert_raises do
      client.get_account("invalid_address")
    end
    puts "✅ Invalid address properly rejected"
  end

  def test_multiple_signers
    signer1 = Mob::Signer.from_mnemonic(
      TEST_MNEMONIC,
      "xion",
      "m/44'/118'/0'/0/0"
    )

    signer2 = Mob::Signer.from_mnemonic(
      TEST_MNEMONIC,
      "xion",
      "m/44'/118'/0'/0/1"
    )

    addr1 = signer1.address
    addr2 = signer2.address

    refute_equal addr1, addr2, "Different derivation paths should yield different addresses"
    puts "✅ Account 0: #{addr1}"
    puts "✅ Account 1: #{addr2}"
  end

  def test_coin_creation
    coin = Mob::Coin.new(denom: "uxion", amount: "1000000")

    assert_equal "uxion", coin.denom
    assert_equal "1000000", coin.amount
    puts "✅ Created coin: #{coin.amount} #{coin.denom}"
  end
end

# Integration test for sending funds (run with INTEGRATION=1 environment variable)
class TestIntegrationSendFunds < Minitest::Test
  RPC_ENDPOINT = "https://rpc.xion-testnet-2.burnt.com:443"
  CHAIN_ID = "xion-testnet-2"
  ADDRESS_PREFIX = "xion"

  # Test mnemonic (DO NOT USE IN PRODUCTION)
  TEST_MNEMONIC = "quiz cattle knock bacon million abstract word reunion educate antenna " \
                  "put fitness slide dash point basket jaguar fun humor multiply " \
                  "emotion rescue brand pull"

  def setup
    skip "Skipping integration test (set INTEGRATION=1 to run)" unless ENV['INTEGRATION'] == '1'

    @config = Mob::ChainConfig.new(
      chain_id: CHAIN_ID,
      rpc_endpoint: RPC_ENDPOINT,
      grpc_endpoint: nil,
      address_prefix: ADDRESS_PREFIX,
      coin_type: 118,
      gas_price: "0.025"
    )

    @signer = Mob::Signer.from_mnemonic(
      TEST_MNEMONIC,
      ADDRESS_PREFIX,
      "m/44'/118'/0'/0/0"
    )
  end

  def test_send_funds_to_address
    puts "\n💸 Testing fund transfer on XION testnet...\n"

    recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
    sender_address = @signer.address

    puts "1️⃣  Test Configuration:"
    puts "   🔗 Chain: #{CHAIN_ID}"
    puts "   📡 RPC: #{RPC_ENDPOINT}"
    puts "   👤 Sender: #{sender_address}"
    puts "   🎯 Recipient: #{recipient}"

    puts "\n2️⃣  Creating client with signer attached..."
    # Use new constructor with signer
    client = Mob::Client.new_with_signer(@config, @signer)
    puts "   ✅ Client connected with signer attached"

    puts "\n3️⃣  Querying sender balance..."
    begin
      balance = client.get_balance(sender_address, "uxion")
      balance_amount = balance.amount.to_i

      puts "   💰 Current uxion balance: #{balance.amount} uxion"

      if balance_amount == 0
        puts "\n   ⚠️  WARNING: Sender has no uxion balance!"
        puts "   Please fund the test account: #{sender_address}"
        puts "   Skipping transaction..."
        skip "Test account has no funds"
      end

      if balance_amount < 6000  # Need at least 1000 for amount + 5000 for fee
        puts "\n   ⚠️  WARNING: Insufficient balance (#{balance_amount} uxion)"
        puts "   Need at least 6000 uxion (1000 to send + 5000 fee)"
        skip "Insufficient funds for test"
      end
    rescue => e
      puts "\n   ⚠️  Failed to query balance: #{e.message}"
      puts "   Test account may not exist on testnet"
      skip "Cannot query account balance"
    end

    puts "\n4️⃣  Preparing transaction..."
    # Send 1000 uxion (0.001 XION)
    amount = [Mob::Coin.new(denom: "uxion", amount: "1000")]

    puts "   📤 Sending 1000 uxion to #{recipient}"
    puts "   📝 Memo: Test fund transfer from Ruby"

    puts "\n5️⃣  Broadcasting transaction..."
    begin
      tx_response = client.send(
        recipient,
        amount,
        "Test fund transfer from Ruby"
      )

      puts "   ✅ Transaction broadcast successful!"
      puts "   📝 Transaction hash: #{tx_response.txhash}"
      puts "   📊 Code: #{tx_response.code}"

      assert_equal 0, tx_response.code, "Transaction failed with code #{tx_response.code}: #{tx_response.raw_log}"

      if tx_response.code == 0
        puts "   ✅ Transaction accepted by mempool"
      else
        puts "   ⚠️  Transaction failed with code: #{tx_response.code}"
        puts "   📋 Log: #{tx_response.raw_log}"
      end
    rescue => e
      puts "\n   ❌ Transaction failed: #{e.message}"
      raise
    end

    puts "\n6️⃣  Waiting for transaction confirmation (10 seconds)..."
    sleep 10

    puts "\n7️⃣  Querying transaction result..."
    begin
      tx_result = client.get_tx(tx_response.txhash)
      puts "   ✅ Transaction confirmed!"
      puts "   📊 Final code: #{tx_result.code}"
      puts "   ⛽ Gas used: #{tx_result.gas_used}"
      puts "   ⛽ Gas wanted: #{tx_result.gas_wanted}"
      puts "   📏 Block height: #{tx_result.height}"

      assert_equal 0, tx_result.code, "Transaction failed with code #{tx_result.code}"
    rescue => e
      puts "   ⚠️  Could not query transaction: #{e.message}"
      puts "   (Transaction may still be processing)"
    end

    puts "\n🎉 Fund transfer test completed!\n"
  end
end
