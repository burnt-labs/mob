#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🔨 Mob Kotlin Bindings Generator${NC}\n"

# Get the root directory (parent of kotlin/)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
KOTLIN_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"
ROOT_DIR="$( cd "$KOTLIN_DIR/.." && pwd )"

echo "📂 Directories:"
echo "   Root: $ROOT_DIR"
echo "   Kotlin: $KOTLIN_DIR"

# Step 1: Build the Rust library
echo -e "\n${YELLOW}Step 1: Building Rust library...${NC}"
cd "$ROOT_DIR"
cargo build --release -p mob
echo -e "${GREEN}✅ Rust library built${NC}"

# Step 2: Generate Kotlin bindings
echo -e "\n${YELLOW}Step 2: Generating Kotlin bindings...${NC}"
cargo run --bin uniffi-bindgen generate \
  --library target/release/libmob.dylib \
  --language kotlin \
  --out-dir kotlin/lib
echo -e "${GREEN}✅ Kotlin bindings generated${NC}"

# Step 3: Copy native library
echo -e "\n${YELLOW}Step 3: Copying native library...${NC}"
cp target/release/libmob.dylib kotlin/lib/
echo -e "${GREEN}✅ Native library copied${NC}"

# Step 4: Verify outputs
echo -e "\n${YELLOW}Step 4: Verifying outputs...${NC}"

if [ -f "$KOTLIN_DIR/lib/uniffi/mob/mob.kt" ]; then
    BINDING_SIZE=$(wc -c < "$KOTLIN_DIR/lib/uniffi/mob/mob.kt" | xargs)
    echo -e "${GREEN}✅ Kotlin bindings: $BINDING_SIZE bytes${NC}"
else
    echo -e "${RED}❌ Kotlin bindings not found${NC}"
    exit 1
fi

if [ -f "$KOTLIN_DIR/lib/libmob.dylib" ]; then
    echo -e "${GREEN}✅ Native library present${NC}"
    file "$KOTLIN_DIR/lib/libmob.dylib"
else
    echo -e "${RED}❌ Native library not found${NC}"
    exit 1
fi

# Step 5: Build Kotlin project
echo -e "\n${YELLOW}Step 5: Building Kotlin project...${NC}"
cd "$KOTLIN_DIR"
./gradlew build --quiet
echo -e "${GREEN}✅ Kotlin project built${NC}"

# Summary
echo -e "\n${GREEN}🎉 Kotlin bindings generated successfully!${NC}\n"
echo "📋 Generated files:"
echo "   - $KOTLIN_DIR/lib/uniffi/mob/mob.kt"
echo "   - $KOTLIN_DIR/lib/libmob.dylib"

echo -e "\n📝 Next steps:"
echo "   Run tests:        cd kotlin && ./gradlew test"
echo "   Integration test: cd kotlin && INTEGRATION=1 ./gradlew test"
echo "   Clean build:      cd kotlin && ./gradlew clean build"

echo -e "\n✅ Done!"
