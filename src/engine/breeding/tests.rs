use super::*;
use crate::data::traits::{TraitCategory, TraitCondition};

fn create_test_kaiju(name: &str, gen: u32, hp: i32, atk: i32) -> Kaiju {
    Kaiju {
        id: new_kaiju_id(),
        token_id: macroquad_toolkit::rng::random_u64(),
        name: name.to_string(),
        generation: gen,
        created_at: now_timestamp(),
        original_breeder: "test".to_string(),
        parent_ids: None,
        visual_seed: macroquad_toolkit::rng::random_u64(),
        genome_hash: format!("{:016x}", macroquad_toolkit::rng::random_u64()),
        stats: KaijuStats::new(hp, atk, 30, 25, 100),
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
fn test_breed_kaiju_basic() {
    let parent_a = create_test_kaiju("Alpha", 2, 300, 60);
    let parent_b = create_test_kaiju("Beta", 3, 320, 55);
    let config = BreedingConfig::default();

    let result = breed_kaiju(&parent_a, &parent_b, 12345, &config).unwrap();

    // Generation should be max + 1
    assert_eq!(result.offspring.generation, 4);
    assert!(result.offspring.alive);
}

#[test]
fn test_cannot_breed_with_self() {
    let kaiju = create_test_kaiju("Solo", 1, 200, 50);
    let config = BreedingConfig::default();

    let result = breed_kaiju(&kaiju, &kaiju, 12345, &config);
    assert!(matches!(result, Err(BreedingError::SameKaiju)));
}

#[test]
fn test_stat_inheritance_variance() {
    let parent_a = create_test_kaiju("Alpha", 0, 300, 60);
    let parent_b = create_test_kaiju("Beta", 0, 320, 55);
    let config = BreedingConfig::default();

    let mut hp_values = Vec::new();
    for seed in 0..100 {
        let result = breed_kaiju(&parent_a, &parent_b, seed, &config).unwrap();
        hp_values.push(result.offspring.stats.hp);
    }

    // Check variance exists
    let min = *hp_values.iter().min().unwrap();
    let max = *hp_values.iter().max().unwrap();
    assert!(max > min, "Should have variance in HP values");
}
