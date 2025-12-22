use mob::{ChainConfig, Coin, Fee, RustSigner};

#[test]
fn test_coin_creation() {
    let coin = Coin::new("uxion", "1000000");

    assert_eq!(coin.denom, "uxion");
    assert_eq!(coin.amount, "1000000");
}

#[test]
fn test_chain_config_creation() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    );

    assert_eq!(config.chain_id, "xion-testnet-2");
    assert_eq!(
        config.rpc_endpoint,
        "https://rpc.xion-testnet-2.burnt.com:443"
    );
    assert_eq!(config.address_prefix, "xion");
    assert_eq!(config.coin_type, 118); // Default Cosmos coin type
    assert_eq!(config.gas_price, "0.025"); // Default gas price
    assert!(config.grpc_endpoint.is_none());
}

#[test]
fn test_chain_config_with_customization() {
    let config = ChainConfig::new(
        "xion-testnet-2",
        "https://rpc.xion-testnet-2.burnt.com:443",
        "xion",
    )
    .with_gas_price("0.05".to_string())
    .with_coin_type(118)
    .with_grpc("https://grpc.xion-testnet-2.burnt.com:443".to_string());

    assert_eq!(config.gas_price, "0.05");
    assert_eq!(config.coin_type, 118);
    assert_eq!(
        config.grpc_endpoint,
        Some("https://grpc.xion-testnet-2.burnt.com:443".to_string())
    );
}

#[test]
fn test_fee_creation() {
    let coins = vec![Coin::new("uxion", "5000")];
    let fee = Fee::new(coins.clone(), 200_000);

    assert_eq!(fee.amount.len(), 1);
    assert_eq!(fee.amount[0].denom, "uxion");
    assert_eq!(fee.amount[0].amount, "5000");
    assert_eq!(fee.gas_limit, 200_000);
    assert!(fee.payer.is_none());
    assert!(fee.granter.is_none());
}

#[test]
fn test_fee_with_payer() {
    let coins = vec![Coin::new("uxion", "5000")];
    let fee = Fee::new(coins, 200_000).with_payer("xion1payer123".to_string());

    assert_eq!(fee.payer, Some("xion1payer123".to_string()));
    assert!(fee.granter.is_none());
}

#[test]
fn test_fee_with_granter() {
    let coins = vec![Coin::new("uxion", "5000")];
    let fee = Fee::new(coins, 200_000).with_granter("xion1granter456".to_string());

    assert!(fee.payer.is_none());
    assert_eq!(fee.granter, Some("xion1granter456".to_string()));
}

#[test]
fn test_signer_from_mnemonic() {
    // Standard test mnemonic (abandon x 11 + about)
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";

    let result = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None);

    assert!(result.is_ok(), "Signer creation should succeed");

    let signer = result.unwrap();
    let address = signer.address();

    // Address should start with the prefix
    assert!(
        address.starts_with("xion"),
        "Address should start with 'xion'"
    );

    // Address should have reasonable length (typically 43 chars for bech32)
    assert!(address.len() > 10, "Address should be reasonable length");
}

#[test]
fn test_signer_public_key() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";

    let signer = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

    let pub_key_hex = signer.public_key_hex();

    // Public key hex should be 66 characters (33 bytes * 2)
    assert_eq!(
        pub_key_hex.len(),
        66,
        "Public key hex should be 66 characters"
    );

    // Should be valid hex
    assert!(
        pub_key_hex.chars().all(|c| c.is_ascii_hexdigit()),
        "Should be valid hex"
    );
}

#[test]
fn test_signer_address_prefix() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";

    let signer = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

    assert_eq!(signer.address_prefix(), "xion");
}

#[test]
fn test_signer_different_prefixes() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";

    let signer_xion =
        RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

    let signer_cosmos =
        RustSigner::from_mnemonic(mnemonic.to_string(), "cosmos".to_string(), None).unwrap();

    // Same mnemonic should generate different addresses for different prefixes
    assert_ne!(signer_xion.address(), signer_cosmos.address());

    // But same public key
    assert_eq!(signer_xion.public_key_hex(), signer_cosmos.public_key_hex());
}

#[test]
fn test_signer_custom_derivation_path() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";

    let signer1 = RustSigner::from_mnemonic(
        mnemonic.to_string(),
        "xion".to_string(),
        Some("m/44'/118'/0'/0/0".to_string()),
    )
    .unwrap();

    let signer2 = RustSigner::from_mnemonic(
        mnemonic.to_string(),
        "xion".to_string(),
        Some("m/44'/118'/0'/0/1".to_string()),
    )
    .unwrap();

    // Different derivation paths should generate different addresses
    assert_ne!(signer1.address(), signer2.address());
    assert_ne!(signer1.public_key_hex(), signer2.public_key_hex());
}

#[test]
fn test_signer_invalid_mnemonic() {
    let result = RustSigner::from_mnemonic(
        "invalid mnemonic phrase".to_string(),
        "xion".to_string(),
        None,
    );

    assert!(result.is_err(), "Should fail with invalid mnemonic");
}

#[test]
fn test_signer_sign_bytes() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    let signer = RustSigner::from_mnemonic(mnemonic.to_string(), "xion".to_string(), None).unwrap();

    let message = b"Hello, XION!".to_vec();
    let result = signer.sign_bytes(message);

    assert!(result.is_ok(), "Signing should succeed");

    let signature = result.unwrap();

    // ECDSA signature should be 64 bytes
    assert_eq!(signature.len(), 64, "Signature should be 64 bytes");
}

#[test]
fn test_multiple_coins() {
    let coins = [Coin::new("uxion", "1000000"), Coin::new("uatom", "500000")];

    assert_eq!(coins.len(), 2);
    assert_eq!(coins[0].denom, "uxion");
    assert_eq!(coins[1].denom, "uatom");
}

#[test]
fn test_coin_clone() {
    let coin1 = Coin::new("uxion", "1000000");
    let coin2 = coin1.clone();

    assert_eq!(coin1.denom, coin2.denom);
    assert_eq!(coin1.amount, coin2.amount);
}

#[test]
fn test_calculate_fee() {
    use mob::transaction::calculate_fee;

    let result = calculate_fee(200_000, "0.025", "uxion");

    assert!(result.is_ok());

    let fee = result.unwrap();
    assert_eq!(fee.gas_limit, 200_000);
    assert_eq!(fee.amount.len(), 1);
    assert_eq!(fee.amount[0].denom, "uxion");
    // 200,000 * 0.025 = 5,000
    assert_eq!(fee.amount[0].amount, "5000");
}

#[test]
fn test_calculate_fee_with_fractional_amount() {
    use mob::transaction::calculate_fee;

    let result = calculate_fee(150_000, "0.025", "uxion");

    assert!(result.is_ok());

    let fee = result.unwrap();
    // 150,000 * 0.025 = 3,750
    assert_eq!(fee.amount[0].amount, "3750");
}
