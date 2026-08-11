use super::*;
use crate::data::tournament::{BracketSystem, TournamentType};
use crate::data::{new_kaiju_id, now_timestamp, KaijuStats};

fn create_test_kaiju(name: &str, hp: i32, atk: i32) -> Kaiju {
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
fn test_start_tournament() {
    let mut tournament = Tournament::new(
        "Test".to_string(),
        TournamentType::NonLethalRanked,
        BracketSystem::SingleElimination,
        8,
    );

    // Register 4 participants
    for _ in 0..4 {
        tournament.register(new_kaiju_id()).unwrap();
    }

    let engine = TournamentEngine::default();
    engine.start_tournament(&mut tournament).unwrap();

    assert!(matches!(
        tournament.status,
        TournamentStatus::InProgress { current_round: 1 }
    ));
    assert!(tournament.bracket.is_some());
}

#[test]
fn test_execute_match() {
    let engine = TournamentEngine::default();

    let kaiju_a = create_test_kaiju("Alpha", 300, 60);
    let kaiju_b = create_test_kaiju("Beta", 280, 55);

    let mut match_obj = Match::new(
        new_kaiju_id(),
        1,
        kaiju_a.id,
        kaiju_b.id,
        Environment::Neutral,
    );

    let mut rating_a = EloRating::default();
    let mut rating_b = EloRating::default();

    let result = engine.execute_match(
        &mut match_obj,
        &kaiju_a,
        &kaiju_b,
        &mut rating_a,
        &mut rating_b,
    );

    assert!(match_obj.is_complete());
    assert!(!result.winner.is_empty());
}

#[test]
fn test_xp_calculation() {
    // Base win
    let xp = calculate_xp_reward(true, &TournamentType::NonLethalRanked, 1, false);
    assert_eq!(xp, 100);

    // Lethal tournament win
    let xp = calculate_xp_reward(true, &TournamentType::LethalWinnerTakesAll, 1, false);
    assert_eq!(xp, 200);

    // Win with upset bonus in round 3
    let xp = calculate_xp_reward(true, &TournamentType::NonLethalRanked, 3, true);
    assert_eq!(xp, 100 + 50 + 50); // base + round + upset
}
