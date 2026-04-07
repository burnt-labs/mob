import Foundation
import Security

/// CryptoSigner implementation backed by the iOS Keychain.
///
/// Stores the private key securely in the iOS Keychain and delegates
/// secp256k1 signing to RustSigner. Apple's CryptoKit does not support
/// secp256k1 (only P-256/P-384/P-521), so the Rust crypto stack handles
/// the actual ECDSA operations while iOS provides secure key storage.
///
/// Features:
/// - Persistent key storage across app launches via iOS Keychain
/// - Optional biometric (Face ID / Touch ID) protection for key access
/// - Automatic key lifecycle management (create, load, delete)
final class KeychainSigner: CryptoSigner {
  private let rustSigner: RustSigner
  private let keychainId: String

  /// The bech32 address derived from this signer's public key.
  let cachedAddress: String

  /// The compressed secp256k1 public key (33 bytes).
  let cachedPublicKey: Data

  /// The bech32 prefix (e.g., "xion").
  let cachedAddressPrefix: String

  private init(rustSigner: RustSigner, keychainId: String) {
    self.rustSigner = rustSigner
    self.keychainId = keychainId
    self.cachedAddress = rustSigner.address()
    self.cachedPublicKey = KeychainSigner.dataFromHex(rustSigner.publicKeyHex())
    self.cachedAddressPrefix = rustSigner.addressPrefix()
  }

  /// Convert a hex string to Data.
  private static func dataFromHex(_ hex: String) -> Data {
    var data = Data()
    var hex = hex
    if hex.hasPrefix("0x") { hex = String(hex.dropFirst(2)) }
    var index = hex.startIndex
    while index < hex.endIndex {
      let nextIndex = hex.index(index, offsetBy: 2, limitedBy: hex.endIndex) ?? hex.endIndex
      if let byte = UInt8(hex[index..<nextIndex], radix: 16) {
        data.append(byte)
      }
      index = nextIndex
    }
    return data
  }

  // MARK: - Factory Methods

  /// Create a new KeychainSigner from a mnemonic phrase.
  ///
  /// The mnemonic is used to derive a private key via BIP32/BIP39,
  /// which is then stored in the iOS Keychain under the given identifier.
  ///
  /// - Parameters:
  ///   - mnemonic: BIP39 mnemonic phrase
  ///   - prefix: Bech32 address prefix (e.g., "xion")
  ///   - derivationPath: Optional HD derivation path (default: m/44'/118'/0'/0/0)
  ///   - identifier: Keychain item identifier for this key
  ///   - requireBiometrics: Whether to require Face ID / Touch ID for key access
  static func fromMnemonic(
    mnemonic: String,
    prefix: String,
    derivationPath: String?,
    identifier: String,
    requireBiometrics: Bool = false
  ) throws -> KeychainSigner {
    let signer = try RustSigner.fromMnemonic(
      mnemonic: mnemonic,
      addressPrefix: prefix,
      derivationPath: derivationPath
    )

    // Store the mnemonic in the Keychain (the private key source)
    try KeychainSigner.storeInKeychain(
      data: Data(mnemonic.utf8),
      identifier: identifier,
      requireBiometrics: requireBiometrics
    )

    return KeychainSigner(rustSigner: signer, keychainId: identifier)
  }

  /// Load a previously stored KeychainSigner from the iOS Keychain.
  ///
  /// - Parameters:
  ///   - identifier: Keychain item identifier used when the key was created
  ///   - prefix: Bech32 address prefix (e.g., "xion")
  ///   - derivationPath: Optional HD derivation path (must match what was used at creation)
  static func load(
    identifier: String,
    prefix: String,
    derivationPath: String?
  ) throws -> KeychainSigner {
    let mnemonicData = try KeychainSigner.loadFromKeychain(identifier: identifier)

    guard let mnemonic = String(data: mnemonicData, encoding: .utf8) else {
      throw KeychainSignerError.corruptedData
    }

    let signer = try RustSigner.fromMnemonic(
      mnemonic: mnemonic,
      addressPrefix: prefix,
      derivationPath: derivationPath
    )

    return KeychainSigner(rustSigner: signer, keychainId: identifier)
  }

  /// Delete a stored key from the iOS Keychain.
  static func delete(identifier: String) throws {
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: KeychainSigner.serviceTag,
      kSecAttrAccount as String: identifier,
    ]

    let status = SecItemDelete(query as CFDictionary)
    if status != errSecSuccess && status != errSecItemNotFound {
      throw KeychainSignerError.keychainError(status)
    }
  }

  /// List all stored key identifiers.
  static func listIdentifiers() throws -> [String] {
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: KeychainSigner.serviceTag,
      kSecReturnAttributes as String: true,
      kSecMatchLimit as String: kSecMatchLimitAll,
    ]

    var result: AnyObject?
    let status = SecItemCopyMatching(query as CFDictionary, &result)

    if status == errSecItemNotFound {
      return []
    }

    guard status == errSecSuccess,
          let items = result as? [[String: Any]] else {
      throw KeychainSignerError.keychainError(status)
    }

    return items.compactMap { $0[kSecAttrAccount as String] as? String }
  }

  /// Check whether a key exists in the Keychain for the given identifier.
  static func exists(identifier: String) -> Bool {
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: KeychainSigner.serviceTag,
      kSecAttrAccount as String: identifier,
    ]

    return SecItemCopyMatching(query as CFDictionary, nil) == errSecSuccess
  }

  // MARK: - CryptoSigner Protocol

  func address() -> String {
    cachedAddress
  }

  func publicKey() -> Data {
    cachedPublicKey
  }

  func addressPrefix() -> String {
    cachedAddressPrefix
  }

  func signBytes(message: Data) throws -> Data {
    try rustSigner.signBytes(message: message)
  }

  // MARK: - Keychain Operations

  private static let serviceTag = "com.burnt.mob.signer"

  private static func storeInKeychain(
    data: Data,
    identifier: String,
    requireBiometrics: Bool
  ) throws {
    // Delete any existing item first
    try? KeychainSigner.delete(identifier: identifier)

    var query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: serviceTag,
      kSecAttrAccount as String: identifier,
      kSecValueData as String: data,
      kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
    ]

    if requireBiometrics {
      var error: Unmanaged<CFError>?
      guard let access = SecAccessControlCreateWithFlags(
        kCFAllocatorDefault,
        kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
        .biometryCurrentSet,
        &error
      ) else {
        throw KeychainSignerError.biometricSetupFailed
      }
      query[kSecAttrAccessControl as String] = access
      query.removeValue(forKey: kSecAttrAccessible as String)
    }

    let status = SecItemAdd(query as CFDictionary, nil)
    guard status == errSecSuccess else {
      throw KeychainSignerError.keychainError(status)
    }
  }

  private static func loadFromKeychain(identifier: String) throws -> Data {
    let query: [String: Any] = [
      kSecClass as String: kSecClassGenericPassword,
      kSecAttrService as String: serviceTag,
      kSecAttrAccount as String: identifier,
      kSecReturnData as String: true,
    ]

    var result: AnyObject?
    let status = SecItemCopyMatching(query as CFDictionary, &result)

    guard status == errSecSuccess, let data = result as? Data else {
      if status == errSecItemNotFound {
        throw KeychainSignerError.notFound(identifier)
      }
      throw KeychainSignerError.keychainError(status)
    }

    return data
  }
}

// MARK: - Errors

enum KeychainSignerError: Error, LocalizedError {
  case notFound(String)
  case corruptedData
  case keychainError(OSStatus)
  case biometricSetupFailed

  var errorDescription: String? {
    switch self {
    case .notFound(let id):
      return "No key found in Keychain for identifier: \(id)"
    case .corruptedData:
      return "Keychain data is corrupted"
    case .keychainError(let status):
      return "Keychain operation failed with status: \(status)"
    case .biometricSetupFailed:
      return "Failed to configure biometric access control"
    }
  }
}
