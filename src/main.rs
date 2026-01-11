use macroquad::prelude::*;

mod data;
mod engine;
mod screens;
mod state;
mod ui;

use data::GameData;
use screens::*;
use state::game_phase::{GamePhase, PhaseStack, PhaseTransition};
use state::GameState;
use ui::actions::UiAction;
use ui::assets::AssetManager;
use ui::colors::dark;
use state::NotificationType;
use engine::breeding::{self, BreedingConfig};
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

    // Main game loop
    loop {
        // Global time update
        state.tick();
        
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
            GamePhase::Breeding => draw_breeding_screen(&state, &mut breeding_state, &assets),
            
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
            handle_ui_action(&mut state, &mut phase_stack, &mut breeding_state, act);
        }

        next_frame().await;
    }
}

/// Handle UI actions and apply phase transitions
fn handle_ui_action(
    state: &mut GameState, 
    stack: &mut PhaseStack, 
    breeding_state: &mut BreedingState,
    action: UiAction
) {
    match action {
        // Navigation
        UiAction::GoToMenu => stack.apply(PhaseTransition::to_menu()),
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
            *state = GameState::default(); // New game
            stack.apply(PhaseTransition::to_laboratory());
        }
        UiAction::ContinueGame => {
            // Logic to load game would go here
             stack.apply(PhaseTransition::to_laboratory());
        }
        UiAction::ExitGame => {
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

                // Get parents (cloned to avoid borrow conflicts)
                let parent_a = state.get_kaiju(id_a).cloned();
                let parent_b = state.get_kaiju(id_b).cloned();

                if let (Some(pa), Some(pb)) = (parent_a, parent_b) {
                    let config = BreedingConfig::default();
                    // Use macroquad's rand for WebGL compatibility
                    let seed = macroquad::rand::rand() as u64;
                    
                    match breeding::breed_kaiju(&pa, &pb, seed, &config) {
                        Ok(result) => {
                            // Success
                            if state.player.spend_gold(cost) {
                                let name = result.offspring.name.clone();
                                state.add_kaiju(result.offspring);
                                state.notify(format!("Breeding successful! {} created.", name), NotificationType::Success);
                                
                                // Reset breeding state
                                *breeding_state = BreedingState::default();
                                
                                stack.apply(PhaseTransition::Replace(GamePhase::Roster));
                            }
                        }
                        Err(e) => {
                             state.notify(format!("Breeding failed: {}", e), NotificationType::Error);
                        }
                    }
                } else {
                     state.notify("Parent not found!".to_string(), NotificationType::Error);
                }
            } else {
                state.notify("Select two parents first!".to_string(), NotificationType::Warning);
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
