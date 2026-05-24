//! Screen module exports.

pub mod arena;
pub mod battle_results;
pub mod breeding;
pub mod kaiju_detail;
pub mod laboratory;
pub mod leaderboard;
pub mod main_menu;
pub mod marketplace;
pub mod roster_view;
pub mod starter_selection; // Added
pub mod tournament_lobby;
pub mod training; // Added

pub use arena::draw_arena_screen;
pub use battle_results::draw_battle_results;
pub use breeding::{draw_breeding_screen, BreedingState};
pub use kaiju_detail::draw_kaiju_detail;
pub use laboratory::draw_laboratory;
pub use leaderboard::draw_leaderboard;
pub use main_menu::draw_main_menu;
pub use marketplace::{draw_marketplace, MarketplaceState};
pub use roster_view::draw_roster_view;
pub use starter_selection::draw_starter_selection;
pub use tournament_lobby::draw_tournament_lobby;
pub use training::draw_training_screen;
