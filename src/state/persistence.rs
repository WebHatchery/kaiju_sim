//! Save/Load persistence for game state.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::state::game_state::GameState;

const SAVE_FILE: &str = "kaiju_sim_save.json";
const SAVE_GAME: &str = "kaiju_sim";
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
            saved_at: crate::data::now_timestamp_string(),
            play_time_seconds: state.game_time.total_ticks / 60,
        },
    };

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_save_path()?;
        macroquad_toolkit::persistence::save_json_atomic(&path, &save_data)
            .map_err(PersistenceError::WriteFailed)?;

        eprintln!("Game saved to: {}", path.display());
    }

    #[cfg(target_arch = "wasm32")]
    {
        macroquad_toolkit::persistence::save_json_key(SAVE_GAME, SAVE_FILE, &save_data)
            .map_err(PersistenceError::WriteFailed)?;
    }

    Ok(())
}

/// Load game state from JSON file
pub fn load_game() -> Result<GameState, PersistenceError> {
    #[cfg(target_arch = "wasm32")]
    {
        let save_data: SaveData =
            macroquad_toolkit::persistence::load_json_key(SAVE_GAME, SAVE_FILE).map_err(|e| {
                if e.contains("No data found") {
                    PersistenceError::SaveNotFound
                } else {
                    PersistenceError::ReadFailed(e)
                }
            })?;

        return Ok(validate_loaded_save(save_data));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_save_path()?;

        if !path.exists() {
            return Err(PersistenceError::SaveNotFound);
        }

        let json = std::fs::read_to_string(&path)
            .map_err(|e| PersistenceError::ReadFailed(e.to_string()))?;

        let save_data: SaveData = serde_json::from_str(&json)
            .map_err(|e| PersistenceError::DeserializationFailed(e.to_string()))?;

        eprintln!("Game loaded from: {}", path.display());
        Ok(validate_loaded_save(save_data))
    }
}

/// Check if save file exists
pub fn save_exists() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        return macroquad_toolkit::persistence::json_key_exists(SAVE_GAME, SAVE_FILE);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        get_save_path().map(|p| p.exists()).unwrap_or(false)
    }
}

/// Delete save file
pub fn delete_save() -> Result<(), PersistenceError> {
    #[cfg(target_arch = "wasm32")]
    {
        return macroquad_toolkit::persistence::delete_json_key(SAVE_GAME, SAVE_FILE)
            .map_err(PersistenceError::DeleteFailed);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = get_save_path()?;
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| PersistenceError::DeleteFailed(e.to_string()))?;
        }
        Ok(())
    }
}

fn validate_loaded_save(save_data: SaveData) -> GameState {
    if save_data.version < SAVE_VERSION {
        eprintln!(
            "Migrating save from version {} to {}",
            save_data.version, SAVE_VERSION
        );
    }

    save_data.game_state
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
    #[cfg(not(target_arch = "wasm32"))]
    last_save: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    last_save: f64,
    #[cfg(not(target_arch = "wasm32"))]
    save_interval: std::time::Duration,
    #[cfg(target_arch = "wasm32")]
    save_interval: f64,
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
            #[cfg(not(target_arch = "wasm32"))]
            last_save: std::time::Instant::now(),
            #[cfg(target_arch = "wasm32")]
            last_save: macroquad::time::get_time(),
            #[cfg(not(target_arch = "wasm32"))]
            save_interval: std::time::Duration::from_secs(60),
            #[cfg(target_arch = "wasm32")]
            save_interval: 60.0,
            enabled: true,
        }
    }

    /// Update auto-save, returns true if save was performed
    pub fn update(&mut self, state: &GameState) -> bool {
        if !self.enabled || !state.player.settings.auto_save_enabled {
            return false;
        }

        let interval_elapsed = {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.last_save.elapsed() >= self.save_interval
            }

            #[cfg(target_arch = "wasm32")]
            {
                macroquad::time::get_time() - self.last_save >= self.save_interval
            }
        };

        if interval_elapsed {
            match save_game(state) {
                Ok(_) => {
                    self.mark_saved();
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
        self.mark_saved();
        Ok(())
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn mark_saved(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.last_save = std::time::Instant::now();
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.last_save = macroquad::time::get_time();
        }
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
