use super::*;
use crate::data::{new_kaiju_id, now_timestamp, KaijuStats};

fn create_test_kaiju(name: &str, hp: i32) -> Kaiju {
    Kaiju {
        id: new_kaiju_id(),
        token_id: 0,
        name: name.to_string(),
        generation: 1,
        created_at: now_timestamp(),
        original_breeder: "test".to_string(),
        parent_ids: None,
        visual_seed: 0,
        genome_hash: "test".to_string(),
        stats: KaijuStats::new(hp, 50, 30, 25, 100),
        traits: vec![],
        hidden_traits: vec![],
        experience: 0,
        alive: true,
        current_owner: "test".to_string(),
        image_uri: None,
        metadata_uri: String::new(),
        tournaments_won: 0,
        history: Vec::new(),
    }
}

#[test]
fn test_battle_state_creation() {
    let a = create_test_kaiju("Alpha", 300);
    let b = create_test_kaiju("Beta", 320);

    let state = BattleState::new(a, b, Environment::Neutral, 12345);

    assert_eq!(state.hp_a, 300);
    assert_eq!(state.hp_b, 320);
    assert_eq!(state.turn_count, 0);
    assert!(!state.is_over());
}

#[test]
fn test_battle_result_formatting() {
    let a = create_test_kaiju("Winner", 100);
    let b = create_test_kaiju("Loser", 0);

    let mut state = BattleState::new(a, b, Environment::Storm, 12345);
    state.hp_b = 0;
    state.battle_log.push(BattleLogEntry {
        turn: 1,
        attacker_name: "Winner".to_string(),
        defender_name: "Loser".to_string(),
        damage: 50,
        hp_remaining: 0,
        special_effects: vec![],
    });

    let result = BattleResult::from_state(state);
    let log = result.format_log();

    assert!(log.contains("VICTORY: Winner wins"));
}
