#if canImport(SwiftProtobuf) && canImport(XionTypes)
import XCTest
@testable import Mob
import SwiftProtobuf
import XionTypes

// Typed tests using xion-types Swift protobuf definitions
// These tests demonstrate using strongly-typed protobuf messages with the mob library
//
// Prerequisites:
// 1. xion-types Swift package must be added as a dependency
// 2. Import the cosmos.bank.v1beta1 and cosmos.tx.v1beta1 modules
//
// Run with: swift test

final class MobTypedTests: XCTestCase {
    let rpcEndpoint = "https://rpc.xion-testnet-2.burnt.com:443"
    let chainId = "xion-testnet-2"
    let addressPrefix = "xion"

    // Test mnemonic (DO NOT USE IN PRODUCTION)
    let testMnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna " +
                       "put fitness slide dash point basket jaguar fun humor multiply " +
                       "emotion rescue brand pull"

    var config: ChainConfig!
    var signer: Signer!

    override func setUp() {
        super.setUp()

        config = ChainConfig(
            chainId: chainId,
            rpcEndpoint: rpcEndpoint,
            grpcEndpoint: nil,
            addressPrefix: addressPrefix,
            coinType: 118,
            gasPrice: "0.025"
        )

        signer = try! Signer.fromMnemonic(
            mnemonic: testMnemonic,
            addressPrefix: addressPrefix,
            derivationPath: "m/44'/118'/0'/0/0"
        )
    }

    // MARK: - Typed Coin Tests

    func testCreateTypedCoin() {
        // Create a strongly-typed Cosmos coin using protobuf
        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "1000000" // 1 XION = 1,000,000 uxion

        XCTAssertEqual("uxion", coin.denom)
        XCTAssertEqual("1000000", coin.amount)
        print("✅ Created typed Cosmos coin: \(coin.amount) \(coin.denom)")
    }

    func testSerializeTypedCoin() throws {
        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "500000"

        // Encode to protobuf bytes
        let data = try coin.serializedData()
        XCTAssertGreaterThan(data.count, 0)
        print("✅ Serialized coin to \(data.count) bytes")

        // Decode from protobuf bytes
        let decoded = try Cosmos_Base_V1beta1_Coin(serializedData: data)
        XCTAssertEqual(coin.denom, decoded.denom)
        XCTAssertEqual(coin.amount, decoded.amount)
        print("✅ Deserialized coin: \(decoded.amount) \(decoded.denom)")
    }

    func testCoinToJSON() throws {
        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "750000"

        // Encode to JSON
        let jsonData = try coin.jsonUTF8Data()
        let jsonString = String(data: jsonData, encoding: .utf8)!

        XCTAssertTrue(jsonString.contains("uxion"))
        XCTAssertTrue(jsonString.contains("750000"))
        print("✅ Coin as JSON: \(jsonString)")

        // Decode from JSON
        let fromJSON = try Cosmos_Base_V1beta1_Coin(jsonUTF8Data: jsonData)
        XCTAssertEqual(coin.denom, fromJSON.denom)
        XCTAssertEqual(coin.amount, fromJSON.amount)
        print("✅ Decoded coin from JSON")
    }

    // MARK: - Typed MsgSend Tests

    func testCreateTypedMsgSend() {
        let senderAddress = signer.address()
        let recipientAddress = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"

        // Create typed MsgSend message
        var msgSend = Cosmos_Bank_V1beta1_MsgSend()
        msgSend.fromAddress = senderAddress
        msgSend.toAddress = recipientAddress

        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "1000"
        msgSend.amount = [coin]

        XCTAssertEqual(senderAddress, msgSend.fromAddress)
        XCTAssertEqual(recipientAddress, msgSend.toAddress)
        XCTAssertEqual(1, msgSend.amount.count)
        XCTAssertEqual("uxion", msgSend.amount[0].denom)
        XCTAssertEqual("1000", msgSend.amount[0].amount)

        print("✅ Created typed MsgSend:")
        print("   From: \(msgSend.fromAddress)")
        print("   To: \(msgSend.toAddress)")
        print("   Amount: \(msgSend.amount[0].amount) \(msgSend.amount[0].denom)")
        print("   Type URL: /\(Cosmos_Bank_V1beta1_MsgSend.protoMessageName)")
    }

    func testSerializeMsgSend() throws {
        let senderAddress = signer.address()

        var msgSend = Cosmos_Bank_V1beta1_MsgSend()
        msgSend.fromAddress = senderAddress
        msgSend.toAddress = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"

        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "2000"
        msgSend.amount = [coin]

        // Serialize to protobuf
        let data = try msgSend.serializedData()
        XCTAssertGreaterThan(data.count, 0)
        print("✅ Serialized MsgSend to \(data.count) bytes")

        // Deserialize
        let decoded = try Cosmos_Bank_V1beta1_MsgSend(serializedData: data)
        XCTAssertEqual(msgSend.fromAddress, decoded.fromAddress)
        XCTAssertEqual(msgSend.toAddress, decoded.toAddress)
        XCTAssertEqual(msgSend.amount.count, decoded.amount.count)
        print("✅ Deserialized MsgSend successfully")
    }

    func testMsgSendToJSON() throws {
        var msgSend = Cosmos_Bank_V1beta1_MsgSend()
        msgSend.fromAddress = "xion1sender..."
        msgSend.toAddress = "xion1recipient..."

        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "5000"
        msgSend.amount = [coin]

        // Convert to JSON
        let jsonData = try msgSend.jsonUTF8Data()
        let jsonString = String(data: jsonData, encoding: .utf8)!

        XCTAssertTrue(jsonString.contains("fromAddress"))
        XCTAssertTrue(jsonString.contains("toAddress"))
        XCTAssertTrue(jsonString.contains("uxion"))
        print("✅ MsgSend as JSON:")
        print("   \(jsonString)")

        // Decode from JSON
        let fromJSON = try Cosmos_Bank_V1beta1_MsgSend(jsonUTF8Data: jsonData)
        XCTAssertEqual(msgSend.fromAddress, fromJSON.fromAddress)
        print("✅ Decoded MsgSend from JSON")
    }

    // MARK: - Typed Query Tests

    func testCreateTypedQueryBalanceRequest() {
        let address = signer.address()

        var request = Cosmos_Bank_V1beta1_QueryBalanceRequest()
        request.address = address
        request.denom = "uxion"

        XCTAssertEqual(address, request.address)
        XCTAssertEqual("uxion", request.denom)

        print("✅ Created typed QueryBalanceRequest:")
        print("   Address: \(request.address)")
        print("   Denom: \(request.denom)")
        print("   Type URL: /\(Cosmos_Bank_V1beta1_QueryBalanceRequest.protoMessageName)")
    }

    func testSerializeQueryBalanceRequest() throws {
        var request = Cosmos_Bank_V1beta1_QueryBalanceRequest()
        request.address = signer.address()
        request.denom = "uxion"

        // Serialize to protobuf
        let data = try request.serializedData()
        XCTAssertGreaterThan(data.count, 0)
        print("✅ Serialized QueryBalanceRequest to \(data.count) bytes")

        // Deserialize
        let decoded = try Cosmos_Bank_V1beta1_QueryBalanceRequest(serializedData: data)
        XCTAssertEqual(request.address, decoded.address)
        XCTAssertEqual(request.denom, decoded.denom)
        print("✅ Deserialized QueryBalanceRequest successfully")
    }

    func testCreateTypedQueryBalanceResponse() throws {
        var response = Cosmos_Bank_V1beta1_QueryBalanceResponse()

        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = "uxion"
        coin.amount = "1000000"
        response.balance = coin

        XCTAssertEqual("uxion", response.balance.denom)
        XCTAssertEqual("1000000", response.balance.amount)

        print("✅ Created typed QueryBalanceResponse:")
        print("   Balance: \(response.balance.amount) \(response.balance.denom)")

        // Test serialization roundtrip
        let data = try response.serializedData()
        let decoded = try Cosmos_Bank_V1beta1_QueryBalanceResponse(serializedData: data)
        XCTAssertEqual(response.balance.amount, decoded.balance.amount)
        print("✅ Roundtrip serialization successful")
    }

    func testCreateTypedQueryAllBalancesRequest() {
        var request = Cosmos_Bank_V1beta1_QueryAllBalancesRequest()
        request.address = signer.address()
        request.resolveDenom = false

        XCTAssertEqual(signer.address(), request.address)
        XCTAssertFalse(request.resolveDenom)

        print("✅ Created typed QueryAllBalancesRequest:")
        print("   Address: \(request.address)")
        print("   Type URL: /\(Cosmos_Bank_V1beta1_QueryAllBalancesRequest.protoMessageName)")
    }

    // MARK: - Type URL and Metadata Tests

    func testProtobufTypeURLs() {
        // Verify type URLs match expected Cosmos SDK format
        XCTAssertEqual("cosmos.base.v1beta1.Coin", Cosmos_Base_V1beta1_Coin.protoMessageName)
        XCTAssertEqual("cosmos.bank.v1beta1.MsgSend", Cosmos_Bank_V1beta1_MsgSend.protoMessageName)
        XCTAssertEqual("cosmos.bank.v1beta1.QueryBalanceRequest", Cosmos_Bank_V1beta1_QueryBalanceRequest.protoMessageName)
        XCTAssertEqual("cosmos.bank.v1beta1.QueryBalanceResponse", Cosmos_Bank_V1beta1_QueryBalanceResponse.protoMessageName)

        print("✅ All protobuf type URLs verified:")
        print("   Coin: /\(Cosmos_Base_V1beta1_Coin.protoMessageName)")
        print("   MsgSend: /\(Cosmos_Bank_V1beta1_MsgSend.protoMessageName)")
        print("   QueryBalanceRequest: /\(Cosmos_Bank_V1beta1_QueryBalanceRequest.protoMessageName)")
        print("   QueryBalanceResponse: /\(Cosmos_Bank_V1beta1_QueryBalanceResponse.protoMessageName)")
    }

    func testMultipleCoins() {
        var msgSend = Cosmos_Bank_V1beta1_MsgSend()
        msgSend.fromAddress = "xion1sender..."
        msgSend.toAddress = "xion1recipient..."

        // Create multiple coins
        var coin1 = Cosmos_Base_V1beta1_Coin()
        coin1.denom = "uxion"
        coin1.amount = "1000"

        var coin2 = Cosmos_Base_V1beta1_Coin()
        coin2.denom = "uatom"
        coin2.amount = "500"

        msgSend.amount = [coin1, coin2]

        XCTAssertEqual(2, msgSend.amount.count)
        XCTAssertEqual("uxion", msgSend.amount[0].denom)
        XCTAssertEqual("uatom", msgSend.amount[1].denom)

        print("✅ Created MsgSend with multiple coins:")
        for coin in msgSend.amount {
            print("   \(coin.amount) \(coin.denom)")
        }
    }
}

// MARK: - Helper Extension

extension MobTypedTests {
    /// Helper to create a typed Cosmos coin
    func makeCoin(denom: String, amount: String) -> Cosmos_Base_V1beta1_Coin {
        var coin = Cosmos_Base_V1beta1_Coin()
        coin.denom = denom
        coin.amount = amount
        return coin
    }
}
#endif
