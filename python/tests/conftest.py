"""
Pytest configuration and shared fixtures for mob library tests.

This module provides reusable fixtures for testing the mob Python bindings.
"""

import pytest
from mob import ChainConfig, Signer


# Test configuration constants
RPC_ENDPOINT = "https://rpc.xion-testnet-2.burnt.com:443"
CHAIN_ID = "xion-testnet-2"
ADDRESS_PREFIX = "xion"

# Test mnemonic (DO NOT USE IN PRODUCTION)
TEST_MNEMONIC = (
    "quiz cattle knock bacon million abstract word reunion educate antenna put fitness "
    "slide dash point basket jaguar fun humor multiply emotion rescue brand pull"
)


@pytest.fixture
def chain_config():
    """
    Create a ChainConfig for XION testnet.

    Returns:
        ChainConfig: Configuration object for xion-testnet-2
    """
    return ChainConfig(
        chain_id=CHAIN_ID,
        rpc_endpoint=RPC_ENDPOINT,
        grpc_endpoint=None,
        address_prefix=ADDRESS_PREFIX,
        coin_type=118,
        gas_price="0.025"
    )


@pytest.fixture
def test_signer():
    """
    Create a test signer from the test mnemonic.

    Returns:
        Signer: A signer instance derived from TEST_MNEMONIC
    """
    return Signer.from_mnemonic(
        TEST_MNEMONIC,
        ADDRESS_PREFIX,
        "m/44'/118'/0'/0/0"
    )


@pytest.fixture
def test_address(test_signer):
    """
    Get the address from the test signer.

    Returns:
        str: The bech32-encoded address
    """
    return test_signer.address()


def pytest_configure(config):
    """
    Configure pytest with custom markers.
    """
    config.addinivalue_line(
        "markers",
        "integration: marks tests as integration tests (requires network and funded account)"
    )
    config.addinivalue_line(
        "markers",
        "slow: marks tests as slow (deselect with '-m \"not slow\"')"
    )
