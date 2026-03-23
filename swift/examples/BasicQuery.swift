import Foundation
import Mob

// Basic RPC query example

let config = ChainConfig(
    chainId: "xion-testnet-2",
    rpcEndpoint: "https://rpc.xion-testnet-2.burnt.com:443",
    grpcEndpoint: nil,
    addressPrefix: "xion",
    coinType: 118,
    gasPrice: "0.025"
)

do {
    let transport = NativeHttpTransport()
    let client = try Client(config: config, transport: transport)

    print("Connected to XION testnet")

    let height = try client.getHeight()
    print("Current block height: \(height)")

    let chainId = try client.getChainId()
    print("Chain ID: \(chainId)")

    let isSynced = try client.isSynced()
    print("Node synced: \(isSynced)")

} catch {
    print("Error: \(error)")
}
