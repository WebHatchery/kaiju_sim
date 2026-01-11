//! Breeding screen - parent selection and offspring preview.

use macroquad::prelude::*;
use crate::state::GameState;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use crate::ui::actions::UiAction;
use crate::ui::spacing::*;
use crate::ui::assets::AssetManager;
use crate::ui::components::{draw_kaiju_card, CardState, CardAction};

// ...

// ... imports

/// Breeding UI state
pub struct BreedingState {
    pub parent_a: Option<uuid::Uuid>,
    pub parent_b: Option<uuid::Uuid>,
}

impl Default for BreedingState {
    fn default() -> Self {
        Self {
            parent_a: None,
            parent_b: None,
        }
    }
}

/// Draw breeding screen
pub fn draw_breeding_screen(
    game_state: &GameState, 
    breeding_state: &mut BreedingState,
    locked_kaiju_ids: &std::collections::HashSet<uuid::Uuid>,
    assets: &AssetManager
) -> Option<UiAction> {
    let sw = screen_width();
    let sh = screen_height();
    
    clear_background(dark::BACKGROUND);
    
    // Header
    draw_rectangle(0.0, 0.0, sw, 60.0, dark::SURFACE);
    draw_text("BREEDING CHAMBER", 20.0, 40.0, FONT_LARGE, dark::TEXT_PRIMARY);
    
    // Back button
    if draw_button(sw - 100.0, 15.0, 80.0, 30.0, "< Back") {
        return Some(UiAction::GoToLaboratory);
    }
    
    // Parent selection area (top half)
    let parent_y = 80.0;
    let slot_w = 200.0;
    let slot_h = 280.0;
    
    // Parent A slot
    let parent_a_x = sw / 4.0 - slot_w / 2.0;
    draw_parent_slot(parent_a_x, parent_y, slot_w, slot_h, "Parent A", 
        breeding_state.parent_a.and_then(|id| game_state.get_kaiju(id)));
    
    // Plus sign
    draw_text_centered("+", sw / 2.0, parent_y + slot_h / 2.0, FONT_HERO, dark::TEXT_MUTED);
    
    // Parent B slot
    let parent_b_x = sw * 3.0 / 4.0 - slot_w / 2.0;
    draw_parent_slot(parent_b_x, parent_y, slot_w, slot_h, "Parent B",
        breeding_state.parent_b.and_then(|id| game_state.get_kaiju(id)));
    
    // Offspring preview area
    let preview_y = parent_y + slot_h + 30.0;
    draw_rectangle(sw / 3.0, preview_y, sw / 3.0, 100.0, dark::SURFACE);
    draw_rectangle_lines(sw / 3.0, preview_y, sw / 3.0, 100.0, 2.0, dark::BORDER);
    
    let can_breed = breeding_state.parent_a.is_some() && breeding_state.parent_b.is_some();
    
    if can_breed {
        draw_text_centered("Offspring Preview", sw / 2.0, preview_y + 30.0, FONT_NORMAL, dark::TEXT_PRIMARY);
        draw_text_centered("Generation: Next Gen", sw / 2.0, preview_y + 55.0, FONT_SMALL, dark::TEXT_SECONDARY);
        
        // Breed button
        if draw_button(sw / 2.0 - 60.0, preview_y + 120.0, 120.0, 40.0, "BREED") {
            return Some(UiAction::ConfirmBreeding);
        }
    } else {
        draw_text_centered("Select two parents to breed", sw / 2.0, preview_y + 50.0, FONT_NORMAL, dark::TEXT_MUTED);
    }
    
    // Available kaiju grid (bottom)
    let grid_y = preview_y + 170.0;
    draw_text("Select Parents", 20.0, grid_y, FONT_MEDIUM, dark::TEXT_SECONDARY);
    
    let (cols, start_x) = grid_cols(sw, CARD_WIDTH, SPACING_SMALL);
    let card_y = grid_y + 20.0;
    
    let available: Vec<_> = game_state.roster.iter()
        .filter(|k| k.alive)
        .filter(|k| !locked_kaiju_ids.contains(&k.id)) // Filter out breeding Kaiju
        .filter(|k| Some(k.id) != breeding_state.parent_a && Some(k.id) != breeding_state.parent_b)
        .collect();
    
    for (i, kaiju) in available.iter().enumerate() {
        let (x, y) = grid_position(i, cols, start_x, card_y, CARD_WIDTH, CARD_HEIGHT, SPACING_SMALL);
        
        if y > sh {
            continue;
        }
        
        let action = draw_kaiju_card(x, y, kaiju, CardState::Normal, assets);
        
        let mouse = mouse_position();
        let hovered = mouse.0 >= x && mouse.0 <= x + CARD_WIDTH && mouse.1 >= y && mouse.1 <= y + CARD_HEIGHT;
        let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

        if matches!(action, Some(CardAction::Select)) || (action.is_none() && clicked) {
            if breeding_state.parent_a.is_none() {
                breeding_state.parent_a = Some(kaiju.id);
            } else if breeding_state.parent_b.is_none() {
                breeding_state.parent_b = Some(kaiju.id);
            }
        }
    }
    
    None
}

/// Draw parent selection slot
fn draw_parent_slot(x: f32, y: f32, w: f32, h: f32, label: &str, kaiju: Option<&crate::data::Kaiju>) {
    draw_rectangle(x, y, w, h, dark::SURFACE);
    draw_rectangle_lines(x, y, w, h, 2.0, dark::BORDER);
    
    draw_text_centered(label, x + w / 2.0, y + 20.0, FONT_SMALL, dark::TEXT_SECONDARY);
    
    if let Some(k) = kaiju {
        draw_text_centered(&k.name, x + w / 2.0, y + h / 2.0, FONT_MEDIUM, dark::TEXT_PRIMARY);
        draw_text_centered(&format!("Gen {}", k.generation), x + w / 2.0, y + h / 2.0 + 25.0, FONT_SMALL, dark::TEXT_SECONDARY);
    } else {
        draw_text_centered("Click to select", x + w / 2.0, y + h / 2.0, FONT_NORMAL, dark::TEXT_MUTED);
    }
}

/// Draw a button
fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str) -> bool {
    let mouse = mouse_position();
    let hovered = mouse.0 >= x && mouse.0 <= x + w && mouse.1 >= y && mouse.1 <= y + h;
    
    let bg = if hovered { dark::ACCENT } else { dark::BUTTON_BG };
    draw_rectangle(x, y, w, h, bg);
    draw_text_centered(text, x + w / 2.0, y + h / 2.0 + 5.0, FONT_NORMAL, dark::TEXT_PRIMARY);
    
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
