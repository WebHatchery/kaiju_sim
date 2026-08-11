use super::*;
use crate::data::traits::{Trait, TraitCondition, TraitInheritance};
use crate::data::{new_kaiju_id, now_timestamp, KaijuStats};

fn create_test_kaiju(name: &str, hp: i32, atk: i32, def: i32, spd: i32) -> Kaiju {
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
        stats: KaijuStats::new(hp, atk, def, spd, 100),
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
fn test_base_damage_calculation() {
    let attacker = create_test_kaiju("A", 100, 50, 20, 30);
    let defender = create_test_kaiju("B", 100, 40, 30, 25);
    let config = CombatConfig::default();
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    // Expected: 50 - (30 * 0.5) = 50 - 15 = 35
    let damage = calculate_damage(
        &attacker,
        &defender,
        &Environment::Neutral,
        &mut rng,
        &config,
    );

    // With ±5% variance, should be 33-37
    assert!((33..=37).contains(&damage), "Damage was {}", damage);
}

#[test]
fn test_minimum_damage_floor() {
    let attacker = create_test_kaiju("A", 100, 10, 10, 10);
    let defender = create_test_kaiju("B", 100, 10, 100, 10);
    let config = CombatConfig::default();
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let damage = calculate_damage(
        &attacker,
        &defender,
        &Environment::Neutral,
        &mut rng,
        &config,
    );

    // Should apply minimum floor and then variance
    assert!(damage >= 4, "Damage was {}", damage);
}

#[test]
fn test_trait_power_bonus() {
    let mut attacker = create_test_kaiju("A", 100, 50, 20, 30);
    attacker.traits.push(Trait {
        id: "electric_breath".to_string(),
        name: "Electric Breath".to_string(),
        category: TraitCategory::Element,
        power: 8,
        inheritance: TraitInheritance::Dominant,
        condition: TraitCondition::Simple("Always".to_string()),
        is_hidden: false,
        description: "Test trait".to_string(),
    });

    let defender = create_test_kaiju("B", 100, 40, 30, 25);
    let config = CombatConfig::default();
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let damage = calculate_damage(
        &attacker,
        &defender,
        &Environment::Neutral,
        &mut rng,
        &config,
    );

    // Expected: (50 - 15 + 8) * 1.0 * ~1.0 = ~43
    assert!((40..=46).contains(&damage), "Damage was {}", damage);
}

#[test]
fn test_full_battle_simulation() {
    let kaiju_a = create_test_kaiju("Flossy", 300, 60, 40, 30);
    let kaiju_b = create_test_kaiju("Reefmaw", 320, 55, 45, 25);

    let config = CombatConfig::default();
    let result = execute_battle(kaiju_a, kaiju_b, Environment::Neutral, 12345, &config);

    assert!(!result.winner.is_empty());
    assert!(!result.loser.is_empty());
    assert!(result.hp_remaining >= 0);
    assert!(result.turns_elapsed > 0);
    assert!(!result.battle_log.is_empty());
}

#[test]
fn test_replay_determinism() {
    let kaiju_a = create_test_kaiju("Alpha", 300, 60, 40, 30);
    let kaiju_b = create_test_kaiju("Beta", 320, 55, 45, 25);
    let config = CombatConfig::default();

    let result1 = execute_battle(
        kaiju_a.clone(),
        kaiju_b.clone(),
        Environment::Storm,
        12345,
        &config,
    );

    let result2 = execute_battle(
        kaiju_a.clone(),
        kaiju_b.clone(),
        Environment::Storm,
        12345,
        &config,
    );

    assert_eq!(result1.winner, result2.winner);
    assert_eq!(result1.hp_remaining, result2.hp_remaining);
    assert_eq!(result1.turns_elapsed, result2.turns_elapsed);
    assert_eq!(result1.battle_log.len(), result2.battle_log.len());
}

#[test]
fn test_speed_determines_first_attacker() {
    let fast = create_test_kaiju("Fast", 100, 50, 30, 100);
    let slow = create_test_kaiju("Slow", 100, 50, 30, 10);

    let config = CombatConfig::default();
    let result = execute_battle(fast, slow, Environment::Neutral, 0, &config);

    assert_eq!(result.battle_log[0].attacker_name, "Fast");
}

#[test]
fn test_variance_distribution() {
    let config = CombatConfig::default();
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    let mut results = Vec::new();
    for _ in 0..1000 {
        let variance = rng.gen_range(config.variance_min..=config.variance_max);
        results.push(variance);
    }

    // All within bounds
    assert!(results.iter().all(|&v| (0.95..=1.05).contains(&v)));

    // Mean should be close to 1.0
    let mean: f32 = results.iter().sum::<f32>() / results.len() as f32;
    assert!((mean - 1.0).abs() < 0.02);
}
