#!/usr/bin/env ruby
# frozen_string_literal: true

# Send Transaction Example
#
# This example demonstrates how to:
# - Create and configure a client with a signer
# - Send tokens to another address
# - Wait for transaction confirmation
# - Query transaction results
#
# WARNING: This example sends real tokens on the testnet!
# Make sure your test account is funded before running.

require_relative '../lib/mob'
require_relative '../lib/native_http_transport'

# Configuration
RECIPIENT_ADDRESS = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
AMOUNT_TO_SEND = "1000" # in uxion (0.001 XION)

# Your mnemonic (replace with your own funded test account)
MNEMONIC = "quiz cattle knock bacon million abstract word reunion educate antenna " \
           "put fitness slide dash point basket jaguar fun humor multiply " \
           "emotion rescue brand pull"

def main
  puts "=" * 60
  puts "XION Transaction Example"
  puts "=" * 60

  # Step 1: Create signer
  puts "\nStep 1: Creating signer from mnemonic..."
  signer = Mob::RustSigner.from_mnemonic(
    MNEMONIC,
    "xion",
    "m/44'/118'/0'/0/0"
  )
  sender_address = signer.address
  puts "   Sender address: #{sender_address}"

  # Step 2: Create and configure client
  puts "\nStep 2: Connecting to XION testnet..."
  config = Mob::ChainConfig.new(
    chain_id: "xion-testnet-2",
    rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpc_endpoint: nil,
    address_prefix: "xion",
    coin_type: 118,
    gas_price: "0.025"
  )
  transport = Mob::NativeHttpTransport.new
  client = Mob::Client.new_with_signer(config, signer, transport)
  puts "   Client connected"

  # Step 3: Check balance
  puts "\nStep 3: Checking balance..."
  balance = client.get_balance(sender_address, "uxion")
  balance_amount = balance.amount.to_i
  balance_xion = balance_amount / 1_000_000.0

  puts "   Current balance: #{balance.amount} uxion (#{format('%.6f', balance_xion)} XION)"

  # Check if we have enough funds (need at least 6000 uxion for tx + gas)
  if balance_amount < 6000
    puts "\nInsufficient funds!"
    puts "   Need at least 6000 uxion, but have #{balance.amount} uxion"
    puts "   Please fund your test account first."
    exit 1
  end

  # Step 4: Send transaction
  puts "\nStep 4: Sending transaction..."
  puts "   Recipient: #{RECIPIENT_ADDRESS}"
  puts "   Amount: #{AMOUNT_TO_SEND} uxion"

  amount = [Mob::Coin.new(denom: "uxion", amount: AMOUNT_TO_SEND)]

  tx_response = client.send(
    to_address: RECIPIENT_ADDRESS,
    amount: amount,
    memo: "Test transaction from mob Ruby example"
  )

  puts "\nTransaction broadcast successful!"
  puts "   Transaction hash: #{tx_response.txhash}"
  puts "   Code: #{tx_response.code} (0 = success)"

  if tx_response.code != 0
    puts "   Transaction failed: #{tx_response.raw_log}"
    exit 1
  end

  # Step 5: Wait for confirmation
  puts "\nStep 5: Waiting for transaction confirmation (10 seconds)..."
  sleep 10

  # Step 6: Query transaction result
  puts "\nStep 6: Querying transaction result..."
  begin
    tx_result = client.get_tx(tx_response.txhash)
    puts "   Transaction confirmed!"
    puts "   Height: #{tx_result.height}"
    puts "   Gas used: #{tx_result.gas_used}"
    puts "   Gas wanted: #{tx_result.gas_wanted}"
  rescue => e
    puts "   Could not query transaction: #{e.message}"
    puts "   This might mean the transaction is still pending."
  end

  # Step 7: Check new balance
  puts "\nStep 7: Checking new balance..."
  new_balance = client.get_balance(sender_address, "uxion")
  new_balance_xion = new_balance.amount.to_i / 1_000_000.0
  puts "   New balance: #{new_balance.amount} uxion (#{format('%.6f', new_balance_xion)} XION)"

  puts "\n" + "=" * 60
  puts "Transaction example complete."
  puts "=" * 60

rescue => e
  puts "\nError: #{e.message}"
  puts e.backtrace.first(5).join("\n")
  exit 1
end

main if __FILE__ == $PROGRAM_NAME
