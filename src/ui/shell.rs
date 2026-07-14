//! Shared dashboard shell and controls used by gameplay screens.

use macroquad::prelude::*;

use crate::data::Kaiju;
use crate::state::{GameState, NotificationType};
use crate::ui::actions::UiAction;
use crate::ui::assets::AssetManager;
use crate::ui::colors::dark;
use crate::ui::typography::*;
use macroquad_toolkit::colors::with_alpha;
use macroquad_toolkit::ui::draw_ui_text;

pub const TOP_BAR_H: f32 = 78.0;
pub const SIDE_BAR_W: f32 = 176.0;
pub const PANEL_GAP: f32 = 18.0;

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
    Settings,
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
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.015, 0.026, 0.036, 1.0));
    draw_circle(
        sw * 0.78,
        sh * 0.20,
        sw * 0.32,
        Color::new(0.02, 0.20, 0.28, 0.12),
    );
    draw_circle(
        sw * 0.24,
        sh * 0.88,
        sw * 0.24,
        Color::new(0.08, 0.22, 0.22, 0.10),
    );

    let grid = 36.0;
    let pulse = ((get_time() as f32 * 0.7).sin() + 1.0) * 0.04;
    let line = Color::new(0.10, 0.32, 0.40, 0.10 + pulse);
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

    let mut scan_y = TOP_BAR_H + ((get_time() as f32 * 18.0) % 6.0);
    while scan_y < sh {
        draw_line(
            0.0,
            scan_y,
            sw,
            scan_y,
            1.0,
            Color::new(0.0, 0.0, 0.0, 0.10),
        );
        scan_y += 6.0;
    }
}

pub fn draw_panel(rect: Rect, title: &str) {
    draw_panel_with_accent(rect, title, dark::ACCENT);
}

pub fn draw_panel_with_accent(rect: Rect, title: &str, accent: Color) {
    draw_rectangle(
        rect.x + 5.0,
        rect.y + 7.0,
        rect.w,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.24),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        with_alpha(dark::SURFACE, 0.92),
    );
    draw_rectangle(rect.x, rect.y, rect.w, 2.0, with_alpha(accent, 0.55));
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        with_alpha(dark::BORDER, 0.45),
    );
    draw_corner_brackets(rect, with_alpha(accent, 0.34));

    if !title.is_empty() {
        draw_ui_text(
            title,
            rect.x + 16.0,
            rect.y + 28.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
    }
}

pub fn draw_button(rect: Rect, label: &str, accent: Color, enabled: bool) -> bool {
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1)) && enabled;
    let bg = if !enabled {
        Color::new(0.05, 0.065, 0.075, 0.68)
    } else if hovered {
        with_alpha(accent, 0.34)
    } else {
        Color::new(0.055, 0.090, 0.115, 0.92)
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        1.0,
        if enabled {
            with_alpha(accent, if hovered { 0.90 } else { 0.40 })
        } else {
            dark::BORDER_SOFT
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if hovered {
            accent
        } else {
            with_alpha(dark::BORDER, 0.55)
        },
    );
    draw_text_centered(
        label,
        rect.x + rect.w / 2.0,
        rect.y + rect.h / 2.0 + 5.0,
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
    draw_ui_text(label, x, y, FONT_TINY, dark::TEXT_SECONDARY);
    let bar_x = x + 44.0;
    let bar_y = y - 8.0;
    let bar_w = (w - 104.0).max(40.0);
    draw_progress_bar(
        Rect::new(bar_x, bar_y, bar_w, 7.0),
        value as f32 / max as f32,
        color,
    );
    draw_text_right(&value.to_string(), x + w, y, FONT_TINY, dark::TEXT_PRIMARY);
}

pub fn draw_progress_bar(rect: Rect, percent: f32, color: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.04, 0.06, 0.075, 1.0),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w * percent.clamp(0.0, 1.0),
        rect.h,
        color,
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, with_alpha(color, 0.24));
}

pub fn draw_portrait(rect: Rect, kaiju: &Kaiju, assets: &AssetManager) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.018, 0.032, 0.045, 1.0),
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

    let mut y = rect.y + 4.0;
    while y < rect.y + rect.h {
        draw_line(
            rect.x,
            y,
            rect.x + rect.w,
            y,
            1.0,
            Color::new(0.08, 0.26, 0.32, 0.10),
        );
        y += 8.0;
    }
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, dark::BORDER_SOFT);
    draw_corner_brackets(rect, with_alpha(dark::ACCENT, 0.42));
}

pub fn draw_notification_rows(state: &GameState, rect: Rect) {
    draw_panel(rect, "LIVE FACILITY FEED");
    let mut y = rect.y + 56.0;
    for notification in state.notifications.iter().rev().take(5) {
        if y + 12.0 > rect.y + rect.h {
            break;
        }
        let color = notification_color(&notification.notification_type);
        let pulse = ((get_time() as f32 * 2.2 + y * 0.02).sin() + 1.0) * 0.18;
        draw_circle(rect.x + 18.0, y - 5.0, 5.0 + pulse, with_alpha(color, 0.35));
        draw_circle(rect.x + 18.0, y - 5.0, 2.5, color);
        draw_text_wrapped(
            &notification.message,
            rect.x + 34.0,
            y,
            rect.w - 50.0,
            17.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
            2,
        );
        y += 38.0;
    }

    if state.notifications.is_empty() {
        draw_ui_text(
            "No new alerts.",
            rect.x + 18.0,
            rect.y + 58.0,
            FONT_SMALL,
            dark::TEXT_MUTED,
        );
    }
}

pub fn draw_table_header(x: f32, y: f32, w: f32, labels: &[(&str, f32)]) {
    draw_rectangle(x, y, w, 30.0, Color::new(0.035, 0.075, 0.098, 0.95));
    draw_rectangle(x, y, w, 1.0, with_alpha(dark::ACCENT, 0.35));
    for (label, offset) in labels {
        draw_ui_text(
            label,
            x + *offset,
            y + 20.0,
            FONT_TINY,
            dark::TEXT_SECONDARY,
        );
    }
}

pub fn draw_status_pill(rect: Rect, label: &str, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, with_alpha(color, 0.13));
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, with_alpha(color, 0.56));
    draw_text_centered(
        label,
        rect.x + rect.w / 2.0,
        rect.y + rect.h / 2.0 + 5.0,
        FONT_TINY,
        color,
    );
}

pub fn draw_metric_tile(rect: Rect, label: &str, value: &str, accent: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.065, 0.082, 0.88),
    );
    draw_rectangle(rect.x, rect.y, 2.0, rect.h, with_alpha(accent, 0.75));
    draw_ui_text(
        label,
        rect.x + 12.0,
        rect.y + 22.0,
        FONT_TINY,
        dark::TEXT_MUTED,
    );
    draw_ui_text(
        value,
        rect.x + 12.0,
        rect.y + 50.0,
        FONT_MEDIUM,
        dark::TEXT_PRIMARY,
    );
}

pub fn draw_trait_chip(x: f32, y: f32, label: &str, color: Color) -> f32 {
    let width = (measure_text_size(label, FONT_TINY).0 + 20.0).clamp(72.0, 170.0);
    draw_rectangle(x, y, width, 24.0, with_alpha(color, 0.12));
    draw_rectangle_lines(x, y, width, 24.0, 1.0, with_alpha(color, 0.55));
    draw_ui_text(
        &ellipsize(label, width - 14.0, FONT_TINY),
        x + 8.0,
        y + 16.0,
        FONT_TINY,
        color,
    );
    width
}

pub fn draw_empty_state(rect: Rect, title: &str, body: &str) {
    draw_text_centered(
        title,
        rect.x + rect.w / 2.0,
        rect.y + rect.h * 0.44,
        FONT_MEDIUM,
        dark::TEXT_SECONDARY,
    );
    draw_text_centered(
        body,
        rect.x + rect.w / 2.0,
        rect.y + rect.h * 0.44 + 28.0,
        FONT_SMALL,
        dark::TEXT_MUTED,
    );
}

fn draw_top_bar(state: &GameState, active: AppSection, sw: f32) {
    draw_rectangle(
        0.0,
        0.0,
        sw,
        TOP_BAR_H,
        Color::new(0.014, 0.024, 0.032, 0.98),
    );
    draw_line(
        0.0,
        TOP_BAR_H,
        sw,
        TOP_BAR_H,
        1.0,
        with_alpha(dark::ACCENT, 0.25),
    );
    draw_ui_text("KAIJU SIM", 20.0, 28.0, FONT_MEDIUM, dark::TEXT_PRIMARY);
    draw_ui_text(
        "CLASSIFIED EVOLUTION FACILITY",
        20.0,
        54.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    draw_ui_text(
        section_label(active),
        SIDE_BAR_W + PANEL_GAP,
        34.0,
        FONT_MEDIUM,
        dark::ACCENT,
    );
    draw_ui_text(
        screen_directive(active),
        SIDE_BAR_W + PANEL_GAP,
        58.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    if let Some(subject) = selected_subject(state) {
        let panel_x = sw - 500.0;
        draw_ui_text("ACTIVE SUBJECT", panel_x, 27.0, FONT_TINY, dark::TEXT_MUTED);
        draw_ui_text(
            &ellipsize(&subject.name, 190.0, FONT_SMALL),
            panel_x,
            52.0,
            FONT_SMALL,
            dark::TEXT_PRIMARY,
        );
        draw_status_pill(
            Rect::new(panel_x + 205.0, 29.0, 62.0, 24.0),
            &format!("GEN {}", subject.generation),
            dark::ACCENT,
        );
        draw_text_right(
            &format!(
                "GOLD {}  ROSTER {}/50",
                state.player.gold,
                state.roster.len()
            ),
            sw - 24.0,
            43.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
    } else {
        draw_text_right(
            &format!(
                "GOLD {}  ROSTER {}/50",
                state.player.gold,
                state.roster.len()
            ),
            sw - 24.0,
            43.0,
            FONT_SMALL,
            dark::TEXT_SECONDARY,
        );
    }
}

fn draw_sidebar(active: AppSection, sh: f32) -> Option<UiAction> {
    draw_rectangle(
        0.0,
        TOP_BAR_H,
        SIDE_BAR_W,
        sh - TOP_BAR_H,
        Color::new(0.020, 0.040, 0.055, 0.96),
    );
    draw_line(
        SIDE_BAR_W,
        TOP_BAR_H,
        SIDE_BAR_W,
        sh,
        1.0,
        with_alpha(dark::ACCENT, 0.20),
    );

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
            "RECORDS",
            AppSection::Leaderboard,
            UiAction::GoToLeaderboard,
        ),
        ("SETTINGS", AppSection::Settings, UiAction::GoToSettings),
    ];

    let mut action = None;
    let mut y = TOP_BAR_H + 28.0;
    for (label, section, ui_action) in nav {
        if draw_nav_item(0.0, y, label, active == section) {
            action = Some(ui_action);
        }
        y += 46.0;
    }

    draw_ui_text("SECURE", 22.0, sh - 58.0, FONT_TINY, dark::TEXT_MUTED);
    draw_ui_text(
        "OFFLINE SAVE",
        22.0,
        sh - 34.0,
        FONT_TINY,
        dark::TEXT_SECONDARY,
    );

    action
}

fn draw_nav_item(x: f32, y: f32, label: &str, selected: bool) -> bool {
    let rect = Rect::new(x, y, SIDE_BAR_W, 38.0);
    let mouse = mouse_position();
    let hovered = rect.contains(vec2(mouse.0, mouse.1));
    if selected {
        draw_rectangle(x, y, 3.0, rect.h, dark::ACCENT);
        draw_rectangle(x, y, rect.w, rect.h, Color::new(0.05, 0.18, 0.24, 0.72));
        draw_line(
            x + 18.0,
            y + rect.h - 1.0,
            x + rect.w - 18.0,
            y + rect.h - 1.0,
            1.0,
            with_alpha(dark::ACCENT, 0.28),
        );
    } else if hovered {
        draw_rectangle(x, y, rect.w, rect.h, Color::new(0.05, 0.09, 0.12, 0.88));
    }

    let color = if selected {
        dark::TEXT_PRIMARY
    } else if hovered {
        dark::ACCENT
    } else {
        dark::TEXT_SECONDARY
    };
    draw_ui_text(
        nav_icon(label),
        x + 20.0,
        y + 25.0,
        FONT_TINY,
        if selected {
            dark::ACCENT
        } else {
            dark::TEXT_MUTED
        },
    );
    draw_ui_text(label, x + 58.0, y + 25.0, FONT_SMALL, color);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

fn draw_corner_brackets(rect: Rect, color: Color) {
    let len = 14.0;
    draw_line(rect.x, rect.y, rect.x + len, rect.y, 1.0, color);
    draw_line(rect.x, rect.y, rect.x, rect.y + len, 1.0, color);
    draw_line(
        rect.x + rect.w,
        rect.y,
        rect.x + rect.w - len,
        rect.y,
        1.0,
        color,
    );
    draw_line(
        rect.x + rect.w,
        rect.y,
        rect.x + rect.w,
        rect.y + len,
        1.0,
        color,
    );
    draw_line(
        rect.x,
        rect.y + rect.h,
        rect.x + len,
        rect.y + rect.h,
        1.0,
        color,
    );
    draw_line(
        rect.x,
        rect.y + rect.h,
        rect.x,
        rect.y + rect.h - len,
        1.0,
        color,
    );
    draw_line(
        rect.x + rect.w,
        rect.y + rect.h,
        rect.x + rect.w - len,
        rect.y + rect.h,
        1.0,
        color,
    );
    draw_line(
        rect.x + rect.w,
        rect.y + rect.h,
        rect.x + rect.w,
        rect.y + rect.h - len,
        1.0,
        color,
    );
}

fn selected_subject(state: &GameState) -> Option<&Kaiju> {
    state
        .selected_kaiju
        .and_then(|id| state.get_kaiju(id))
        .or_else(|| state.roster.iter().find(|kaiju| kaiju.alive))
        .or_else(|| state.roster.first())
}

fn nav_icon(label: &str) -> &'static str {
    match label {
        "LABORATORY" => "LAB",
        "ROSTER" => "RST",
        "TRAINING" => "TRN",
        "ARENA" => "ARN",
        "BREEDING" => "BRD",
        "RECORDS" => "REC",
        _ => "SET",
    }
}

fn section_label(active: AppSection) -> &'static str {
    match active {
        AppSection::Laboratory => "LABORATORY",
        AppSection::Roster => "ROSTER",
        AppSection::Training => "TRAINING",
        AppSection::Arena => "ARENA",
        AppSection::Breeding => "BREEDING",
        AppSection::Leaderboard => "RECORDS",
        AppSection::Detail => "KAIJU RECORD",
        AppSection::Results => "BATTLE RESULT",
        AppSection::Settings => "SETTINGS",
        AppSection::Marketplace => "MARKETPLACE",
    }
}

fn screen_directive(active: AppSection) -> &'static str {
    match active {
        AppSection::Laboratory => "Monitor. Act.",
        AppSection::Roster => "Choose specimen.",
        AppSection::Training => "Run one program.",
        AppSection::Arena => "Commit fighter.",
        AppSection::Breeding => "Pair bloodlines.",
        AppSection::Leaderboard => "Proven bloodlines.",
        AppSection::Detail => "Lineage and traits.",
        AppSection::Results => "Battle audit.",
        AppSection::Settings => "Facility controls.",
        AppSection::Marketplace => "Exchange locked.",
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
