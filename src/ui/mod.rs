//! UI module - components, theming, and actions.

pub mod actions;
pub mod colors;
pub mod components;
pub mod spacing;
pub mod typography;

// Re-exports
pub use actions::UiAction;
pub use colors::{dark, stat_color, trait_color, StatType};
pub use components::{draw_kaiju_card, draw_stat_bar, CardState, CardAction};
pub use spacing::*;
pub use typography::*;
