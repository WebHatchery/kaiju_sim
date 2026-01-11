//! Extended color palette for Kaiju Sim UI.

use macroquad::prelude::Color;

/// Dark theme colors
pub mod dark {
    use super::*;

    // Base colors
    pub const BACKGROUND: Color = Color::new(0.08, 0.09, 0.10, 1.0); // #14171a
    pub const SURFACE: Color = Color::new(0.12, 0.13, 0.15, 1.0);    // #1e2226
    pub const PANEL: Color = Color::new(0.16, 0.17, 0.19, 1.0);      // #282b30
    pub const BORDER: Color = Color::new(0.25, 0.27, 0.30, 1.0);     // #404550
    
    // Text colors
    pub const TEXT_PRIMARY: Color = Color::new(0.93, 0.93, 0.93, 1.0);   // #ededed
    pub const TEXT_SECONDARY: Color = Color::new(0.65, 0.68, 0.70, 1.0); // #a5adb3
    pub const TEXT_MUTED: Color = Color::new(0.45, 0.48, 0.50, 1.0);     // #737a80
    
    // Semantic colors
    pub const ACCENT: Color = Color::new(0.26, 0.52, 0.96, 1.0);     // #4285f4
    pub const POSITIVE: Color = Color::new(0.30, 0.69, 0.31, 1.0);   // #4caf50
    pub const WARNING: Color = Color::new(1.0, 0.76, 0.03, 1.0);     // #ffc107
    pub const NEGATIVE: Color = Color::new(0.96, 0.26, 0.21, 1.0);   // #f44336
    
    // Status colors
    pub const ALIVE: Color = POSITIVE;
    pub const DEAD: Color = Color::new(0.39, 0.39, 0.43, 1.0);       // #64646e
    pub const IN_BATTLE: Color = WARNING;
    
    // Trait category colors
    pub const ELEMENT: Color = Color::new(0.26, 0.52, 0.96, 1.0);    // Blue
    pub const MODIFIER: Color = Color::new(0.61, 0.15, 0.69, 1.0);   // Purple
    pub const MUTATION: Color = Color::new(0.96, 0.26, 0.21, 1.0);   // Red
    pub const SYNERGY: Color = Color::new(0.30, 0.69, 0.31, 1.0);    // Green
    pub const HIDDEN: Color = Color::new(0.31, 0.31, 0.35, 1.0);     // Dark grey
    
    // Tournament type colors
    pub const RANKED: Color = Color::new(1.0, 0.76, 0.03, 1.0);      // Gold
    pub const LETHAL: Color = Color::new(0.83, 0.18, 0.18, 1.0);     // Dark Red
    pub const SPECIAL: Color = Color::new(0.48, 0.12, 0.64, 1.0);    // Deep Purple
    
    // Stat colors
    pub const HP_COLOR: Color = Color::new(0.30, 0.69, 0.31, 1.0);   // Green
    pub const ATK_COLOR: Color = Color::new(0.96, 0.26, 0.21, 1.0);  // Red
    pub const DEF_COLOR: Color = Color::new(0.13, 0.59, 0.95, 1.0);  // Blue
    pub const SPD_COLOR: Color = Color::new(1.0, 0.76, 0.03, 1.0);   // Yellow
    
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
