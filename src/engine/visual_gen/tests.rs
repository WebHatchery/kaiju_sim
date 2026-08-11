use super::*;
use crate::data::{new_kaiju_id, KaijuStats};

fn create_test_kaiju(seed: u64) -> Kaiju {
    Kaiju {
        id: new_kaiju_id(),
        token_id: 0,
        name: "TestKaiju".to_string(),
        generation: 1,
        created_at: 0,
        original_breeder: "test".to_string(),
        parent_ids: None,
        visual_seed: seed,
        genome_hash: "test".to_string(),
        stats: KaijuStats::new(300, 60, 40, 50, 100),
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
fn test_deterministic_appearance() {
    let kaiju = create_test_kaiju(12345);
    let app1 = generate_appearance(&kaiju);
    let app2 = generate_appearance(&kaiju);

    assert_eq!(app1.body_type, app2.body_type);
    assert_eq!(app1.element, app2.element);
    assert!((app1.scale - app2.scale).abs() < 0.001);
}

#[test]
fn test_size_category() {
    assert_eq!(determine_size(100), SizeCategory::Small);
    assert_eq!(determine_size(200), SizeCategory::Medium);
    assert_eq!(determine_size(350), SizeCategory::Large);
    assert_eq!(determine_size(500), SizeCategory::Massive);
}
