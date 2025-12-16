"""
Account Query Example

This example demonstrates how to:
- Create a signer from a mnemonic
- Query account information
- Check account balances
"""

import asyncio
from mob import ChainConfig, Client, Signer


async def main():
    # Create chain configuration
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        bech32_prefix="xion"
    )

    # Create a signer from mnemonic (example - replace with your own)
    print("🔑 Creating signer from mnemonic...")
    mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull"

    signer = Signer.from_mnemonic(
        mnemonic=mnemonic,
        prefix="xion",
        derivation_path=None  # Uses default: m/44'/118'/0'/0/0
    )

    # Get the account address
    address = signer.get_address()
    print(f"📍 Address: {address}")

    # Create client and query account info
    print("\n🔗 Connecting to XION testnet...")
    client = await Client.new(config)

    # Query account information
    print(f"\n📊 Querying account information for {address}...")
    try:
        account_info = await client.get_account(address)
        print(f"✅ Account found!")
        print(f"   Account number: {account_info.account_number}")
        print(f"   Sequence: {account_info.sequence}")
    except Exception as e:
        print(f"❌ Error querying account: {e}")

    # Query account balance
    print(f"\n💰 Querying balance...")
    try:
        balance = await client.get_balance(address, "uxion")
        amount_xion = int(balance.amount) / 1_000_000  # Convert from uxion to xion
        print(f"✅ Balance: {balance.amount} uxion ({amount_xion:.6f} XION)")
    except Exception as e:
        print(f"❌ Error querying balance: {e}")

    print("\n✨ Query complete!")


if __name__ == "__main__":
    asyncio.run(main())
