//! State hashing for kaiju entities.

use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Compute deterministic hash of kaiju state
pub fn compute_kaiju_state_hash(
    kaiju_id: Uuid,
    owner_user_id: Uuid,
    custody_state: &str,
    alive: bool,
    state_version: i32,
    stats: &Value,
    traits: &Value,
) -> String {
    // Canonical representation
    let state_repr = format!(
        "kaiju_state:{}:{}:{}:{}:{}:{}:{}",
        kaiju_id,
        owner_user_id,
        custody_state,
        alive,
        state_version,
        stats.to_string(), // Deterministic JSON serialization
        traits.to_string()
    );

    let hash = Sha256::digest(state_repr.as_bytes());
    hex::encode(hash)
}

/// Compute a simple state hash from core fields
pub fn compute_simple_state_hash(
    kaiju_id: &str,
    owner_user_id: &str,
    custody_state: &str,
    alive: bool,
    state_version: i32,
) -> String {
    let state_repr = format!(
        "kaiju_state:{}:{}:{}:{}:{}",
        kaiju_id, owner_user_id, custody_state, alive, state_version
    );

    let hash = Sha256::digest(state_repr.as_bytes());
    hex::encode(hash)
}

/// Verify state hash matches expected
pub fn verify_state_hash(
    kaiju_id: Uuid,
    owner_user_id: Uuid,
    custody_state: &str,
    alive: bool,
    state_version: i32,
    stats: &Value,
    traits: &Value,
    expected_hash: &str,
) -> bool {
    let computed = compute_kaiju_state_hash(
        kaiju_id,
        owner_user_id,
        custody_state,
        alive,
        state_version,
        stats,
        traits,
    );

    computed == expected_hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_state_hash_determinism() {
        let kaiju_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let stats = json!({"hp": 100, "attack": 50});
        let traits = json!(["fire", "flying"]);

        let hash1 = compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats, &traits);

        let hash2 = compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats, &traits);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_state_change_changes_hash() {
        let kaiju_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let stats1 = json!({"hp": 100, "attack": 50});
        let stats2 = json!({"hp": 100, "attack": 51}); // Changed
        let traits = json!(["fire"]);

        let hash1 =
            compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats1, &traits);

        let hash2 =
            compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats2, &traits);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_state_hash() {
        let kaiju_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let stats = json!({"hp": 100});
        let traits = json!([]);

        let hash = compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats, &traits);

        assert!(verify_state_hash(
            kaiju_id, owner_id, "server", true, 1, &stats, &traits, &hash
        ));

        assert!(!verify_state_hash(
            kaiju_id, owner_id, "server", false, 1, &stats, &traits, &hash
        ));
    }
}
