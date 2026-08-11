use super::*;
use crate::crypto::keygen::{KeyPurpose, ServerKeyPair};
use crate::crypto::verifier::Verifier;

#[test]
fn test_transfer_signature() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);
    let signer = Signer::new(key_pair.secret_key);

    let sig = signer.sign_transfer(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        Utc::now(),
        0,
        "gift",
    );

    assert!(!sig.signature.is_empty());
    assert_eq!(sig.transfer_type, "gift");
}

#[test]
fn test_ownership_proof() {
    let key_pair = ServerKeyPair::generate(KeyPurpose::Ownership);
    let signer = Signer::new(key_pair.secret_key);

    let proof = signer.sign_ownership(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Utc::now(),
        "test_state_hash".to_string(),
    );

    assert!(!proof.signature.is_empty());
    assert!(!proof.public_key.is_empty());
}
