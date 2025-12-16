#!/usr/bin/env ruby
# frozen_string_literal: true

# Basic RPC Query Example
#
# This example demonstrates how to:
# - Create a client connection to XION testnet
# - Query basic blockchain information
# - Check node sync status

require_relative '../lib/mob'

def main
  puts "=" * 60
  puts "🔗 Mob Ruby Client - Basic Query Example"
  puts "=" * 60

  # Create chain configuration for XION testnet-2
  puts "\n1️⃣  Creating chain configuration..."
  config = Mob::ChainConfig.new(
    chain_id: "xion-testnet-2",
    rpc_endpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpc_endpoint: nil,
    address_prefix: "xion",
    coin_type: 118,
    gas_price: "0.025"
  )
  puts "   ✅ Chain ID: #{config.chain_id}"
  puts "   ✅ RPC: #{config.rpc_endpoint}"

  # Create RPC client
  puts "\n2️⃣  Connecting to XION testnet..."
  client = Mob::Client.new(config)
  puts "   ✅ Client connected"

  # Query the latest block height
  puts "\n3️⃣  Querying blockchain height..."
  height = client.get_height
  puts "   ✅ Current block height: #{height.to_s.reverse.gsub(/(\d{3})(?=\d)/, '\\1,').reverse}"

  # Check sync status
  puts "\n4️⃣  Checking node sync status..."
  is_synced = client.is_synced
  sync_status = is_synced ? "✅ Synced" : "⏳ Syncing"
  puts "   #{sync_status}"

  # Get chain ID
  puts "\n5️⃣  Verifying chain ID..."
  chain_id = client.get_chain_id
  puts "   ✅ Chain ID: #{chain_id}"

  puts "\n" + "=" * 60
  puts "✨ Query complete!"
  puts "=" * 60

rescue => e
  puts "\n❌ Error: #{e.message}"
  puts e.backtrace.first(5).join("\n")
  exit 1
end

main if __FILE__ == $PROGRAM_NAME
