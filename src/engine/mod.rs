//! Game logic services (stateless).
//!
//! This module contains the core game logic for breeding, combat, tournaments, synergies, and research.

pub mod bracket_generator;
pub mod breeding;
pub mod combat;
pub mod research;
pub mod synergies;
pub mod tournament_engine;

// Re-export commonly used types
pub use bracket_generator::{BracketGenerator, SingleEliminationGenerator, SwissGenerator};
pub use breeding::{breed_kaiju, BreedingConfig, BreedingError, BreedingResult};
pub use combat::{execute_battle, BattleSimulator, CombatConfig};
pub use research::{DecodingLayer, ResearchFacility, TraitKnowledge};
pub use synergies::{IncompatibilityMatrix, SynergyDatabase, SynergyDefinition};
pub use tournament_engine::TournamentEngine;
