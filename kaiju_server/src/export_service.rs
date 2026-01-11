//! Export service for kaiju data with cryptographic signatures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Kaiju export data with cryptographic proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaijuExportData {
    pub kaiju_id: Uuid,
    pub name: String,
    pub generation: u32,
    pub genome_hash: String,
    pub visual_seed: u64,
    pub stats: ExportedStats,
    pub visible_traits: Vec<String>,
    pub experience_level: u32,
    pub created_at: DateTime<Utc>,
    pub parent_ids: Option<(Uuid, Uuid)>,
    pub owner_user_id: Uuid,
    pub alive: bool,
    // Cryptographic proof
    pub state_hash: String,
    pub export_timestamp: DateTime<Utc>,
    pub server_signature: String,
}

/// Stats for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

/// Export format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Binary,
}

/// Export package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaijuExportPackage {
    pub format_version: u32,
    pub data: KaijuExportData,
    pub checksum: String,
}

/// Export service
pub struct ExportService {
    server_key: String, // In production, use proper key management
}

impl Default for ExportService {
    fn default() -> Self {
        Self::new("default_server_key")
    }
}

impl ExportService {
    pub fn new(server_key: &str) -> Self {
        Self {
            server_key: server_key.to_string(),
        }
    }

    /// Create export package for a kaiju
    pub fn create_export(
        &self,
        kaiju_id: Uuid,
        name: String,
        generation: u32,
        genome_hash: String,
        visual_seed: u64,
        stats: ExportedStats,
        visible_traits: Vec<String>,
        experience_level: u32,
        created_at: DateTime<Utc>,
        parent_ids: Option<(Uuid, Uuid)>,
        owner_user_id: Uuid,
        alive: bool,
    ) -> KaijuExportPackage {
        let export_timestamp = Utc::now();
        
        // Create state hash
        let state_hash = self.compute_state_hash(
            kaiju_id,
            &genome_hash,
            alive,
            &stats,
        );

        // Sign the export
        let server_signature = self.sign_export(&state_hash, export_timestamp);

        let data = KaijuExportData {
            kaiju_id,
            name,
            generation,
            genome_hash,
            visual_seed,
            stats,
            visible_traits,
            experience_level,
            created_at,
            parent_ids,
            owner_user_id,
            alive,
            state_hash: state_hash.clone(),
            export_timestamp,
            server_signature,
        };

        let checksum = self.compute_checksum(&data);

        KaijuExportPackage {
            format_version: 1,
            data,
            checksum,
        }
    }

    /// Verify an export package
    pub fn verify_export(&self, package: &KaijuExportPackage) -> VerificationResult {
        // Check format version
        if package.format_version != 1 {
            return VerificationResult::UnsupportedVersion;
        }

        // Verify checksum
        let computed_checksum = self.compute_checksum(&package.data);
        if computed_checksum != package.checksum {
            return VerificationResult::TamperedData;
        }

        // Verify signature
        let expected_sig = self.sign_export(
            &package.data.state_hash,
            package.data.export_timestamp,
        );
        if expected_sig != package.data.server_signature {
            return VerificationResult::InvalidSignature;
        }

        // Verify state hash
        let computed_state_hash = self.compute_state_hash(
            package.data.kaiju_id,
            &package.data.genome_hash,
            package.data.alive,
            &package.data.stats,
        );
        if computed_state_hash != package.data.state_hash {
            return VerificationResult::GenomeMismatch;
        }

        VerificationResult::Valid {
            kaiju_id: package.data.kaiju_id,
            export_timestamp: package.data.export_timestamp,
        }
    }

    /// Export to JSON string
    pub fn to_json(&self, package: &KaijuExportPackage) -> Result<String, ExportError> {
        serde_json::to_string_pretty(package).map_err(|e| ExportError::SerializationFailed(e.to_string()))
    }

    /// Import from JSON string
    pub fn from_json(&self, json: &str) -> Result<KaijuExportPackage, ExportError> {
        serde_json::from_str(json).map_err(|e| ExportError::DeserializationFailed(e.to_string()))
    }

    // Helper functions
    fn compute_state_hash(
        &self,
        kaiju_id: Uuid,
        genome_hash: &str,
        alive: bool,
        stats: &ExportedStats,
    ) -> String {
        // In production, use proper cryptographic hashing
        format!(
            "state:{}:{}:{}:{}:{}:{}:{}",
            kaiju_id, genome_hash, alive, stats.hp, stats.attack, stats.defense, stats.speed
        )
    }

    fn sign_export(&self, state_hash: &str, timestamp: DateTime<Utc>) -> String {
        // In production, use proper cryptographic signing (Ed25519, etc.)
        format!("sig:{}:{}:{}", self.server_key, state_hash, timestamp.timestamp())
    }

    fn compute_checksum(&self, data: &KaijuExportData) -> String {
        // In production, use proper checksum (CRC32, SHA256, etc.)
        format!(
            "checksum:{}:{}:{}",
            data.kaiju_id, data.generation, data.export_timestamp.timestamp()
        )
    }
}

/// Verification result
#[derive(Debug, Clone)]
pub enum VerificationResult {
    Valid {
        kaiju_id: Uuid,
        export_timestamp: DateTime<Utc>,
    },
    InvalidSignature,
    TamperedData,
    GenomeMismatch,
    UnsupportedVersion,
}

/// Export error
#[derive(Debug)]
pub enum ExportError {
    KaijuNotFound,
    NotOwner,
    SerializationFailed(String),
    DeserializationFailed(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KaijuNotFound => write!(f, "Kaiju not found"),
            Self::NotOwner => write!(f, "Not the owner"),
            Self::SerializationFailed(e) => write!(f, "Serialization failed: {}", e),
            Self::DeserializationFailed(e) => write!(f, "Deserialization failed: {}", e),
        }
    }
}

impl std::error::Error for ExportError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_export() {
        let service = ExportService::new("test_key");
        
        let package = service.create_export(
            Uuid::new_v4(),
            "TestKaiju".to_string(),
            2,
            "abc123".to_string(),
            12345,
            ExportedStats { hp: 300, attack: 50, defense: 40, speed: 60 },
            vec!["Fire Breath".to_string()],
            5,
            Utc::now(),
            None,
            Uuid::new_v4(),
            true,
        );

        let result = service.verify_export(&package);
        assert!(matches!(result, VerificationResult::Valid { .. }));
    }

    #[test]
    fn test_tampered_export() {
        let service = ExportService::new("test_key");
        
        let mut package = service.create_export(
            Uuid::new_v4(),
            "TestKaiju".to_string(),
            2,
            "abc123".to_string(),
            12345,
            ExportedStats { hp: 300, attack: 50, defense: 40, speed: 60 },
            vec![],
            5,
            Utc::now(),
            None,
            Uuid::new_v4(),
            true,
        );

        // Tamper with data
        package.data.stats.hp = 9999;

        let result = service.verify_export(&package);
        assert!(matches!(result, VerificationResult::TamperedData | VerificationResult::GenomeMismatch));
    }

    #[test]
    fn test_json_roundtrip() {
        let service = ExportService::new("test_key");
        
        let package = service.create_export(
            Uuid::new_v4(),
            "TestKaiju".to_string(),
            1,
            "def456".to_string(),
            54321,
            ExportedStats { hp: 200, attack: 40, defense: 30, speed: 50 },
            vec!["Ice Breath".to_string()],
            3,
            Utc::now(),
            None,
            Uuid::new_v4(),
            true,
        );

        let json = service.to_json(&package).unwrap();
        let restored = service.from_json(&json).unwrap();

        assert!(matches!(service.verify_export(&restored), VerificationResult::Valid { .. }));
    }
}
