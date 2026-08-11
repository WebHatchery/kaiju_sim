//! Signature creation for transfers and ownership proofs.

use chrono::{DateTime, Utc};
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Transfer signature payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSignature {
    pub kaiju_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub nonce: i64,
    pub transfer_type: String,
    pub signature: String,
}

/// Ownership proof payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipProofPayload {
    pub kaiju_id: Uuid,
    pub owner_user_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub state_hash: String,
    pub signature: String,
    pub public_key: String,
}

/// Signer for creating cryptographic signatures
pub struct Signer {
    secret_key: SecretKey,
    public_key: PublicKey,
    secp: Secp256k1<secp256k1::All>,
}

impl Signer {
    /// Create a new signer from a secret key
    pub fn new(secret_key: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Self {
            secret_key,
            public_key,
            secp,
        }
    }

    /// Get the public key as hex
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    /// Sign a transfer
    pub fn sign_transfer(
        &self,
        kaiju_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
        timestamp: DateTime<Utc>,
        nonce: i64,
        transfer_type: &str,
    ) -> TransferSignature {
        let message = format!(
            "transfer:{}:{}:{}:{}:{}:{}",
            kaiju_id,
            from_user_id,
            to_user_id,
            timestamp.timestamp(),
            nonce,
            transfer_type
        );

        let signature = self.sign_message(&message);

        TransferSignature {
            kaiju_id,
            from_user_id,
            to_user_id,
            timestamp,
            nonce,
            transfer_type: transfer_type.to_string(),
            signature,
        }
    }

    /// Sign ownership proof
    pub fn sign_ownership(
        &self,
        kaiju_id: Uuid,
        owner_user_id: Uuid,
        timestamp: DateTime<Utc>,
        state_hash: String,
    ) -> OwnershipProofPayload {
        let message = format!(
            "ownership:{}:{}:{}:{}",
            kaiju_id,
            owner_user_id,
            timestamp.timestamp(),
            state_hash
        );

        let signature = self.sign_message(&message);

        OwnershipProofPayload {
            kaiju_id,
            owner_user_id,
            timestamp,
            state_hash,
            signature,
            public_key: self.public_key_hex(),
        }
    }

    /// Low-level message signing
    fn sign_message(&self, message: &str) -> String {
        let message_hash = Sha256::digest(message.as_bytes());
        let msg = Message::from_digest_slice(&message_hash).expect("hash is always 32 bytes");

        let signature: Signature = self.secp.sign_ecdsa(&msg, &self.secret_key);
        hex::encode(signature.serialize_compact())
    }
}

#[cfg(test)]
mod tests;
