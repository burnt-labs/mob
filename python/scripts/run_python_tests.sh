#!/bin/bash
set -e

echo "===================="
echo "Running Python Tests"
echo "===================="

# Ensure we're in the project root
cd "$(dirname "$0")/../.."

# Check if pytest is installed
if ! python3 -m pytest --version &> /dev/null; then
    echo "pytest not found. Installing..."
    pip3 install pytest pytest-asyncio
fi

# Check if the package is installed
if ! python3 -c "import mob" 2>/dev/null; then
    echo ""
    echo "WARNING: mob package not installed."
    echo "Building and installing package first..."
    echo ""

    # Build the package using maturin
    if command -v maturin &> /dev/null; then
        cd python
        maturin develop
        cd ..
    else
        echo "maturin not found. Installing..."
        pip3 install maturin
        cd python
        maturin develop
        cd ..
    fi
fi

echo ""
echo "Running tests..."
echo ""

# Run unit tests (excluding integration tests)
echo "Running unit tests..."
python3 -m pytest python/tests/ -v -m "not integration"

echo ""
echo "To run integration tests (requires funded test account):"
echo "  python3 -m pytest python/tests/ -v -m integration"
echo ""

echo "===================="
echo "Tests completed!"
echo "===================="
