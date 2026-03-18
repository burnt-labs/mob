#!/bin/bash
set -e

# Build mob library for Android targets and generate Kotlin bindings for React Native
# Requires: Android NDK, cargo-ndk (cargo install cargo-ndk)
# Outputs .so files and Kotlin bindings to react-native/android/

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ROOT_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"
RN_ANDROID_DIR="$ROOT_DIR/react-native/android"

echo "Building mob for Android..."

# Verify cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null; then
    echo "Error: cargo-ndk not found. Install with: cargo install cargo-ndk"
    exit 1
fi

# Verify ANDROID_NDK_HOME is set
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "Error: ANDROID_NDK_HOME not set."
    echo "Set it to your NDK path, e.g.: export ANDROID_NDK_HOME=\$HOME/Library/Android/sdk/ndk/<version>"
    exit 1
fi

# Ensure Rust targets are installed
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  x86_64-linux-android \
  i686-linux-android 2>/dev/null || true

# Build for all Android ABIs
echo "Building native libraries for all ABIs..."
cargo ndk \
  -t aarch64-linux-android \
  -t armv7-linux-androideabi \
  -t x86_64-linux-android \
  -t i686-linux-android \
  -o "$RN_ANDROID_DIR/src/main/jniLibs" \
  build --release -p mob

# Generate Kotlin bindings using host build
echo "Generating Kotlin bindings..."
cargo build --release -p mob

# Detect host library extension
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
elif [[ "$OSTYPE" == "linux"* ]]; then
    LIB_EXT="so"
else
    LIB_EXT="dll"
fi

mkdir -p "$RN_ANDROID_DIR/src/main/java/uniffi/mob"

cargo run --bin uniffi-bindgen generate \
  --library "target/release/libmob.$LIB_EXT" \
  --language kotlin \
  --out-dir "$RN_ANDROID_DIR/src/main/java"

echo "Android build complete."
echo "  JNI libraries: $RN_ANDROID_DIR/src/main/jniLibs/"
echo "  Kotlin bindings: $RN_ANDROID_DIR/src/main/java/uniffi/mob/mob.kt"
