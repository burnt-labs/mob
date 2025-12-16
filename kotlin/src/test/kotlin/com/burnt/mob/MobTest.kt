package com.burnt.mob

import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows
import uniffi.mob.*
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Kotlin tests for mob library RPC queries against XION testnet
 *
 * Run with: ./gradlew test
 */
class MobTest {
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
    fun testCreateClient() {
        val client = Client(config)
        assertNotNull(client)
    }

    @Test
    fun testGetHeight() {
        val client = Client(config)
        val height = client.getHeight()

        assertTrue(height > 0u, "Height should be greater than 0")
        println("✅ Current block height: $height")
    }

    @Test
    fun testGetChainId() {
        val client = Client(config)
        val chainId = client.getChainId()

        assertEquals(CHAIN_ID, chainId)
        println("✅ Chain ID: $chainId")
    }

    @Test
    fun testIsSynced() {
        val client = Client(config)
        val isSynced = client.isSynced()

        assertNotNull(isSynced)
        println("✅ Node synced: $isSynced")
    }

    @Test
    fun testCreateSigner() {
        val signer = Signer.fromMnemonic(
            mnemonic = TEST_MNEMONIC,
            addressPrefix = ADDRESS_PREFIX,
            derivationPath = "m/44'/118'/0'/0/0"
        )

        assertNotNull(signer)
        val address = signer.address()
        assertTrue(address.startsWith(ADDRESS_PREFIX), "Address should start with $ADDRESS_PREFIX")
        println("✅ Signer address: $address")
    }

    @Test
    fun testGetAccount() {
        val client = Client(config)
        val address = signer.address()

        val accountInfo = client.getAccount(address)

        assertEquals(address, accountInfo.address)
        assertTrue(accountInfo.accountNumber >= 0u)
        assertTrue(accountInfo.sequence >= 0u)
        println("✅ Account number: ${accountInfo.accountNumber}, Sequence: ${accountInfo.sequence}")
    }

    @Test
    fun testGetBalance() {
        val client = Client(config)
        val address = signer.address()

        val balance = client.getBalance(address, "uxion")

        assertEquals("uxion", balance.denom)
        assertNotNull(balance.amount)
        println("✅ Balance: ${balance.amount} ${balance.denom}")
    }

    @Test
    fun testSignMessage() {
        val message = "Hello, XION!".toByteArray()
        val signature = signer.signBytes(message)

        assertNotNull(signature)
        assertTrue(signature.isNotEmpty())
        println("✅ Signed message, signature length: ${signature.size} bytes")
    }

    @Test
    fun testInvalidMnemonic() {
        assertThrows<MobException.KeyDerivation> {
            Signer.fromMnemonic(
                mnemonic = "invalid mnemonic words",
                addressPrefix = "xion",
                derivationPath = "m/44'/118'/0'/0/0"
            )
        }
        println("✅ Invalid mnemonic properly rejected")
    }

    @Test
    fun testInvalidAddress() {
        val client = Client(config)

        assertThrows<Exception> {
            client.getAccount("invalid_address")
        }
        println("✅ Invalid address properly rejected")
    }

    @Test
    fun testMultipleSigners() {
        val signer1 = Signer.fromMnemonic(
            mnemonic = TEST_MNEMONIC,
            addressPrefix = "xion",
            derivationPath = "m/44'/118'/0'/0/0"
        )

        val signer2 = Signer.fromMnemonic(
            mnemonic = TEST_MNEMONIC,
            addressPrefix = "xion",
            derivationPath = "m/44'/118'/0'/0/1"
        )

        val addr1 = signer1.address()
        val addr2 = signer2.address()

        assertTrue(addr1 != addr2, "Different derivation paths should yield different addresses")
        println("✅ Account 0: $addr1")
        println("✅ Account 1: $addr2")
    }

    @Test
    fun testCoinCreation() {
        val coin = Coin(denom = "uxion", amount = "1000000")

        assertEquals("uxion", coin.denom)
        assertEquals("1000000", coin.amount)
        println("✅ Created coin: ${coin.amount} ${coin.denom}")
    }
}
