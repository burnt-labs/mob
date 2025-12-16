#!/bin/bash
set -e

# Script to generate Ruby bindings for mob library
# Run this from the project root directory

echo "🔨 Building Rust library..."
cargo build --release -p mob

echo "📝 Generating Ruby bindings..."
cargo run --bin uniffi-bindgen generate \
  --library target/release/libmob.dylib \
  --language ruby \
  --out-dir ruby/lib

echo "📦 Copying library to ruby/lib..."
cp target/release/libmob.dylib ruby/lib/

echo "🔧 Fixing library path in generated bindings..."
# Replace the ffi_lib line to use a relative path
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS sed
  sed -i '' "s/ffi_lib 'mob'/ffi_lib File.expand_path('libmob.dylib', __dir__)/" ruby/lib/mob.rb
else
  # Linux sed
  sed -i "s/ffi_lib 'mob'/ffi_lib File.expand_path('libmob.dylib', __dir__)/" ruby/lib/mob.rb
fi

echo "✅ Ruby bindings generated successfully!"
echo ""
echo "Run tests with:"
echo "  ruby test/test_rpc_queries.rb"
echo ""
echo "Run integration tests with:"
echo "  INTEGRATION=1 ruby test/test_rpc_queries.rb"
