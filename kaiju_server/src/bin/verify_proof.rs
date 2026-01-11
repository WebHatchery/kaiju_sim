//! Verify ownership proofs offline.
//!
//! Usage: cargo run --bin verify-proof -- --proof-file proof.json

use clap::Parser;
use serde_json::from_str;

use kaiju_server::crypto::{signer::OwnershipProofPayload, Verifier};

#[derive(Parser)]
#[command(name = "kaiju-verify")]
#[command(about = "Verify Kaiju ownership proofs offline")]
struct Args {
    /// Path to proof JSON file
    #[arg(short, long)]
    proof_file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Read proof file
    let proof_json = std::fs::read_to_string(&args.proof_file)?;
    let proof: OwnershipProofPayload = from_str(&proof_json)?;

    // Verify
    let verifier = Verifier::new();
    let is_valid = verifier.verify_ownership(&proof);

    if is_valid {
        println!("✓ Ownership proof is VALID");
        println!("  Kaiju ID:   {}", proof.kaiju_id);
        println!("  Owner:      {}", proof.owner_user_id);
        println!("  Timestamp:  {}", proof.timestamp);
        println!("  State Hash: {}", proof.state_hash);
    } else {
        println!("✗ Ownership proof is INVALID");
        std::process::exit(1);
    }

    Ok(())
}
