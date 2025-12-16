package com.burnt.mob

import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import uniffi.mob.*
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Integration test for sending funds (run with INTEGRATION=1 environment variable)
 *
 * Run with: INTEGRATION=1 ./gradlew test
 */
class IntegrationTest {
    companion object {
        const val RPC_ENDPOINT = "https://rpc.xion-testnet-2.burnt.com:443"
        const val CHAIN_ID = "xion-testnet-2"
        const val ADDRESS_PREFIX = "xion"

        // Test mnemonic (DO NOT USE IN PRODUCTION)
        const val TEST_MNEMONIC = "quiz cattle knock bacon million abstract word reunion educate antenna " +
                "put fitness slide dash point basket jaguar fun humor multiply " +
                "emotion rescue brand pull"

        init {
            // Load native library
            System.loadLibrary("mob")
        }
    }

    private lateinit var config: ChainConfig
    private lateinit var signer: Signer

    @BeforeEach
    fun setUp() {
        config = ChainConfig(
            chainId = CHAIN_ID,
            rpcEndpoint = RPC_ENDPOINT,
            grpcEndpoint = null,
            addressPrefix = ADDRESS_PREFIX,
            coinType = 118u,
            gasPrice = "0.025"
        )

        signer = Signer.fromMnemonic(
            mnemonic = TEST_MNEMONIC,
            addressPrefix = ADDRESS_PREFIX,
            derivationPath = "m/44'/118'/0'/0/0"
        )
    }

    @Test
    fun testSendFundsToAddress() {
        println("\n💸 Testing fund transfer on XION testnet...\n")

        val recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
        val senderAddress = signer.address()

        println("1️⃣  Test Configuration:")
        println("   🔗 Chain: $CHAIN_ID")
        println("   📡 RPC: $RPC_ENDPOINT")
        println("   👤 Sender: $senderAddress")
        println("   🎯 Recipient: $recipient")

        println("\n2️⃣  Creating client with signer attached...")
        val client = Client.newWithSigner(config, signer)
        println("   ✅ Client connected with signer attached")

        println("\n3️⃣  Querying sender balance...")
        val balance = client.getBalance(senderAddress, "uxion")
        val balanceAmount = balance.amount.toULongOrNull() ?: 0u

        println("   💰 Current uxion balance: ${balance.amount} uxion")

        if (balanceAmount == 0uL) {
            println("\n   ⚠️  WARNING: Sender has no uxion balance!")
            println("   Please fund the test account: $senderAddress")
            println("   Skipping transaction...")
            return
        }

        if (balanceAmount < 6000u) {
            println("\n   ⚠️  WARNING: Insufficient balance ($balanceAmount uxion)")
            println("   Need at least 6000 uxion (1000 to send + 5000 fee)")
            return
        }

        println("\n4️⃣  Preparing transaction...")
        val amount = listOf(Coin(denom = "uxion", amount = "1000"))

        println("   📤 Sending 1000 uxion to $recipient")
        println("   📝 Memo: Test fund transfer from Kotlin")

        println("\n5️⃣  Broadcasting transaction...")
        val txResponse = client.send(
            toAddress = recipient,
            amount = amount,
            memo = "Test fund transfer from Kotlin"
        )

        println("   ✅ Transaction broadcast successful!")
        println("   📝 Transaction hash: ${txResponse.txhash}")
        println("   📊 Code: ${txResponse.code}")

        assertEquals(0u, txResponse.code, "Transaction failed with code ${txResponse.code}: ${txResponse.rawLog}")

        if (txResponse.code == 0u) {
            println("   ✅ Transaction accepted by mempool")
        } else {
            println("   ⚠️  Transaction failed with code: ${txResponse.code}")
            println("   📋 Log: ${txResponse.rawLog}")
        }

        println("\n6️⃣  Waiting for transaction confirmation (10 seconds)...")
        Thread.sleep(10000)

        println("\n7️⃣  Querying transaction result...")
        val txResult = client.getTx(txResponse.txhash)
        println("   ✅ Transaction confirmed!")
        println("   📊 Final code: ${txResult.code}")
        println("   ⛽ Gas used: ${txResult.gasUsed}")
        println("   ⛽ Gas wanted: ${txResult.gasWanted}")
        println("   📏 Block height: ${txResult.height}")

        assertEquals(0u, txResult.code, "Transaction failed with code ${txResult.code}")

        println("\n🎉 Fund transfer test completed!\n")
    }
}
