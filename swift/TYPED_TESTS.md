# Typed Tests Using xion-types

The `MobTypedTests.swift` file demonstrates using strongly-typed Swift protobuf definitions from the xion-types library with the mob client.

## Prerequisites

### Add xion-types as a Dependency

To use the typed tests, you need to add xion-types as a Swift Package dependency.

#### Option 1: Add to Package.swift

Update your `Package.swift` to include xion-types:

```swift
// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "Mob",
    platforms: [
        .macOS(.v12),
        .iOS(.v15)
    ],
    products: [
        .library(
            name: "Mob",
            targets: ["Mob", "MobFFI"]),
    ],
    dependencies: [
        // Add xion-types dependency
        .package(url: "https://github.com/burnt-labs/xion-types", branch: "main")
    ],
    targets: [
        .target(
            name: "Mob",
            dependencies: [
                "MobFFI",
                // Add xion-types as a target dependency
                .product(name: "XionTypes", package: "xion-types")
            ],
            path: "Sources/Mob"
        ),
        .target(
            name: "MobFFI",
            dependencies: [],
            path: "Sources/MobFFI"
        ),
        .testTarget(
            name: "MobTests",
            dependencies: [
                "Mob",
                // Add xion-types to test target
                .product(name: "XionTypes", package: "xion-types")
            ],
            path: "Tests/MobTests"
        )
    ]
)
```

#### Option 2: Use Local xion-types

For development, you can use a local checkout:

```swift
dependencies: [
    .package(path: "/path/to/xion-types/swift")
],
```

### Import in Test Files

In your test files, import the xion-types modules:

```swift
import XCTest
@testable import Mob
import SwiftProtobuf
// Import xion-types cosmos modules
```

Note: The actual import statements depend on how xion-types Swift package is structured.

## What the Typed Tests Cover

### Coin Tests
- Creating strongly-typed `Cosmos_Base_V1beta1_Coin` objects
- Serializing coins to protobuf bytes
- Converting coins to/from JSON
- Protobuf roundtrip serialization

### MsgSend Tests
- Creating typed `Cosmos_Bank_V1beta1_MsgSend` messages
- Setting sender, recipient, and amount fields
- Serializing messages to protobuf format
- JSON encoding/decoding
- Multiple coin transfers

### Query Tests
- Creating `Cosmos_Bank_V1beta1_QueryBalanceRequest` queries
- Building `Cosmos_Bank_V1beta1_QueryBalanceResponse` objects
- Creating `Cosmos_Bank_V1beta1_QueryAllBalancesRequest` queries
- Protobuf serialization of queries

### Type URL Tests
- Verifying protobuf message type URLs
- Checking Cosmos SDK format compliance

## Running the Tests

Run all tests including typed tests:

```bash
swift test
```

Run only typed tests:

```bash
swift test --filter MobTypedTests
```

Run a specific typed test:

```bash
swift test --filter MobTypedTests/testCreateTypedMsgSend
```

## Benefits of Typed Tests

1. **Type Safety**: Compile-time checking of message structure
2. **IDE Support**: Autocomplete and type hints for all fields
3. **Documentation**: Generated from proto definitions
4. **Compatibility**: Guaranteed compatibility with Cosmos SDK
5. **Validation**: Proto field validation at compile time

## Example Usage

### Creating a Typed Coin

```swift
var coin = Cosmos_Base_V1beta1_Coin()
coin.denom = "uxion"
coin.amount = "1000000"
```

### Creating a Typed MsgSend

```swift
var msgSend = Cosmos_Bank_V1beta1_MsgSend()
msgSend.fromAddress = signer.address()
msgSend.toAddress = "xion1recipient..."
msgSend.amount = [coin]

// Serialize to protobuf
let data = try msgSend.serializedData()

// Convert to JSON
let jsonData = try msgSend.jsonUTF8Data()
```

### Creating a Typed Query

```swift
var request = Cosmos_Bank_V1beta1_QueryBalanceRequest()
request.address = "xion1..."
request.denom = "uxion"

// Serialize for ABCI query
let queryData = try request.serializedData()
```

## Type URLs Reference

All protobuf messages have associated type URLs for use with `Any` types:

- **Coin**: `cosmos.base.v1beta1.Coin`
- **MsgSend**: `cosmos.bank.v1beta1.MsgSend`
- **QueryBalanceRequest**: `cosmos.bank.v1beta1.QueryBalanceRequest`
- **QueryBalanceResponse**: `cosmos.bank.v1beta1.QueryBalanceResponse`
- **QueryAllBalancesRequest**: `cosmos.bank.v1beta1.QueryAllBalancesRequest`

Access via: `MessageType.protoMessageName`

## Integration with Mob Client

While these tests demonstrate protobuf type usage, the mob client currently uses its own internal types. Future integration could:

1. Accept protobuf messages directly in client methods
2. Return protobuf-typed responses
3. Provide conversion utilities between mob types and protobuf types

## Further Reading

- [xion-types Repository](https://github.com/burnt-labs/xion-types)
- [SwiftProtobuf Documentation](https://github.com/apple/swift-protobuf)
- [Cosmos SDK Protobuf](https://docs.cosmos.network/main/build/building-modules/messages-and-queries)
