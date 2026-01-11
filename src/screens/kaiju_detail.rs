//! Kaiju detail view screen.

use macroquad::prelude::*;
use crate::state::GameState;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use crate::ui::actions::UiAction;
use crate::ui::spacing::*;
use crate::ui::assets::AssetManager;
use uuid::Uuid;

/// Draw kaiju detail screen
pub fn draw_kaiju_detail(state: &GameState, kaiju_id: Uuid, assets: &AssetManager) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    
    // Find kaiju
    let kaiju = match state.get_kaiju(kaiju_id) {
        Some(k) => k,
        None => return Some(UiAction::Back), // Should not happen
    };

    clear_background(dark::BACKGROUND);
    
    // Header
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text(&kaiju.name, 20.0, 40.0, FONT_LARGE, WHITE);
    
    // Back button
    if draw_back_button(sw - 100.0, 15.0) {
        return Some(UiAction::Back);
    }
    
    // Layout: Left column (Image + Basic Info), Right column (Stats + Traits + Lineage)
    let left_w = sw * 0.4;
    let right_w = sw - left_w - SPACING_LARGE * 3.0;
    
    let content_y = 80.0;
    let image_size = left_w - 40.0; // Square image filling width
    let left_h = image_size + 150.0; // Height for image + info
    
    // LEFT PANEL (Glass style)
    draw_rectangle(SPACING_LARGE, content_y, left_w, left_h, Color::new(0.08, 0.08, 0.1, 0.8));
    draw_rectangle_lines(SPACING_LARGE, content_y, left_w, left_h, 2.0, Color::new(0.3, 0.3, 0.3, 0.3));
    
    // Big Image drawing
    let img_x = SPACING_LARGE + 20.0;
    let img_y = content_y + 20.0;
    
    let mut drawn = false;
    if let Some(uri) = &kaiju.image_uri {
        let key = assets.get_filename_from_url(uri);
        if let Some(tex) = assets.get_texture(&key) {
            draw_texture_ex(
                tex,
                img_x,
                img_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(image_size, image_size)),
                    ..Default::default()
                },
            );
            drawn = true;
        }
    }
    
    if !drawn {
        draw_rectangle(img_x, img_y, image_size, image_size, dark::PANEL);
        draw_text_centered("?", img_x + image_size/2.0, img_y + image_size/2.0, FONT_HERO, GRAY);
    }
    
    // Border for image
    draw_rectangle_lines(img_x, img_y, image_size, image_size, 2.0, dark::ACCENT);
    
    // Basic Info below image
    let info_y = img_y + image_size + 30.0;
    draw_text(&format!("Generation: {}", kaiju.generation), img_x, info_y, FONT_MEDIUM, dark::TEXT_SECONDARY);
    draw_text(&format!("Genome: {}", &kaiju.genome_hash[0..8]), img_x, info_y + 35.0, FONT_SMALL, dark::TEXT_MUTED);
    
    let status_text = if kaiju.alive { "ALIVE" } else { "DECEASED" };
    let status_color = if kaiju.alive { GREEN } else { RED };
    draw_text(status_text, img_x + image_size - 100.0, info_y, FONT_MEDIUM, status_color);

    // RIGHT COLUMN - STATS (Panel)
    let right_x = SPACING_LARGE * 2.0 + left_w;
    
    // Stats Panel
    let stats_h = 300.0;
    draw_rectangle(right_x, content_y, right_w, stats_h, Color::new(0.08, 0.08, 0.1, 0.8));
    draw_rectangle_lines(right_x, content_y, right_w, stats_h, 2.0, Color::new(0.3, 0.3, 0.3, 0.3));
    
    draw_text("Combat Stats", right_x + 20.0, content_y + 35.0, FONT_MEDIUM, WHITE);
    
    let stats_y = content_y + 60.0;
    let stat_gap = 40.0;
    
    draw_stat_row(right_x + 20.0, stats_y, "HP", kaiju.stats.hp, 1000, dark::HP_COLOR);
    draw_stat_row(right_x + 20.0, stats_y + stat_gap, "Attack", kaiju.stats.attack, 200, dark::ATK_COLOR);
    draw_stat_row(right_x + 20.0, stats_y + stat_gap * 2.0, "Defense", kaiju.stats.defense, 200, dark::DEF_COLOR);
    draw_stat_row(right_x + 20.0, stats_y + stat_gap * 3.0, "Speed", kaiju.stats.speed, 200, dark::SPD_COLOR);
    draw_stat_row(right_x + 20.0, stats_y + stat_gap * 4.0, "Energy", kaiju.stats.energy, 100, dark::ACCENT);

    // TRAITS Panel
    let traits_y = content_y + stats_h + SPACING_LARGE;
    let traits_h = sh - traits_y - SPACING_LARGE;
    
    draw_rectangle(right_x, traits_y, right_w, traits_h, Color::new(0.08, 0.08, 0.1, 0.8));
    draw_rectangle_lines(right_x, traits_y, right_w, traits_h, 2.0, Color::new(0.3, 0.3, 0.3, 0.3));
    
    draw_text("Traits", right_x + 20.0, traits_y + 35.0, FONT_MEDIUM, WHITE);
    
    let mut t_y = traits_y + 60.0;
    if kaiju.traits.is_empty() {
        draw_text("No traits detected.", right_x + 20.0, t_y, FONT_NORMAL, dark::TEXT_MUTED);
    } else {
        for t in &kaiju.traits {
            // Trait Pill
            draw_rectangle(right_x + 20.0, t_y - 25.0, 250.0, 35.0, Color::new(0.2, 0.2, 0.25, 1.0));
            draw_rectangle_lines(right_x + 20.0, t_y - 25.0, 250.0, 35.0, 1.0, dark::ACCENT);
            
            draw_text(&t.name, right_x + 35.0, t_y, FONT_NORMAL, WHITE);
            // draw_text(&t.description, right_x + 300.0, t_y, FONT_SMALL, GRAY); // If space permits
            t_y += 50.0;
        }
    }

    None
}

fn draw_stat_row(x: f32, y: f32, label: &str, value: i32, max: i32, color: Color) {
    draw_text(label, x, y, FONT_NORMAL, dark::TEXT_SECONDARY);
    
    let bar_x = x + 100.0;
    let bar_w = 300.0;
    let bar_h = 16.0;
    
    // Bg
    draw_rectangle(bar_x, y - 12.0, bar_w, bar_h, dark::PANEL);
    
    // Fill
    let pct = (value as f32 / max as f32).min(1.0);
    draw_rectangle(bar_x, y - 12.0, bar_w * pct, bar_h, color);
    
    // Text value
    draw_text(&value.to_string(), bar_x + bar_w + 10.0, y, FONT_NORMAL, dark::TEXT_PRIMARY);
}

fn draw_back_button(x: f32, y: f32) -> bool {
    let w = 80.0;
    let h = 30.0;
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    
    let bg = if hovered { dark::BUTTON_HOVER } else { dark::BUTTON_BG };
    draw_rectangle(x, y, w, h, bg);
    draw_text_centered("< Back", x + w / 2.0, y + 20.0, FONT_SMALL, dark::TEXT_PRIMARY);
    
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
