//! Generate server keys for the Kaiju server.
//!
//! Usage: cargo run --bin generate-keys

use kaiju_server::crypto::keygen::{generate_all_keys, KeyPurpose};

fn main() {
    println!("=== Kaiju Server Key Generation ===\n");
    println!("Generating server key pairs...\n");

    let keys = generate_all_keys();

    for key in &keys {
        println!("--- {} Key ---", key.purpose);
        println!("Purpose:    {}", key.purpose);
        println!("Public Key: {}", key.public_key_hex());
        println!("Secret Key: {}", key.secret_key_hex());
        println!("ETH Address: {}", key.eth_address());
        println!();
    }

    println!("=== Environment Configuration ===\n");
    println!("Add the following to your .env file:\n");

    // Find ownership key for primary use
    let ownership_key = keys
        .iter()
        .find(|k| k.purpose == KeyPurpose::Ownership)
        .unwrap();
    println!("SERVER_SECRET_KEY={}", ownership_key.secret_key_hex());

    println!("\n=== IMPORTANT ===");
    println!("1. Store these keys securely (e.g., password manager, vault)");
    println!("2. Never commit the secret keys to version control");
    println!("3. The public keys can be shared publicly for verification");
    println!("4. Consider rotating keys periodically for security");
}
