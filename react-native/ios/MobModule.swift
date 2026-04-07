import ExpoModulesCore

public class MobModule: Module {
  // In-memory registries keyed by string identifiers
  private var signers: [String: RustSigner] = [:]
  private var keychainSigners: [String: KeychainSigner] = [:]
  private var clients: [String: Client] = [:]
  private var sessionManagers: [String: MobSessionManager] = [:]
  private var clientCounter: Int = 0
  private var sessionCounter: Int = 0
  private let transport: HttpTransport = NativeHttpTransport()

  public func definition() -> ModuleDefinition {
    Name("Mob")

    // MARK: - Signer Management

    AsyncFunction("createSignerFromMnemonic") { (mnemonic: String, prefix: String, derivationPath: String?) -> [String: String] in
      let signer = try RustSigner.fromMnemonic(
        mnemonic: mnemonic,
        addressPrefix: prefix,
        derivationPath: derivationPath
      )
      let address = signer.address()
      let pubKeyHex = signer.publicKeyHex()
      self.signers[address] = signer
      return ["address": address, "publicKeyHex": pubKeyHex]
    }

    AsyncFunction("signBytes") { (signerAddress: String, message: [UInt8]) -> [UInt8] in
      guard let signer = self.signers[signerAddress] else {
        throw MobModuleError.signerNotFound(signerAddress)
      }
      let result = try signer.signBytes(message: Data(message))
      return [UInt8](result)
    }

    AsyncFunction("destroySigner") { (address: String) in
      self.signers.removeValue(forKey: address)
    }

    // MARK: - Keychain Signer (iOS Secure Key Storage)

    AsyncFunction("createKeychainSigner") { (mnemonic: String, prefix: String, derivationPath: String?, identifier: String, requireBiometrics: Bool?) -> [String: String] in
      let signer = try KeychainSigner.fromMnemonic(
        mnemonic: mnemonic,
        prefix: prefix,
        derivationPath: derivationPath,
        identifier: identifier,
        requireBiometrics: requireBiometrics ?? false
      )
      let address = signer.address()
      self.keychainSigners[address] = signer
      return [
        "address": address,
        "publicKeyHex": signer.publicKey().map { String(format: "%02x", $0) }.joined(),
      ]
    }

    AsyncFunction("loadKeychainSigner") { (identifier: String, prefix: String, derivationPath: String?) -> [String: String] in
      let signer = try KeychainSigner.load(
        identifier: identifier,
        prefix: prefix,
        derivationPath: derivationPath
      )
      let address = signer.address()
      self.keychainSigners[address] = signer
      return [
        "address": address,
        "publicKeyHex": signer.publicKey().map { String(format: "%02x", $0) }.joined(),
      ]
    }

    AsyncFunction("deleteKeychainSigner") { (identifier: String) in
      // Remove from in-memory registry if loaded
      self.keychainSigners = self.keychainSigners.filter { _, signer in
        return KeychainSigner.exists(identifier: identifier)
      }
      try KeychainSigner.delete(identifier: identifier)
    }

    AsyncFunction("listKeychainSigners") { () -> [String] in
      return try KeychainSigner.listIdentifiers()
    }

    AsyncFunction("keychainSignerExists") { (identifier: String) -> Bool in
      return KeychainSigner.exists(identifier: identifier)
    }

    AsyncFunction("keychainSignBytes") { (signerAddress: String, message: [UInt8]) -> [UInt8] in
      guard let signer = self.keychainSigners[signerAddress] else {
        throw MobModuleError.signerNotFound(signerAddress)
      }
      let result = try signer.signBytes(message: Data(message))
      return [UInt8](result)
    }

    // MARK: - Client Management

    AsyncFunction("createClient") { (config: [String: Any]) -> String in
      let chainConfig = try self.parseChainConfig(config)
      let client = try Client(config: chainConfig, transport: self.transport)
      let clientId = self.nextClientId()
      self.clients[clientId] = client
      return clientId
    }

    AsyncFunction("createClientWithSigner") { (config: [String: Any], signerAddress: String) -> String in
      guard let signer = self.signers[signerAddress] else {
        throw MobModuleError.signerNotFound(signerAddress)
      }
      let chainConfig = try self.parseChainConfig(config)
      let client = try Client.newWithSigner(config: chainConfig, signer: signer, transport: self.transport)
      let clientId = self.nextClientId()
      self.clients[clientId] = client
      return clientId
    }

    AsyncFunction("createClientWithKeychainSigner") { (config: [String: Any], signerAddress: String) -> String in
      guard let signer = self.keychainSigners[signerAddress] else {
        throw MobModuleError.signerNotFound(signerAddress)
      }
      let chainConfig = try self.parseChainConfig(config)
      let client = try Client.newWithCryptoSigner(config: chainConfig, signer: signer, transport: self.transport)
      let clientId = self.nextClientId()
      self.clients[clientId] = client
      return clientId
    }

    AsyncFunction("destroyClient") { (clientId: String) in
      self.clients.removeValue(forKey: clientId)
    }

    // MARK: - Read Queries

    AsyncFunction("getAccount") { (clientId: String, address: String) -> [String: Any] in
      let client = try self.requireClient(clientId)
      let info = try client.getAccount(address: address)
      return [
        "address": info.address,
        "accountNumber": info.accountNumber,
        "sequence": info.sequence,
        "pubKey": info.pubKey as Any,
      ]
    }

    AsyncFunction("getBalance") { (clientId: String, address: String, denom: String) -> [String: String] in
      let client = try self.requireClient(clientId)
      let coin = try client.getBalance(address: address, denom: denom)
      return ["denom": coin.denom, "amount": coin.amount]
    }

    AsyncFunction("getAllBalances") { (clientId: String, address: String) -> [[String: String]] in
      let client = try self.requireClient(clientId)
      let coins = try client.getAllBalances(address: address)
      return coins.map { ["denom": $0.denom, "amount": $0.amount] }
    }

    AsyncFunction("getHeight") { (clientId: String) -> UInt64 in
      let client = try self.requireClient(clientId)
      return try client.getHeight()
    }

    AsyncFunction("getTx") { (clientId: String, hash: String) -> [String: Any] in
      let client = try self.requireClient(clientId)
      let tx = try client.getTx(hash: hash)
      return self.serializeTxResponse(tx)
    }

    AsyncFunction("hasGrants") { (clientId: String, granter: String, grantee: String) -> Bool in
      let client = try self.requireClient(clientId)
      return try client.hasGrants(granter: granter, grantee: grantee)
    }

    AsyncFunction("queryContractSmart") { (clientId: String, contractAddress: String, queryMsg: [UInt8]) -> [UInt8] in
      let client = try self.requireClient(clientId)
      let result = try client.queryContractSmart(contractAddress: contractAddress, queryMsg: Data(queryMsg))
      return [UInt8](result)
    }

    // MARK: - Transactions

    AsyncFunction("send") { (clientId: String, toAddress: String, amount: [[String: String]], memo: String?) -> [String: Any] in
      let client = try self.requireClient(clientId)
      let coins = try self.parseCoins(amount)
      let tx = try client.send(toAddress: toAddress, amount: coins, memo: memo)
      return self.serializeTxResponse(tx)
    }

    AsyncFunction("executeContract") { (clientId: String, contractAddress: String, msg: [UInt8], funds: [[String: String]], memo: String?, gasLimit: UInt64?) -> [String: Any] in
      let client = try self.requireClient(clientId)
      let fundCoins = try self.parseCoins(funds)
      let tx = try client.executeContract(
        contractAddress: contractAddress,
        msg: Data(msg),
        funds: fundCoins,
        memo: memo,
        gasLimit: gasLimit
      )
      return self.serializeTxResponse(tx)
    }

    AsyncFunction("signAndBroadcastMulti") { (clientId: String, messages: [[String: Any]], memo: String?, gasLimit: UInt64?) -> [String: Any] in
      let client = try self.requireClient(clientId)
      let msgs = try self.parseMessages(messages)
      let tx = try client.signAndBroadcastMulti(messages: msgs, memo: memo, gasLimit: gasLimit)
      return self.serializeTxResponse(tx)
    }

    // MARK: - Session Manager

    AsyncFunction("createSessionManager") { (addressPrefix: String) -> String in
      let mgr = MobSessionManager(addressPrefix: addressPrefix)
      let sessionId = self.nextSessionId()
      self.sessionManagers[sessionId] = mgr
      return sessionId
    }

    AsyncFunction("sessionGenerateKey") { (sessionId: String) -> [String: String] in
      let mgr = try self.requireSessionManager(sessionId)
      let info = try mgr.generateSessionKey()
      return ["address": info.address, "publicKeyHex": info.publicKeyHex]
    }

    AsyncFunction("sessionActivate") { (sessionId: String, granter: String, grantee: String, createdAt: UInt64, expiresAt: UInt64, description: String?, config: [String: Any]) in
      let mgr = try self.requireSessionManager(sessionId)
      let metadata = SessionMetadata(
        granter: granter,
        grantee: grantee,
        createdAt: createdAt,
        expiresAt: expiresAt,
        description: description
      )
      let chainConfig = try self.parseChainConfig(config)
      try mgr.activate(metadata: metadata, config: chainConfig, transport: self.transport)
    }

    AsyncFunction("sessionExport") { (sessionId: String) -> [UInt8] in
      let mgr = try self.requireSessionManager(sessionId)
      let result = try mgr.exportSession()
      return [UInt8](result)
    }

    AsyncFunction("sessionRestore") { (data: [UInt8], config: [String: Any]) -> String in
      let chainConfig = try self.parseChainConfig(config)
      let mgr = try MobSessionManager.restore(data: Data(data), config: chainConfig, transport: self.transport)
      let sessionId = self.nextSessionId()
      self.sessionManagers[sessionId] = mgr
      return sessionId
    }

    AsyncFunction("sessionDeactivate") { (sessionId: String) in
      if let mgr = self.sessionManagers[sessionId] {
        mgr.deactivate()
      }
      self.sessionManagers.removeValue(forKey: sessionId)
    }

    AsyncFunction("sessionIsActive") { (sessionId: String) -> Bool in
      guard let mgr = self.sessionManagers[sessionId] else { return false }
      return mgr.isActive()
    }

    AsyncFunction("sessionGranterAddress") { (sessionId: String) -> String? in
      return self.sessionManagers[sessionId]?.granterAddress()
    }

    AsyncFunction("sessionGranteeAddress") { (sessionId: String) -> String? in
      return self.sessionManagers[sessionId]?.granteeAddress()
    }

    AsyncFunction("sessionSignBytes") { (sessionId: String, message: [UInt8]) -> [UInt8] in
      let mgr = try self.requireSessionManager(sessionId)
      let result = try mgr.signBytes(message: Data(message))
      return [UInt8](result)
    }

    // Session-scoped transactions
    AsyncFunction("sessionSend") { (sessionId: String, toAddress: String, amount: [[String: String]], memo: String?) -> [String: Any] in
      let mgr = try self.requireSessionManager(sessionId)
      let client = try mgr.client()
      let coins = try self.parseCoins(amount)
      let tx = try client.send(toAddress: toAddress, amount: coins, memo: memo)
      return self.serializeTxResponse(tx)
    }

    AsyncFunction("sessionExecuteContract") { (sessionId: String, contractAddress: String, msg: [UInt8], funds: [[String: String]], memo: String?, gasLimit: UInt64?) -> [String: Any] in
      let mgr = try self.requireSessionManager(sessionId)
      let client = try mgr.client()
      let fundCoins = try self.parseCoins(funds)
      let tx = try client.executeContract(
        contractAddress: contractAddress,
        msg: Data(msg),
        funds: fundCoins,
        memo: memo,
        gasLimit: gasLimit
      )
      return self.serializeTxResponse(tx)
    }

    AsyncFunction("sessionSignAndBroadcastMulti") { (sessionId: String, messages: [[String: Any]], memo: String?, gasLimit: UInt64?) -> [String: Any] in
      let mgr = try self.requireSessionManager(sessionId)
      let client = try mgr.client()
      let msgs = try self.parseMessages(messages)
      let tx = try client.signAndBroadcastMulti(messages: msgs, memo: memo, gasLimit: gasLimit)
      return self.serializeTxResponse(tx)
    }

    AsyncFunction("sessionQueryContractSmart") { (sessionId: String, contractAddress: String, queryMsg: [UInt8]) -> [UInt8] in
      let mgr = try self.requireSessionManager(sessionId)
      let client = try mgr.client()
      let result = try client.queryContractSmart(contractAddress: contractAddress, queryMsg: Data(queryMsg))
      return [UInt8](result)
    }
  }

  // MARK: - Helpers

  private func nextClientId() -> String {
    clientCounter += 1
    return "client_\(clientCounter)"
  }

  private func nextSessionId() -> String {
    sessionCounter += 1
    return "session_\(sessionCounter)"
  }

  private func requireClient(_ clientId: String) throws -> Client {
    guard let client = clients[clientId] else {
      throw MobModuleError.clientNotFound(clientId)
    }
    return client
  }

  private func requireSessionManager(_ sessionId: String) throws -> MobSessionManager {
    guard let mgr = sessionManagers[sessionId] else {
      throw MobModuleError.sessionNotFound(sessionId)
    }
    return mgr
  }

  private func parseChainConfig(_ dict: [String: Any]) throws -> ChainConfig {
    guard let chainId = dict["chainId"] as? String,
          let rpcEndpoint = dict["rpcEndpoint"] as? String,
          let addressPrefix = dict["addressPrefix"] as? String else {
      throw MobModuleError.invalidInput("Missing required ChainConfig fields: chainId, rpcEndpoint, addressPrefix")
    }
    let config = ChainConfig(
      chainId: chainId,
      rpcEndpoint: rpcEndpoint,
      grpcEndpoint: dict["grpcEndpoint"] as? String,
      addressPrefix: addressPrefix,
      coinType: (dict["coinType"] as? NSNumber)?.uint32Value ?? 118,
      gasPrice: dict["gasPrice"] as? String ?? "0.025",
      feeGranter: dict["feeGranter"] as? String
    )
    return config
  }

  private func parseCoins(_ dicts: [[String: String]]) throws -> [Coin] {
    return try dicts.map { dict in
      guard let denom = dict["denom"], let amount = dict["amount"] else {
        throw MobModuleError.invalidInput("Coin requires 'denom' and 'amount'")
      }
      return Coin(denom: denom, amount: amount)
    }
  }

  private func parseMessages(_ dicts: [[String: Any]]) throws -> [Message] {
    return try dicts.map { dict in
      guard let typeUrl = dict["typeUrl"] as? String,
            let value = dict["value"] as? [UInt8] else {
        throw MobModuleError.invalidInput("Message requires 'typeUrl' and 'value'")
      }
      return Message(typeUrl: typeUrl, value: Data(value))
    }
  }

  private func serializeTxResponse(_ tx: TxResponse) -> [String: Any] {
    return [
      "txhash": tx.txhash,
      "code": tx.code,
      "rawLog": tx.rawLog,
      "gasWanted": tx.gasWanted,
      "gasUsed": tx.gasUsed,
      "height": tx.height,
    ]
  }
}

// MARK: - Errors

enum MobModuleError: Error, LocalizedError {
  case signerNotFound(String)
  case clientNotFound(String)
  case sessionNotFound(String)
  case invalidInput(String)

  var errorDescription: String? {
    switch self {
    case .signerNotFound(let address):
      return "Signer not found for address: \(address)"
    case .clientNotFound(let id):
      return "Client not found: \(id)"
    case .sessionNotFound(let id):
      return "Session manager not found: \(id)"
    case .invalidInput(let msg):
      return "Invalid input: \(msg)"
    }
  }
}
