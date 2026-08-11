use super::*;
use crate::data::new_kaiju_id;

#[test]
fn test_single_elimination_8_participants() {
    let tournament_id = new_kaiju_id();
    let participants: Vec<KaijuId> = (0..8).map(|_| new_kaiju_id()).collect();

    let generator = SingleEliminationGenerator;
    let bracket = generator
        .generate(tournament_id, participants.clone(), Environment::Neutral)
        .unwrap();

    assert_eq!(bracket.participant_count, 8);
    assert_eq!(bracket.matches.len(), 4); // First round only
    assert_eq!(bracket.rounds.len(), 3); // 3 rounds for 8 participants
}

#[test]
fn test_seeded_pairings() {
    let participants: Vec<KaijuId> = (0..8).map(|_| new_kaiju_id()).collect();
    let pairings = SingleEliminationGenerator::create_seeded_pairings(&participants);

    assert_eq!(pairings.len(), 4);
    // 1v8, 2v7, 3v6, 4v5
    assert_eq!(pairings[0].0, participants[0]); // Seed 1
    assert_eq!(pairings[0].1, participants[7]); // Seed 8
}

#[test]
fn test_swiss_first_round() {
    let tournament_id = new_kaiju_id();
    let participants: Vec<KaijuId> = (0..8).map(|_| new_kaiju_id()).collect();

    let generator = SwissGenerator { total_rounds: 3 };
    let bracket = generator
        .generate(tournament_id, participants, Environment::Storm)
        .unwrap();

    assert_eq!(bracket.matches.len(), 4);
    assert_eq!(bracket.rounds.len(), 3);
}

#[test]
fn test_insufficient_participants() {
    let tournament_id = new_kaiju_id();
    let participants = vec![new_kaiju_id()]; // Only 1

    let generator = SingleEliminationGenerator;
    let result = generator.generate(tournament_id, participants, Environment::Neutral);

    assert!(matches!(
        result,
        Err(BracketError::InsufficientParticipants)
    ));
}

#[test]
fn test_environment_selection() {
    let fixed = crate::data::tournament::EnvironmentMode::Fixed(Environment::Volcanic);
    assert!(matches!(
        select_environment(&fixed, 1),
        Environment::Volcanic
    ));

    let rotating = crate::data::tournament::EnvironmentMode::RotatingPerRound;
    // Should rotate through environments
    let env1 = select_environment(&rotating, 1);
    let env2 = select_environment(&rotating, 2);
    assert_ne!(format!("{:?}", env1), format!("{:?}", env2));
}
