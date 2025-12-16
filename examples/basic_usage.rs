use mob::{ChainConfig, Client, Coin, Signer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mob Library - Basic Usage Example\n");

    // 1. Create chain configuration
    println!("1. Creating chain configuration...");
    let config = ChainConfig::new(
        "xion-testnet-1",
        "https://rpc.xion-testnet-1.burnt.com:443",
        "xion",
    )
    .with_gas_price("0.025".to_string())
    .with_coin_type(118);

    println!(
        "   Chain ID: {}\n   RPC: {}\n   Prefix: {}\n",
        config.chain_id, config.rpc_endpoint, config.address_prefix
    );

    // 2. Create a signer from mnemonic
    println!("2. Creating signer from mnemonic...");
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let signer = Signer::from_mnemonic(mnemonic, "xion", None)?;

    println!("   Address: {}", signer.address());
    println!("   Public Key: {}\n", signer.public_key_hex());

    // 3. Create RPC client
    println!("3. Creating RPC client...");
    let mut client = Client::new(config).await?;
    println!("   Client created successfully\n");

    // 4. Attach signer to client
    println!("4. Attaching signer to client...");
    client.attach_signer(signer).await?;
    println!("   Signer attached successfully\n");

    // 5. Query account information
    println!("5. Querying account information...");
    if let Some(account) = client.account() {
        println!("   Address: {}", account.address);
        match account.account_number() {
            Ok(num) => println!("   Account Number: {}", num),
            Err(_) => println!("   Account Number: Not yet fetched"),
        }
        match account.sequence() {
            Ok(seq) => println!("   Sequence: {}", seq),
            Err(_) => println!("   Sequence: Not yet fetched"),
        }
    }
    println!();

    // 6. Get chain status
    println!("6. Querying chain status...");
    match client.get_height().await {
        Ok(height) => println!("   Latest block height: {}", height),
        Err(e) => println!("   Error getting height: {}", e),
    }

    match client.is_synced().await {
        Ok(synced) => println!("   Node synced: {}", synced),
        Err(e) => println!("   Error checking sync status: {}", e),
    }
    println!();

    println!("✅ Basic usage example completed successfully!");

    Ok(())
}
