#!/usr/bin/env ruby
# frozen_string_literal: true

# Account Query Example
#
# This example demonstrates how to:
# - Create a signer from a mnemonic
# - Query account information
# - Check account balances

require_relative '../lib/mob'

def main
  puts "=" * 60
  puts "🔐 Mob Ruby Client - Account Query Example"
  puts "=" * 60

  # Create a signer from mnemonic (example - replace with your own)
  puts "\n1️⃣  Creating signer from mnemonic..."
  mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna " \
             "put fitness slide dash point basket jaguar fun humor multiply " \
             "emotion rescue brand pull"

  signer = Mob::Signer.from_mnemonic(
    mnemonic,
    "xion",
    "m/44'/118'/0'/0/0"
  )

  # Get the account address
  address = signer.address
  puts "   ✅ Address: #{address}"

  # Create client and query account info
  puts "\n2️⃣  Connecting to XION testnet..."
  config = Mob::ChainConfig.new(
    chain_id: "xion-testnet-2",
    rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpc_endpoint: nil,
    address_prefix: "xion",
    coin_type: 118,
    gas_price: "0.025"
  )
  client = Mob::Client.new(config)
  puts "   ✅ Client connected"

  # Query account information
  puts "\n3️⃣  Querying account information for #{address}..."
  begin
    account_info = client.get_account(address)
    puts "   ✅ Account found!"
    puts "      Account number: #{account_info.account_number}"
    puts "      Sequence: #{account_info.sequence}"
  rescue => e
    puts "   ❌ Error querying account: #{e.message}"
  end

  # Query account balance
  puts "\n4️⃣  Querying balance..."
  begin
    balance = client.get_balance(address, "uxion")
    amount_xion = balance.amount.to_i / 1_000_000.0
    puts "   ✅ Balance: #{balance.amount} uxion (#{format('%.6f', amount_xion)} XION)"
  rescue => e
    puts "   ❌ Error querying balance: #{e.message}"
  end

  puts "\n" + "=" * 60
  puts "✨ Query complete!"
  puts "=" * 60

rescue => e
  puts "\n❌ Error: #{e.message}"
  puts e.backtrace.first(5).join("\n")
  exit 1
end

main if __FILE__ == $PROGRAM_NAME
