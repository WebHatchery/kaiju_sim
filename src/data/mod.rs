//! Data structures and JSON loading.

pub mod environments;
pub mod genome;
pub mod kaiju;
pub mod lineage;
pub mod loader;
pub mod ranking;
pub mod tournament;
pub mod training;
pub mod traits;
pub mod types;

// Re-export commonly used types
pub use environments::Environment;
pub use genome::{Genome, GenomeStats};
pub use kaiju::{Kaiju, KaijuStats};
pub use lineage::{Lineage, LineageHighlight};
pub use loader::GameData;
pub use ranking::EloRating;
pub use tournament::{BracketSystem, Match, MatchResult, Tournament, TournamentType};
pub use training::TrainingFocus;
pub use traits::{Trait, TraitCategory, TraitCondition, TraitInheritance};
pub use types::*;
