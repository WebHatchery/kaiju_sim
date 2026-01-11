use macroquad::prelude::*;

mod data;
mod engine;
mod screens;
mod state;
mod ui;

use data::GameData;

fn window_conf() -> Conf {
    Conf {
        window_title: "Kaiju Breeding Simulator".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Load game data
    let game_data = match GameData::load() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load game data: {}", e);
            eprintln!(
                "Make sure assets/ folder contains traits.json, balance.json, and tournaments.json"
            );
            return;
        }
    };

    println!("Loaded {} traits", game_data.traits.traits.len());
    println!(
        "Loaded {} tournaments",
        game_data.tournaments.tournaments.len()
    );

    // Main game loop
    loop {
        clear_background(Color::from_rgba(20, 20, 25, 255));

        // Draw loading message
        let text = "Phase 1 Complete - Data Models Loaded";
        let font_size = 40.0;
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        let x = screen_width() / 2.0 - text_size.width / 2.0;
        let y = screen_height() / 2.0;

        draw_text(text, x, y, font_size, WHITE);

        // Draw trait count
        let stats_text = format!(
            "Traits: {} | Tournaments: {}",
            game_data.traits.traits.len(),
            game_data.tournaments.tournaments.len()
        );
        let stats_size = measure_text(&stats_text, None, 20, 1.0);
        let stats_x = screen_width() / 2.0 - stats_size.width / 2.0;
        draw_text(&stats_text, stats_x, y + 50.0, 20.0, LIGHTGRAY);

        // Draw instructions
        let instructions = "Press ESC to exit";
        let instr_size = measure_text(instructions, None, 16, 1.0);
        let instr_x = screen_width() / 2.0 - instr_size.width / 2.0;
        draw_text(instructions, instr_x, y + 100.0, 16.0, GRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
