//! Kaiju-specific particle burst presets, built on macroquad-toolkit's pooled
//! particle system.

pub use macroquad_toolkit::fx::{BurstConfig, ParticleSystem};

/// Preset particle burst effects for combat/status feedback. Each preset
/// returns the particle count alongside its [`BurstConfig`], for use with
/// [`ParticleSystem::spawn_burst`].
pub mod presets {
    use super::*;
    use crate::ui::colors::dark;
    use macroquad::prelude::Color;
    use std::f32::consts::{PI, TAU};

    /// Hit spark effect: white sparks radiating outward in all directions.
    pub fn hit_spark() -> (usize, BurstConfig) {
        (
            15,
            BurstConfig {
                speed: (60.0, 140.0),
                size: (2.0, 4.0),
                life: (0.2, 0.3),
                colors: vec![Color::new(1.0, 1.0, 1.0, 1.0)],
                direction: 0.0,
                spread: TAU,
                drag: 0.15,
                gravity: 0.0,
                shrink: true,
            },
        )
    }

    /// Fire burst effect: orange sparks in an upward cone.
    pub fn fire_burst() -> (usize, BurstConfig) {
        (
            25,
            BurstConfig {
                speed: (40.0, 120.0),
                size: (4.0, 8.0),
                life: (0.35, 0.5),
                colors: vec![Color::new(1.0, 0.5, 0.1, 1.0)],
                direction: -PI / 2.0,
                spread: PI / 2.0,
                drag: 0.15,
                gravity: 0.0,
                shrink: true,
            },
        )
    }

    /// Ice shatter effect: pale blue shards radiating outward in all directions.
    pub fn ice_shatter() -> (usize, BurstConfig) {
        (
            20,
            BurstConfig {
                speed: (40.0, 110.0),
                size: (3.0, 6.0),
                life: (0.4, 0.6),
                colors: vec![Color::new(0.6, 0.9, 1.0, 1.0)],
                direction: 0.0,
                spread: TAU,
                drag: 0.15,
                gravity: 0.0,
                shrink: true,
            },
        )
    }

    /// Electric zap effect: fast blue sparks radiating outward in all directions.
    pub fn electric_zap() -> (usize, BurstConfig) {
        (
            30,
            BurstConfig {
                speed: (60.0, 160.0),
                size: (1.5, 3.0),
                life: (0.15, 0.25),
                colors: vec![Color::new(0.4, 0.7, 1.0, 1.0)],
                direction: 0.0,
                spread: TAU,
                drag: 0.15,
                gravity: 0.0,
                shrink: true,
            },
        )
    }

    /// Healing aura effect: green motes rising in an upward cone.
    pub fn healing_aura() -> (usize, BurstConfig) {
        (
            15,
            BurstConfig {
                speed: (40.0, 80.0),
                size: (3.0, 5.0),
                life: (0.8, 1.0),
                colors: vec![dark::POSITIVE],
                direction: -PI / 2.0,
                spread: PI / 3.0,
                drag: 0.15,
                gravity: 0.0,
                shrink: true,
            },
        )
    }
}
