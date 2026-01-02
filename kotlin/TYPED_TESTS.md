# Typed Protobuf Tests for Kotlin

This document explains how to use the typed protobuf tests with xion-types in the Kotlin client.

## Overview

The `MobTypedTests.kt` file demonstrates using strongly-typed protobuf messages from xion-types instead of untyped dictionaries or string manipulation. This provides:

- **Type Safety**: Compile-time verification of message structure
- **IDE Support**: Autocomplete and inline documentation
- **Protobuf Validation**: Automatic field validation
- **Cosmos SDK Compatibility**: Guaranteed compatibility with Cosmos SDK message formats

## Adding xion-types Dependency

### Option 1: Local Path Dependency

If you have xion-types checked out locally:

```kotlin
// In build.gradle.kts
dependencies {
    // Add the xion-types kotlin protobuf library
    implementation(files("/path/to/xion-types/kotlin/types"))

    // Or as a source set
    implementation(project(":xion-types-kotlin"))
}
```

### Option 2: Published Artifact

If xion-types is published to a Maven repository:

```kotlin
// In build.gradle.kts
dependencies {
    implementation("com.burnt:xion-types:0.0.0-dev")

    // Required protobuf dependencies
    implementation("com.google.protobuf:protobuf-kotlin:3.25.1")
    implementation("com.google.protobuf:protobuf-java-util:3.25.1")
}
```

### Option 3: Git Submodule

Add xion-types as a git submodule and include it in your build:

```bash
git submodule add https://github.com/burnt-labs/xion-types.git
```

```kotlin
// In settings.gradle.kts
include(":xion-types:kotlin")
project(":xion-types:kotlin").projectDir = file("xion-types/kotlin")

// In build.gradle.kts
dependencies {
    implementation(project(":xion-types:kotlin"))
}
```

## Required Imports

```kotlin
import cosmos.bank.v1beta1.*        // Bank module types (MsgSend, queries)
import cosmos.base.v1beta1.*        // Base types (Coin, DecCoin)
import cosmos.auth.v1beta1.*        // Auth module types (accounts)
import cosmos.tx.v1beta1.*          // Transaction types
import com.google.protobuf.util.JsonFormat  // For JSON conversion
```

## Running the Tests

```bash
# Run all tests including typed tests
./gradlew test

# Run only typed tests
./gradlew test --tests MobTypedTests

# Run specific typed test
./gradlew test --tests MobTypedTests.testCreateTypedCoin
```

## Example Usage Patterns

### Creating a Typed Coin

```kotlin
import cosmos.base.v1beta1.*

val coin = coin {
    denom = "uxion"
    amount = "1000000"
}
```

### Creating a Typed MsgSend

```kotlin
import cosmos.bank.v1beta1.*
import cosmos.base.v1beta1.*

val msgSend = msgSend {
    fromAddress = "xion1..."
    toAddress = "xion1..."
    amount += coin {
        denom = "uxion"
        amount = "1000000"
    }
}
```

### Adding Multiple Coins

```kotlin
val msgSend = msgSend {
    fromAddress = "xion1..."
    toAddress = "xion1..."
    amount += listOf(
        coin { denom = "uxion"; amount = "1000000" },
        coin { denom = "ustake"; amount = "500000" }
    )
}
```

### Creating Query Requests

```kotlin
import cosmos.bank.v1beta1.*

val balanceRequest = queryBalanceRequest {
    address = "xion1..."
    denom = "uxion"
}

val allBalancesRequest = queryAllBalancesRequest {
    address = "xion1..."
}
```

### Serialization and Deserialization

```kotlin
// Serialize to protobuf bytes
val bytes = msgSend.toByteArray()

// Deserialize from protobuf bytes
val decoded = Tx.MsgSend.parseFrom(bytes)

// Convert to JSON
val json = JsonFormat.printer().print(msgSend)

// Parse from JSON
val builder = Tx.MsgSend.newBuilder()
JsonFormat.parser().merge(json, builder)
val msgFromJson = builder.build()
```

### Modifying Messages

```kotlin
// Create initial message
var msgSend = msgSend {
    fromAddress = "xion1..."
    toAddress = "xion1..."
}

// Modify using copy
msgSend = msgSend.copy {
    amount += coin {
        denom = "uxion"
        amount = "1000000"
    }
}
```

## Test Coverage

The typed tests cover:

1. **Coin Operations**
   - Creating typed coins
   - Serialization/deserialization
   - JSON encoding/decoding
   - DecCoin (fractional amounts)

2. **MsgSend Operations**
   - Creating typed MsgSend messages
   - Single and multiple coins
   - Message serialization
   - JSON encoding
   - Message modification

3. **Query Operations**
   - QueryBalanceRequest
   - QueryAllBalancesRequest
   - Request serialization

4. **Type Verification**
   - Type URL validation
   - Descriptor verification
   - Empty message defaults

## Benefits of Typed Messages

### Before (Untyped)

```kotlin
// String-based message construction - error-prone
val msgJson = """
{
  "from_address": "$fromAddress",
  "to_address": "$toAddress",
  "amount": [{"denom": "uxion", "amount": "1000000"}]
}
"""
```

### After (Typed)

```kotlin
// Strongly-typed construction - compile-time safe
val msgSend = msgSend {
    fromAddress = fromAddress
    toAddress = toAddress
    amount += coin { denom = "uxion"; amount = "1000000" }
}
```

## Type URLs

Common Cosmos SDK type URLs used in transactions:

| Message Type | Type URL |
|--------------|----------|
| MsgSend | /cosmos.bank.v1beta1.MsgSend |
| Coin | /cosmos.base.v1beta1.Coin |
| DecCoin | /cosmos.base.v1beta1.DecCoin |
| QueryBalanceRequest | /cosmos.bank.v1beta1.QueryBalanceRequest |

## Integration with Mob Client

The typed messages can be serialized and used with the mob client:

```kotlin
// Create typed message
val msgSend = msgSend {
    fromAddress = signer.address()
    toAddress = recipientAddress
    amount += coin { denom = "uxion"; amount = "1000000" }
}

// Serialize to bytes for signing
val msgBytes = msgSend.toByteArray()

// Use with mob client (when supported)
// client.broadcastTx(msgBytes)
```

## Additional Resources

- [Protocol Buffers Kotlin Documentation](https://protobuf.dev/reference/kotlin/kotlin-generated/)
- [xion-types Repository](https://github.com/burnt-labs/xion-types)
- [Cosmos SDK Proto Definitions](https://github.com/cosmos/cosmos-sdk/tree/main/proto)
