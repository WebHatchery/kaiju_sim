//! Signature verification for transfers and ownership proofs.

use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};
use sha2::{Digest, Sha256};

use super::signer::{OwnershipProofPayload, TransferSignature};

/// Verifier for validating cryptographic signatures
pub struct Verifier {
    secp: Secp256k1<secp256k1::All>,
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Verifier {
    /// Create a new verifier
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }

    /// Verify transfer signature
    pub fn verify_transfer(&self, transfer: &TransferSignature, public_key: &PublicKey) -> bool {
        let message = format!(
            "transfer:{}:{}:{}:{}:{}:{}",
            transfer.kaiju_id,
            transfer.from_user_id,
            transfer.to_user_id,
            transfer.timestamp.timestamp(),
            transfer.nonce,
            transfer.transfer_type
        );

        self.verify_message(&message, &transfer.signature, public_key)
    }

    /// Verify ownership proof
    pub fn verify_ownership(&self, proof: &OwnershipProofPayload) -> bool {
        let message = format!(
            "ownership:{}:{}:{}:{}",
            proof.kaiju_id,
            proof.owner_user_id,
            proof.timestamp.timestamp(),
            proof.state_hash
        );

        // Decode public key from hex
        let Ok(pubkey_bytes) = hex::decode(&proof.public_key) else {
            return false;
        };

        let Ok(public_key) = PublicKey::from_slice(&pubkey_bytes) else {
            return false;
        };

        self.verify_message(&message, &proof.signature, &public_key)
    }

    /// Low-level signature verification
    fn verify_message(&self, message: &str, signature_hex: &str, public_key: &PublicKey) -> bool {
        // Decode signature
        let Ok(sig_bytes) = hex::decode(signature_hex) else {
            return false;
        };

        let Ok(signature) = Signature::from_compact(&sig_bytes) else {
            return false;
        };

        // Hash message
        let message_hash = Sha256::digest(message.as_bytes());
        let Ok(msg) = Message::from_digest_slice(&message_hash) else {
            return false;
        };

        // Verify
        self.secp.verify_ecdsa(&msg, &signature, public_key).is_ok()
    }
}

#[cfg(test)]
mod tests {
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
}
