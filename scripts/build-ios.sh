#!/bin/bash
set -e

# Build mob library for iOS targets and generate Swift bindings for React Native
# Outputs XCFramework and Swift bindings to react-native/ios/

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"
RN_IOS_DIR="$ROOT_DIR/react-native/ios"

echo "Building mob for iOS..."

# Ensure Rust targets are installed
rustup target add aarch64-apple-ios aarch64-apple-ios-sim 2>/dev/null || true

# Build static library for device (arm64)
echo "Building for aarch64-apple-ios..."
cargo build --release --target aarch64-apple-ios -p mob

# Build static library for simulator (arm64)
echo "Building for aarch64-apple-ios-sim..."
cargo build --release --target aarch64-apple-ios-sim -p mob

# Generate Swift bindings using the macOS build (bindings are platform-independent)
echo "Generating Swift bindings..."
cargo build --release --target aarch64-apple-darwin -p mob
cargo run --bin uniffi-bindgen generate \
  --library target/aarch64-apple-darwin/release/libmob.dylib \
  --language swift \
  --out-dir "$RN_IOS_DIR/generated"

# Create XCFramework output directory
mkdir -p "$RN_IOS_DIR/Frameworks"
rm -rf "$RN_IOS_DIR/Frameworks/libmob.xcframework"

# Prepare staging area for XCFramework
XCFW_BUILD="$ROOT_DIR/target/xcframework-build"
rm -rf "$XCFW_BUILD"
mkdir -p "$XCFW_BUILD/device/headers" "$XCFW_BUILD/simulator/headers"

# Copy static libraries
cp target/aarch64-apple-ios/release/libmob.a "$XCFW_BUILD/device/"
cp target/aarch64-apple-ios-sim/release/libmob.a "$XCFW_BUILD/simulator/"

# Copy headers to separate directories (so libmob.a isn't treated as a header)
cp "$RN_IOS_DIR/generated/mobFFI.h" "$XCFW_BUILD/device/headers/"
cp "$RN_IOS_DIR/generated/mobFFI.modulemap" "$XCFW_BUILD/device/headers/module.modulemap"
cp "$RN_IOS_DIR/generated/mobFFI.h" "$XCFW_BUILD/simulator/headers/"
cp "$RN_IOS_DIR/generated/mobFFI.modulemap" "$XCFW_BUILD/simulator/headers/module.modulemap"

# Create XCFramework
xcodebuild -create-xcframework \
  -library "$XCFW_BUILD/device/libmob.a" \
  -headers "$XCFW_BUILD/device/headers" \
  -library "$XCFW_BUILD/simulator/libmob.a" \
  -headers "$XCFW_BUILD/simulator/headers" \
  -output "$RN_IOS_DIR/Frameworks/libmob.xcframework"

# Clean up
rm -rf "$XCFW_BUILD"

# Copy module.modulemap for Swift include path discovery
cp "$RN_IOS_DIR/generated/mobFFI.modulemap" "$RN_IOS_DIR/generated/module.modulemap"

echo "iOS build complete."
echo "  XCFramework: $RN_IOS_DIR/Frameworks/libmob.xcframework"
echo "  Swift bindings: $RN_IOS_DIR/generated/"
