#!/bin/bash
set -e

# Script to generate Swift bindings and XCFramework for mob library
# Run this from the project root directory

echo "🔨 Building Rust library for macOS arm64..."
cargo build --release --target aarch64-apple-darwin -p mob

echo "📝 Generating Swift bindings..."
cargo run --bin uniffi-bindgen generate \
  --library target/aarch64-apple-darwin/release/libmob.dylib \
  --language swift \
  --out-dir swift/lib

echo "📦 Creating XCFramework..."
# Create build directory
mkdir -p swift/xcframework-build/macos

# Copy library and headers
cp target/aarch64-apple-darwin/release/libmob.dylib swift/xcframework-build/macos/
cp swift/lib/mobFFI.h swift/xcframework-build/macos/
cp swift/lib/mobFFI.modulemap swift/xcframework-build/macos/

# Remove old XCFramework if exists
rm -rf swift/lib/libmob.xcframework

# Create XCFramework
cd swift/xcframework-build/macos
xcodebuild -create-xcframework \
  -library libmob.dylib \
  -headers . \
  -output ../../lib/libmob.xcframework
cd ../../..

# Copy Swift source to Sources directory
echo "📋 Setting up Swift Package structure..."
mkdir -p swift/Sources/Mob
cp swift/lib/mob.swift swift/Sources/Mob/

# Create MobFFI module with headers
mkdir -p swift/Sources/MobFFI/include
cp swift/lib/mobFFI.h swift/Sources/MobFFI/include/
cp swift/lib/mobFFI.modulemap swift/Sources/MobFFI/include/module.modulemap

# Copy library for development use
cp target/aarch64-apple-darwin/release/libmob.dylib swift/lib/

# Clean up build directory
rm -rf swift/xcframework-build

echo "✅ Swift bindings and XCFramework created successfully!"
echo ""
echo "Build and test with:"
echo "  cd swift && swift build"
echo "  cd swift && swift test"
echo ""
echo "The XCFramework is located at:"
echo "  swift/lib/libmob.xcframework"
