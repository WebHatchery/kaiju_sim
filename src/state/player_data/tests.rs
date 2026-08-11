use super::*;

#[test]
fn test_default_player() {
    let player = PlayerData::default();
    assert_eq!(player.gold, 1000);
    assert_eq!(player.player_level, 1);
}

#[test]
fn test_gold_operations() {
    let mut player = PlayerData::default();

    player.add_gold(500);
    assert_eq!(player.gold, 1500);

    assert!(player.spend_gold(1000));
    assert_eq!(player.gold, 500);

    assert!(!player.spend_gold(1000)); // Not enough
    assert_eq!(player.gold, 500);
}

#[test]
fn test_experience_level_up() {
    let mut player = PlayerData::default();
    assert_eq!(player.player_level, 1);

    // Level 1 needs 100 XP
    let leveled_up = player.add_experience(100);
    assert!(leveled_up);
    assert_eq!(player.player_level, 2);
}
