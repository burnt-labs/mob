#!/bin/bash
# Setup git hooks for the mob repository

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOKS_DIR="$REPO_ROOT/.git/hooks"

echo "Setting up git hooks..."

# Create pre-commit hook
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/bash
# Pre-commit hook to check Rust formatting and clippy

set -e

echo "Running pre-commit checks..."

# Check if there are any Rust files staged
RUST_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)

if [ -z "$RUST_FILES" ]; then
    echo "No Rust files staged, skipping checks"
    exit 0
fi

echo "Checking Rust files in core/"

# Change to core directory
cd core

# Check formatting
echo "Checking code formatting with cargo fmt..."
if ! cargo fmt --check; then
    echo "❌ Code formatting check failed!"
    echo "Run 'cargo fmt' in the core directory to fix formatting issues"
    exit 1
fi

# Run clippy
echo "Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy check failed!"
    echo "Fix the issues reported by clippy before committing"
    exit 1
fi

echo "✅ All pre-commit checks passed!"
exit 0
EOF

chmod +x "$HOOKS_DIR/pre-commit"

echo "✅ Git hooks installed successfully!"
echo "Pre-commit hook will check:"
echo "  - Rust code formatting (cargo fmt)"
echo "  - Clippy lints (cargo clippy)"
