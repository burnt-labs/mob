import XCTest
@testable import Mob

// Swift tests for mob library RPC queries against XION testnet
//
// Run with: swift test
// Run integration tests: INTEGRATION=1 swift test

final class MobTests: XCTestCase {
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

    func testCreateClient() throws {
        let client = try Client(config: config)
        XCTAssertNotNil(client)
    }

    func testGetHeight() throws {
        let client = try Client(config: config)
        let height = try client.getHeight()

        XCTAssertGreaterThan(height, 0, "Height should be greater than 0")
        print("✅ Current block height: \(height)")
    }

    func testGetChainId() throws {
        let client = try Client(config: config)
        let chainId = try client.getChainId()

        XCTAssertEqual(self.chainId, chainId)
        print("✅ Chain ID: \(chainId)")
    }

    func testIsSynced() throws {
        let client = try Client(config: config)
        let isSynced = try client.isSynced()

        XCTAssertNotNil(isSynced)
        print("✅ Node synced: \(isSynced)")
    }

    func testCreateSigner() throws {
        let signer = try Signer.fromMnemonic(
            mnemonic: testMnemonic,
            addressPrefix: addressPrefix,
            derivationPath: "m/44'/118'/0'/0/0"
        )

        XCTAssertNotNil(signer)
        let address = signer.address()
        XCTAssertTrue(address.hasPrefix(addressPrefix), "Address should start with \(addressPrefix)")
        print("✅ Signer address: \(address)")
    }

    func testGetAccount() throws {
        let client = try Client(config: config)
        let address = signer.address()

        let accountInfo = try client.getAccount(address: address)

        XCTAssertEqual(address, accountInfo.address)
        XCTAssertGreaterThanOrEqual(accountInfo.accountNumber, 0)
        XCTAssertGreaterThanOrEqual(accountInfo.sequence, 0)
        print("✅ Account number: \(accountInfo.accountNumber), Sequence: \(accountInfo.sequence)")
    }

    func testGetBalance() throws {
        let client = try Client(config: config)
        let address = signer.address()

        let balance = try client.getBalance(address: address, denom: "uxion")

        XCTAssertEqual("uxion", balance.denom)
        XCTAssertGreaterThanOrEqual(UInt64(balance.amount) ?? 0, 0)
        print("✅ Balance: \(balance.amount) \(balance.denom)")
    }

    func testSignMessage() throws {
        let message = "Hello, XION!".data(using: .utf8)!
        let signature = try signer.signBytes(message: message)

        XCTAssertNotNil(signature)
        XCTAssertGreaterThan(signature.count, 0)
        print("✅ Signed message, signature length: \(signature.count) bytes")
    }

    func testInvalidMnemonic() {
        XCTAssertThrowsError(try Signer.fromMnemonic(
            mnemonic: "invalid mnemonic words",
            addressPrefix: "xion",
            derivationPath: "m/44'/118'/0'/0/0"
        ))
        print("✅ Invalid mnemonic properly rejected")
    }

    func testInvalidAddress() throws {
        let client = try Client(config: config)

        XCTAssertThrowsError(try client.getAccount(address: "invalid_address"))
        print("✅ Invalid address properly rejected")
    }

    func testMultipleSigners() throws {
        let signer1 = try Signer.fromMnemonic(
            mnemonic: testMnemonic,
            addressPrefix: "xion",
            derivationPath: "m/44'/118'/0'/0/0"
        )

        let signer2 = try Signer.fromMnemonic(
            mnemonic: testMnemonic,
            addressPrefix: "xion",
            derivationPath: "m/44'/118'/0'/0/1"
        )

        let addr1 = signer1.address()
        let addr2 = signer2.address()

        XCTAssertNotEqual(addr1, addr2, "Different derivation paths should yield different addresses")
        print("✅ Account 0: \(addr1)")
        print("✅ Account 1: \(addr2)")
    }

    func testCoinCreation() {
        let coin = Coin(denom: "uxion", amount: "1000000")

        XCTAssertEqual("uxion", coin.denom)
        XCTAssertEqual("1000000", coin.amount)
        print("✅ Created coin: \(coin.amount) \(coin.denom)")
    }
}

// Integration test for sending funds (run with INTEGRATION=1 environment variable)
final class MobIntegrationTests: XCTestCase {
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

        // Skip if INTEGRATION environment variable is not set
        guard ProcessInfo.processInfo.environment["INTEGRATION"] == "1" else {
            return
        }

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

    func testSendFundsToAddress() throws {
        // Skip if INTEGRATION environment variable is not set
        guard ProcessInfo.processInfo.environment["INTEGRATION"] == "1" else {
            print("⏭️  Skipping integration test (set INTEGRATION=1 to run)")
            return
        }

        print("\n💸 Testing fund transfer on XION testnet...\n")

        let recipient = "xion14yy92ae8eq0q3ezy9nasumt65hwdgryvpkf0a4"
        let senderAddress = signer.address()

        print("1️⃣  Test Configuration:")
        print("   🔗 Chain: \(chainId)")
        print("   📡 RPC: \(rpcEndpoint)")
        print("   👤 Sender: \(senderAddress)")
        print("   🎯 Recipient: \(recipient)")

        print("\n2️⃣  Creating client with signer attached...")
        let client = try Client.newWithSigner(config: config, signer: signer)
        print("   ✅ Client connected with signer attached")

        print("\n3️⃣  Querying sender balance...")
        let balance = try client.getBalance(address: senderAddress, denom: "uxion")
        guard let balanceAmount = UInt64(balance.amount) else {
            XCTFail("Could not parse balance amount")
            return
        }

        print("   💰 Current uxion balance: \(balance.amount) uxion")

        guard balanceAmount > 0 else {
            print("\n   ⚠️  WARNING: Sender has no uxion balance!")
            print("   Please fund the test account: \(senderAddress)")
            print("   Skipping transaction...")
            return
        }

        guard balanceAmount >= 6000 else {
            print("\n   ⚠️  WARNING: Insufficient balance (\(balanceAmount) uxion)")
            print("   Need at least 6000 uxion (1000 to send + 5000 fee)")
            return
        }

        print("\n4️⃣  Preparing transaction...")
        let amount = [Coin(denom: "uxion", amount: "1000")]

        print("   📤 Sending 1000 uxion to \(recipient)")
        print("   📝 Memo: Test fund transfer from Swift")

        print("\n5️⃣  Broadcasting transaction...")
        let txResponse = try client.send(
            toAddress: recipient,
            amount: amount,
            memo: "Test fund transfer from Swift"
        )

        print("   ✅ Transaction broadcast successful!")
        print("   📝 Transaction hash: \(txResponse.txhash)")
        print("   📊 Code: \(txResponse.code)")

        XCTAssertEqual(0, txResponse.code, "Transaction failed with code \(txResponse.code): \(txResponse.rawLog)")

        if txResponse.code == 0 {
            print("   ✅ Transaction accepted by mempool")
        } else {
            print("   ⚠️  Transaction failed with code: \(txResponse.code)")
            print("   📋 Log: \(txResponse.rawLog)")
        }

        print("\n6️⃣  Waiting for transaction confirmation (10 seconds)...")
        sleep(10)

        print("\n7️⃣  Querying transaction result...")
        let txResult = try client.getTx(hash: txResponse.txhash)
        print("   ✅ Transaction confirmed!")
        print("   📊 Final code: \(txResult.code)")
        print("   ⛽ Gas used: \(txResult.gasUsed)")
        print("   ⛽ Gas wanted: \(txResult.gasWanted)")
        print("   📏 Block height: \(txResult.height)")

        XCTAssertEqual(0, txResult.code, "Transaction failed with code \(txResult.code)")

        print("\n🎉 Fund transfer test completed!\n")
    }
}
