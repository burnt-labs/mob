package com.burnt.mob

import com.google.protobuf.util.JsonFormat
import cosmos.bank.v1beta1.*
import cosmos.base.v1beta1.*
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Typed protobuf tests demonstrating usage of xion-types generated Kotlin types
 *
 * These tests showcase strongly-typed protobuf message construction using
 * xion-types definitions instead of untyped dictionaries/strings.
 *
 * Run with: ./gradlew test
 */
class MobTypedTests {
    companion object {
        // Test addresses for demonstration
        const val SENDER_ADDRESS = "xion1abc123def456ghi789jkl012mno345pqr678st"
        const val RECIPIENT_ADDRESS = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
        const val TEST_DENOM = "uxion"
        const val TEST_AMOUNT = "1000000"
    }

    @Test
    fun testCreateTypedCoin() {
        // Create coin using typed builder DSL
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        assertEquals(TEST_DENOM, coin.denom)
        assertEquals(TEST_AMOUNT, coin.amount)
        println("✅ Created typed coin: ${coin.amount} ${coin.denom}")
    }

    @Test
    fun testCoinSerialization() {
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        // Serialize to bytes
        val bytes = coin.toByteArray()
        assertNotNull(bytes)
        assertTrue(bytes.isNotEmpty())

        // Deserialize back
        val decoded = CoinOuterClass.Coin.parseFrom(bytes)
        assertEquals(coin.denom, decoded.denom)
        assertEquals(coin.amount, decoded.amount)
        println("✅ Serialized coin: ${bytes.size} bytes")
    }

    @Test
    fun testCoinJsonEncoding() {
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        // Convert to JSON
        val json = JsonFormat.printer().print(coin)
        assertNotNull(json)
        assertTrue(json.contains(TEST_DENOM))
        assertTrue(json.contains(TEST_AMOUNT))
        println("✅ Coin JSON: $json")
    }

    @Test
    fun testCoinJsonDecoding() {
        val json = """{"denom":"$TEST_DENOM","amount":"$TEST_AMOUNT"}"""

        // Parse from JSON
        val builder = CoinOuterClass.Coin.newBuilder()
        JsonFormat.parser().merge(json, builder)
        val coin = builder.build()

        assertEquals(TEST_DENOM, coin.denom)
        assertEquals(TEST_AMOUNT, coin.amount)
        println("✅ Parsed coin from JSON: ${coin.amount} ${coin.denom}")
    }

    @Test
    fun testCreateTypedMsgSend() {
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        // Create MsgSend using typed builder
        val msgSend = msgSend {
            fromAddress = SENDER_ADDRESS
            toAddress = RECIPIENT_ADDRESS
            amount += coin
        }

        assertEquals(SENDER_ADDRESS, msgSend.fromAddress)
        assertEquals(RECIPIENT_ADDRESS, msgSend.toAddress)
        assertEquals(1, msgSend.amountCount)
        assertEquals(TEST_DENOM, msgSend.getAmount(0).denom)
        assertEquals(TEST_AMOUNT, msgSend.getAmount(0).amount)
        println("✅ Created typed MsgSend from $SENDER_ADDRESS to $RECIPIENT_ADDRESS")
    }

    @Test
    fun testMsgSendWithMultipleCoins() {
        val uxionCoin = coin {
            denom = "uxion"
            amount = "1000000"
        }

        val ustakeCoin = coin {
            denom = "ustake"
            amount = "500000"
        }

        val msgSend = msgSend {
            fromAddress = SENDER_ADDRESS
            toAddress = RECIPIENT_ADDRESS
            amount += listOf(uxionCoin, ustakeCoin)
        }

        assertEquals(2, msgSend.amountCount)
        assertEquals("uxion", msgSend.getAmount(0).denom)
        assertEquals("ustake", msgSend.getAmount(1).denom)
        println("✅ Created MsgSend with ${msgSend.amountCount} coins")
    }

    @Test
    fun testMsgSendSerialization() {
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        val msgSend = msgSend {
            fromAddress = SENDER_ADDRESS
            toAddress = RECIPIENT_ADDRESS
            amount += coin
        }

        // Serialize to bytes
        val bytes = msgSend.toByteArray()
        assertNotNull(bytes)
        assertTrue(bytes.isNotEmpty())

        // Deserialize back
        val decoded = Tx.MsgSend.parseFrom(bytes)
        assertEquals(msgSend.fromAddress, decoded.fromAddress)
        assertEquals(msgSend.toAddress, decoded.toAddress)
        assertEquals(msgSend.amountCount, decoded.amountCount)
        println("✅ Serialized MsgSend: ${bytes.size} bytes")
    }

    @Test
    fun testMsgSendJsonEncoding() {
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        val msgSend = msgSend {
            fromAddress = SENDER_ADDRESS
            toAddress = RECIPIENT_ADDRESS
            amount += coin
        }

        // Convert to JSON
        val json = JsonFormat.printer().print(msgSend)
        assertNotNull(json)
        assertTrue(json.contains(SENDER_ADDRESS))
        assertTrue(json.contains(RECIPIENT_ADDRESS))
        assertTrue(json.contains(TEST_DENOM))
        println("✅ MsgSend JSON: $json")
    }

    @Test
    fun testCreateTypedQueryBalanceRequest() {
        val request = queryBalanceRequest {
            address = SENDER_ADDRESS
            denom = TEST_DENOM
        }

        assertEquals(SENDER_ADDRESS, request.address)
        assertEquals(TEST_DENOM, request.denom)
        println("✅ Created typed QueryBalanceRequest for $SENDER_ADDRESS")
    }

    @Test
    fun testQueryBalanceRequestSerialization() {
        val request = queryBalanceRequest {
            address = SENDER_ADDRESS
            denom = TEST_DENOM
        }

        // Serialize to bytes
        val bytes = request.toByteArray()
        assertNotNull(bytes)
        assertTrue(bytes.isNotEmpty())

        // Deserialize back
        val decoded = QueryOuterClass.QueryBalanceRequest.parseFrom(bytes)
        assertEquals(request.address, decoded.address)
        assertEquals(request.denom, decoded.denom)
        println("✅ Serialized QueryBalanceRequest: ${bytes.size} bytes")
    }

    @Test
    fun testCreateTypedQueryAllBalancesRequest() {
        val request = queryAllBalancesRequest {
            address = SENDER_ADDRESS
        }

        assertEquals(SENDER_ADDRESS, request.address)
        println("✅ Created typed QueryAllBalancesRequest for $SENDER_ADDRESS")
    }

    @Test
    fun testMsgSendTypeUrl() {
        val msgSend = msgSend {
            fromAddress = SENDER_ADDRESS
            toAddress = RECIPIENT_ADDRESS
        }

        // Verify type URL matches Cosmos SDK convention
        val expectedTypeUrl = "/cosmos.bank.v1beta1.MsgSend"
        val actualDescriptor = msgSend.descriptorForType.fullName

        assertTrue(actualDescriptor.contains("cosmos.bank.v1beta1.MsgSend"))
        println("✅ MsgSend type descriptor: $actualDescriptor")
        println("   Expected type URL: $expectedTypeUrl")
    }

    @Test
    fun testCoinTypeUrl() {
        val coin = coin {
            denom = TEST_DENOM
            amount = TEST_AMOUNT
        }

        // Verify type URL matches Cosmos SDK convention
        val actualDescriptor = coin.descriptorForType.fullName

        assertTrue(actualDescriptor.contains("cosmos.base.v1beta1.Coin"))
        println("✅ Coin type descriptor: $actualDescriptor")
        println("   Expected type URL: /cosmos.base.v1beta1.Coin")
    }

    @Test
    fun testDecCoinCreation() {
        // Create DecCoin (decimal coin for fractional amounts)
        val decCoin = decCoin {
            denom = TEST_DENOM
            amount = "1000000.123456789"
        }

        assertEquals(TEST_DENOM, decCoin.denom)
        assertEquals("1000000.123456789", decCoin.amount)
        println("✅ Created typed DecCoin: ${decCoin.amount} ${decCoin.denom}")
    }

    @Test
    fun testMsgSendModification() {
        // Create initial message
        var msgSend = msgSend {
            fromAddress = SENDER_ADDRESS
            toAddress = RECIPIENT_ADDRESS
            amount += coin {
                denom = TEST_DENOM
                amount = "1000000"
            }
        }

        // Modify using copy builder
        msgSend = msgSend.copy {
            amount += coin {
                denom = "ustake"
                amount = "500000"
            }
        }

        assertEquals(2, msgSend.amountCount)
        assertEquals("uxion", msgSend.getAmount(0).denom)
        assertEquals("ustake", msgSend.getAmount(1).denom)
        println("✅ Modified MsgSend now has ${msgSend.amountCount} coins")
    }

    @Test
    fun testEmptyMessage() {
        // Create empty message and verify defaults
        val msgSend = msgSend {}

        assertEquals("", msgSend.fromAddress)
        assertEquals("", msgSend.toAddress)
        assertEquals(0, msgSend.amountCount)
        println("✅ Created empty MsgSend with default values")
    }
}
