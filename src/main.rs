#![allow(unused)]

use macroquad::prelude::*;

mod data;
mod engine;
mod screens;
mod server_bridge;
mod state;
mod ui;

use data::{Environment, GameData, TrainingFocus};
use engine::mvp;
use engine::{breed_kaiju, BattleSimulator};
use screens::*;
use state::game_phase::{GamePhase, PhaseStack, PhaseTransition};
use state::persistence::{self, AutoSaveManager};
use state::{GameState, LastBattleReport, NotificationType};
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
    let game_data = match GameData::load() {
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

    let mut assets = AssetManager::new();
    println!("Loading assets...");
    assets.load_all_assets().await;
    println!("Assets loaded.");

    let mut state = GameState::default();
    let mut phase_stack = PhaseStack::new(GamePhase::MainMenu);
    let mut breeding_state = BreedingState::default();
    let mut auto_save = AutoSaveManager::new();

    loop {
        state.tick();
        auto_save.update(&state);

        if is_key_pressed(KeyCode::F5) {
            let _ = persistence::delete_save();
            state = GameState::default();
            phase_stack = PhaseStack::new(GamePhase::MainMenu);
            println!("Debug: Save Deleted & State Reset. Restarting...");
        }

        let action = match phase_stack.current() {
            GamePhase::Loading => None,
            GamePhase::MainMenu => draw_main_menu(),
            GamePhase::Laboratory => draw_laboratory(&state),
            GamePhase::Roster => draw_roster_view(&state, &assets),
            GamePhase::Training => {
                draw_training_screen(&state, &assets, game_data.balance.mvp.training.cost)
            }
            GamePhase::Breeding => {
                let locked_kaiju_ids = std::collections::HashSet::new();
                draw_breeding_screen(&state, &mut breeding_state, &locked_kaiju_ids, &assets)
            }
            GamePhase::TournamentLobby => {
                draw_arena_screen(&state, &assets, game_data.balance.mvp.battle.entry_fee)
            }
            GamePhase::Leaderboard => draw_leaderboard(&state),
            GamePhase::Battle | GamePhase::Results => draw_battle_results(&state, &assets),
            GamePhase::KaijuDetail(id) => {
                let action = draw_kaiju_detail(&state, *id, &assets);
                if let Some(UiAction::Back) = action {
                    Some(UiAction::Back)
                } else {
                    action
                }
            }
            GamePhase::StarterSelection => draw_starter_selection(&assets).await,
            GamePhase::Marketplace => {
                draw_placeholder("Marketplace is outside the local MVP", &state);
                if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::GoToLaboratory)
                } else {
                    None
                }
            }
            _ => {
                draw_placeholder(
                    &format!("Unknown Phase: {:?}", phase_stack.current()),
                    &state,
                );
                if is_key_pressed(KeyCode::Escape) {
                    Some(UiAction::Back)
                } else {
                    None
                }
            }
        };

        if let Some(act) = action {
            handle_ui_action(
                &mut state,
                &mut phase_stack,
                &mut breeding_state,
                &mut auto_save,
                &mut assets,
                &game_data,
                act,
            )
            .await;
        }

        next_frame().await;
    }
}

/// Handle UI actions and apply phase transitions
async fn handle_ui_action(
    state: &mut GameState,
    stack: &mut PhaseStack,
    breeding_state: &mut BreedingState,
    auto_save: &mut AutoSaveManager,
    assets: &mut AssetManager,
    game_data: &GameData,
    action: UiAction,
) {
    match action {
        // Navigation
        UiAction::GoToMenu => {
            if let Err(e) = persistence::save_game(state) {
                eprintln!("Failed to save game on exit to menu: {}", e);
            }
            stack.apply(PhaseTransition::to_menu());
        }
        UiAction::GoToLaboratory => stack.apply(PhaseTransition::to_laboratory()),
        UiAction::GoToRoster => stack.apply(PhaseTransition::Replace(GamePhase::Roster)),
        UiAction::GoToTraining => stack.apply(PhaseTransition::to_training()),
        UiAction::GoToBreeding => {
            *breeding_state = BreedingState::default(); // Reset state
            stack.apply(PhaseTransition::to_breeding());
        }
        UiAction::GoToTournament => {
            stack.apply(PhaseTransition::Replace(GamePhase::TournamentLobby))
        }
        UiAction::GoToLeaderboard => stack.apply(PhaseTransition::Replace(GamePhase::Leaderboard)),
        UiAction::GoToSettings => println!("Settings clicked"),
        UiAction::Back => stack.apply(PhaseTransition::Pop),

        // System
        UiAction::NewGame => {
            stack.apply(PhaseTransition::Replace(GamePhase::StarterSelection));
        }
        UiAction::SelectStarter(choice) => {
            *state = GameState::new_local_game(&choice, &game_data.traits.traits);
            let _ = auto_save.force_save(state);
            stack.apply(PhaseTransition::to_laboratory());
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
            #[cfg(not(target_arch = "wasm32"))]
            std::process::exit(0);
            #[cfg(target_arch = "wasm32")]
            stack.apply(PhaseTransition::to_menu());
        }

        // Kaiju Interaction
        UiAction::SelectKaiju(id) => {
            state.selected_kaiju = Some(id);
        }
        UiAction::ViewKaijuDetails(id) => {
            stack.apply(PhaseTransition::show_kaiju_detail(id));
        }
        UiAction::TrainKaiju { kaiju_id, focus } => {
            train_kaiju(state, kaiju_id, focus, game_data, auto_save);
        }

        // Breeding
        UiAction::ConfirmBreeding => {
            breed_selected_kaiju(state, breeding_state, game_data, auto_save);
            *breeding_state = BreedingState::default();
            stack.apply(PhaseTransition::to_laboratory());
        }

        UiAction::StartBattle(kaiju_id) => {
            if run_local_battle(state, kaiju_id, game_data, auto_save) {
                stack.apply(PhaseTransition::to_results());
            }
        }
        UiAction::GoToMarketplace => stack.apply(PhaseTransition::Replace(GamePhase::Marketplace)),

        // Catch-all
        _ => {
            println!("Action not yet handled: {:?}", action);
        }
    }
}

fn train_kaiju(
    state: &mut GameState,
    kaiju_id: uuid::Uuid,
    focus: TrainingFocus,
    game_data: &GameData,
    auto_save: &mut AutoSaveManager,
) {
    let outcome = mvp::roll_training(focus, next_seed(state), &game_data.balance.mvp.training);

    if state.get_kaiju(kaiju_id).is_none() {
        state.notify(
            "Training target not found.".to_string(),
            NotificationType::Error,
        );
        return;
    }

    if !state.player.spend_gold(outcome.cost) {
        state.notify(
            format!("Training requires {} gold.", outcome.cost),
            NotificationType::Warning,
        );
        return;
    }

    let kaiju = state
        .get_kaiju_mut(kaiju_id)
        .expect("training target was checked before spending");

    match outcome.focus {
        TrainingFocus::Endurance => kaiju.stats.hp += outcome.stat_gain * 5,
        TrainingFocus::Power => kaiju.stats.attack += outcome.stat_gain,
        TrainingFocus::Guard => kaiju.stats.defense += outcome.stat_gain,
        TrainingFocus::Reflex => kaiju.stats.speed += outcome.stat_gain,
    }
    kaiju.experience += outcome.xp_gain;
    let kaiju_name = kaiju.name.clone();

    state.player.add_experience(outcome.xp_gain as u64);
    state.notify(
        format!(
            "{} completed {} training: +{} {}, +{} XP.",
            kaiju_name,
            outcome.focus.label(),
            if matches!(outcome.focus, TrainingFocus::Endurance) {
                outcome.stat_gain * 5
            } else {
                outcome.stat_gain
            },
            outcome.focus.stat_label(),
            outcome.xp_gain
        ),
        NotificationType::Success,
    );
    let _ = auto_save.force_save(state);
}

fn breed_selected_kaiju(
    state: &mut GameState,
    breeding_state: &BreedingState,
    game_data: &GameData,
    auto_save: &mut AutoSaveManager,
) {
    let (Some(id_a), Some(id_b)) = (breeding_state.parent_a, breeding_state.parent_b) else {
        state.notify(
            "Select two parents before breeding.".to_string(),
            NotificationType::Warning,
        );
        return;
    };

    let Some(parent_a) = state.get_kaiju(id_a).cloned() else {
        state.notify("Parent A not found.".to_string(), NotificationType::Error);
        return;
    };
    let Some(parent_b) = state.get_kaiju(id_b).cloned() else {
        state.notify("Parent B not found.".to_string(), NotificationType::Error);
        return;
    };

    let cost = game_data.balance.mvp.breeding.cost;
    if !state.player.spend_gold(cost) {
        state.notify(
            format!("Breeding requires {} gold.", cost),
            NotificationType::Warning,
        );
        return;
    }

    let config = mvp::breeding_config_from_balance(&game_data.balance.breeding);
    match breed_kaiju(&parent_a, &parent_b, next_seed(state), &config) {
        Ok(result) => {
            let token_id = state.next_token_id();
            let mut offspring = result.offspring;
            offspring.token_id = token_id;
            offspring.parent_ids = Some((parent_a.token_id, parent_b.token_id));
            offspring.name = mvp::offspring_name(&parent_a, &parent_b, token_id);
            offspring.current_owner = state.player.player_id.clone();
            offspring.original_breeder = state.player.player_id.clone();
            offspring.image_uri = Some(mvp::image_for_kaiju(&offspring));

            state.player.stats.total_kaiju_bred += 1;
            state.player.stats.highest_generation = state
                .player
                .stats
                .highest_generation
                .max(offspring.generation);
            let offspring_name = offspring.name.clone();
            state.add_kaiju(offspring);
            state.notify(
                format!(
                    "{} hatched from {} and {}.",
                    offspring_name, parent_a.name, parent_b.name
                ),
                NotificationType::Success,
            );
            let _ = auto_save.force_save(state);
        }
        Err(err) => {
            state.player.add_gold(cost);
            state.notify(format!("Breeding failed: {}", err), NotificationType::Error);
        }
    }
}

fn run_local_battle(
    state: &mut GameState,
    kaiju_id: uuid::Uuid,
    game_data: &GameData,
    auto_save: &mut AutoSaveManager,
) -> bool {
    let entry_fee = game_data.balance.mvp.battle.entry_fee;
    if !state.player.spend_gold(entry_fee) {
        state.notify(
            format!("Arena entry requires {} gold.", entry_fee),
            NotificationType::Warning,
        );
        return false;
    }

    let Some(player_kaiju) = state.get_kaiju(kaiju_id).cloned() else {
        state.player.add_gold(entry_fee);
        state.notify("Fighter not found.".to_string(), NotificationType::Error);
        return false;
    };

    let seed = next_seed(state);
    let environment = environment_for_seed(seed);
    let opponent = mvp::generate_opponent(&player_kaiju, seed ^ 0xA51CE, &game_data.traits.traits);
    let simulator =
        BattleSimulator::new(mvp::combat_config_from_balance(&game_data.balance.combat));
    let result = simulator.simulate(player_kaiju.clone(), opponent.clone(), environment, seed);
    let won = result.winner == player_kaiju.name;

    let gold_reward = if won {
        game_data.balance.mvp.battle.win_gold
    } else {
        game_data.balance.mvp.battle.loss_gold
    };
    let xp_reward = if won {
        game_data.balance.mvp.battle.win_xp
    } else {
        game_data.balance.mvp.battle.loss_xp
    };

    state.player.add_gold(gold_reward);
    state.player.add_experience(xp_reward as u64);
    state.player.stats.total_battles_fought += 1;
    if won {
        state.player.stats.total_battles_won += 1;
    }

    if let Some(kaiju) = state.get_kaiju_mut(kaiju_id) {
        kaiju.experience += xp_reward;
        if won {
            kaiju.tournaments_won += 1;
        }
    }

    state.last_battle = Some(LastBattleReport {
        player_kaiju_id: kaiju_id,
        opponent,
        result,
        won,
        gold_reward,
        xp_reward,
    });

    let message = if won {
        format!("{} won the arena bout.", player_kaiju.name)
    } else {
        format!("{} lost the arena bout.", player_kaiju.name)
    };
    state.notify(
        message,
        if won {
            NotificationType::Success
        } else {
            NotificationType::Warning
        },
    );
    let _ = auto_save.force_save(state);
    true
}

fn environment_for_seed(seed: u64) -> Environment {
    let environments = Environment::all();
    environments[(seed as usize) % environments.len()].clone()
}

fn next_seed(state: &GameState) -> u64 {
    ((macroquad::rand::rand() as u64) << 32) ^ state.game_time.total_ticks
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
