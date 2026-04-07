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

        const val TEST_MNEMONIC = "quiz cattle knock bacon million abstract word reunion educate antenna " +
                "put fitness slide dash point basket jaguar fun humor multiply " +
                "emotion rescue brand pull"

        init {
            System.loadLibrary("mob")
        }
    }

    private lateinit var config: ChainConfig
    private lateinit var signer: RustSigner
    private lateinit var transport: NativeHttpTransport

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

        signer = RustSigner.fromMnemonic(
            mnemonic = TEST_MNEMONIC,
            addressPrefix = ADDRESS_PREFIX,
            derivationPath = "m/44'/118'/0'/0/0"
        )

        transport = NativeHttpTransport()
    }

    @Test
    fun testSendFundsToAddress() {
        println("\nTesting fund transfer on XION testnet...\n")

        val recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
        val senderAddress = signer.address()

        println("1. Creating client with signer attached...")
        val client = Client.newWithSigner(config, signer, transport)

        println("\n2. Querying sender balance...")
        val balance = client.getBalance(senderAddress, "uxion")
        val balanceAmount = balance.amount.toULongOrNull() ?: 0u
        println("   Current balance: ${balance.amount} uxion")

        if (balanceAmount == 0uL) {
            println("   Sender has no balance, skipping")
            return
        }

        if (balanceAmount < 6000u) {
            println("   Insufficient balance ($balanceAmount uxion), skipping")
            return
        }

        println("\n3. Broadcasting transaction...")
        val amount = listOf(Coin(denom = "uxion", amount = "1000"))
        val txResponse = client.send(
            toAddress = recipient,
            amount = amount,
            memo = "Test fund transfer from Kotlin"
        )

        println("   Transaction hash: ${txResponse.txhash}")
        println("   Code: ${txResponse.code}")
        assertEquals(0u, txResponse.code)

        println("\n4. Waiting for confirmation (10 seconds)...")
        Thread.sleep(10000)

        println("\n5. Querying transaction result...")
        val txResult = client.getTx(txResponse.txhash)
        println("   Confirmed at height: ${txResult.height}")
        assertEquals(0u, txResult.code)

        println("\nFund transfer test completed.\n")
    }
}
