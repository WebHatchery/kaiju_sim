use super::*;

#[test]
fn test_tournament_creation() {
    let tournament = Tournament::new(
        "Test Tournament".to_string(),
        TournamentType::NonLethalRanked,
        BracketSystem::SingleElimination,
        8,
    );

    assert_eq!(tournament.name, "Test Tournament");
    assert_eq!(tournament.max_participants, 8);
    assert!(matches!(
        tournament.status,
        TournamentStatus::RegistrationOpen
    ));
    assert!(!tournament.is_lethal());
}

#[test]
fn test_lethal_tournament() {
    let tournament = Tournament::new(
        "Death Arena".to_string(),
        TournamentType::LethalWinnerTakesAll,
        BracketSystem::SingleElimination,
        8,
    );

    assert!(tournament.is_lethal());
}

#[test]
fn test_registration() {
    let mut tournament = Tournament::new(
        "Test".to_string(),
        TournamentType::NonLethalRanked,
        BracketSystem::SingleElimination,
        2,
    );

    let kaiju1 = new_kaiju_id();
    let kaiju2 = new_kaiju_id();

    assert!(tournament.register(kaiju1).is_ok());
    assert!(tournament.register(kaiju2).is_ok());
    assert!(!tournament.can_register()); // Full

    // Cannot register same kaiju twice
    assert!(matches!(
        tournament.register(kaiju1),
        Err(TournamentError::RegistrationClosed)
    ));
}

#[test]
fn test_match_creation() {
    let tournament_id = new_kaiju_id();
    let kaiju_a = new_kaiju_id();
    let kaiju_b = new_kaiju_id();

    let match_obj = Match::new(tournament_id, 1, kaiju_a, kaiju_b, Environment::Neutral);

    assert_eq!(match_obj.round, 1);
    assert!(!match_obj.is_complete());
    assert!(match_obj.winner().is_none());
}
