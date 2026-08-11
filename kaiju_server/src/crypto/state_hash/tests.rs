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

    let hash1 = compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats1, &traits);

    let hash2 = compute_kaiju_state_hash(kaiju_id, owner_id, "server", true, 1, &stats2, &traits);

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
