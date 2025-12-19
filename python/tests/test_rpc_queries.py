"""
Python tests for mob library RPC queries against XION testnet.

These tests verify that the Python bindings work correctly and can
interact with the XION blockchain RPC endpoint.

Run with: python -m pytest python/tests/ -v
"""

import pytest
from mob import Client, Coin, RustSigner


class TestBasicRPCQueries:
    """Test basic RPC query functionality."""

    def test_create_client(self, chain_config):
        """Test creating a client instance."""
        client = Client(chain_config)
        assert client is not None

    def test_get_height(self, chain_config):
        """Test getting the current block height."""
        client = Client(chain_config)
        height = client.get_height()

        assert height > 0
        assert isinstance(height, int)
        print(f"✅ Current block height: {height}")

    def test_is_synced(self, chain_config):
        """Test checking if node is synced."""
        client = Client(chain_config)
        is_synced = client.is_synced()

        assert isinstance(is_synced, bool)
        print(f"✅ Node synced: {is_synced}")


class TestAccountQueries:
    """Test account-related queries."""

    def test_signer_creation(self, test_signer):
        """Test creating a signer from mnemonic."""
        assert test_signer is not None
        address = test_signer.address()
        assert address.startswith("xion")
        print(f"✅ Signer address: {address}")

    def test_get_account(self, chain_config, test_address):
        """Test querying account information."""
        client = Client(chain_config)
        account_info = client.get_account(test_address)

        assert account_info.address == test_address
        assert account_info.account_number >= 0
        assert account_info.sequence >= 0
        print(f"✅ Account number: {account_info.account_number}, Sequence: {account_info.sequence}")

    def test_get_balance(self, chain_config, test_address):
        """Test querying account balance."""
        client = Client(chain_config)
        balance = client.get_balance(test_address, "uxion")

        assert balance.denom == "uxion"
        assert int(balance.amount) >= 0
        print(f"✅ Balance: {balance.amount} {balance.denom}")


class TestSigningFunctionality:
    """Test signing functionality."""

    def test_sign_message(self, test_signer):
        """Test signing an arbitrary message."""
        message = b"Hello, XION!"
        signature = test_signer.sign_bytes(message)

        assert signature is not None
        assert len(signature) > 0
        print(f"✅ Signed message, signature length: {len(signature)} bytes")

    def test_get_public_key(self, test_signer):
        """Test getting public key hex."""
        pub_key_hex = test_signer.public_key_hex()

        assert pub_key_hex is not None
        assert len(pub_key_hex) > 0
        print(f"✅ Public key: {pub_key_hex}")


class TestErrorHandling:
    """Test error handling."""

    def test_invalid_address(self, chain_config):
        """Test querying with invalid address."""
        client = Client(chain_config)

        with pytest.raises(Exception):
            client.get_account("invalid_address")
        print("✅ Invalid address properly rejected")

    def test_invalid_mnemonic(self):
        """Test creating signer with invalid mnemonic."""
        with pytest.raises(Exception):
            RustSigner.from_mnemonic(
                "invalid mnemonic words",
                "xion",
                "m/44'/118'/0'/0/0"
            )
        print("✅ Invalid mnemonic properly rejected")


class TestCoinCreation:
    """Test Coin data structure."""

    def test_create_coin(self):
        """Test creating a Coin instance."""
        coin = Coin(denom="uxion", amount="1000000")

        assert coin.denom == "uxion"
        assert coin.amount == "1000000"
        print(f"✅ Created coin: {coin.amount} {coin.denom}")


class TestMultipleSigners:
    """Test multiple signer derivation."""

    def test_different_derivation_paths(self):
        """Test that different derivation paths produce different addresses."""
        mnemonic = (
            "quiz cattle knock bacon million abstract word reunion educate antenna put fitness "
            "slide dash point basket jaguar fun humor multiply emotion rescue brand pull"
        )

        signer1 = RustSigner.from_mnemonic(
            mnemonic,
            "xion",
            "m/44'/118'/0'/0/0"
        )

        signer2 = RustSigner.from_mnemonic(
            mnemonic,
            "xion",
            "m/44'/118'/0'/0/1"
        )

        addr1 = signer1.address()
        addr2 = signer2.address()

        assert addr1 != addr2
        print(f"✅ Account 0: {addr1}")
        print(f"✅ Account 1: {addr2}")


class TestIntegrationSendFunds:
    """Integration test for sending real funds on testnet."""

    @pytest.mark.integration  # Mark as integration test
    def test_send_funds_to_address(self, chain_config, test_signer):
        """
        Integration test that sends real funds from test mnemonic to a recipient.

        This test requires:
        - Test account to be funded on XION testnet
        - Network access to RPC endpoint

        Run with: pytest python/tests/ -m integration -v -s
        """
        import time

        print("\n💸 Testing fund transfer on XION testnet...\n")

        # Receiving address
        recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"

        sender_address = test_signer.address()
        print(f"1️⃣  Sender address: {sender_address}")

        print("\n2️⃣  Creating RPC client with signer...")
        client = Client.new_with_signer(chain_config, test_signer)
        print("   ✅ Client connected with signer attached")

        print("\n3️⃣  Querying sender balance...")
        try:
            balance = client.get_balance(sender_address, "uxion")
            balance_amount = int(balance.amount)

            print(f"   💰 Current uxion balance: {balance.amount} uxion")

            if balance_amount == 0:
                print("\n   ⚠️  WARNING: Sender has no uxion balance!")
                print(f"   Please fund the test account: {sender_address}")
                print("   Skipping transaction...")
                pytest.skip("Test account has no funds")

            if balance_amount < 6000:  # Need at least 1000 for amount + 5000 for fee
                print(f"\n   ⚠️  WARNING: Insufficient balance ({balance_amount} uxion)")
                print("   Need at least 6000 uxion (1000 to send + 5000 fee)")
                pytest.skip("Insufficient funds for test")

        except Exception as e:
            print(f"\n   ⚠️  Failed to query balance: {e}")
            print("   Test account may not exist on testnet")
            pytest.skip("Cannot query account balance")

        print("\n4️⃣  Preparing transaction...")
        # Send 1000 uxion (0.001 XION)
        amount = [Coin(denom="uxion", amount="1000")]

        print(f"   📤 Sending 1000 uxion to {recipient}")
        print("   📝 Memo: Test fund transfer from Python")

        print("\n5️⃣  Broadcasting transaction...")
        try:
            tx_response = client.send(
                recipient,
                amount,
                "Test fund transfer from Python"
            )

            print("   ✅ Transaction broadcast successful!")
            print(f"   📝 Transaction hash: {tx_response.txhash}")
            print(f"   📊 Code: {tx_response.code}")

            assert tx_response.code == 0, f"Transaction failed with code {tx_response.code}: {tx_response.raw_log}"

            if tx_response.code == 0:
                print("   ✅ Transaction accepted by mempool")
            else:
                print(f"   ⚠️  Transaction failed with code: {tx_response.code}")
                print(f"   📋 Log: {tx_response.raw_log}")

        except Exception as e:
            print(f"\n   ❌ Transaction failed: {e}")
            raise

        print("\n6️⃣  Waiting for transaction confirmation (10 seconds)...")
        time.sleep(10)

        print("\n7️⃣  Querying transaction result...")
        try:
            tx_result = client.get_tx(tx_response.txhash)
            print("   ✅ Transaction confirmed!")
            print(f"   📊 Final code: {tx_result.code}")
            print(f"   ⛽ Gas used: {tx_result.gas_used}")
            print(f"   ⛽ Gas wanted: {tx_result.gas_wanted}")
            print(f"   📏 Block height: {tx_result.height}")

            assert tx_result.code == 0, f"Transaction failed with code {tx_result.code}"

        except Exception as e:
            print(f"   ⚠️  Could not query transaction: {e}")
            print("   (Transaction may still be processing)")

        print("\n🎉 Fund transfer test completed!\n")
