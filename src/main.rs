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
                draw_placeholder(&format!("Detail View: {}", id), &state);
                if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::Back)
                } else {
                    None
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
            println!("Breeding triggered!");
            // In a real implementation:
            // 1. Check/Deduct gold
            // 2. Generate offspring genetics
            // 3. Add to roster
            // 4. Trigger image generation
            
            // For now, simple mock:
            stack.apply(PhaseTransition::Replace(GamePhase::Roster));
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
