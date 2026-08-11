use super::*;
use crate::crypto::keygen::{KeyPurpose, ServerKeyPair};
use crate::crypto::signer::Signer;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_transfer_signature_verification() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);
    let signer = Signer::new(key_pair.secret_key);
    let verifier = Verifier::new();

    let transfer = signer.sign_transfer(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Utc::now(),
        0,
        "gift",
    );

    assert!(verifier.verify_transfer(&transfer, &key_pair.public_key));
}

#[test]
fn test_ownership_proof_verification() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);
    let signer = Signer::new(key_pair.secret_key);
    let verifier = Verifier::new();

    let proof = signer.sign_ownership(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Utc::now(),
        "test_state_hash_123".to_string(),
    );

    assert!(verifier.verify_ownership(&proof));
}

#[test]
fn test_tampered_signature_fails() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);
    let signer = Signer::new(key_pair.secret_key);
    let verifier = Verifier::new();

    let mut transfer = signer.sign_transfer(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Utc::now(),
        0,
        "gift",
    );

    // Tamper with nonce
    transfer.nonce = 999;

    assert!(!verifier.verify_transfer(&transfer, &key_pair.public_key));
}
