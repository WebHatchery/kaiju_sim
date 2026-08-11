use super::*;

#[test]
fn test_stats_floors() {
    let stats = KaijuStats::new(10, 5, 2, 3, 10);
    assert_eq!(stats.hp, 50); // Floor applied
    assert_eq!(stats.attack, 10); // Floor applied
    assert_eq!(stats.defense, 5); // Floor applied
    assert_eq!(stats.speed, 5); // Floor applied
    assert_eq!(stats.energy, 50); // Floor applied
}

#[test]
fn test_stats_serialization() {
    let stats = KaijuStats::new(200, 50, 40, 30, 100);
    let json = serde_json::to_string(&stats).unwrap();
    let deserialized: KaijuStats = serde_json::from_str(&json).unwrap();
    assert_eq!(stats, deserialized);
}

#[test]
fn test_new_wild_kaiju() {
    let stats = KaijuStats::new(200, 50, 40, 30, 100);
    let kaiju = Kaiju::new_wild("TestKaiju".to_string(), stats, vec![]);

    assert_eq!(kaiju.generation, 0);
    assert_eq!(kaiju.name, "TestKaiju");
    assert!(kaiju.alive);
    assert!(kaiju.can_breed());
    assert!(kaiju.can_compete());
    assert_eq!(kaiju.parent_ids, None);
    assert_eq!(kaiju.history.len(), 1);
    assert_eq!(kaiju.history[0].kind, KaijuEventKind::Created);
}

#[test]
fn test_battle_rating_calculation() {
    let stats = KaijuStats::new(200, 50, 40, 30, 100);
    let kaiju = Kaiju::new_wild("Warrior".to_string(), stats, vec![]);

    let rating = kaiju.battle_rating();
    assert!(rating > 0);
}

#[test]
fn test_missing_history_defaults_when_loading_old_save() {
    let stats = KaijuStats::new(200, 50, 40, 30, 100);
    let kaiju = Kaiju::new_wild("Legacy".to_string(), stats, vec![]);
    let mut value = serde_json::to_value(&kaiju).unwrap();
    value.as_object_mut().unwrap().remove("history");

    let loaded: Kaiju = serde_json::from_value(value).unwrap();
    assert!(loaded.history.is_empty());
}
