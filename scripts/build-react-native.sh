#!/bin/bash
set -e

# Build mob native libraries for React Native (iOS + Android)
# Usage:
#   ./scripts/build-react-native.sh          # Build both platforms
#   ./scripts/build-react-native.sh ios      # Build iOS only
#   ./scripts/build-react-native.sh android  # Build Android only

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

PLATFORM="${1:-all}"

case "$PLATFORM" in
    ios)
        "$SCRIPT_DIR/build-ios.sh"
        ;;
    android)
        "$SCRIPT_DIR/build-android.sh"
        ;;
    all)
        "$SCRIPT_DIR/build-ios.sh"
        "$SCRIPT_DIR/build-android.sh"
        ;;
    *)
        echo "Usage: $0 [ios|android|all]"
        exit 1
        ;;
esac

echo "React Native build complete."
