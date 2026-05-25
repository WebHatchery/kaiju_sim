//! Shared dashboard shell and controls used by gameplay screens.

use macroquad::prelude::*;

use crate::data::Kaiju;
use crate::state::{GameState, NotificationType};
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::typography::*;

pub const TOP_BAR_H: f32 = 72.0;
pub const SIDE_BAR_W: f32 = 190.0;
pub const PANEL_GAP: f32 = 14.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppSection {
    Laboratory,
    Roster,
    Training,
    Arena,
    Breeding,
    Leaderboard,
    Detail,
    Results,
    Marketplace,
}

pub struct ScreenFrame {
    pub content: Rect,
    pub nav_action: Option<UiAction>,
}

pub fn draw_app_shell(state: &GameState, active: AppSection) -> ScreenFrame {
    let sw = screen_width();
    let sh = screen_height();
    draw_background(sw, sh);
    draw_top_bar(state, active, sw);
    let nav_action = draw_sidebar(active, sh);

    ScreenFrame {
        content: Rect::new(
            SIDE_BAR_W + PANEL_GAP,
            TOP_BAR_H + PANEL_GAP,
            sw - SIDE_BAR_W - PANEL_GAP * 2.0,
            sh - TOP_BAR_H - PANEL_GAP * 2.0,
        ),
        nav_action,
    }
}

pub fn draw_background(sw: f32, sh: f32) {
    clear_background(dark::BACKGROUND);
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.03, 0.06, 0.09, 1.0));
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.12));

    let grid = 32.0;
    let line = Color::new(0.10, 0.20, 0.29, 0.18);
    let mut x = 0.0;
    while x < sw {
        draw_line(x, TOP_BAR_H, x, sh, 1.0, line);
        x += grid;
    }
    let mut y = TOP_BAR_H;
    while y < sh {
        draw_line(0.0, y, sw, y, 1.0, line);
        y += grid;
    }
}

pub fn draw_panel(rect: Rect, title: &str) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.045, 0.070, 0.095, 0.92),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        1.0,
        Color::new(0.24, 0.55, 0.92, 0.50),
    );
    if !title.is_empty() {
        draw_text(
            title,
            rect.x + 12.0,
            rect.y + 25.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
    }
}

pub fn draw_button(rect: Rect, label: &str, accent: Color, enabled: bool) -> bool {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1)) && enabled;
    let bg = if !enabled {
        Color::new(0.07, 0.08, 0.10, 0.70)
    } else if hovered {
        Color::new(accent.r, accent.g, accent.b, 0.65)
    } else {
        Color::new(0.08, 0.11, 0.15, 0.86)
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if hovered { accent } else { dark::BORDER },
    );
    draw_text_centered(
        label,
        rect.x + rect.w / 2.0,
        rect.y + rect.h / 2.0 + 6.0,
        FONT_SMALL,
        if enabled {
            dark::TEXT_PRIMARY
        } else {
            dark::TEXT_MUTED
        },
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

pub fn draw_stat_meter(x: f32, y: f32, w: f32, label: &str, value: i32, max: i32, color: Color) {
    draw_text(label, x, y, FONT_TINY, dark::TEXT_SECONDARY);
    let bar_x = x + 42.0;
    let bar_y = y - 7.0;
    let bar_w = (w - 92.0).max(40.0);
    draw_rectangle(bar_x, bar_y, bar_w, 6.0, Color::new(0.12, 0.16, 0.20, 1.0));
    draw_rectangle(
        bar_x,
        bar_y,
        bar_w * (value as f32 / max as f32).clamp(0.0, 1.0),
        6.0,
        color,
    );
    draw_text_right(&value.to_string(), x + w, y, FONT_TINY, dark::TEXT_PRIMARY);
}

pub fn draw_portrait(rect: Rect, kaiju: &Kaiju, assets: &AssetManager) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.03, 0.05, 0.07, 1.0),
    );
    let mut drawn = false;
    if let Some(uri) = &kaiju.image_uri {
        let key = assets.get_filename_from_url(uri);
        if let Some(tex) = assets.get_texture(&key) {
            draw_texture_ex(
                tex,
                rect.x,
                rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rect.w, rect.h)),
                    ..Default::default()
                },
            );
            drawn = true;
        }
    }
    if !drawn {
        draw_text_centered(
            "?",
            rect.x + rect.w / 2.0,
            rect.y + rect.h / 2.0 + 10.0,
            FONT_LARGE,
            dark::TEXT_MUTED,
        );
    }
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER);
}

pub fn draw_notification_rows(state: &GameState, rect: Rect) {
    draw_panel(rect, "RECENT ACTIVITY");
    let mut y = rect.y + 48.0;
    for notification in state.notifications.iter().rev().take(5) {
        if y + 8.0 > rect.y + rect.h {
            break;
        }
        let color = notification_color(&notification.notification_type);
        draw_circle(rect.x + 16.0, y - 5.0, 4.0, color);
        draw_text(
            &notification.message,
            rect.x + 30.0,
            y,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
        y += 24.0;
    }
}

pub fn draw_table_header(x: f32, y: f32, w: f32, labels: &[(&str, f32)]) {
    draw_rectangle(x, y, w, 28.0, Color::new(0.06, 0.09, 0.12, 0.95));
    for (label, offset) in labels {
        draw_text(
            label,
            x + *offset,
            y + 19.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    }
}

fn draw_top_bar(state: &GameState, active: AppSection, sw: f32) {
    draw_rectangle(
        0.0,
        0.0,
        sw,
        TOP_BAR_H,
        Color::new(0.025, 0.035, 0.045, 0.98),
    );
    draw_line(0.0, TOP_BAR_H, sw, TOP_BAR_H, 1.0, dark::BORDER);
    draw_text(
        "KAIJU BREEDING SIMULATOR",
        20.0,
        30.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
    draw_text(section_label(active), 20.0, 55.0, FONT_MEDIUM, dark::ACCENT);
    draw_text_right(
        &format!(
            "Gold: {}  |  Roster: {} / 50",
            state.player.gold,
            state.roster.len()
        ),
        sw - 44.0,
        36.0,
        FONT_SMALL,
        dark::TEXT_PRIMARY,
    );
    draw_rectangle_lines(sw - 36.0, 18.0, 20.0, 20.0, 1.0, dark::TEXT_SECONDARY);
}

fn draw_sidebar(active: AppSection, sh: f32) -> Option<UiAction> {
    draw_rectangle(
        0.0,
        TOP_BAR_H,
        SIDE_BAR_W,
        sh - TOP_BAR_H,
        Color::new(0.035, 0.055, 0.075, 0.96),
    );
    draw_line(SIDE_BAR_W, TOP_BAR_H, SIDE_BAR_W, sh, 1.0, dark::BORDER);

    let nav = [
        (
            "LABORATORY",
            AppSection::Laboratory,
            UiAction::GoToLaboratory,
        ),
        ("ROSTER", AppSection::Roster, UiAction::GoToRoster),
        ("TRAINING", AppSection::Training, UiAction::GoToTraining),
        ("ARENA", AppSection::Arena, UiAction::GoToTournament),
        ("BREEDING", AppSection::Breeding, UiAction::GoToBreeding),
        (
            "LEADERBOARD",
            AppSection::Leaderboard,
            UiAction::GoToLeaderboard,
        ),
        ("MENU", AppSection::Marketplace, UiAction::GoToMenu),
    ];

    let mut action = None;
    let mut y = TOP_BAR_H + 28.0;
    for (label, section, ui_action) in nav {
        if draw_nav_item(0.0, y, label, active == section) {
            action = Some(ui_action);
        }
        y += 48.0;
    }

    action
}

fn draw_nav_item(x: f32, y: f32, label: &str, selected: bool) -> bool {
    let rect = Rect::new(x, y, SIDE_BAR_W, 38.0);
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    if selected {
        draw_rectangle(x, y, 3.0, rect.h, dark::ACCENT);
        draw_rectangle(x, y, rect.w, rect.h, Color::new(0.08, 0.16, 0.24, 0.70));
    } else if hovered {
        draw_rectangle(x, y, rect.w, rect.h, Color::new(0.07, 0.10, 0.13, 0.85));
    }

    draw_text(
        nav_icon(label),
        x + 22.0,
        y + 25.0,
        FONT_SMALL,
        if selected {
            dark::ACCENT
        } else {
            dark::TEXT_SECONDARY
        },
    );
    draw_text(
        label,
        x + 64.0,
        y + 25.0,
        FONT_SMALL,
        if selected {
            dark::TEXT_PRIMARY
        } else {
            dark::TEXT_SECONDARY
        },
    );

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn nav_icon(label: &str) -> &'static str {
    match label {
        "LABORATORY" => "LAB",
        "ROSTER" => "RST",
        "TRAINING" => "TRN",
        "ARENA" => "ARN",
        "BREEDING" => "BRD",
        "LEADERBOARD" => "LDR",
        _ => "MNU",
    }
}

fn section_label(active: AppSection) -> &'static str {
    match active {
        AppSection::Laboratory => "LABORATORY",
        AppSection::Roster => "ROSTER",
        AppSection::Training => "TRAINING",
        AppSection::Arena => "ARENA",
        AppSection::Breeding => "BREEDING",
        AppSection::Leaderboard => "LEADERBOARD",
        AppSection::Detail => "KAIJU RECORD",
        AppSection::Results => "BATTLE RESULT",
        AppSection::Marketplace => "MARKETPLACE",
    }
}

fn notification_color(kind: &NotificationType) -> Color {
    match kind {
        NotificationType::Info => dark::ACCENT,
        NotificationType::Success => dark::POSITIVE,
        NotificationType::Warning => dark::WARNING,
        NotificationType::Error => dark::NEGATIVE,
    }
}
