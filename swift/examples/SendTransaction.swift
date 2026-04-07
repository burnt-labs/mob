import Foundation
import Mob

// Send transaction example
// WARNING: This sends real funds on testnet

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

// Test mnemonic (DO NOT USE IN PRODUCTION)
let mnemonic = "your test mnemonic here"
let recipient = "xion1recipient..."

do {
    // Create signer
    let signer = try RustSigner.fromMnemonic(
        mnemonic: mnemonic,
        addressPrefix: "xion",
        derivationPath: "m/44'/118'/0'/0/0"
    )

    let senderAddress = signer.address()
    print("Sender: \(senderAddress)")

    // Create client with signer and native transport
    let transport = NativeHttpTransport()
    let client = try Client.newWithSigner(config: config, signer: signer, transport: transport)

    // Check balance
    let balance = try client.getBalance(address: senderAddress, denom: "uxion")
    print("Balance: \(balance.amount) \(balance.denom)")

    // Send transaction
    let amount = [Coin(denom: "uxion", amount: "1000")]
    let txResponse = try client.send(
        toAddress: recipient,
        amount: amount,
        memo: "Test transaction from Swift"
    )

    print("Transaction sent!")
    print("Hash: \(txResponse.txhash)")
    print("Code: \(txResponse.code)")

    if txResponse.code == 0 {
        print("Transaction successful!")
    } else {
        print("Transaction failed: \(txResponse.rawLog)")
    }

} catch {
    print("Error: \(error)")
}
