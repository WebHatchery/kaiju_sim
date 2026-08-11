//! Key generation for server signature keys.

use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

/// Server signature key pair
pub struct ServerKeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub purpose: KeyPurpose,
}

/// Purpose of a key pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPurpose {
    Ownership,
    Battle,
    Mint,
    Admin,
}

impl std::fmt::Display for KeyPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ownership => write!(f, "ownership"),
            Self::Battle => write!(f, "battle"),
            Self::Mint => write!(f, "mint"),
            Self::Admin => write!(f, "admin"),
        }
    }
}

impl ServerKeyPair {
    /// Generate new random key pair
    pub fn generate(purpose: KeyPurpose) -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        Self {
            secret_key,
            public_key,
            purpose,
        }
    }

    /// Load from hex-encoded secret
    pub fn from_hex(hex_secret: &str, purpose: KeyPurpose) -> Result<Self, secp256k1::Error> {
        let bytes = hex::decode(hex_secret).map_err(|_| secp256k1::Error::InvalidSecretKey)?;

        let secret_key = SecretKey::from_slice(&bytes)?;
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        Ok(Self {
            secret_key,
            public_key,
            purpose,
        })
    }

    /// Export public key as hex
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key.serialize())
    }

    /// Export secret key as hex (DANGEROUS - only for backup)
    pub fn secret_key_hex(&self) -> String {
        hex::encode(self.secret_key.secret_bytes())
    }

    /// Derive Ethereum-style address from public key
    pub fn eth_address(&self) -> String {
        let pubkey_bytes = self.public_key.serialize_uncompressed();
        let hash = Sha256::digest(&pubkey_bytes[1..]); // Skip first byte (0x04)
        format!("0x{}", hex::encode(&hash[12..])) // Last 20 bytes
    }
}

/// Generate all server keys on first setup
pub fn generate_all_keys() -> Vec<ServerKeyPair> {
    vec![
        ServerKeyPair::generate(KeyPurpose::Ownership),
        ServerKeyPair::generate(KeyPurpose::Battle),
        ServerKeyPair::generate(KeyPurpose::Mint),
        ServerKeyPair::generate(KeyPurpose::Admin),
    ]
}

#[cfg(test)]
mod tests;
