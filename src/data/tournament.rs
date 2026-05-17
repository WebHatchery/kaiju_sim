//! Tournament data structures for competitive kaiju battles.
//!
//! Supports multiple tournament types, bracket systems, and match tracking.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::environments::Environment;
use crate::data::types::KaijuId;

/// Unique tournament identifier
pub type TournamentId = Uuid;

/// Unique match identifier
pub type MatchId = Uuid;

/// Tournament type determining rules and stakes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TournamentType {
    /// Standard ranked tournament, losers survive
    NonLethalRanked,
    /// Winner takes all, losers die permanently
    LethalWinnerTakesAll,
    /// Restricted by generation range
    GenerationRestricted { min_gen: u32, max_gen: Option<u32> },
    /// Special event with custom modifiers
    SpecialEvent { name: String, xp_multiplier: f32 },
}

/// Bracket system for tournament structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BracketSystem {
    /// Single loss elimination
    SingleElimination,
    /// Two losses elimination
    DoubleElimination,
    /// Fixed rounds, pairing by record
    Swiss { rounds: u32 },
}

/// Environment selection mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EnvironmentMode {
    /// Same environment for all matches
    Fixed(Environment),
    /// Random environment per match
    #[default]
    Random,
    /// Different environment each round
    RotatingPerRound,
}

/// Tournament status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TournamentStatus {
    /// Accepting registrations
    RegistrationOpen,
    /// Registration closed, not yet started
    RegistrationClosed,
    /// Tournament in progress
    InProgress { current_round: u32 },
    /// Tournament completed
    Completed { winner_id: KaijuId },
    /// Tournament cancelled
    Cancelled { reason: String },
}

/// Complete tournament definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    /// Unique identifier
    pub id: TournamentId,
    /// Display name
    pub name: String,
    /// Tournament type (lethal/non-lethal)
    pub tournament_type: TournamentType,
    /// Bracket system
    pub bracket_system: BracketSystem,
    /// Environment mode
    pub environment_mode: EnvironmentMode,
    /// Maximum participants
    pub max_participants: u32,
    /// Minimum ranking threshold (for lethal)
    pub min_ranking_threshold: Option<i32>,
    /// Registered participants
    pub participants: Vec<KaijuId>,
    /// Generated bracket
    pub bracket: Option<Bracket>,
    /// Current round number
    pub current_round: u32,
    /// Tournament status
    pub status: TournamentStatus,
    /// Reward structure
    pub rewards: RewardStructure,
}

impl Tournament {
    /// Create a new tournament
    pub fn new(
        name: String,
        tournament_type: TournamentType,
        bracket_system: BracketSystem,
        max_participants: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            tournament_type,
            bracket_system,
            environment_mode: EnvironmentMode::default(),
            max_participants,
            min_ranking_threshold: None,
            participants: Vec::new(),
            bracket: None,
            current_round: 0,
            status: TournamentStatus::RegistrationOpen,
            rewards: RewardStructure::default(),
        }
    }

    /// Check if tournament is lethal
    pub fn is_lethal(&self) -> bool {
        matches!(self.tournament_type, TournamentType::LethalWinnerTakesAll)
    }

    /// Check if registration is open
    pub fn can_register(&self) -> bool {
        matches!(self.status, TournamentStatus::RegistrationOpen)
            && self.participants.len() < self.max_participants as usize
    }

    /// Register a participant
    pub fn register(&mut self, kaiju_id: KaijuId) -> Result<(), TournamentError> {
        if !self.can_register() {
            return Err(TournamentError::RegistrationClosed);
        }

        if self.participants.contains(&kaiju_id) {
            return Err(TournamentError::AlreadyRegistered);
        }

        self.participants.push(kaiju_id);
        Ok(())
    }
}

/// Tournament bracket structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bracket {
    /// Bracket type
    pub bracket_type: BracketSystem,
    /// All matches in the tournament
    pub matches: Vec<Match>,
    /// Rounds in order
    pub rounds: Vec<Round>,
    /// Total participants
    pub participant_count: u32,
}

/// A single round in the bracket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    /// Round number (1-indexed)
    pub number: u32,
    /// Matches in this round
    pub match_ids: Vec<MatchId>,
    /// Round status
    pub status: RoundStatus,
    /// Environment for this round
    pub environment: Environment,
}

/// Round status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RoundStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
}

/// A single match between two kaiju
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    /// Unique match identifier
    pub id: MatchId,
    /// Parent tournament
    pub tournament_id: TournamentId,
    /// Round number
    pub round: u32,
    /// First participant
    pub kaiju_a: KaijuId,
    /// Second participant
    pub kaiju_b: KaijuId,
    /// Battle environment
    pub environment: Environment,
    /// Match result (None if not yet played)
    pub result: Option<MatchResult>,
    /// Random seed for deterministic replay
    pub battle_seed: u64,
}

impl Match {
    /// Create a new match
    pub fn new(
        tournament_id: TournamentId,
        round: u32,
        kaiju_a: KaijuId,
        kaiju_b: KaijuId,
        environment: Environment,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            tournament_id,
            round,
            kaiju_a,
            kaiju_b,
            environment,
            result: None,
            battle_seed: rand::random(),
        }
    }

    /// Check if match has been played
    pub fn is_complete(&self) -> bool {
        self.result.is_some()
    }

    /// Get winner ID if match is complete
    pub fn winner(&self) -> Option<KaijuId> {
        self.result.as_ref().map(|r| r.winner)
    }

    /// Get loser ID if match is complete
    pub fn loser(&self) -> Option<KaijuId> {
        self.result.as_ref().map(|r| r.loser)
    }
}

/// Result of a completed match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    /// Winner's kaiju ID
    pub winner: KaijuId,
    /// Loser's kaiju ID
    pub loser: KaijuId,
    /// Number of turns
    pub turns: u32,
    /// Final HP (winner, loser)
    pub final_hp: (i32, i32),
    /// Whether this was an upset
    pub was_upset: bool,
    /// Probability of upset occurring
    pub upset_probability: f32,
}

/// Reward structure for tournament placements
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RewardStructure {
    /// Rewards by placement
    pub placement_rewards: Vec<PlacementReward>,
    /// Participation reward for all
    pub participation_xp: u32,
    /// Participation currency
    pub participation_currency: u32,
}

/// Reward for a specific placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementReward {
    /// Placement (1 = winner, 2 = runner-up, etc.)
    pub placement: u32,
    /// XP reward
    pub xp: u32,
    /// Ranking points change
    pub ranking_points: i32,
    /// Optional title
    pub title: Option<String>,
    /// Currency reward
    pub currency: u32,
}

/// Tournament errors
#[derive(Debug, Clone)]
pub enum TournamentError {
    RegistrationClosed,
    AlreadyRegistered,
    TournamentFull,
    KaijuDead,
    NotOwner,
    GenerationMismatch,
    RankingTooLow,
    InsufficientParticipants,
    MatchNotFound,
    RoundNotComplete,
}

impl std::fmt::Display for TournamentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegistrationClosed => write!(f, "Registration is closed"),
            Self::AlreadyRegistered => write!(f, "Already registered in this tournament"),
            Self::TournamentFull => write!(f, "Tournament is full"),
            Self::KaijuDead => write!(f, "Cannot register dead kaiju"),
            Self::NotOwner => write!(f, "You do not own this kaiju"),
            Self::GenerationMismatch => write!(f, "Kaiju generation does not meet requirements"),
            Self::RankingTooLow => write!(f, "Kaiju ranking too low for this tournament"),
            Self::InsufficientParticipants => write!(f, "Not enough participants"),
            Self::MatchNotFound => write!(f, "Match not found"),
            Self::RoundNotComplete => write!(f, "Round not yet complete"),
        }
    }
}

impl std::error::Error for TournamentError {}

#[cfg(test)]
mod tests {
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

        let kaiju1 = Uuid::new_v4();
        let kaiju2 = Uuid::new_v4();

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
        let tournament_id = Uuid::new_v4();
        let kaiju_a = Uuid::new_v4();
        let kaiju_b = Uuid::new_v4();

        let match_obj = Match::new(tournament_id, 1, kaiju_a, kaiju_b, Environment::Neutral);

        assert_eq!(match_obj.round, 1);
        assert!(!match_obj.is_complete());
        assert!(match_obj.winner().is_none());
    }
}
