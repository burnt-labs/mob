"""
Mob - Multi-platform signing client for XION blockchain

This package provides Python bindings to the Mob Rust library, enabling
secure key management, transaction signing, and blockchain interactions
for the XION network.

Example:
    >>> import mob
    >>> signer = mob.Signer.from_mnemonic(
    ...     "your mnemonic phrase here",
    ...     "xion",
    ...     None
    ... )
    >>> print(signer.address())
    xion1...

For more information, see: https://github.com/burnt-labs/mob
"""

from .mob import *

__version__ = "0.1.0"
__author__ = "Burnt Labs"
__all__ = [
    # Classes
    "Signer",
    "Client",
    # Types
    "Coin",
    "Fee",
    "ChainConfig",
    "AccountInfo",
    "TxResponse",
    "BroadcastMode",
    # Errors
    "MobError",
]
