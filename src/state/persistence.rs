//! Save/Load persistence for game state.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::state::game_state::GameState;

const SAVE_FILE: &str = "kaiju_sim_save.json";
const SAVE_VERSION: u32 = 1;

/// Save data wrapper with versioning
#[derive(Serialize, Deserialize)]
struct SaveData {
    version: u32,
    game_state: GameState,
    metadata: SaveMetadata,
}

/// Save file metadata
#[derive(Serialize, Deserialize)]
struct SaveMetadata {
    saved_at: String,
    play_time_seconds: u64,
}

/// Save game state to JSON file
pub fn save_game(state: &GameState) -> Result<(), PersistenceError> {
    let save_data = SaveData {
        version: SAVE_VERSION,
        game_state: state.clone(),
        metadata: SaveMetadata {
            saved_at: chrono::Utc::now().to_rfc3339(),
            play_time_seconds: state.game_time.total_ticks / 60,
        },
    };

    let json = serde_json::to_string_pretty(&save_data)
        .map_err(|e| PersistenceError::SerializationFailed(e.to_string()))?;

    let path = get_save_path()?;
    std::fs::write(&path, json).map_err(|e| PersistenceError::WriteFailed(e.to_string()))?;

    eprintln!("Game saved to: {}", path.display());
    Ok(())
}

/// Load game state from JSON file
pub fn load_game() -> Result<GameState, PersistenceError> {
    let path = get_save_path()?;

    if !path.exists() {
        return Err(PersistenceError::SaveNotFound);
    }

    let json =
        std::fs::read_to_string(&path).map_err(|e| PersistenceError::ReadFailed(e.to_string()))?;

    let save_data: SaveData = serde_json::from_str(&json)
        .map_err(|e| PersistenceError::DeserializationFailed(e.to_string()))?;

    // Version migration placeholder
    if save_data.version < SAVE_VERSION {
        eprintln!(
            "Migrating save from version {} to {}",
            save_data.version, SAVE_VERSION
        );
    }

    eprintln!("Game loaded from: {}", path.display());
    Ok(save_data.game_state)
}

/// Check if save file exists
pub fn save_exists() -> bool {
    get_save_path().map(|p| p.exists()).unwrap_or(false)
}

/// Delete save file
pub fn delete_save() -> Result<(), PersistenceError> {
    let path = get_save_path()?;
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| PersistenceError::DeleteFailed(e.to_string()))?;
    }
    Ok(())
}

/// Get platform-specific save file path
fn get_save_path() -> Result<PathBuf, PersistenceError> {
    // Try to use app data directory
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(data_dir) = dirs::data_dir() {
            let app_dir = data_dir.join("kaiju_sim");
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| PersistenceError::PathError(e.to_string()))?;
            return Ok(app_dir.join(SAVE_FILE));
        }
    }

    // Fallback to current directory
    Ok(PathBuf::from(SAVE_FILE))
}

/// Persistence errors
#[derive(Debug)]
pub enum PersistenceError {
    SaveNotFound,
    SerializationFailed(String),
    DeserializationFailed(String),
    WriteFailed(String),
    ReadFailed(String),
    DeleteFailed(String),
    PathError(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaveNotFound => write!(f, "Save file not found"),
            Self::SerializationFailed(e) => write!(f, "Serialization failed: {}", e),
            Self::DeserializationFailed(e) => write!(f, "Deserialization failed: {}", e),
            Self::WriteFailed(e) => write!(f, "Write failed: {}", e),
            Self::ReadFailed(e) => write!(f, "Read failed: {}", e),
            Self::DeleteFailed(e) => write!(f, "Delete failed: {}", e),
            Self::PathError(e) => write!(f, "Path error: {}", e),
        }
    }
}

impl std::error::Error for PersistenceError {}

/// Auto-save manager
pub struct AutoSaveManager {
    last_save: std::time::Instant,
    save_interval: std::time::Duration,
    enabled: bool,
}

impl Default for AutoSaveManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoSaveManager {
    pub fn new() -> Self {
        Self {
            last_save: std::time::Instant::now(),
            save_interval: std::time::Duration::from_secs(60),
            enabled: true,
        }
    }

    /// Update auto-save, returns true if save was performed
    pub fn update(&mut self, state: &GameState) -> bool {
        if !self.enabled || !state.player.settings.auto_save_enabled {
            return false;
        }

        if self.last_save.elapsed() >= self.save_interval {
            match save_game(state) {
                Ok(_) => {
                    self.last_save = std::time::Instant::now();
                    eprintln!("Auto-save complete");
                    true
                }
                Err(e) => {
                    eprintln!("Auto-save failed: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Force immediate save
    pub fn force_save(&mut self, state: &GameState) -> Result<(), PersistenceError> {
        save_game(state)?;
        self.last_save = std::time::Instant::now();
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_path() {
        let path = get_save_path();
        assert!(path.is_ok());
    }
}
