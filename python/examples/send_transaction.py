"""
Send Transaction Example

This example demonstrates how to:
- Create and configure a client with a signer
- Send tokens to another address
- Wait for transaction confirmation
- Query transaction results

WARNING: This example sends real tokens on the testnet!
Make sure your test account is funded before running.
"""

import time

from mob import ChainConfig, Client, RustSigner, Coin, NativeHttpTransport


def main():
    # Configuration
    RECIPIENT_ADDRESS = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
    AMOUNT_TO_SEND = "1000"  # in uxion (0.001 XION)

    # Your mnemonic (replace with your own funded test account)
    MNEMONIC = (
        "quiz cattle knock bacon million abstract word reunion educate antenna "
        "put fitness slide dash point basket jaguar fun humor multiply "
        "emotion rescue brand pull"
    )

    print("=" * 60)
    print("XION Transaction Example")
    print("=" * 60)

    # Step 1: Create signer
    print("\nStep 1: Creating signer from mnemonic...")
    signer = RustSigner.from_mnemonic(
        mnemonic=MNEMONIC,
        address_prefix="xion",
        derivation_path=None,
    )
    sender_address = signer.address()
    print(f"   Sender address: {sender_address}")

    # Step 2: Create and configure client
    print("\nStep 2: Connecting to XION testnet...")
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        address_prefix="xion",
    )
    transport = NativeHttpTransport()
    client = Client.new_with_signer(config, signer, transport)

    # Step 3: Check balance
    print("\nStep 3: Checking balance...")
    balance = client.get_balance(sender_address, "uxion")
    balance_amount = int(balance.amount)
    balance_xion = balance_amount / 1_000_000

    print(f"   Current balance: {balance.amount} uxion ({balance_xion:.6f} XION)")

    # Check if we have enough funds (need at least 6000 uxion for tx + gas)
    if balance_amount < 6000:
        print(f"\nInsufficient funds!")
        print(f"   Need at least 6000 uxion, but have {balance.amount} uxion")
        print(f"   Please fund your test account first.")
        return

    # Step 4: Send transaction
    print("\nStep 4: Sending transaction...")
    print(f"   Recipient: {RECIPIENT_ADDRESS}")
    print(f"   Amount: {AMOUNT_TO_SEND} uxion")

    amount = [Coin(denom="uxion", amount=AMOUNT_TO_SEND)]

    try:
        tx_response = client.send(
            to_address=RECIPIENT_ADDRESS,
            amount=amount,
            memo="Test transaction from mob Python example",
        )

        print(f"\nTransaction broadcast successful!")
        print(f"   Transaction hash: {tx_response.txhash}")
        print(f"   Code: {tx_response.code} (0 = success)")

        if tx_response.code != 0:
            print(f"   Transaction failed: {tx_response.raw_log}")
            return

    except Exception as e:
        print(f"\nTransaction failed: {e}")
        return

    # Step 5: Wait for confirmation
    print("\nStep 5: Waiting for transaction confirmation (10 seconds)...")
    time.sleep(10)

    # Step 6: Query transaction result
    print("\nStep 6: Querying transaction result...")
    try:
        tx_result = client.get_tx(tx_response.txhash)
        print(f"   Transaction confirmed!")
        print(f"   Height: {tx_result.height}")
        print(f"   Gas used: {tx_result.gas_used}")
        print(f"   Gas wanted: {tx_result.gas_wanted}")

    except Exception as e:
        print(f"   Could not query transaction: {e}")
        print(f"   This might mean the transaction is still pending.")

    # Step 7: Check new balance
    print("\nStep 7: Checking new balance...")
    new_balance = client.get_balance(sender_address, "uxion")
    new_balance_xion = int(new_balance.amount) / 1_000_000
    print(f"   New balance: {new_balance.amount} uxion ({new_balance_xion:.6f} XION)")

    print("\n" + "=" * 60)
    print("Transaction example complete.")
    print("=" * 60)


if __name__ == "__main__":
    main()
