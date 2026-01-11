use macroquad::prelude::*;
use crate::data::Kaiju;
use crate::ui::assets::AssetManager;

pub fn draw_kaiju_card(
    x: f32,
    y: f32,
    kaiju: &Kaiju,
    state: CardState,
    assets: &AssetManager,
) -> Option<CardAction> {
    // ...

    // Portrait placeholder
    let portrait_h = 90.0;
    
    let mut drawn = false;
    if let Some(uri) = &kaiju.image_uri {
        if let Some(path_str) = std::path::Path::new(uri).file_name() {
             let key = path_str.to_string_lossy();
             if let Some(tex) = assets.get_texture(&key) {
                draw_texture_ex(
                    *tex,
                    x + 10.0,
                    y + 10.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(CARD_WIDTH - 20.0, portrait_h)),
                        ..Default::default()
                    },
                );
                drawn = true;
             }
        }
    }
    
    if !drawn {
        draw_rectangle(x + 10.0, y + 10.0, CARD_WIDTH - 20.0, portrait_h, dark::PANEL);
    }
    
    // Generation badge
    let gen_text = format!("Gen {}", kaiju.generation);
    draw_rectangle(x + 10.0, y + 10.0, 50.0, 18.0, dark::ACCENT);
    draw_text(&gen_text, x + 14.0, y + 24.0, FONT_TINY, dark::TEXT_PRIMARY);
    
    // Name
    let name_y = y + portrait_h + 25.0;
    draw_text_bold(&kaiju.name, x + 10.0, name_y, FONT_MEDIUM, dark::TEXT_PRIMARY);
    
    // Stat bars
    let stats_y = name_y + 15.0;
    draw_stat_mini(x + 10.0, stats_y, "HP", kaiju.stats.hp, 500, dark::HP_COLOR);
    draw_stat_mini(x + 10.0, stats_y + 18.0, "ATK", kaiju.stats.attack, 100, dark::ATK_COLOR);
    draw_stat_mini(x + 10.0, stats_y + 36.0, "DEF", kaiju.stats.defense, 100, dark::DEF_COLOR);
    draw_stat_mini(x + 10.0, stats_y + 54.0, "SPD", kaiju.stats.speed, 100, dark::SPD_COLOR);
    
    // Status overlay for dead kaiju
    if !kaiju.alive {
        draw_rectangle(x, y, CARD_WIDTH, CARD_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.5));
        draw_text_centered("DECEASED", x + CARD_WIDTH / 2.0, y + CARD_HEIGHT / 2.0, FONT_MEDIUM, dark::DEAD);
    }
    
    // Buttons row at bottom
    let btn_y = y + CARD_HEIGHT - 35.0;
    let btn_w = (CARD_WIDTH - 25.0) / 2.0;
    
    let select_hovered = is_mouse_in_rect(x + 5.0, btn_y, btn_w, 28.0, mouse);
    let view_hovered = is_mouse_in_rect(x + btn_w + 15.0, btn_y, btn_w, 28.0, mouse);
    
    // Select button
    let select_bg = if select_hovered { dark::BUTTON_HOVER } else { dark::BUTTON_BG };
    draw_rectangle(x + 5.0, btn_y, btn_w, 28.0, select_bg);
    draw_text_centered("Select", x + 5.0 + btn_w / 2.0, btn_y + 19.0, FONT_SMALL, dark::TEXT_PRIMARY);
    
    // View button
    let view_bg = if view_hovered { dark::BUTTON_HOVER } else { dark::BUTTON_BG };
    draw_rectangle(x + btn_w + 15.0, btn_y, btn_w, 28.0, view_bg);
    draw_text_centered("View", x + btn_w + 15.0 + btn_w / 2.0, btn_y + 19.0, FONT_SMALL, dark::TEXT_PRIMARY);
    
    // Handle clicks
    if is_mouse_button_pressed(MouseButton::Left) && kaiju.alive {
        if select_hovered {
            return Some(CardAction::Select);
        }
        if view_hovered {
            return Some(CardAction::ViewDetails);
        }
    }
    
    None
}

/// Draw mini stat bar
fn draw_stat_mini(x: f32, y: f32, label: &str, value: i32, max: i32, color: Color) {
    let bar_x = x + 30.0;
    let bar_w = CARD_WIDTH - 50.0;
    let bar_h = 12.0;
    
    draw_text(label, x, y + 10.0, FONT_TINY, dark::TEXT_SECONDARY);
    
    // Background
    draw_rectangle(bar_x, y, bar_w, bar_h, dark::PANEL);
    
    // Fill
    let fill_w = (value as f32 / max as f32).min(1.0) * bar_w;
    draw_rectangle(bar_x, y, fill_w, bar_h, color);
    
    // Value
    draw_text_right(&value.to_string(), x + CARD_WIDTH - 15.0, y + 10.0, FONT_TINY, dark::TEXT_PRIMARY);
}

fn is_mouse_in_rect(x: f32, y: f32, w: f32, h: f32, mouse: (f32, f32)) -> bool {
    mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h
}
