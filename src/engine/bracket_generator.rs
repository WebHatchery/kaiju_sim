//! Bracket generation algorithms for tournaments.
//!
//! Supports Single Elimination, Double Elimination, and Swiss systems.

use crate::data::environments::Environment;
use crate::data::tournament::{
    Bracket, BracketSystem, Match, MatchId, Round, RoundStatus, TournamentId,
};
use crate::data::types::{random_index, shuffle_slice, KaijuId};

/// Bracket generator interface
pub trait BracketGenerator {
    fn generate(
        &self,
        tournament_id: TournamentId,
        participants: Vec<KaijuId>,
        environment: Environment,
    ) -> Result<Bracket, BracketError>;
}

/// Single elimination bracket generator
pub struct SingleEliminationGenerator;

impl SingleEliminationGenerator {
    /// Create seeded pairings (1v8, 2v7, 3v6, 4v5 for 8 participants)
    fn create_seeded_pairings(seeded: &[KaijuId]) -> Vec<(KaijuId, KaijuId)> {
        let n = seeded.len();
        let mut pairings = Vec::new();

        for i in 0..(n / 2) {
            pairings.push((seeded[i], seeded[n - 1 - i]));
        }

        pairings
    }

    /// Calculate number of rounds needed
    fn calculate_rounds(participant_count: usize) -> u32 {
        (participant_count as f32).log2().ceil() as u32
    }
}

impl BracketGenerator for SingleEliminationGenerator {
    fn generate(
        &self,
        tournament_id: TournamentId,
        participants: Vec<KaijuId>,
        environment: Environment,
    ) -> Result<Bracket, BracketError> {
        if participants.len() < 2 {
            return Err(BracketError::InsufficientParticipants);
        }

        let rounds_count = Self::calculate_rounds(participants.len());
        let mut matches = Vec::new();
        let mut rounds = Vec::new();

        // Create first round pairings
        let pairings = Self::create_seeded_pairings(&participants);
        let mut round_1_match_ids = Vec::new();

        for (kaiju_a, kaiju_b) in pairings {
            let match_obj = Match::new(tournament_id, 1, kaiju_a, kaiju_b, environment.clone());
            round_1_match_ids.push(match_obj.id);
            matches.push(match_obj);
        }

        rounds.push(Round {
            number: 1,
            match_ids: round_1_match_ids,
            status: RoundStatus::Pending,
            environment: environment.clone(),
        });

        // Create placeholder rounds (matches will be created as winners advance)
        for round_num in 2..=rounds_count {
            rounds.push(Round {
                number: round_num,
                match_ids: Vec::new(),
                status: RoundStatus::Pending,
                environment: environment.clone(),
            });
        }

        Ok(Bracket {
            bracket_type: BracketSystem::SingleElimination,
            matches,
            rounds,
            participant_count: participants.len() as u32,
        })
    }
}

/// Swiss system bracket generator
pub struct SwissGenerator {
    pub total_rounds: u32,
}

impl SwissGenerator {
    /// Create initial random pairings for first round
    fn create_initial_pairings(
        &self,
        tournament_id: TournamentId,
        mut participants: Vec<KaijuId>,
        environment: Environment,
    ) -> Vec<Match> {
        shuffle_slice(&mut participants);

        let mut matches = Vec::new();
        for chunk in participants.chunks(2) {
            if chunk.len() == 2 {
                matches.push(Match::new(
                    tournament_id,
                    1,
                    chunk[0],
                    chunk[1],
                    environment.clone(),
                ));
            }
            // Odd participant gets a bye (handled separately)
        }

        matches
    }
}

impl BracketGenerator for SwissGenerator {
    fn generate(
        &self,
        tournament_id: TournamentId,
        participants: Vec<KaijuId>,
        environment: Environment,
    ) -> Result<Bracket, BracketError> {
        if participants.len() < 2 {
            return Err(BracketError::InsufficientParticipants);
        }

        // Create first round with random pairings
        let first_round_matches =
            self.create_initial_pairings(tournament_id, participants.clone(), environment.clone());

        let match_ids: Vec<MatchId> = first_round_matches.iter().map(|m| m.id).collect();

        let mut rounds = vec![Round {
            number: 1,
            match_ids,
            status: RoundStatus::Pending,
            environment: environment.clone(),
        }];

        // Create placeholder rounds
        for round_num in 2..=self.total_rounds {
            rounds.push(Round {
                number: round_num,
                match_ids: Vec::new(),
                status: RoundStatus::Pending,
                environment: environment.clone(),
            });
        }

        Ok(Bracket {
            bracket_type: BracketSystem::Swiss {
                rounds: self.total_rounds,
            },
            matches: first_round_matches,
            rounds,
            participant_count: participants.len() as u32,
        })
    }
}

/// Generate next round pairings for Swiss system based on current records
pub fn generate_swiss_next_round(
    tournament_id: TournamentId,
    round_number: u32,
    records: Vec<(KaijuId, u32, u32)>, // (kaiju_id, wins, losses)
    environment: Environment,
) -> Vec<Match> {
    use std::collections::HashMap;

    // Group by record
    let mut groups: HashMap<(u32, u32), Vec<KaijuId>> = HashMap::new();
    for (kaiju_id, wins, losses) in records {
        groups.entry((wins, losses)).or_default().push(kaiju_id);
    }

    // Pair within groups
    let mut matches = Vec::new();
    for ((_wins, _losses), mut kaiju_list) in groups {
        shuffle_slice(&mut kaiju_list);

        for chunk in kaiju_list.chunks(2) {
            if chunk.len() == 2 {
                matches.push(Match::new(
                    tournament_id,
                    round_number,
                    chunk[0],
                    chunk[1],
                    environment.clone(),
                ));
            }
        }
    }

    matches
}

/// Select environment for a round based on tournament settings
pub fn select_environment(
    mode: &crate::data::tournament::EnvironmentMode,
    round: u32,
) -> Environment {
    match mode {
        crate::data::tournament::EnvironmentMode::Fixed(env) => env.clone(),
        crate::data::tournament::EnvironmentMode::Random => {
            let environments = Environment::all();
            environments[random_index(environments.len())].clone()
        }
        crate::data::tournament::EnvironmentMode::RotatingPerRound => {
            let environments = Environment::all();
            environments[(round as usize - 1) % environments.len()].clone()
        }
    }
}

/// Bracket generation errors
#[derive(Debug, Clone)]
pub enum BracketError {
    InsufficientParticipants,
    InvalidBracketType,
    OddParticipantCount,
}

impl std::fmt::Display for BracketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientParticipants => write!(f, "Need at least 2 participants"),
            Self::InvalidBracketType => write!(f, "Invalid bracket type"),
            Self::OddParticipantCount => write!(f, "Odd number of participants"),
        }
    }
}

impl std::error::Error for BracketError {}

#[cfg(test)]
mod tests {
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
}
