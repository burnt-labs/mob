use mob::{ChainConfig, Client, HttpTransport, RustSigner, UreqTransport};
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Mob Library - Execute Contract Example\n");

    // Setup
    let config = ChainConfig::new(
        "xion-testnet-1".to_string(),
        "https://rpc.xion-testnet-1.burnt.com:443".to_string(),
        "xion".to_string(),
    );

    let transport: Arc<dyn HttpTransport> = Arc::new(UreqTransport::new());

    let mnemonic = std::env::var("MNEMONIC").expect("MNEMONIC environment variable not set");

    let signer = RustSigner::from_mnemonic(mnemonic, "xion".to_string(), None)?;
    println!("Sender address: {}\n", signer.address());

    // Create and configure client
    let mut client = Client::new_with_transport(config, transport);
    client.attach_crypto_signer(Arc::new(signer)).await?;

    // Contract address (replace with actual contract address)
    let contract_address =
        std::env::var("CONTRACT_ADDRESS").unwrap_or_else(|_| "xion1contract...".to_string());

    // Example contract message (increment counter)
    let contract_msg = json!({
        "increment": {}
    });

    let msg_bytes = serde_json::to_vec(&contract_msg)?;

    // Funds to send with the message (if any)
    let funds = vec![]; // Empty for this example

    println!("Executing contract:");
    println!("  Contract: {}", contract_address);
    println!("  Message: {}\n", contract_msg);

    // Execute the contract
    println!("Broadcasting transaction...");
    match client.execute_contract(
        contract_address,
        msg_bytes,
        funds,
        None, // granter
        None, // fee_granter
        Some("Execute contract".to_string()),
        None, // gas_limit
    ) {
        Ok(response) => {
            println!("\n✅ Contract execution successful!");
            println!("  Tx Hash: {}", response.txhash);
            println!("  Code: {}", response.code);
            println!("  Height: {}", response.height);
            println!("  Gas Used: {}", response.gas_used);
            println!("  Gas Wanted: {}", response.gas_wanted);

            if response.code == 0 {
                println!("  Log: {}", response.raw_log);
            } else {
                println!("  Error Log: {}", response.raw_log);
            }
        }
        Err(e) => {
            println!("\n❌ Contract execution failed: {}", e);
        }
    }

    Ok(())
}
