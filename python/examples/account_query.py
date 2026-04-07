"""
Account Query Example

This example demonstrates how to:
- Create a signer from a mnemonic
- Query account information
- Check account balances
"""

from mob import ChainConfig, Client, RustSigner, NativeHttpTransport


def main():
    # Create chain configuration
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        address_prefix="xion",
    )

    # Create a signer from mnemonic (example - replace with your own)
    print("Creating signer from mnemonic...")
    mnemonic = (
        "quiz cattle knock bacon million abstract word reunion educate antenna "
        "put fitness slide dash point basket jaguar fun humor multiply "
        "emotion rescue brand pull"
    )

    signer = RustSigner.from_mnemonic(
        mnemonic=mnemonic,
        address_prefix="xion",
        derivation_path=None,  # Uses default: m/44'/118'/0'/0/0
    )

    # Get the account address
    address = signer.address()
    print(f"Address: {address}")

    # Create client and query account info
    print("\nConnecting to XION testnet...")
    transport = NativeHttpTransport()
    client = Client(config, transport)

    # Query account information
    print(f"\nQuerying account information for {address}...")
    try:
        account_info = client.get_account(address)
        print(f"Account found!")
        print(f"   Account number: {account_info.account_number}")
        print(f"   Sequence: {account_info.sequence}")
    except Exception as e:
        print(f"Error querying account: {e}")

    # Query account balance
    print(f"\nQuerying balance...")
    try:
        balance = client.get_balance(address, "uxion")
        amount_xion = int(balance.amount) / 1_000_000
        print(f"Balance: {balance.amount} uxion ({amount_xion:.6f} XION)")
    except Exception as e:
        print(f"Error querying balance: {e}")

    print("\nQuery complete.")


if __name__ == "__main__":
    main()
