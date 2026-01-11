//! Starter Selection Screen
//! Allows the user to choose their starting Kaiju element.

use macroquad::prelude::*;
use crate::ui::*;
use crate::ui::assets::AssetManager;

pub async fn draw_starter_selection(assets: &AssetManager) -> Option<UiAction> {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, dark::BACKGROUND);

    // Title
    draw_text_centered("Choose Your Starter Kaiju", screen_w / 2.0, 80.0, FONT_TITLE, dark::TEXT_PRIMARY);

    let card_w = 260.0; // Wider than normal cards
    let card_h = 380.0;
    let gap = 40.0;
    let total_w = (card_w * 3.0) + (gap * 2.0);
    let start_x = (screen_w - total_w) / 2.0;
    let start_y = 150.0;

    // --- Fire Option ---
    let fire_x = start_x;
    if draw_starter_card(fire_x, start_y, card_w, card_h, "Ignis", "Fire", "High Attack", assets, "kaiju_fire_elemental_1768091138860.png", dark::ATK_COLOR).await {
        return Some(UiAction::SelectStarter("Fire".to_string()));
    }

    // --- Ice Option ---
    let ice_x = fire_x + card_w + gap;
    if draw_starter_card(ice_x, start_y, card_w, card_h, "Glacies", "Ice", "High Defense", assets, "kaiju_ice_elemental_1768091156648.png", dark::DEF_COLOR).await {
        return Some(UiAction::SelectStarter("Ice".to_string()));
    }

    // --- Electric Option ---
    let elec_x = ice_x + card_w + gap;
    if draw_starter_card(elec_x, start_y, card_w, card_h, "Volt", "Electric", "High Speed", assets, "kaiju_electric_elemental_1768091175509.png", dark::SPD_COLOR).await {
        return Some(UiAction::SelectStarter("Electric".to_string()));
    }

    // Back button
    let btn_w = 150.0;
    let btn_h = 40.0;
    let btn_x = screen_w / 2.0 - btn_w / 2.0;
    let btn_y = screen_h - 60.0;
    let mouse = mouse_position();
    let btn_hovered = mouse.0 >= btn_x && mouse.0 <= btn_x + btn_w && mouse.1 >= btn_y && mouse.1 <= btn_y + btn_h;
    
    let btn_bg = if btn_hovered { dark::BUTTON_HOVER } else { dark::BUTTON_BG };
    draw_rectangle(btn_x, btn_y, btn_w, btn_h, btn_bg);
    draw_text_centered("Back", screen_w / 2.0, btn_y + 26.0, FONT_MEDIUM, dark::TEXT_PRIMARY);
    
    if btn_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return Some(UiAction::GoToMenu);
    }

    None
}

async fn draw_starter_card(
    x: f32, 
    y: f32, 
    w: f32, 
    h: f32, 
    name: &str, 
    element: &str, 
    desc: &str,
    assets: &AssetManager,
    image_name: &str,
    color: Color
) -> bool {
    let mouse = mouse_position();
    let is_hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    
    // Card Base
    draw_rectangle(x, y, w, h, dark::SURFACE);
    
    // Border
    let border_color = if is_hovered { color } else { dark::BORDER };
    let border_thick = if is_hovered { 3.0 } else { 2.0 };
    draw_rectangle_lines(x, y, w, h, border_thick, border_color);

    // Image
    let img_h = 200.0;
    draw_rectangle(x + 10.0, y + 10.0, w - 20.0, img_h, dark::PANEL);
    
    // Fetch image from server if needed (lazy load/cache handled by main loop largely, but here we invoke simplistic)
    // Ideally asset manager has them. For visual consistency we assume they exist or use placeholder.
    if let Some(tex) = assets.get_texture(image_name) {
         draw_texture_ex(
            tex,
            x + 10.0,
            y + 10.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w - 20.0, img_h)),
                ..Default::default()
            },
        );
    } else {
        // Trigger download if not present (simple hack for this screen)
        // In real loop, we'd pre-load. Here we just show text if missing.
        draw_text_centered("Loading...", x + w/2.0, y + 100.0, FONT_SMALL, dark::TEXT_SECONDARY);
    }

    // Info
    let content_y = y + img_h + 30.0;
    draw_text_centered(name, x + w/2.0, content_y, FONT_TITLE, color);
    draw_text_centered(element, x + w/2.0, content_y + 30.0, FONT_MEDIUM, dark::TEXT_PRIMARY);
    draw_text_centered(desc, x + w/2.0, content_y + 60.0, FONT_SMALL, dark::TEXT_SECONDARY);

    // Button
    let btn_y = y + h - 50.0;
    let btn_hover = mouse.0 >= x + 20.0 && mouse.0 <= x + w - 20.0 && mouse.1 >= btn_y && mouse.1 <= btn_y + 40.0;
    let btn_color = if btn_hover { color } else { dark::BUTTON_BG };
    
    draw_rectangle(x + 20.0, btn_y, w - 40.0, 40.0, btn_color);
    draw_text_centered("Choose", x + w/2.0, btn_y + 25.0, FONT_MEDIUM, dark::TEXT_PRIMARY);

    if is_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return true;
    }
    
    false
}
