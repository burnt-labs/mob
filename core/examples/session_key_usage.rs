use mob::{RustSigner, SessionMetadata, SessionSigner};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mob Library - Session Key Usage Example\n");

    println!("=== Step 1: Create Main Account RustSigner ===");
    // In a real scenario, this would be your main account's mnemonic
    let main_mnemonic = "quiz cattle knock bacon million abstract word reunion educate antenna put fitness slide dash point basket jaguar fun humor multiply emotion rescue brand pull";
    let main_signer =
        RustSigner::from_mnemonic(main_mnemonic.to_string(), "xion".to_string(), None)?;

    println!("Main account address: {}", main_signer.address());
    println!("Main account pub key: {}\n", main_signer.public_key_hex());

    println!("=== Step 2: Create Session Key ===");
    // Generate a session key (in production, you'd generate this securely)
    // For this example, we'll use a different derivation path to create a different key
    let session_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    let session_key_signer = RustSigner::from_mnemonic(
        session_mnemonic.to_string(),
        "xion".to_string(),
        Some("m/44'/118'/0'/0/1".to_string()),
    )?;

    println!("Session key address: {}", session_key_signer.address());
    println!(
        "Session key pub key: {}\n",
        session_key_signer.public_key_hex()
    );

    println!("=== Step 3: Create Session Metadata ===");
    // Session expires in 1 hour (3600 seconds)
    let duration_seconds = 3600;
    let metadata = SessionMetadata::with_duration(
        main_signer.address(),
        session_key_signer.address(),
        duration_seconds,
    )
    .with_description("Example session for testing".to_string());

    println!("Granter (main account): {}", metadata.granter);
    println!("Grantee (session key): {}", metadata.grantee);
    println!("Created at: {} (Unix timestamp)", metadata.created_at);
    println!("Expires at: {} (Unix timestamp)", metadata.expires_at);
    println!("Duration: {} seconds (1 hour)", duration_seconds);
    println!("Remaining: {} seconds\n", metadata.remaining_seconds());

    println!("=== Step 4: Create Session Signer ===");
    let session_signer = SessionSigner::with_signer(Arc::new(session_key_signer), metadata)?;

    println!("Session signer created successfully!");
    println!("✓ Granter address: {}", session_signer.granter_address());
    println!("✓ Grantee address: {}", session_signer.grantee_address());
    println!("✓ Is expired: {}", session_signer.is_expired());
    println!(
        "✓ Remaining time: {} seconds\n",
        session_signer.remaining_seconds()
    );

    println!("=== Step 5: Alternative - Create from Private Key ===");
    // You can also create a session signer directly from a private key
    let private_key = vec![0x42; 32]; // Example private key (32 bytes)
    let session_from_key_signer = RustSigner::from_private_key(&private_key, "xion")?;
    let session_from_key_address = session_from_key_signer.address();
    let session_from_key = SessionSigner::with_signer(
        Arc::new(session_from_key_signer),
        SessionMetadata::with_duration(
            main_signer.address(),
            session_from_key_address,
            7200, // 2 hours
        ),
    )?;

    println!("Session signer from private key:");
    println!("✓ Granter: {}", session_from_key.granter_address());
    println!("✓ Grantee: {}", session_from_key.grantee_address());
    println!(
        "✓ Duration: {} seconds (2 hours)\n",
        session_from_key.remaining_seconds()
    );

    println!("=== Step 6: Using Session RustSigner with Client ===");
    println!("Note: In production, you would:");
    println!("1. First grant authorization from main account to session key");
    println!("   (using MsgGrant with SendAuthorization or GenericAuthorization)");
    println!("2. Then use the SessionRustSigner to sign transactions");
    println!("3. All messages will be automatically wrapped in MsgExec (authz)");
    println!();

    println!("Example transaction flow:");
    println!("  Main Account -> Grants authorization -> Session Key");
    println!("  Session Key -> Signs with MsgExec wrapper -> Executes as Main Account");
    println!();

    println!("=== Step 7: Session Expiration Handling ===");
    println!("Session metadata includes expiration checking:");
    println!("  - is_expired(): {}", session_signer.is_expired());
    println!(
        "  - remaining_seconds(): {}",
        session_signer.remaining_seconds()
    );
    println!();
    println!("If you try to sign with an expired session:");

    // Create an expired session
    let expired_metadata = SessionMetadata::new(
        main_signer.address(),
        session_signer.grantee_address(),
        0, // Already expired
    );

    let expired_key = RustSigner::from_mnemonic(
        session_mnemonic.to_string(),
        "xion".to_string(),
        Some("m/44'/118'/0'/0/2".to_string()),
    )?;

    let expired_result = SessionSigner::with_signer(Arc::new(expired_key), expired_metadata);

    match expired_result {
        Ok(_) => println!("  ✗ Should have failed with expired session"),
        Err(e) => println!("  ✓ Correctly rejected: {}", e),
    }
    println!();

    println!("=== Key Benefits of Session Keys ===");
    println!("1. Time-limited access - sessions automatically expire");
    println!("2. Lower security risk - compromised session key has limited lifetime");
    println!("3. Scoped permissions - session can be limited to specific operations");
    println!("4. No main key exposure - main account key stays secure");
    println!("5. Revocable - main account can revoke authorization anytime");
    println!();

    println!("✅ Session key usage example completed successfully!");

    Ok(())
}
