//! Session management: Initialization and Sync logic.

use crate::server_bridge;
use crate::state::game_state::GameState;
use crate::state::persistence;
use crate::ui::assets::AssetManager;
use uuid::Uuid;

/// Initialize the game session.
/// 1. Load User ID from disk (if exists).
/// 2. Sync with Server (if ID exists).
/// 3. Return fresh GameState.
pub async fn initialize_session(assets: &mut AssetManager) -> GameState {
    let mut state = GameState::default();
    // Default gold 0 to indicate pending sync
    state.player.gold = 0;

    // 1. Try Load ID from Save
    match persistence::load_game() {
        Ok(loaded_state) => {
            if !loaded_state.player.player_id.is_empty() {
                println!(
                    "[SESSION] Found saved User ID: {}",
                    loaded_state.player.player_id
                );
                state.player.player_id = loaded_state.player.player_id;
            }
        }
        Err(_) => {
            println!("[SESSION] No save file found. Starting fresh.");
        }
    }

    // 2. Auto-Sync if we have an ID
    if let Ok(user_id) = Uuid::parse_str(&state.player.player_id) {
        println!("[SESSION] Syncing with server for User ID: {}", user_id);
        match server_bridge::login_to_server(Some(user_id), None) {
            Ok(response) => {
                println!(
                    "[SESSION] Sync Successful! Gold: {}, Kaiju: {}",
                    response.gold,
                    response.roster.len()
                );
                state.player.gold = response.gold as i64;
                state.player.player_id = response.user_id.to_string();
                state.roster = response.roster;

                // 3. Ensure assets for roster are loaded
                for kaiju in &state.roster {
                    if let Some(url) = &kaiju.image_uri {
                        if let Some(path) = assets.download_if_missing(url) {
                            let key = assets.get_filename_from_url(url);
                            assets.load_texture(&key, &path).await;
                        }
                    }
                }
            }
            Err(e) => {
                println!("[SESSION] Sync Failed: {}", e);
                // We keep the ID but state remains default (0 gold).
                // User might need to login manually or retry.
            }
        }
    } else {
        println!("[SESSION] No valid User ID to sync. Waiting for Login/Start.");
    }

    state
}

/// Force a re-sync with the server (e.g. after purchase failure).
pub async fn force_resync(state: &mut GameState, assets: &mut AssetManager) {
    if let Ok(user_id) = Uuid::parse_str(&state.player.player_id) {
        println!("[SESSION] Force Re-Sync for User ID: {}", user_id);
        match server_bridge::login_to_server(Some(user_id), None) {
            Ok(response) => {
                println!("[SESSION] Re-sync Successful. Gold: {}", response.gold);
                state.player.gold = response.gold as i64;
                state.roster = response.roster;

                // Reload assets
                for kaiju in &state.roster {
                    if let Some(url) = &kaiju.image_uri {
                        if let Some(path) = assets.download_if_missing(url) {
                            let key = assets.get_filename_from_url(url);
                            assets.load_texture(&key, &path).await;
                        }
                    }
                }
            }
            Err(e) => println!("[SESSION] Re-sync failed: {}", e),
        }
    }
}
