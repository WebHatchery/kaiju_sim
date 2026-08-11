use super::*;

#[test]
fn test_new_game_state() {
    let state = GameState::new();
    assert_eq!(state.roster.len(), 2);
    assert_eq!(state.player.gold, 1000);
    assert!(state.roster.iter().all(|kaiju| kaiju
        .history
        .iter()
        .any(|event| event.kind == KaijuEventKind::JoinedRoster)));
}

#[test]
fn test_get_kaiju() {
    let state = GameState::new();
    let first_id = state.roster[0].id;

    assert!(state.get_kaiju(first_id).is_some());
    assert!(state.get_kaiju(crate::data::new_kaiju_id()).is_none());
}

#[test]
fn test_living_kaiju() {
    let mut state = GameState::new();
    assert_eq!(state.living_count(), 2);

    let first_id = state.roster[0].id;
    state.kill_kaiju(first_id);
    assert_eq!(state.living_count(), 1);
}

#[test]
fn test_notifications() {
    let mut state = GameState::new();
    state.notify("Test".to_string(), NotificationType::Info);
    assert_eq!(state.notifications.len(), 1);

    // Add 15 more (should cap at 10)
    for i in 0..15 {
        state.notify(format!("Msg {}", i), NotificationType::Info);
    }
    assert_eq!(state.notifications.len(), 10);
}
