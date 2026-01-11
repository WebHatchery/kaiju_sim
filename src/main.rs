#![allow(unused)]

use macroquad::prelude::*;

mod data;
mod engine;
mod screens;
mod state;
mod ui;
mod server_bridge;

use data::GameData;
use screens::*;
use state::game_phase::{GamePhase, PhaseStack, PhaseTransition};
use state::GameState;
use state::persistence::{self, AutoSaveManager};
use ui::actions::UiAction;
use ui::assets::AssetManager;
use ui::colors::dark;
use state::NotificationType;
// use engine::breeding::{self, BreedingConfig}; // Replaced by server bridge
// use rand::Rng; // Removed for WebGL compatibility

fn window_conf() -> Conf {
    Conf {
        window_title: "Kaiju Breeding Simulator".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 0. Verify Server Connection (Mandatory)
    println!("Connecting to Kaiju Server...");
    match server_bridge::check_server_health() {
        Ok(_) => println!("Server Connected."),
        Err(e) => {
            eprintln!("CRITICAL ERROR: Kaiju Server not found!");
            eprintln!("Details: {}", e);
            eprintln!("Please run 'cargo run --bin kaiju-server' in another terminal.");
            std::process::exit(1);
        }
    }

    // 1. Load static game data
    let _game_data = match GameData::load() {
        Ok(data) => {
            println!("Loaded {} traits", data.traits.traits.len());
            data
        }
        Err(e) => {
            eprintln!("Failed to load game data: {}", e);
            // In a real app we'd show an error screen
            return;
        }
    };

    // 2. Initialize mutable game state
    // TODO: Load from save if exists
    let mut state = GameState::default();

    // 3. Initialize asset manager
    let mut assets = AssetManager::new();
    println!("Loading assets...");
    assets.load_all_assets().await;
    println!("Assets loaded.");

    // 4. Initialize phase stack
    let mut phase_stack = PhaseStack::new(GamePhase::MainMenu);
    
    // 5. Initialize UI states
    let mut breeding_state = BreedingState::default();
    let mut marketplace_state = MarketplaceState::default();
    
    // 6. Initialize AutoSave Manager
    let mut auto_save = AutoSaveManager::new();

    // 7. Track pending breeding jobs
    let mut pending_breeding_jobs: Vec<uuid::Uuid> = Vec::new();
    let mut locked_kaiju_ids: std::collections::HashSet<uuid::Uuid> = std::collections::HashSet::new();

    // Main game loop
    loop {
        // Global time update
        state.tick();
        
        // Auto-save check
        auto_save.update(&state);
        
        // Poll pending breeding jobs for completion
        let mut completed_jobs = Vec::new();
        for job_id in &pending_breeding_jobs {
            match server_bridge::check_breeding_status(*job_id) {
                Ok(status) => {
                    println!("[CLIENT] Job {} status: {:?}", job_id, status.status);
                    if status.status == server_bridge::BreedingJobStatus::Complete {
                        println!("[CLIENT] Job complete! Offspring: {:?}", status.offspring.is_some());
                        if let Some(offspring) = status.offspring {
                            // Cache the new image
                            if let Some(url) = &offspring.image_uri {
                                println!("[CLIENT] Downloading image from: {}", url);
                                if let Some(path) = assets.download_if_missing(url) {
                                    let key = assets.get_filename_from_url(url);
                                    assets.load_texture(&key, &path).await;
                                }
                            }
                            
                            let name = offspring.name.clone();
                            state.add_kaiju(offspring);
                            state.notify(format!("{} has hatched!", name), NotificationType::Success);
                            println!("[CLIENT] Added {} to roster!", name);
                        }
                        completed_jobs.push(*job_id);
                    } else if status.status == server_bridge::BreedingJobStatus::Failed {
                        if let Some(err) = status.error_message {
                            state.notify(format!("Breeding failed: {}", err), NotificationType::Error);
                        }
                        completed_jobs.push(*job_id);
                    }
                }
                Err(e) => {
                    println!("[CLIENT] Poll error for job {}: {}", job_id, e);
                }
            }
        }
        
        // Remove completed jobs and unlock parents
        for job_id in completed_jobs {
            pending_breeding_jobs.retain(|id| *id != job_id);
            // Refresh locked Kaiju list from server
            if let Ok(locked) = server_bridge::get_locked_kaiju() {
                locked_kaiju_ids.clear();
                for id in locked {
                    locked_kaiju_ids.insert(id);
                }
            }
        }
        
        // Input handling for dev/debug
        if is_key_pressed(KeyCode::F5) {
             state = GameState::default();
             phase_stack = PhaseStack::new(GamePhase::MainMenu);
             println!("Debug: State Reset");
        }

        // Determine current phase and draw appropriate screen
        let action = match phase_stack.current() {
            GamePhase::Loading => None, 
            GamePhase::MainMenu => draw_main_menu(),
            GamePhase::Laboratory => draw_laboratory(&state),
            GamePhase::Roster => draw_roster_view(&state, &assets),
            GamePhase::Breeding => draw_breeding_screen(&state, &mut breeding_state, &locked_kaiju_ids, &assets),
            
            // WIP Screens
            GamePhase::TournamentLobby => {
                draw_placeholder("Tournament Lobby (Coming Soon)", &state);
                if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::GoToLaboratory)
                } else {
                    None
                }
            }
            GamePhase::Leaderboard => {
                draw_placeholder("Leaderboard (Coming Soon)", &state);
                 if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::GoToLaboratory)
                } else {
                    None
                }
            }
            GamePhase::Battle => {
                draw_placeholder("Battle Simulation (Coming Soon)", &state);
                 if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::GoToLaboratory)
                } else {
                    None
                }
            }
            GamePhase::Results => {
                draw_placeholder("Battle Results", &state);
                 if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::GoToLaboratory)
                } else {
                    None
                }
            }
            GamePhase::KaijuDetail(id) => {
                let action = draw_kaiju_detail(&state, *id, &assets);
                if let Some(UiAction::Back) = action {
                     Some(UiAction::Back)
                } else {
                     action
                }
            }
            GamePhase::StarterSelection => {
                 draw_starter_selection(&assets).await
            }
            GamePhase::Marketplace => {
                draw_marketplace(&mut marketplace_state, state.player.gold, &assets)
            }
            _ => {
                draw_placeholder(&format!("Unknown Phase: {:?}", phase_stack.current()), &state);
                if is_key_pressed(KeyCode::Escape) {
                     Some(UiAction::Back)
                } else {
                    None
                }
            }
        };

        // Handle returned action
        if let Some(act) = action {
            handle_ui_action(&mut state, &mut phase_stack, &mut breeding_state, &mut marketplace_state, &mut pending_breeding_jobs, &mut locked_kaiju_ids, &mut auto_save, &mut assets, act).await;
        }

        next_frame().await;
    }
}

/// Handle UI actions and apply phase transitions
async fn handle_ui_action(
    state: &mut GameState, 
    stack: &mut PhaseStack, 
    breeding_state: &mut BreedingState,
    marketplace_state: &mut MarketplaceState,
    pending_breeding_jobs: &mut Vec<uuid::Uuid>,
    locked_kaiju_ids: &mut std::collections::HashSet<uuid::Uuid>,
    auto_save: &mut AutoSaveManager,
    assets: &mut AssetManager,
    action: UiAction
) {
    match action {
        // Navigation
        UiAction::GoToMenu => {
            if let Err(e) = persistence::save_game(state) {
                eprintln!("Failed to save game on exit to menu: {}", e);
            }
            stack.apply(PhaseTransition::to_menu());
        },
        UiAction::GoToLaboratory => stack.apply(PhaseTransition::to_laboratory()),
        UiAction::GoToRoster => stack.apply(PhaseTransition::Replace(GamePhase::Roster)),
        UiAction::GoToBreeding => {
            *breeding_state = BreedingState::default(); // Reset state
            stack.apply(PhaseTransition::to_breeding());
        },
        UiAction::GoToTournament => stack.apply(PhaseTransition::to_tournament_lobby()),
        UiAction::GoToLeaderboard => stack.apply(PhaseTransition::Replace(GamePhase::Leaderboard)),
        UiAction::GoToSettings => println!("Settings clicked"),
        UiAction::Back => stack.apply(PhaseTransition::Pop),
        
        // System
        UiAction::NewGame => {
            // Go to Starter Selection
            stack.apply(PhaseTransition::Replace(GamePhase::StarterSelection));
        }
        UiAction::SelectStarter(choice) => {
            // Server Authoritative Login with Choice
            match server_bridge::login_to_server(None, Some(choice.clone())) {
                Ok(response) => {
                    *state = GameState::default(); // Reset Local
                    state.player.gold = response.gold as i64;
                    state.roster.clear();
                    
                    println!("Logged in as User: {}", response.user_id);
                    
                    for kaiju in response.roster {
                        // Cache Image
                        if let Some(url) = &kaiju.image_uri {
                            if let Some(path) = assets.download_if_missing(url) {
                                let key = assets.get_filename_from_url(url);
                                assets.load_texture(&key, &path).await;
                            }
                        }
                        state.add_kaiju(kaiju);
                    }
                    
                    // Force initial save with authoritative data
                    let _ = auto_save.force_save(state);
                    stack.apply(PhaseTransition::to_laboratory());
                }
                Err(e) => {
                     eprintln!("Failed to login to server: {}", e);
                     // TODO: Show error UI
                }
            }
        }
        UiAction::ContinueGame => {
             match persistence::load_game() {
                Ok(loaded_state) => {
                    *state = loaded_state;
                    stack.apply(PhaseTransition::to_laboratory());
                }
                Err(e) => {
                    eprintln!("Failed to load game: {}", e);
                    // In a real UI we would show a toast/notification here
                }
             }
        }
        UiAction::ExitGame => {
            if let Err(e) = persistence::save_game(state) {
                eprintln!("Failed to save game on exit: {}", e);
            }
            std::process::exit(0);
        }

        // Kaiju Interaction
        UiAction::SelectKaiju(id) => {
            state.selected_kaiju = Some(id);
        }
        UiAction::ViewKaijuDetails(id) => {
            stack.apply(PhaseTransition::show_kaiju_detail(id));
        }

        // Breeding
        UiAction::ConfirmBreeding => {
            if let (Some(id_a), Some(id_b)) = (breeding_state.parent_a, breeding_state.parent_b) {
                // Check cost
                let cost = 100;
                if state.player.gold < cost {
                    state.notify("Insufficient gold!".to_string(), NotificationType::Error);
                    return;
                }

                if state.get_kaiju(id_a).is_some() && state.get_kaiju(id_b).is_some() {
                    // Use server bridge for authoritative breeding
                    let seed = macroquad::rand::rand() as u64;
                    
                    // Start async breeding job (non-blocking)
                    match server_bridge::start_breeding(id_a, id_b, seed) {
                        Ok(response) => {
                            // Charge gold
                            state.player.spend_gold(cost);
                            
                            // Track job and lock parents
                            pending_breeding_jobs.push(response.job_id);
                            for id in &response.locked_kaiju {
                                locked_kaiju_ids.insert(*id);
                            }
                            
                            state.notify(response.message, NotificationType::Info);
                            
                            // Reset breeding state and go back
                            *breeding_state = BreedingState::default();
                            stack.apply(PhaseTransition::to_laboratory());
                        }
                        Err(e) => {
                            state.notify(format!("Server Error: {}", e), NotificationType::Error);
                            eprintln!("Breeding failed: {}", e);
                        }
                    }
                } else {
                     state.notify("Parent not found!".to_string(), NotificationType::Error);
                }
            } else {
                state.notify("Select two parents first!".to_string(), NotificationType::Warning);
            }
        }

        // Marketplace
        UiAction::GoToMarketplace => {
            *marketplace_state = MarketplaceState::default(); // Reset to fetch fresh data
            stack.apply(PhaseTransition::Replace(GamePhase::Marketplace));
        }
        UiAction::PurchaseKaiju(item_id) => {
            let user_id = uuid::Uuid::nil(); // TODO: Use actual user ID
            match server_bridge::purchase_kaiju(user_id, &item_id) {
                Ok(response) => {
                    if response.success {
                        if let Some(kaiju) = response.kaiju {
                            // Cache Image
                            if let Some(url) = &kaiju.image_uri {
                                if let Some(path) = assets.download_if_missing(url) {
                                    let key = assets.get_filename_from_url(url);
                                    assets.load_texture(&key, &path).await;
                                }
                            }
                            
                            state.add_kaiju(kaiju);
                            state.player.gold = response.new_gold as i64;
                            state.notify(response.message, NotificationType::Success);
                            
                            // Navigate to Roster to show the new Kaiju
                            stack.apply(PhaseTransition::Replace(GamePhase::Roster));
                        }
                    } else {
                        state.notify(response.message, NotificationType::Error);
                    }
                }
                Err(e) => {
                    state.notify(format!("Purchase failed: {}", e), NotificationType::Error);
                }
            }
        }

        // Catch-all
        _ => {
            println!("Action not yet handled: {:?}", action);
        }
    }
}

fn draw_placeholder(title: &str, _state: &GameState) {
    clear_background(dark::BACKGROUND);
    let text = format!("{}", title);
    let size = measure_text(&text, None, 40, 1.0);
    draw_text(
        &text,
        screen_width() / 2.0 - size.width / 2.0,
        screen_height() / 2.0,
        40.0,
        WHITE,
    );
    
    let sub = "Press ESC to return";
    let sub_size = measure_text(sub, None, 20, 1.0);
     draw_text(
        sub,
        screen_width() / 2.0 - sub_size.width / 2.0,
        screen_height() / 2.0 + 50.0,
        20.0,
        GRAY,
    );
}
