"""
Basic RPC Query Example

This example demonstrates how to:
- Create a client connection to XION testnet
- Query basic blockchain information
- Check node sync status
"""

import asyncio
from mob import ChainConfig, Client


async def main():
    # Create chain configuration for XION testnet-2
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        bech32_prefix="xion"
    )

    # Create RPC client
    print("🔗 Connecting to XION testnet...")
    client = await Client.new(config)

    # Query the latest block height
    print("\n📊 Querying blockchain information...")
    height = await client.get_height()
    print(f"✅ Current block height: {height}")

    # Check sync status
    is_synced = await client.is_synced()
    sync_status = "✅ Synced" if is_synced else "⏳ Syncing"
    print(f"{sync_status}")

    # Get chain ID
    chain_id = await client.get_chain_id()
    print(f"⛓️  Chain ID: {chain_id}")

    print("\n✨ Query complete!")


if __name__ == "__main__":
    asyncio.run(main())
