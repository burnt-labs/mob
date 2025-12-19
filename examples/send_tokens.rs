use mob::{ChainConfig, Client, Coin, RustSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mob Library - Send Tokens Example\n");

    // Setup
    let config = ChainConfig::new(
        "xion-testnet-1",
        "https://rpc.xion-testnet-1.burnt.com:443",
        "xion",
    );

    let mnemonic = std::env::var("MNEMONIC")
        .expect("MNEMONIC environment variable not set");

    let signer = RustSigner::from_mnemonic(&mnemonic, "xion", None)?;
    println!("Sender address: {}\n", signer.address());

    // Create and configure client
    let mut client = Client::new(config).await?;
    client.attach_signer(signer).await?;

    // Recipient address (replace with actual address)
    let recipient = std::env::var("RECIPIENT")
        .unwrap_or_else(|_| "xion1recipient...".to_string());

    // Amount to send
    let amount = vec![Coin::new("uxion", "1000000")]; // 1 XION

    println!("Preparing to send:");
    println!("  To: {}", recipient);
    println!("  Amount: {} {}\n", amount[0].amount, amount[0].denom);

    // Send the transaction
    println!("Broadcasting transaction...");
    match client
        .send(&recipient, amount, Some("Test transfer".to_string()))
        .await
    {
        Ok(response) => {
            println!("\n✅ Transaction successful!");
            println!("  Tx Hash: {}", response.txhash);
            println!("  Code: {}", response.code);
            println!("  Height: {}", response.height);
            println!("  Gas Used: {}", response.gas_used);
            println!("  Gas Wanted: {}", response.gas_wanted);

            if response.code != 0 {
                println!("  Error Log: {}", response.raw_log);
            }
        }
        Err(e) => {
            println!("\n❌ Transaction failed: {}", e);
        }
    }

    Ok(())
}
