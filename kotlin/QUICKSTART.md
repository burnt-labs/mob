# Mob Kotlin Quick Start Guide

This guide provides practical examples for using the mob Kotlin bindings with XION blockchain.

## Installation

Add the necessary dependencies to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation(kotlin("stdlib"))
    implementation("net.java.dev.jna:jna:5.14.0")
}

sourceSets {
    main {
        kotlin { srcDir("path/to/mob/kotlin/lib/uniffi") }
        resources { srcDir("path/to/mob/kotlin/lib") }
    }
}
```

## Basic Setup

### Import the Library

```kotlin
import uniffi.mob.*
```

### Configure Chain Connection

```kotlin
val config = ChainConfig(
    chainId = "xion-testnet-2",
    rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint = null,
    addressPrefix = "xion",
    coinType = 118u,
    gasPrice = "0.025"
)
```

## Common Operations

### 1. Query Blockchain State

```kotlin
// Create a read-only client
val client = Client(config)

// Get current block height
val height = client.getHeight()
println("Current height: $height")

// Check if node is synced
val synced = client.isSynced()
println("Node synced: $synced")

// Get chain ID
val chainId = client.getChainId()
println("Chain ID: $chainId")
```

### 2. Check Account Balance

```kotlin
val address = "xion1sxu85s77uf6r0rydud7jx6xvygn8cdu3gns84q"

// Query balance
val balance = client.getBalance(address, "uxion")
println("Balance: ${balance.amount} ${balance.denom}")

// Convert to human-readable format (1 XION = 1,000,000 uxion)
val balanceAmount = balance.amount.toULongOrNull() ?: 0u
val xionAmount = balanceAmount.toDouble() / 1_000_000.0
println("Balance: $xionAmount XION")
```

### 3. Get Account Information

```kotlin
val account = client.getAccount(address)
println("Account number: ${account.accountNumber}")
println("Sequence: ${account.sequence}")
println("Address: ${account.address}")
```

### 4. Create a Signer from Mnemonic

```kotlin
val mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna " +
               "put fitness slide dash point basket jaguar fun humor multiply " +
               "emotion rescue brand pull"

val signer = Signer.fromMnemonic(
    mnemonic = mnemonic,
    addressPrefix = "xion",
    derivationPath = "m/44'/118'/0'/0/0"
)

val address = signer.address()
println("Address: $address")

val publicKey = signer.publicKey()
println("Public key: ${publicKey.joinToString("")}")
```

### 5. Sign a Message

```kotlin
val message = "Hello, XION!".toByteArray()

val signature = signer.signBytes(message.toList())
println("Signature length: ${signature.size} bytes")
```

### 6. Send Tokens

```kotlin
// Create client with signer attached
val client = Client.newWithSigner(config, signer)

// Prepare transaction
val recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
val amount = listOf(Coin(denom = "uxion", amount = "1000"))

// Broadcast transaction
val txResponse = client.send(
    toAddress = recipient,
    amount = amount,
    memo = "Payment for services"
)

println("Transaction hash: ${txResponse.txhash}")
println("Code: ${txResponse.code}")

if (txResponse.code == 0u) {
    println("Transaction successful!")
} else {
    println("Transaction failed: ${txResponse.rawLog}")
}
```

### 7. Query Transaction Status

```kotlin
val txHash = "YOUR_TX_HASH_HERE"

val txResult = client.getTx(txHash)
println("Transaction status:")
println("  Code: ${txResult.code}")
println("  Height: ${txResult.height}")
println("  Gas used: ${txResult.gasUsed}")
println("  Gas wanted: ${txResult.gasWanted}")
```

## Complete Example: Send Transaction Flow

```kotlin
import uniffi.mob.*

fun main() {
    // 1. Configure chain
    val config = ChainConfig(
        chainId = "xion-testnet-2",
        rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443",
        grpcEndpoint = null,
        addressPrefix = "xion",
        coinType = 118u,
        gasPrice = "0.025"
    )

    // 2. Create signer from mnemonic
    val mnemonic = System.getenv("XION_MNEMONIC")
        ?: throw Exception("Set XION_MNEMONIC environment variable")

    val signer = Signer.fromMnemonic(
        mnemonic = mnemonic,
        addressPrefix = "xion",
        derivationPath = "m/44'/118'/0'/0/0"
    )

    val senderAddress = signer.address()
    println("Sender address: $senderAddress")

    // 3. Create client
    val client = Client.newWithSigner(config, signer)

    // 4. Check balance
    val balance = client.getBalance(senderAddress, "uxion")
    println("Current balance: ${balance.amount} uxion")

    val balanceAmount = balance.amount.toULongOrNull() ?: 0u
    if (balanceAmount < 6000u) {
        println("Insufficient balance. Need at least 6000 uxion.")
        return
    }

    // 5. Send transaction
    val recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
    val amount = listOf(Coin(denom = "uxion", amount = "1000"))

    println("Sending 1000 uxion to $recipient...")
    val txResponse = client.send(
        toAddress = recipient,
        amount = amount,
        memo = "Test transaction"
    )

    println("Transaction broadcast!")
    println("  Hash: ${txResponse.txhash}")
    println("  Code: ${txResponse.code}")

    if (txResponse.code != 0u) {
        println("  Error: ${txResponse.rawLog}")
        return
    }

    // 6. Wait for confirmation
    println("Waiting for confirmation...")
    Thread.sleep(10000)

    // 7. Query transaction result
    val txResult = client.getTx(txResponse.txhash)
    println("Transaction confirmed!")
    println("  Height: ${txResult.height}")
    println("  Gas used: ${txResult.gasUsed}")
    println("  Code: ${txResult.code}")

    if (txResult.code == 0u) {
        println("Transaction successful")
    } else {
        println("Transaction failed")
    }
}
```

## Error Handling

```kotlin
try {
    val client = Client(config)
    val balance = client.getBalance(address, "uxion")
    println("Balance: ${balance.amount}")
} catch (e: Exception) {
    when {
        e.message?.contains("connection refused") == true ->
            println("Cannot connect to RPC endpoint")
        e.message?.contains("not found") == true ->
            println("Account or transaction not found")
        else ->
            println("Error: ${e.message}")
    }
}
```

## Working with Multiple Accounts

```kotlin
val mnemonic1 = "first mnemonic phrase..."
val mnemonic2 = "second mnemonic phrase..."

val signer1 = Signer.fromMnemonic(mnemonic1, "xion", "m/44'/118'/0'/0/0")
val signer2 = Signer.fromMnemonic(mnemonic2, "xion", "m/44'/118'/0'/0/0")

println("Account 1: ${signer1.address()}")
println("Account 2: ${signer2.address()}")

// Create separate clients for each signer
val client1 = Client.newWithSigner(config, signer1)
val client2 = Client.newWithSigner(config, signer2)
```

## Best Practices

### 1. Secure Mnemonic Storage

Do not hardcode mnemonics:
```kotlin
val mnemonic = "quiz cattle knock bacon..." // NEVER DO THIS
```

Use environment variables or secure key storage:
```kotlin
val mnemonic = System.getenv("XION_MNEMONIC")
    ?: throw Exception("XION_MNEMONIC not set")
```

### 2. Check Balances Before Transactions

```kotlin
val balance = client.getBalance(senderAddress, "uxion")
val balanceAmount = balance.amount.toULongOrNull() ?: 0u

// Need amount to send + gas fees
val amountToSend = 1000u
val estimatedGas = 5000u

if (balanceAmount < amountToSend + estimatedGas) {
    println("Insufficient balance")
    return
}
```

### 3. Handle Transaction Failures

```kotlin
val txResponse = client.send(recipient, amount, memo)

if (txResponse.code != 0u) {
    println("Transaction failed: ${txResponse.rawLog}")
    // Don't proceed with confirmation check
    return
}

// Wait and confirm
Thread.sleep(10000)
val txResult = client.getTx(txResponse.txhash)

if (txResult.code != 0u) {
    println("Transaction confirmed but failed in block")
}
```

### 4. Use Appropriate Gas Prices

```kotlin
// Mainnet might need higher gas prices
val mainnetConfig = ChainConfig(
    chainId = "xion-mainnet-1",
    rpcEndpoint = "https://rpc.xion-mainnet-1.burnt.com:443",
    grpcEndpoint = null,
    addressPrefix = "xion",
    coinType = 118u,
    gasPrice = "0.05"  // Adjust based on network congestion
)
```

## Testing

### Unit Tests

Use the test account mnemonic for development:
```kotlin
const val TEST_MNEMONIC = "quiz cattle knock bacon million abstract word reunion educate antenna " +
                         "put fitness slide dash point basket jaguar fun humor multiply " +
                         "emotion rescue brand pull"
```

This generates address: `xion1sxu85s77uf6r0rydud7jx6xvygn8cdu3gns84q`

### Integration Tests

Run with funded test account:
```bash
INTEGRATION=1 ./gradlew test
```

## Additional Resources

- [Kotlin README](README.md) - Installation and setup
- [XION Documentation](https://docs.xion.burnt.com) - Blockchain details
- [Mob Repository](https://github.com/burnt-labs/mob) - Source code

## Support

For issues or questions:
- GitHub Issues: https://github.com/burnt-labs/mob/issues
- Discord: https://discord.gg/burnt
