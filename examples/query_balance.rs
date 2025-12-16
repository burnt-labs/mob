use mob::{ChainConfig, Client, Signer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mob Library - Query Balance Example\n");

    // Setup
    let config = ChainConfig::new(
        "xion-testnet-1",
        "https://rpc.xion-testnet-1.burnt.com:443",
        "xion",
    );

    // Create client
    let client = Client::new(config).await?;
    println!("Connected to chain: {}\n", client.config().chain_id);

    // Address to query (can be from environment or use a signer)
    let address = if let Ok(mnemonic) = std::env::var("MNEMONIC") {
        let signer = Signer::from_mnemonic(&mnemonic, "xion", None)?;
        signer.address()
    } else {
        std::env::var("ADDRESS")
            .unwrap_or_else(|_| "xion1address...".to_string())
    };

    println!("Querying balances for: {}\n", address);

    // Query specific balance
    println!("1. Querying uxion balance...");
    match client.get_balance(&address, "uxion").await {
        Ok(balance) => {
            println!(
                "   Balance: {} {}",
                balance.amount, balance.denom
            );
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    // Query all balances
    println!("\n2. Querying all balances...");
    match client.get_all_balances(&address).await {
        Ok(balances) => {
            if balances.is_empty() {
                println!("   No balances found");
            } else {
                for balance in balances {
                    println!(
                        "   {} {}",
                        balance.amount, balance.denom
                    );
                }
            }
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    // Query account info
    println!("\n3. Querying account information...");
    match client.get_account(&address).await {
        Ok(account_info) => {
            println!("   Address: {}", account_info.address);
            println!("   Account Number: {}", account_info.account_number);
            println!("   Sequence: {}", account_info.sequence);
            if let Some(pub_key) = account_info.pub_key {
                println!("   Public Key: {}", pub_key);
            }
        }
        Err(e) => {
            println!("   Error: {}", e);
        }
    }

    println!("\n✅ Query completed!");

    Ok(())
}
