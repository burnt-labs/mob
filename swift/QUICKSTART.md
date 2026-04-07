# Quick Start Guide

This guide will help you get started with the Mob Swift bindings for the XION blockchain.

## Basic RPC Query

```swift
import Mob

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

do {
    let client = try Client(config: config)

    // Query blockchain height
    let height = try client.getHeight()
    print("Current height: \(height)")

    // Query chain ID
    let chainId = try client.getChainId()
    print("Chain ID: \(chainId)")

    // Check sync status
    let isSynced = try client.isSynced()
    print("Node synced: \(isSynced)")

} catch {
    print("Error: \(error)")
}
```

## Creating a Signer

```swift
import Mob

do {
    // Create from mnemonic
    let signer = try Signer.fromMnemonic(
        mnemonic: "your twelve or twenty-four word mnemonic here",
        prefix: "xion",
        derivationPath: "m/44'/118'/0'/0/0"
    )

    // Get address
    let address = signer.address()
    print("Address: \(address)")

    // Get public key
    let pubKey = signer.publicKey()
    print("Public key: \(pubKey)")

} catch {
    print("Error: \(error)")
}
```

## Querying Account Information

```swift
import Mob

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

do {
    let client = try Client(config: config)
    let address = "xion1..."

    // Get account info
    let account = try client.getAccount(address: address)
    print("Account number: \(account.accountNumber)")
    print("Sequence: \(account.sequence)")

    // Get balance
    let balance = try client.getBalance(address: address, denom: "uxion")
    print("Balance: \(balance.amount) \(balance.denom)")

} catch {
    print("Error: \(error)")
}
```

## Signing Messages

```swift
import Mob

do {
    let signer = try Signer.fromMnemonic(
        mnemonic: "your mnemonic here",
        prefix: "xion",
        derivationPath: "m/44'/118'/0'/0/0"
    )

    // Sign arbitrary message
    let message = "Hello, XION!"
    let signature = try signer.signBytes(message: message)
    print("Signature: \(signature)")

} catch {
    print("Error: \(error)")
}
```

## Sending a Transaction

```swift
import Mob

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

do {
    // Create signer
    let signer = try Signer.fromMnemonic(
        mnemonic: "your mnemonic here",
        prefix: "xion",
        derivationPath: "m/44'/118'/0'/0/0"
    )

    // Create client with signer attached
    let client = try Client.newWithSigner(config: config, signer: signer)

    // Send transaction
    let recipient = "xion1recipient..."
    let amount = [Coin(denom: "uxion", amount: "1000000")]

    let txResponse = try client.send(
        toAddress: recipient,
        amount: amount,
        memo: "My first transaction"
    )

    print("Transaction hash: \(txResponse.txhash)")
    print("Code: \(txResponse.code)")  // 0 = success

    if txResponse.code == 0 {
        print("Transaction successful!")
    } else {
        print("Transaction failed: \(txResponse.rawLog)")
    }

} catch {
    print("Error: \(error)")
}
```

## Querying Transaction Results

```swift
import Mob

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

do {
    let client = try Client(config: config)

    // Query by transaction hash
    let txHash = "ABC123..."
    let txResult = try client.getTx(hash: txHash)

    print("Block height: \(txResult.height)")
    print("Gas used: \(txResult.gasUsed)")
    print("Gas wanted: \(txResult.gasWanted)")
    print("Result code: \(txResult.code)")

} catch {
    print("Error: \(error)")
}
```

## Complete Example

Here's a complete example that ties everything together:

```swift
import Foundation
import Mob

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

do {
    // Create signer from mnemonic
    let mnemonic = ProcessInfo.processInfo.environment["TEST_MNEMONIC"] ?? "your test mnemonic here"
    let signer = try Signer.fromMnemonic(
        mnemonic: mnemonic,
        prefix: "xion",
        derivationPath: "m/44'/118'/0'/0/0"
    )

    let address = signer.address()
    print("Using address: \(address)")

    // Create client with signer
    let client = try Client.newWithSigner(config: config, signer: signer)

    // Query current height
    let height = try client.getHeight()
    print("Current height: \(height)")

    // Query balance
    let balance = try client.getBalance(address: address, denom: "uxion")
    print("Balance: \(balance.amount) \(balance.denom)")

    // Send tokens
    let recipient = "xion1recipient..."
    let amount = [Coin(denom: "uxion", amount: "1000")]

    let txResponse = try client.send(
        toAddress: recipient,
        amount: amount,
        memo: "Test transaction"
    )

    if txResponse.code == 0 {
        print("Transaction successful")
        print("Tx hash: \(txResponse.txhash)")
    } else {
        print("Transaction failed")
        print("Error: \(txResponse.rawLog)")
    }

} catch {
    print("Error: \(error)")
}
```

## Error Handling

Swift bindings use standard Swift error handling:

```swift
do {
    let client = try Client(config: config)
    let height = try client.getHeight()
    print("Height: \(height)")
} catch let error as MobError {
    switch error {
    case .NetworkError(let message):
        print("Network error: \(message)")
    case .SigningError(let message):
        print("Signing error: \(message)")
    default:
        print("Error: \(error)")
    }
} catch {
    print("Unexpected error: \(error)")
}
```

## Using with iOS/macOS Apps

### SwiftUI Example

```swift
import SwiftUI
import Mob

struct ContentView: View {
    @State private var height: UInt64?
    @State private var error: String?

    var body: some View {
        VStack {
            if let height = height {
                Text("Block Height: \(height)")
            } else if let error = error {
                Text("Error: \(error)")
                    .foregroundColor(.red)
            } else {
                ProgressView()
            }
        }
        .task {
            await loadHeight()
        }
    }

    func loadHeight() async {
        let config = ChainConfig(
            chainId: "xion-testnet-2",
            rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
            grpcEndpoint: nil,
            addressPrefix: "xion",
            coinType: 118,
            gasPrice: "0.025"
        )

        do {
            let client = try Client(config: config)
            let fetchedHeight = try client.getHeight()
            await MainActor.run {
                self.height = fetchedHeight
            }
        } catch {
            await MainActor.run {
                self.error = error.localizedDescription
            }
        }
    }
}
```

## Next Steps

- Check out the [examples](examples/) directory for more detailed examples
- Read the [API Reference](README.md#api-reference) for complete documentation
- Run the [test suite](README.md#running-tests) to verify your setup

## Security Notes

Never use production mnemonics in code or tests.

Use environment variables for sensitive data:

```swift
guard let mnemonic = ProcessInfo.processInfo.environment["MNEMONIC"] else {
    fatalError("MNEMONIC not set")
}
```

## Troubleshooting

If you encounter issues:

1. Verify the library is in the correct location: `swift/lib/libmob.dylib`
2. Check network connectivity to the RPC endpoint
3. Ensure your account has sufficient balance for transactions
4. Review the [Troubleshooting](README.md#troubleshooting) section in the main README
