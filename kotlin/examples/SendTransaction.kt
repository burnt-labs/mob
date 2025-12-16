package com.burnt.mob.examples

import uniffi.mob.*

/**
 * Send Transaction Example
 *
 * Demonstrates the complete flow for sending tokens:
 * 1. Configure chain connection
 * 2. Create signer from mnemonic
 * 3. Check sender balance
 * 4. Send transaction
 * 5. Wait for confirmation
 * 6. Query transaction result
 *
 * ⚠️ WARNING: This example uses a test mnemonic and sends real transactions
 * on testnet. Make sure the account has sufficient balance.
 *
 * Run with: ./gradlew run -PmainClass=com.burnt.mob.examples.SendTransactionKt
 */
fun main() {
    println("💸 Mob Kotlin - Send Transaction Example\n")

    // Step 1: Configure chain connection
    println("1️⃣  Configuring chain connection...")
    val config = ChainConfig(
        chainId = "xion-testnet-2",
        rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443",
        grpcEndpoint = null,
        addressPrefix = "xion",
        coinType = 118u,
        gasPrice = "0.025"
    )
    println("   ✅ Connected to ${config.chainId}\n")

    // Step 2: Create signer from mnemonic
    println("2️⃣  Creating signer from mnemonic...")

    // Option 1: Use environment variable (RECOMMENDED)
    val mnemonic = System.getenv("XION_MNEMONIC") ?: run {
        println("   ⚠️  XION_MNEMONIC environment variable not set")
        println("   Using test mnemonic (DO NOT USE IN PRODUCTION)")

        // Test mnemonic - generates address: xion1sxu85s77uf6r0rydud7jx6xvygn8cdu3gns84q
        "quiz cattle knock bacon million abstract word reunion educate antenna " +
        "put fitness slide dash point basket jaguar fun humor multiply " +
        "emotion rescue brand pull"
    }

    val signer = Signer.fromMnemonic(
        mnemonic = mnemonic,
        addressPrefix = "xion",
        derivationPath = "m/44'/118'/0'/0/0"
    )

    val senderAddress = signer.address()
    println("   ✅ Signer created")
    println("   👤 Address: $senderAddress\n")

    // Step 3: Check sender balance
    println("3️⃣  Checking sender balance...")
    val client = Client.newWithSigner(config, signer)

    val balance = try {
        client.getBalance(senderAddress, "uxion")
    } catch (e: Exception) {
        println("   ❌ Error querying balance: ${e.message}")
        return
    }

    val balanceAmount = balance.amount.toULongOrNull() ?: 0u
    println("   💰 Current balance: ${balance.amount} uxion")

    if (balanceAmount == 0uL) {
        println("   ⚠️  WARNING: Sender has no uxion balance!")
        println("   Please fund the account: $senderAddress")
        println("   Skipping transaction...")
        return
    }

    if (balanceAmount < 6000u) {
        println("   ⚠️  WARNING: Insufficient balance ($balanceAmount uxion)")
        println("   Need at least 6000 uxion (1000 to send + ~5000 for gas)")
        return
    }

    val xionBalance = balanceAmount.toDouble() / 1_000_000.0
    println("   💰 Balance: $xionBalance XION\n")

    // Step 4: Send transaction
    println("4️⃣  Preparing and sending transaction...")
    val recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
    val amount = listOf(Coin(denom = "uxion", amount = "1000"))
    val memo = "Test transaction from Kotlin"

    println("   📤 Sending:")
    println("      From: $senderAddress")
    println("      To: $recipient")
    println("      Amount: 1000 uxion (0.001 XION)")
    println("      Memo: $memo")

    val txResponse = try {
        client.send(
            toAddress = recipient,
            amount = amount,
            memo = memo
        )
    } catch (e: Exception) {
        println("   ❌ Error broadcasting transaction: ${e.message}")
        return
    }

    println("   ✅ Transaction broadcast successful!")
    println("   📝 Transaction hash: ${txResponse.txhash}")
    println("   📊 Code: ${txResponse.code}")

    if (txResponse.code != 0u) {
        println("   ❌ Transaction failed with code ${txResponse.code}")
        println("   📋 Log: ${txResponse.rawLog}\n")
        return
    }

    println("   ✅ Transaction accepted by mempool\n")

    // Step 5: Wait for confirmation
    println("5️⃣  Waiting for transaction confirmation...")
    println("   ⏳ Waiting 10 seconds...")
    Thread.sleep(10000)

    // Step 6: Query transaction result
    println("\n6️⃣  Querying transaction result...")
    val txResult = try {
        client.getTx(txResponse.txhash)
    } catch (e: Exception) {
        println("   ❌ Error querying transaction: ${e.message}")
        println("   Transaction may still be pending or failed to confirm")
        return
    }

    println("   ✅ Transaction confirmed!")
    println("   📊 Final code: ${txResult.code}")
    println("   📏 Block height: ${txResult.height}")
    println("   ⛽ Gas used: ${txResult.gasUsed}")
    println("   ⛽ Gas wanted: ${txResult.gasWanted}")

    if (txResult.code == 0u) {
        println("\n🎉 Transaction completed successfully!")
        println("   View on explorer: https://explorer.burnt.com/xion-testnet-2/tx/${txResponse.txhash}")
    } else {
        println("\n❌ Transaction failed with code ${txResult.code}")
    }

    // Step 7: Check updated balance
    println("\n7️⃣  Checking updated balance...")
    val newBalance = try {
        client.getBalance(senderAddress, "uxion")
    } catch (e: Exception) {
        println("   ❌ Error querying balance: ${e.message}")
        return
    }

    val newBalanceAmount = newBalance.amount.toULongOrNull() ?: 0u
    val newXionBalance = newBalanceAmount.toDouble() / 1_000_000.0
    println("   💰 New balance: ${newBalance.amount} uxion ($newXionBalance XION)")

    val spent = balanceAmount - newBalanceAmount
    println("   📉 Amount spent: $spent uxion (includes gas fees)\n")

    println("✅ Example completed!")
}
