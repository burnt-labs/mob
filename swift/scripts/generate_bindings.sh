#!/bin/bash
set -e

# Script to generate the Swift package artifacts from the iOS build flow.
# Run this from the project root directory.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RN_IOS_DIR="$ROOT_DIR/react-native/ios"
SWIFT_DIR="$ROOT_DIR/swift"

echo "🔨 Building iOS artifacts..."
"$ROOT_DIR/scripts/build-ios.sh"

echo "📋 Syncing Swift package artifacts..."
mkdir -p "$SWIFT_DIR/lib" "$SWIFT_DIR/Sources/Mob"
rsync -a --delete "$RN_IOS_DIR/Frameworks/libmob.xcframework/" "$SWIFT_DIR/lib/libmob.xcframework/"
cp "$RN_IOS_DIR/generated/mob.swift" "$SWIFT_DIR/Sources/Mob/mob.swift"

echo "✅ Swift bindings and XCFramework created successfully!"
echo ""
echo "Package artifacts are ready under:"
echo "  swift/Sources/Mob/"
echo "  swift/lib/libmob.xcframework"
echo ""
echo "The XCFramework is located at:"
echo "  swift/lib/libmob.xcframework"
