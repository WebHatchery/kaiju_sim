//! Laboratory dashboard screen.

use crate::state::GameState;
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::components::{draw_kaiju_card, CardAction, CardState};
use crate::ui::shell::*;
use crate::ui::spacing::*;
use crate::ui::typography::*;
use macroquad::prelude::*;

pub fn draw_laboratory(state: &GameState, assets: &AssetManager) -> Option<UiAction> {
    let frame = draw_app_shell(state, AppSection::Laboratory);
    if frame.nav_action.is_some() {
        return frame.nav_action;
    }

    let c = frame.content;
    let right_w = 255.0;
    let main_w = c.w - right_w - PANEL_GAP;
    let top_h = 380.0;
    let queue_h = 132.0;
    let bottom_h = (c.h - top_h - queue_h - PANEL_GAP * 2.0).max(0.0);

    let active_panel = Rect::new(c.x, c.y, main_w, top_h);
    draw_panel(active_panel, "ACTIVE KAIJU");
    if let Some(action) = draw_active_kaiju(state, assets, active_panel) {
        return Some(action);
    }

    let status_panel = Rect::new(c.x + main_w + PANEL_GAP, c.y, right_w, top_h);
    draw_facility_status(status_panel);

    let train_panel = Rect::new(c.x, c.y + top_h + PANEL_GAP, main_w * 0.48, queue_h);
    draw_training_summary(state, train_panel);

    let breed_panel = Rect::new(
        train_panel.x + train_panel.w + PANEL_GAP,
        train_panel.y,
        main_w - train_panel.w - PANEL_GAP,
        queue_h,
    );
    draw_breeding_summary(state, breed_panel);

    let activity_panel = Rect::new(
        c.x,
        train_panel.y + queue_h + PANEL_GAP,
        main_w * 0.48,
        bottom_h,
    );
    draw_notification_rows(state, activity_panel);

    let news_panel = Rect::new(
        activity_panel.x + activity_panel.w + PANEL_GAP,
        activity_panel.y,
        main_w - activity_panel.w - PANEL_GAP,
        bottom_h,
    );
    draw_news_feed(news_panel);

    let top_panel = Rect::new(
        c.x + main_w + PANEL_GAP,
        status_panel.y + top_h + PANEL_GAP,
        right_w,
        c.h - top_h - PANEL_GAP,
    );
    draw_top_kaiju(state, top_panel);

    None
}

fn draw_active_kaiju(state: &GameState, assets: &AssetManager, panel: Rect) -> Option<UiAction> {
    let card_gap = 14.0;
    for (index, kaiju) in state.roster.iter().take(3).enumerate() {
        let x = panel.x + 14.0 + index as f32 * (CARD_WIDTH + card_gap);
        let y = panel.y + 38.0;
        let action = draw_kaiju_card(x, y, kaiju, CardState::Normal, assets);
        if let Some(card_action) = action {
            return Some(match card_action {
                CardAction::Select => UiAction::SelectKaiju(kaiju.id),
                CardAction::ViewDetails => UiAction::ViewKaijuDetails(kaiju.id),
            });
        }
    }
    None
}

fn draw_facility_status(rect: Rect) {
    draw_panel(rect, "FACILITY STATUS");
    let facilities = [
        ("Hatchery", 3, 0.72, dark::ACCENT),
        ("Training Center", 3, 0.68, dark::POSITIVE),
        ("Research Lab", 2, 0.56, dark::WARNING),
        ("Recovery Center", 2, 0.46, dark::POSITIVE),
        ("Record Archive", 1, 0.30, dark::ACCENT),
    ];

    let mut y = rect.y + 58.0;
    for (name, level, pct, color) in facilities {
        draw_text(name, rect.x + 14.0, y, FONT_SMALL, dark::TEXT_SECONDARY);
        draw_text_right(
            &format!("LVL {}", level),
            rect.x + rect.w - 14.0,
            y,
            FONT_TINY,
            dark::TEXT_PRIMARY,
        );
        draw_rectangle(
            rect.x + 14.0,
            y + 8.0,
            rect.w - 28.0,
            4.0,
            Color::new(0.12, 0.16, 0.20, 1.0),
        );
        draw_rectangle(rect.x + 14.0, y + 8.0, (rect.w - 28.0) * pct, 4.0, color);
        y += 44.0;
    }
}

fn draw_training_summary(state: &GameState, rect: Rect) {
    draw_panel(rect, "TRAINING QUEUE");
    let mut y = rect.y + 52.0;
    for kaiju in state.roster.iter().filter(|k| k.alive).take(2) {
        draw_text(&kaiju.name, rect.x + 16.0, y, FONT_SMALL, dark::ACCENT);
        draw_text(
            &format!("LVL {}  XP {}", kaiju.generation + 1, kaiju.experience),
            rect.x + 16.0,
            y + 21.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
        draw_rectangle(
            rect.x + 155.0,
            y - 8.0,
            rect.w - 180.0,
            6.0,
            Color::new(0.12, 0.16, 0.20, 1.0),
        );
        draw_rectangle(
            rect.x + 155.0,
            y - 8.0,
            (rect.w - 180.0) * 0.55,
            6.0,
            dark::POSITIVE,
        );
        y += 56.0;
    }
}

fn draw_breeding_summary(state: &GameState, rect: Rect) {
    draw_panel(rect, "BREEDING RECORDS");
    let bred = state.player.stats.total_kaiju_bred;
    draw_text(
        &format!("OFFSPRING HATCHED: {}", bred),
        rect.x + 16.0,
        rect.y + 58.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_text(
        "Every hatch is recorded in the parents' and offspring's history.",
        rect.x + 16.0,
        rect.y + 86.0,
        FONT_SMALL,
        dark::TEXT_SECONDARY,
    );
    if draw_button(
        Rect::new(rect.x + rect.w - 130.0, rect.y + rect.h - 48.0, 110.0, 30.0),
        "ADD PAIR",
        dark::ACCENT,
        true,
    ) {
        // Navigation is handled by the sidebar; summary buttons are decorative for now.
    }
}

fn draw_news_feed(rect: Rect) {
    draw_panel(rect, "NEWS FEED");
    if rect.h < 58.0 {
        return;
    }
    let story_h = (rect.h - 44.0).min(42.0).max(18.0);
    draw_rectangle(
        rect.x + 14.0,
        rect.y + 32.0,
        rect.w - 28.0,
        story_h,
        Color::new(0.10, 0.12, 0.13, 1.0),
    );
    if rect.h > 72.0 {
        draw_text(
            "FRIDAY COLOSSEUM IS LIVE!",
            rect.x + 24.0,
            rect.y + rect.h - 24.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
    }
}

fn draw_top_kaiju(state: &GameState, rect: Rect) {
    draw_panel(rect, "TOP KAIJU");
    draw_table_header(
        rect.x + 12.0,
        rect.y + 42.0,
        rect.w - 24.0,
        &[("RANK", 8.0), ("NAME", 64.0), ("RATING", 176.0)],
    );

    let mut ranked = state.roster.iter().collect::<Vec<_>>();
    ranked.sort_by_key(|kaiju| std::cmp::Reverse((kaiju.tournaments_won, kaiju.battle_rating())));

    let mut y = rect.y + 92.0;
    for (index, kaiju) in ranked.iter().take(6).enumerate() {
        draw_text(
            &(index + 1).to_string(),
            rect.x + 24.0,
            y,
            FONT_SMALL,
            dark::WARNING,
        );
        draw_text(
            &kaiju.name,
            rect.x + 76.0,
            y,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_text_right(
            &kaiju.battle_rating().to_string(),
            rect.x + rect.w - 22.0,
            y,
            FONT_SMALL,
            dark::WARNING,
        );
        y += 30.0;
    }
}
