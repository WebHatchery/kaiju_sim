//! Breeding screen - parent selection and offspring preview.

use crate::data::Kaiju;
use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::{dark, trait_color};
use crate::ui::shell::*;
use crate::ui::typography::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

#[derive(Default)]
pub struct BreedingState {
    pub parent_a: Option<uuid::Uuid>,
    pub parent_b: Option<uuid::Uuid>,
}

pub fn draw_breeding_screen(
    game_state: &GameState,
    breeding_state: &mut BreedingState,
    locked_kaiju_ids: &std::collections::HashSet<uuid::Uuid>,
    assets: &AssetManager,
) -> Option<UiAction> {
    let frame = draw_app_shell(game_state, AppSection::Breeding);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let top_h = (c.h * 0.54).clamp(300.0, 344.0);
    let chamber = Rect::new(c.x, c.y, c.w, top_h);
    let roster_panel = Rect::new(c.x, c.y + top_h + PANEL_GAP, c.w, c.h - top_h - PANEL_GAP);

    if let Some(action) = draw_breeding_chamber(chamber, game_state, breeding_state, assets) {
        return Some(action);
    }
    draw_candidate_roster(
        roster_panel,
        game_state,
        breeding_state,
        locked_kaiju_ids,
        assets,
    );

    None
}

fn draw_breeding_chamber(
    rect: Rect,
    game_state: &GameState,
    breeding_state: &BreedingState,
    assets: &AssetManager,
) -> Option<UiAction> {
    draw_panel_with_accent(rect, "EXPERIMENTAL BREEDING CHAMBER", dark::ACCENT);
    draw_ui_text(
        "Pair two living bloodlines, preview inheritance pressure, then start incubation.",
        rect.x + 18.0,
        rect.y + 56.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let slot_w = 250.0;
    let slot_h = rect.h - 100.0;
    let a_rect = Rect::new(rect.x + 18.0, rect.y + 82.0, slot_w, slot_h);
    let b_rect = Rect::new(rect.x + 286.0, rect.y + 82.0, slot_w, slot_h);
    let preview_rect = Rect::new(rect.x + 554.0, rect.y + 82.0, rect.w - 572.0, slot_h);

    let parent_a = breeding_state
        .parent_a
        .and_then(|id| game_state.get_kaiju(id));
    let parent_b = breeding_state
        .parent_b
        .and_then(|id| game_state.get_kaiju(id));
    draw_parent_slot(a_rect, "PARENT A", parent_a, assets, dark::ACCENT);
    draw_parent_slot(b_rect, "PARENT B", parent_b, assets, dark::WARNING);
    draw_offspring_preview(preview_rect, parent_a, parent_b);

    if parent_a.is_some()
        && parent_b.is_some()
        && draw_button(
            Rect::new(rect.x + rect.w - 190.0, rect.y + 42.0, 170.0, 32.0),
            "START INCUBATION",
            dark::ACCENT,
            true,
        )
    {
        return Some(UiAction::ConfirmBreeding);
    }

    None
}

fn draw_parent_slot(
    rect: Rect,
    label: &str,
    kaiju: Option<&Kaiju>,
    assets: &AssetManager,
    accent: Color,
) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.030, 0.055, 0.074, 0.94),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        2.0,
        Color::new(accent.r, accent.g, accent.b, 0.70),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(accent.r, accent.g, accent.b, 0.30),
    );
    draw_ui_text(
        label,
        rect.x + 12.0,
        rect.y + 24.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );

    if let Some(k) = kaiju {
        draw_portrait(
            Rect::new(rect.x + 12.0, rect.y + 42.0, 104.0, 104.0),
            k,
            assets,
        );
        draw_ui_text(
            &ellipsize(&k.name, rect.w - 138.0, FONT_MEDIUM),
            rect.x + 130.0,
            rect.y + 74.0,
            FONT_MEDIUM,
            dark::TEXT_PRIMARY,
        );
        draw_ui_text(
            &format!("GEN {} | R{}", k.generation, k.battle_rating()),
            rect.x + 130.0,
            rect.y + 100.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
        if let Some(trait_def) = k.traits.first() {
            draw_trait_chip(
                rect.x + 130.0,
                rect.y + 118.0,
                &trait_def.name,
                trait_color(&trait_def.category),
            );
        }
        draw_ui_text(
            lineage_note(k),
            rect.x + 12.0,
            rect.y + rect.h - 20.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    } else {
        draw_ui_text(
            "Awaiting specimen assignment",
            rect.x + 16.0,
            rect.y + 94.0,
            FONT_SMALL,
            dark::TEXT_MUTED,
        );
        draw_ui_text(
            "Select from eligible roster below.",
            rect.x + 16.0,
            rect.y + 122.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    }
}

fn draw_offspring_preview(rect: Rect, parent_a: Option<&Kaiju>, parent_b: Option<&Kaiju>) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.030, 0.055, 0.074, 0.94),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        2.0,
        Color::new(dark::ACCENT.r, dark::ACCENT.g, dark::ACCENT.b, 0.70),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(dark::ACCENT.r, dark::ACCENT.g, dark::ACCENT.b, 0.30),
    );
    draw_ui_text(
        "OFFSPRING PROJECTION",
        rect.x + 14.0,
        rect.y + 24.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );

    if let (Some(a), Some(b)) = (parent_a, parent_b) {
        let generation = a.generation.max(b.generation) + 1;
        draw_ui_text(
            &format!("GENERATION {}", generation),
            rect.x + 16.0,
            rect.y + 58.0,
            FONT_MEDIUM,
            dark::ACCENT,
        );
        draw_ui_text(
            &format!("{} x {}", a.name, b.name),
            rect.x + 16.0,
            rect.y + 84.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );

        let stat_x = rect.x + 16.0;
        let stat_y = rect.y + 118.0;
        draw_projection_bar(
            stat_x,
            stat_y,
            rect.w * 0.48,
            "HP",
            (a.stats.hp + b.stats.hp) / 2,
            2500,
            dark::HP_COLOR,
        );
        draw_projection_bar(
            stat_x,
            stat_y + 25.0,
            rect.w * 0.48,
            "ATK",
            (a.stats.attack + b.stats.attack) / 2,
            300,
            dark::ATK_COLOR,
        );
        draw_projection_bar(
            stat_x,
            stat_y + 50.0,
            rect.w * 0.48,
            "DEF",
            (a.stats.defense + b.stats.defense) / 2,
            300,
            dark::DEF_COLOR,
        );
        draw_projection_bar(
            stat_x,
            stat_y + 75.0,
            rect.w * 0.48,
            "SPD",
            (a.stats.speed + b.stats.speed) / 2,
            300,
            dark::SPD_COLOR,
        );

        let right_x = rect.x + rect.w * 0.58;
        draw_ui_text("INHERITANCE", right_x, stat_y, FONT_TINY, dark::TEXT_MUTED);
        draw_status_pill(
            Rect::new(right_x, stat_y + 18.0, 120.0, 26.0),
            mutation_window(a, b),
            dark::WARNING,
        );
        draw_status_pill(
            Rect::new(right_x, stat_y + 54.0, 120.0, 26.0),
            egg_rarity(generation, a, b),
            dark::ACCENT,
        );
        draw_ui_text(
            "Traits may pass forward.",
            right_x,
            stat_y + 100.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    } else {
        draw_empty_state(
            rect,
            "Projection locked",
            "Choose two parents to open the incubator model.",
        );
    }
}

fn draw_candidate_roster(
    rect: Rect,
    game_state: &GameState,
    breeding_state: &mut BreedingState,
    locked_kaiju_ids: &std::collections::HashSet<uuid::Uuid>,
    assets: &AssetManager,
) {
    draw_panel_with_accent(rect, "ELIGIBLE BLOODLINES", dark::TEXT_SECONDARY);
    draw_ui_text(
        "Click a specimen to fill the next open parent slot.",
        rect.x + 16.0,
        rect.y + 52.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let available = game_state
        .roster
        .iter()
        .filter(|k| k.alive)
        .filter(|k| !locked_kaiju_ids.contains(&k.id))
        .filter(|k| Some(k.id) != breeding_state.parent_a && Some(k.id) != breeding_state.parent_b)
        .collect::<Vec<_>>();

    let cols = ((rect.w - 32.0 + PANEL_GAP) / (310.0 + PANEL_GAP))
        .floor()
        .max(1.0) as usize;
    let card_w = ((rect.w - 32.0) - (cols as f32 - 1.0) * PANEL_GAP) / cols as f32;
    let mut selected = None;
    for (index, kaiju) in available.iter().enumerate() {
        let col = index % cols;
        let row = index / cols;
        let card = Rect::new(
            rect.x + 16.0 + col as f32 * (card_w + PANEL_GAP),
            rect.y + 78.0 + row as f32 * 128.0,
            card_w,
            112.0,
        );
        if card.y + card.h > rect.y + rect.h {
            continue;
        }
        if draw_candidate_card(card, kaiju, assets) {
            selected = Some(kaiju.id);
        }
    }

    if let Some(id) = selected {
        if breeding_state.parent_a.is_none() {
            breeding_state.parent_a = Some(id);
        } else if breeding_state.parent_b.is_none() {
            breeding_state.parent_b = Some(id);
        }
    }

    if available.is_empty() {
        draw_empty_state(
            rect,
            "No eligible parents",
            "Living, unlocked kaiju appear here when available.",
        );
    }
}

fn draw_candidate_card(rect: Rect, kaiju: &Kaiju, assets: &AssetManager) -> bool {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.030, 0.055, 0.074, 0.92),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if hovered {
            dark::ACCENT
        } else {
            dark::BORDER_SOFT
        },
    );
    draw_portrait(
        Rect::new(rect.x + 10.0, rect.y + 10.0, 88.0, 88.0),
        kaiju,
        assets,
    );
    draw_ui_text(
        &ellipsize(&kaiju.name, rect.w - 132.0, FONT_MEDIUM),
        rect.x + 112.0,
        rect.y + 36.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "GEN {} | rating {} | events {}",
            kaiju.generation,
            kaiju.battle_rating(),
            kaiju.history.len()
        ),
        rect.x + 112.0,
        rect.y + 62.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    if let Some(trait_def) = kaiju.traits.first() {
        draw_trait_chip(
            rect.x + 112.0,
            rect.y + 76.0,
            &trait_def.name,
            trait_color(&trait_def.category),
        );
    }
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_projection_bar(x: f32, y: f32, w: f32, label: &str, value: i32, max: i32, color: Color) {
    draw_ui_text(label, x, y, FONT_TINY, dark::TEXT_SECONDARY);
    draw_progress_bar(
        Rect::new(x + 42.0, y - 8.0, w - 42.0, 7.0),
        value as f32 / max as f32,
        color,
    );
}

fn lineage_note(kaiju: &Kaiju) -> &'static str {
    if kaiju.parent_ids.is_some() {
        "Bred lineage verified"
    } else {
        "Original source"
    }
}

fn mutation_window(a: &Kaiju, b: &Kaiju) -> &'static str {
    if a.hidden_traits.len() + b.hidden_traits.len() > 0 || a.generation.max(b.generation) >= 2 {
        "ELEVATED"
    } else {
        "STABLE"
    }
}

fn egg_rarity(generation: u32, a: &Kaiju, b: &Kaiju) -> &'static str {
    if generation >= 3 || a.traits.len() + b.traits.len() >= 4 {
        "RARE EGG"
    } else if generation >= 1 {
        "LINEAGE EGG"
    } else {
        "BASE EGG"
    }
}
