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
    draw_text(&kaiju.name, 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);
    
    // Back button
    if draw_back_button(sw - 100.0, 15.0) {
        return Some(UiAction::Back);
    }
    
    // Layout: Left column (Image + Basic Info), Right column (Stats + Traits + Lineage)
    let left_w = sw * 0.4;
    let right_w = sw - left_w - SPACING_LARGE * 3.0;
    
    let content_y = 80.0;
    
    // IMAGE CARD
    draw_rectangle(SPACING_LARGE, content_y, left_w, 400.0, dark::SURFACE);
    draw_rectangle_lines(SPACING_LARGE, content_y, left_w, 400.0, 2.0, dark::BORDER);
    
    // Big image drawing
    let img_area_h = 300.0;
    let img_area_w = left_w - 20.0;
    draw_rectangle(SPACING_LARGE + 10.0, content_y + 10.0, img_area_w, img_area_h, dark::PANEL);
    
    if let Some(uri) = &kaiju.image_uri {
        if let Some(path_str) = std::path::Path::new(uri).file_name() {
             let key = path_str.to_string_lossy();
             if let Some(tex) = assets.get_texture(&key) {
                // Keep aspect ratio
                let aspect = tex.width() / tex.height();
                let draw_w = img_area_h * aspect; // Fit height
                let draw_x = SPACING_LARGE + 10.0 + (img_area_w - draw_w) / 2.0;

                draw_texture_ex(
                    tex,
                    draw_x,
                    content_y + 10.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(draw_w, img_area_h)),
                        ..Default::default()
                    },
                );
             }
        }
    }
    
    // Basic Info below image
    let info_y = content_y + img_area_h + 30.0;
    draw_text(&format!("Generation: {}", kaiju.generation), SPACING_LARGE + 20.0, info_y, FONT_MEDIUM, dark::TEXT_SECONDARY);
    draw_text(&format!("Genome: {}", &kaiju.genome_hash[0..8]), SPACING_LARGE + 20.0, info_y + 30.0, FONT_SMALL, dark::TEXT_MUTED);

    // RIGHT COLUMN - STATS
    let right_x = SPACING_LARGE * 2.0 + left_w;
    
    draw_text("Combat Stats", right_x, content_y, FONT_MEDIUM, dark::TEXT_PRIMARY);
    
    let stats_y = content_y + 40.0;
    draw_stat_row(right_x, stats_y, "HP", kaiju.stats.hp, 1000, dark::HP_COLOR);
    draw_stat_row(right_x, stats_y + 30.0, "Attack", kaiju.stats.attack, 200, dark::ATK_COLOR);
    draw_stat_row(right_x, stats_y + 60.0, "Defense", kaiju.stats.defense, 200, dark::DEF_COLOR);
    draw_stat_row(right_x, stats_y + 90.0, "Speed", kaiju.stats.speed, 200, dark::SPD_COLOR);
    draw_stat_row(right_x, stats_y + 120.0, "Energy", kaiju.stats.energy, 100, dark::ACCENT);

    // TRAITS
    let traits_y = stats_y + 180.0;
    draw_text("Traits", right_x, traits_y, FONT_MEDIUM, dark::TEXT_PRIMARY);
    
    let mut t_y = traits_y + 40.0;
    if kaiju.traits.is_empty() {
        draw_text("None", right_x, t_y, FONT_NORMAL, dark::TEXT_MUTED);
    } else {
        for t in &kaiju.traits {
            draw_rectangle(right_x, t_y - 20.0, 200.0, 30.0, dark::PANEL);
            draw_text(&t.name, right_x + 10.0, t_y, FONT_NORMAL, dark::TEXT_PRIMARY);
            t_y += 40.0;
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
