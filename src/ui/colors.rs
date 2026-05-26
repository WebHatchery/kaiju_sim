//! Extended color palette for Kaiju Sim UI.

use macroquad::prelude::Color;

/// Dark theme colors
pub mod dark {
    use super::*;

    // Base colors
    pub const BACKGROUND: Color = Color::new(0.018, 0.032, 0.045, 1.0);
    pub const SURFACE: Color = Color::new(0.035, 0.060, 0.082, 1.0);
    pub const PANEL: Color = Color::new(0.060, 0.086, 0.112, 1.0);
    pub const PANEL_ALT: Color = Color::new(0.078, 0.105, 0.134, 1.0);
    pub const BORDER: Color = Color::new(0.17, 0.28, 0.36, 1.0);
    pub const BORDER_SOFT: Color = Color::new(0.12, 0.23, 0.30, 0.62);

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::new(0.90, 0.96, 0.98, 1.0);
    pub const TEXT_SECONDARY: Color = Color::new(0.58, 0.72, 0.76, 1.0);
    pub const TEXT_MUTED: Color = Color::new(0.34, 0.45, 0.50, 1.0);

    // Semantic colors
    pub const ACCENT: Color = Color::new(0.20, 0.72, 0.92, 1.0);
    pub const ACCENT_DIM: Color = Color::new(0.09, 0.36, 0.48, 1.0);
    pub const POSITIVE: Color = Color::new(0.34, 0.80, 0.55, 1.0);
    pub const WARNING: Color = Color::new(0.96, 0.70, 0.24, 1.0);
    pub const NEGATIVE: Color = Color::new(0.92, 0.30, 0.28, 1.0);

    // Status colors
    pub const ALIVE: Color = POSITIVE;
    pub const DEAD: Color = Color::new(0.39, 0.39, 0.43, 1.0); // #64646e
    pub const IN_BATTLE: Color = WARNING;

    // Trait category colors
    pub const ELEMENT: Color = Color::new(0.20, 0.72, 0.92, 1.0);
    pub const MODIFIER: Color = Color::new(0.55, 0.42, 0.86, 1.0);
    pub const MUTATION: Color = Color::new(0.94, 0.42, 0.36, 1.0);
    pub const SYNERGY: Color = Color::new(0.34, 0.80, 0.55, 1.0);
    pub const HIDDEN: Color = Color::new(0.25, 0.31, 0.34, 1.0);

    // Tournament type colors
    pub const RANKED: Color = Color::new(0.96, 0.70, 0.24, 1.0);
    pub const LETHAL: Color = Color::new(0.83, 0.18, 0.18, 1.0);
    pub const SPECIAL: Color = Color::new(0.55, 0.42, 0.86, 1.0);

    // Stat colors
    pub const HP_COLOR: Color = Color::new(0.34, 0.80, 0.55, 1.0);
    pub const ATK_COLOR: Color = Color::new(0.94, 0.42, 0.36, 1.0);
    pub const DEF_COLOR: Color = Color::new(0.24, 0.62, 0.92, 1.0);
    pub const SPD_COLOR: Color = Color::new(0.96, 0.70, 0.24, 1.0);

    // Button colors
    pub const BUTTON_BG: Color = Color::new(0.22, 0.24, 0.27, 1.0);
    pub const BUTTON_HOVER: Color = Color::new(0.30, 0.32, 0.36, 1.0);
    pub const BUTTON_ACTIVE: Color = Color::new(0.26, 0.52, 0.96, 1.0);
}

/// Stat type for color mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
    HP,
    Attack,
    Defense,
    Speed,
}

/// Get color for stat type
pub fn stat_color(stat: StatType) -> Color {
    match stat {
        StatType::HP => dark::HP_COLOR,
        StatType::Attack => dark::ATK_COLOR,
        StatType::Defense => dark::DEF_COLOR,
        StatType::Speed => dark::SPD_COLOR,
    }
}

/// Get trait category color
pub fn trait_color(category: &crate::data::TraitCategory) -> Color {
    match category {
        crate::data::TraitCategory::Element => dark::ELEMENT,
        crate::data::TraitCategory::Modifier => dark::MODIFIER,
        crate::data::TraitCategory::Mutation => dark::MUTATION,
        crate::data::TraitCategory::Synergy => dark::SYNERGY,
    }
}

/// Get tournament type color
pub fn tournament_color(is_lethal: bool) -> Color {
    if is_lethal {
        dark::LETHAL
    } else {
        dark::RANKED
    }
}
