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
        stats, // Deterministic JSON serialization
        traits
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
#[allow(clippy::too_many_arguments)] // mirrors the independent kaiju state fields hashed by compute_kaiju_state_hash; a param struct would just move the same fields
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
mod tests;
