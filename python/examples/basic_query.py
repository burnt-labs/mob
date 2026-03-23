"""
Basic RPC Query Example

This example demonstrates how to:
- Create a client connection to XION testnet
- Query basic blockchain information
- Check node sync status
"""

from mob import ChainConfig, Client, NativeHttpTransport


def main():
    # Create chain configuration for XION testnet-2
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        address_prefix="xion",
    )

    # Create RPC client with platform-native HTTP transport
    print("Connecting to XION testnet...")
    transport = NativeHttpTransport()
    client = Client(config, transport)

    # Query the latest block height
    print("\nQuerying blockchain information...")
    height = client.get_height()
    print(f"Current block height: {height}")

    # Check sync status
    is_synced = client.is_synced()
    print(f"Synced: {is_synced}")

    # Get chain ID
    chain_id = client.get_chain_id()
    print(f"Chain ID: {chain_id}")

    print("\nQuery complete.")


if __name__ == "__main__":
    main()
