package com.burnt.mob.examples

import com.burnt.mob.NativeHttpTransport
import uniffi.mob.*

/**
 * Basic Query Example
 *
 * Demonstrates simple read-only blockchain queries:
 * - Block height
 * - Node sync status
 * - Chain ID
 * - Account balance
 * - Account information
 */
fun main() {
    println("Mob Kotlin - Basic Query Example\n")

    // Configure connection to XION testnet
    val config = ChainConfig(
        chainId = "xion-testnet-2",
        rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443",
        grpcEndpoint = null,
        addressPrefix = "xion",
        coinType = 118u,
        gasPrice = "0.025"
    )

    // Create a read-only client with platform-native HTTP transport
    val transport = NativeHttpTransport()
    val client = Client(config, transport)
    println("Connected to XION testnet\n")

    // Query 1: Current block height
    println("Query 1: Block Height")
    try {
        val height = client.getHeight()
        println("   Current height: $height\n")
    } catch (e: Exception) {
        println("   Error: ${e.message}\n")
    }

    // Query 2: Node sync status
    println("Query 2: Node Sync Status")
    try {
        val synced = client.isSynced()
        println("   Is synced: $synced\n")
    } catch (e: Exception) {
        println("   Error: ${e.message}\n")
    }

    // Query 3: Chain ID
    println("Query 3: Chain ID")
    try {
        val chainId = client.getChainId()
        println("   Chain ID: $chainId\n")
    } catch (e: Exception) {
        println("   Error: ${e.message}\n")
    }

    // Query 4: Account balance
    println("Query 4: Account Balance")
    val testAddress = "xion1sxu85s77uf6r0rydud7jx6xvygn8cdu3gns84q"
    try {
        val balance = client.getBalance(testAddress, "uxion")
        println("   Address: $testAddress")
        println("   Denom: ${balance.denom}")
        println("   Amount: ${balance.amount} uxion")

        // Convert to human-readable XION (1 XION = 1,000,000 uxion)
        val balanceAmount = balance.amount.toULongOrNull() ?: 0u
        val xionAmount = balanceAmount.toDouble() / 1_000_000.0
        println("   Amount: $xionAmount XION\n")
    } catch (e: Exception) {
        println("   Error: ${e.message}\n")
    }

    // Query 5: Account information
    println("Query 5: Account Information")
    try {
        val account = client.getAccount(testAddress)
        println("   Address: ${account.address}")
        println("   Account number: ${account.accountNumber}")
        println("   Sequence: ${account.sequence}\n")
    } catch (e: Exception) {
        println("   Error: ${e.message}\n")
    }

    println("All queries completed!")
}
