//! Game state management.

pub mod battle_state;
pub mod game_phase;
pub mod game_state;
pub mod hall_of_fame;
pub mod persistence;
pub mod player_data;
pub mod session;

// Re-export commonly used types
pub use battle_state::{BattleLogEntry, BattleResult, BattleState};
pub use game_phase::{GamePhase, PhaseStack, PhaseTransition};
pub use game_state::{GameState, Notification, NotificationType};
pub use hall_of_fame::{HallOfFame, HallOfFameEntry, LegacyRecord};
pub use persistence::{load_game, save_game, save_exists, AutoSaveManager, PersistenceError};
pub use player_data::{PlayerData, PlayerSettings, PlayerStats};
