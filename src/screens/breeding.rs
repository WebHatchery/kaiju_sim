//! Breeding screen - parent selection and offspring preview.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::components::{draw_kaiju_card, CardAction, CardState};
use crate::ui::shell::*;
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

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
    let top_h = 250.0;
    let select_panel = Rect::new(c.x, c.y, c.w, top_h);
    draw_panel(select_panel, "BREEDING CHAMBER");

    let parent_w = 280.0;
    let parent_h = 156.0;
    let a_rect = Rect::new(
        select_panel.x + 20.0,
        select_panel.y + 58.0,
        parent_w,
        parent_h,
    );
    let b_rect = Rect::new(
        select_panel.x + parent_w + 72.0,
        select_panel.y + 58.0,
        parent_w,
        parent_h,
    );
    let preview_rect = Rect::new(
        b_rect.x + parent_w + 28.0,
        select_panel.y + 58.0,
        select_panel.w - b_rect.x - parent_w - 48.0,
        parent_h,
    );

    draw_parent_slot(
        a_rect,
        "PARENT A",
        breeding_state
            .parent_a
            .and_then(|id| game_state.get_kaiju(id)),
        assets,
    );
    draw_parent_slot(
        b_rect,
        "PARENT B",
        breeding_state
            .parent_b
            .and_then(|id| game_state.get_kaiju(id)),
        assets,
    );
    draw_offspring_preview(preview_rect, breeding_state, game_state);

    if breeding_state.parent_a.is_some()
        && breeding_state.parent_b.is_some()
        && draw_button(
            Rect::new(
                preview_rect.x + preview_rect.w - 120.0,
                preview_rect.y + preview_rect.h - 42.0,
                100.0,
                30.0,
            ),
            "BREED",
            dark::ACCENT,
            true,
        )
    {
        return Some(UiAction::ConfirmBreeding);
    }

    let roster_panel = Rect::new(c.x, c.y + top_h + PANEL_GAP, c.w, c.h - top_h - PANEL_GAP);
    draw_panel(roster_panel, "SELECT PARENTS");
    draw_text(
        "Choose two living kaiju. Parent and offspring history entries are created after hatching.",
        roster_panel.x + 14.0,
        roster_panel.y + 52.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );

    let available: Vec<_> = game_state
        .roster
        .iter()
        .filter(|k| k.alive)
        .filter(|k| !locked_kaiju_ids.contains(&k.id))
        .filter(|k| Some(k.id) != breeding_state.parent_a && Some(k.id) != breeding_state.parent_b)
        .collect();

    let cols = ((roster_panel.w - 28.0 + PANEL_GAP) / (CARD_WIDTH + PANEL_GAP))
        .floor()
        .max(1.0) as usize;
    let start_x = roster_panel.x + 14.0;
    let start_y = roster_panel.y + 74.0;
    for (i, kaiju) in available.iter().enumerate() {
        let x = start_x + (i % cols) as f32 * (CARD_WIDTH + PANEL_GAP);
        let y = start_y + (i / cols) as f32 * (CARD_HEIGHT + PANEL_GAP);
        if y > roster_panel.y + roster_panel.h {
            continue;
        }
        let action = draw_kaiju_card(x, y, kaiju, CardState::Normal, assets);
        let mouse = mouse_position();
        let clicked = Rect::new(x, y, CARD_WIDTH, CARD_HEIGHT).contains(vec2(mouse.0, mouse.1))
            && is_mouse_button_pressed(MouseButton::Left);
        if matches!(action, Some(CardAction::Select)) || clicked {
            if breeding_state.parent_a.is_none() {
                breeding_state.parent_a = Some(kaiju.id);
            } else if breeding_state.parent_b.is_none() {
                breeding_state.parent_b = Some(kaiju.id);
            }
        }
    }

    None
}

fn draw_parent_slot(
    rect: Rect,
    label: &str,
    kaiju: Option<&crate::data::Kaiju>,
    assets: &AssetManager,
) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.060, 0.082, 0.94),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER);
    draw_text(
        label,
        rect.x + 12.0,
        rect.y + 24.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );
    if let Some(k) = kaiju {
        draw_portrait(
            Rect::new(rect.x + 12.0, rect.y + 40.0, 88.0, 88.0),
            k,
            assets,
        );
        draw_text(
            &k.name,
            rect.x + 116.0,
            rect.y + 68.0,
            FONT_MEDIUM,
            dark::ACCENT,
        );
        draw_text(
            &format!("GEN {} | RATING {}", k.generation, k.battle_rating()),
            rect.x + 116.0,
            rect.y + 96.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    } else {
        draw_text(
            "Select from roster below",
            rect.x + 16.0,
            rect.y + 90.0,
            FONT_SMALL,
            dark::TEXT_MUTED,
        );
    }
}

fn draw_offspring_preview(rect: Rect, breeding_state: &BreedingState, game_state: &GameState) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.060, 0.082, 0.94),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER);
    draw_text(
        "OFFSPRING PREVIEW",
        rect.x + 12.0,
        rect.y + 24.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    if let (Some(a), Some(b)) = (
        breeding_state
            .parent_a
            .and_then(|id| game_state.get_kaiju(id)),
        breeding_state
            .parent_b
            .and_then(|id| game_state.get_kaiju(id)),
    ) {
        draw_text(
            &format!("GEN {}", a.generation.max(b.generation) + 1),
            rect.x + 16.0,
            rect.y + 62.0,
            FONT_MEDIUM,
            dark::ACCENT,
        );
        draw_text(
            &format!("{} x {}", a.name, b.name),
            rect.x + 16.0,
            rect.y + 92.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text(
            "Stats blend with narrow variance. Traits inherit probabilistically.",
            rect.x + 16.0,
            rect.y + 122.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    } else {
        draw_text(
            "Waiting for two parents.",
            rect.x + 16.0,
            rect.y + 82.0,
            FONT_SMALL,
            dark::TEXT_MUTED,
        );
    }
}
