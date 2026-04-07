import Foundation

public typealias Signer = RustSigner

public extension ChainConfig {
    init(
        chainId: String,
        rpcEndpoint: String,
        addressPrefix: String,
        grpcEndpoint: String? = nil,
        coinType: UInt32 = 118,
        gasPrice: String = "0.025",
        feeGranter: String? = nil
    ) {
        self.init(
            chainId: chainId,
            rpcEndpoint: rpcEndpoint,
            grpcEndpoint: grpcEndpoint,
            addressPrefix: addressPrefix,
            coinType: coinType,
            gasPrice: gasPrice,
            feeGranter: feeGranter
        )
    }
}

public extension Client {
    convenience init(config: ChainConfig) throws {
        try self.init(config: config, transport: NativeHttpTransport())
    }

    static func newWithSigner(config: ChainConfig, signer: RustSigner) throws -> Client {
        try self.newWithSigner(config: config, signer: signer, transport: NativeHttpTransport())
    }

    static func newWithCryptoSigner(
        config: ChainConfig,
        signer: CryptoSigner
    ) throws -> Client {
        try self.newWithCryptoSigner(
            config: config,
            signer: signer,
            transport: NativeHttpTransport()
        )
    }

    static func newWithSessionSigner(
        config: ChainConfig,
        signer: RustSigner,
        metadata: SessionMetadata
    ) throws -> Client {
        try self.newWithSessionSigner(
            config: config,
            signer: signer,
            metadata: metadata,
            transport: NativeHttpTransport()
        )
    }

    static func newWithSessionCryptoSigner(
        config: ChainConfig,
        signer: CryptoSigner,
        metadata: SessionMetadata
    ) throws -> Client {
        try self.newWithSessionCryptoSigner(
            config: config,
            signer: signer,
            metadata: metadata,
            transport: NativeHttpTransport()
        )
    }
}

public extension MobSessionManager {
    static func restore(data: Data, config: ChainConfig) throws -> MobSessionManager {
        try self.restore(data: data, config: config, transport: NativeHttpTransport())
    }

    func activate(metadata: SessionMetadata, config: ChainConfig) throws {
        try activate(metadata: metadata, config: config, transport: NativeHttpTransport())
    }
}
