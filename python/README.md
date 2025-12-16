# Mob Python Bindings

Python bindings for the Mob library - a multi-platform signing client for the XION blockchain.

## Quick Development Setup

It's recommended to use a virtual environment. We recommend using [uv](https://github.com/astral-sh/uv) for faster package management:

```bash
# Option 1: Using uv (recommended - faster)
# Install uv if you haven't: curl -LsSf https://astral.sh/uv/install.sh | sh
uv venv
source .venv/bin/activate  # On Unix/macOS
# or
.\.venv\Scripts\activate  # On Windows

# Option 2: Using standard venv
python3 -m venv .venv
source .venv/bin/activate  # On Unix/macOS

# Install with development dependencies
python dev.py install --dev

# Run tests
python dev.py test

# Run examples
python dev.py examples
```

See the [Development Commands](#development-commands) section below for all available commands.

## Overview

Mob provides a comprehensive Python interface for interacting with the XION blockchain, including:

- 🔐 **Key Management** - Mnemonic-based key derivation and private key management
- 📝 **Transaction Building** - Intuitive API for building and signing transactions
- 🌐 **RPC Client** - Full-featured client for interacting with XION nodes
- 🔄 **Account Abstraction** - Support for XION's account abstraction features
- 🦀 **Pure Rust Core** - High-performance core logic with Python bindings via UniFFI

## Installation

### Prerequisites

- Python 3.8 or later
- Rust toolchain (for building from source)
- maturin (`pip install maturin`)

### Building from Source

```bash
# From the python/ directory
maturin develop

# Or with release optimizations
maturin develop --release
```

### Installing the Package

```bash
# Install in development mode
pip install -e .

# Or build a wheel
maturin build --release
pip install target/wheels/mob-*.whl
```

## Quick Start

### Basic RPC Query

```python
import asyncio
from mob import ChainConfig, Client

async def main():
    # Create chain configuration
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        bech32_prefix="xion"
    )

    # Create client
    client = await Client.new(config)

    # Query blockchain height
    height = await client.get_height()
    print(f"Current height: {height}")

asyncio.run(main())
```

### Creating a Signer

```python
from mob import Signer

# Create from mnemonic
signer = Signer.from_mnemonic(
    mnemonic="your twelve or twenty-four word mnemonic here",
    prefix="xion",
    derivation_path=None  # Uses default: m/44'/118'/0'/0/0
)

# Get address
address = signer.get_address()
print(f"Address: {address}")
```

### Sending a Transaction

```python
import asyncio
from mob import ChainConfig, Client, Signer, Coin

async def send_tokens():
    # Configure client
    config = ChainConfig(
        chain_id="xion-testnet-2",
        rpc_endpoint="https://rpc.xion-testnet-2.burnt.com:443",
        bech32_prefix="xion"
    )

    # Create signer
    signer = Signer.from_mnemonic(
        mnemonic="your mnemonic here",
        prefix="xion",
        derivation_path=None
    )

    # Create client
    client = await Client.new(config)

    # Send transaction
    tx_response = await client.send(
        to_address="xion1recipient...",
        amount=[Coin(denom="uxion", amount="1000000")],
        memo="My first transaction"
    )

    print(f"Transaction hash: {tx_response.txhash}")
    print(f"Code: {tx_response.code}")  # 0 = success

asyncio.run(send_tokens())
```

## Development Commands

The `dev.py` script provides all necessary development commands:

```bash
# Installation
python dev.py install           # Install package
python dev.py install --dev     # Install with dev dependencies

# Testing
python dev.py test              # Run tests (excludes integration tests)
python dev.py test --verbose    # Run tests with verbose output

# Run integration tests (requires funded test account)
pytest tests/ -m integration -v -s

# Code Quality
python dev.py lint              # Check code quality (black, isort, mypy)
python dev.py format            # Format code with black and isort

# Building
python dev.py build             # Build wheel package

# Examples
python dev.py examples          # Run example scripts

# Cleanup
python dev.py clean             # Clean build artifacts
```

### Requirements

Development dependencies are listed in `requirements-dev.txt`:
- `maturin` - Build tool for Rust extensions
- `pytest` and `pytest-asyncio` - Testing framework
- `black`, `isort`, `mypy` - Code quality tools

Install all dev dependencies with:
```bash
python dev.py install --dev
```

## Documentation

- **[Testing Guide](docs/testing.md)** - Comprehensive guide to running tests
- **[Examples](examples/)** - Sample code demonstrating various features
  - `basic_query.py` - Basic RPC queries
  - `account_query.py` - Account information and balances
  - `send_transaction.py` - Sending tokens

## Project Structure

```
python/
├── mob/                # Python package source
│   └── __init__.py
├── tests/              # Test suite
│   ├── conftest.py     # Pytest fixtures
│   └── test_rpc_queries.py
├── docs/               # Documentation
│   └── testing.md      # Testing guide
├── examples/           # Usage examples
│   ├── basic_query.py
│   ├── account_query.py
│   └── send_transaction.py
├── README.md           # This file
└── pyproject.toml      # Python package configuration
```

## Running Examples

Before running examples, make sure to build the package:

```bash
# From the python/ directory
maturin develop
```

Then run any example:

```bash
python examples/basic_query.py
python examples/account_query.py
python examples/send_transaction.py  # ⚠️ Requires funded test account
```

## Running Tests

The test suite uses pytest and pytest-asyncio. Install test dependencies:

```bash
pip install pytest pytest-asyncio
```

Run tests:

```bash
# Run all unit tests (excludes integration tests)
pytest tests/

# Run all tests including integration tests
pytest tests/ -m integration

# Run with verbose output
pytest tests/ -v

# Run specific test file
pytest tests/test_rpc_queries.py
```

For more details, see the [Testing Guide](docs/testing.md).

## API Reference

### ChainConfig

Configuration for connecting to a blockchain network.

```python
config = ChainConfig(
    chain_id: str,         # e.g., "xion-testnet-2"
    rpc_endpoint: str,     # e.g., "https://rpc.xion-testnet-2.burnt.com:443"
    bech32_prefix: str     # e.g., "xion"
)
```

### Client

Async RPC client for blockchain interaction.

**Methods:**

- `await Client.new(config: ChainConfig) -> Client` - Create new client
- `await get_height() -> int` - Get latest block height
- `await get_chain_id() -> str` - Get chain ID
- `await is_synced() -> bool` - Check if node is synced
- `await get_account(address: str) -> AccountInfo` - Get account info
- `await get_balance(address: str, denom: str) -> Coin` - Get balance
- `await get_tx(hash: str) -> TxResponse` - Get transaction by hash
- `await send(to_address: str, amount: List[Coin], memo: str) -> TxResponse` - Send tokens

### Signer

Key management and signing functionality.

**Methods:**

- `Signer.from_mnemonic(mnemonic: str, prefix: str, derivation_path: Optional[str]) -> Signer` - Create from mnemonic
- `get_address() -> str` - Get bech32 address
- `get_public_key() -> bytes` - Get public key bytes

### Types

**Coin:**
```python
Coin(denom: str, amount: str)
```

**AccountInfo:**
```python
AccountInfo(
    address: str,
    account_number: int,
    sequence: int
)
```

**TxResponse:**
```python
TxResponse(
    txhash: str,
    code: int,
    raw_log: str,
    height: int,
    gas_used: int,
    gas_wanted: int
)
```

## Development

### Building for Development

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Build with release optimizations
maturin develop --release
```

### Running Tests

```bash
# Unit tests only
pytest tests/ -v

# Including integration tests (requires funded test account)
pytest tests/ -m integration -v
```

### Code Quality

```bash
# Format Python code
black python/

# Type checking
mypy python/

# Linting
ruff python/
```

## Requirements

- Python >= 3.8
- asyncio support
- Network access for RPC endpoints (in tests and examples)

## License

See the root LICENSE file in the main repository.

## Contributing

Contributions are welcome! Please see the main repository for contribution guidelines.

## Support

For issues and questions:
- Open an issue in the main repository
- Check the [Testing Guide](docs/testing.md) for troubleshooting

## Related Projects

- [Mob Core](../core/) - Rust core library
- [XION Blockchain](https://xion.burnt.com) - The XION blockchain
