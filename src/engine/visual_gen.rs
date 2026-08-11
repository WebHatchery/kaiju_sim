//! Visual seed and appearance generation from genome data.

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::data::{Kaiju, Trait, TraitCategory};

/// Body type for kaiju appearance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    Quadruped,
    Bipedal,
    Serpentine,
    Winged,
    Aquatic,
}

impl BodyType {
    /// Get all body types
    pub fn all() -> &'static [BodyType] {
        &[
            BodyType::Quadruped,
            BodyType::Bipedal,
            BodyType::Serpentine,
            BodyType::Winged,
            BodyType::Aquatic,
        ]
    }

    /// Get sprite filename for this body type
    pub fn sprite_name(&self) -> &'static str {
        match self {
            BodyType::Quadruped => "kaiju_quadruped_neutral",
            BodyType::Bipedal => "kaiju_bipedal_neutral",
            BodyType::Serpentine => "kaiju_serpentine_neutral",
            BodyType::Winged => "kaiju_bipedal_neutral", // Fallback
            BodyType::Aquatic => "kaiju_serpentine_neutral", // Fallback
        }
    }
}

/// Element type for elemental kaiju
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    None,
    Fire,
    Ice,
    Electric,
    Aquatic,
}

impl Element {
    /// Get sprite filename for this element
    pub fn sprite_name(&self) -> Option<&'static str> {
        match self {
            Element::None => None,
            Element::Fire => Some("kaiju_fire_elemental"),
            Element::Ice => Some("kaiju_ice_elemental"),
            Element::Electric => Some("kaiju_electric_elemental"),
            Element::Aquatic => None, // Use body type sprite
        }
    }
}

/// Size category based on stats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeCategory {
    Small,
    Medium,
    Large,
    Massive,
}

/// Color scheme for kaiju
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary: (u8, u8, u8),
    pub secondary: (u8, u8, u8),
    pub accent: (u8, u8, u8),
    pub glow: Option<(u8, u8, u8)>,
}

/// Complete appearance data
#[derive(Debug, Clone)]
pub struct Appearance {
    pub body_type: BodyType,
    pub element: Element,
    pub size_category: SizeCategory,
    pub color_scheme: ColorScheme,
    pub scale: f32,
    pub sprite_path: String,
}

/// Generate appearance from kaiju data deterministically
pub fn generate_appearance(kaiju: &Kaiju) -> Appearance {
    let mut rng = ChaCha8Rng::seed_from_u64(kaiju.visual_seed);

    // Determine body type from stats
    let body_type = determine_body_type(&mut rng, kaiju);

    // Determine element from traits
    let element = determine_element(&kaiju.traits);

    // Determine size from HP
    let size_category = determine_size(kaiju.stats.hp);

    // Generate color scheme
    let color_scheme = generate_color_scheme(&mut rng, element);

    // Scale variation
    let scale = 0.9 + rng.gen::<f32>() * 0.2; // 0.9 to 1.1

    // Determine sprite path
    let sprite_path = if let Some(elem_sprite) = element.sprite_name() {
        format!("assets/sprites/kaiju/{}.png", elem_sprite)
    } else {
        format!("assets/sprites/kaiju/{}.png", body_type.sprite_name())
    };

    Appearance {
        body_type,
        element,
        size_category,
        color_scheme,
        scale,
        sprite_path,
    }
}

/// Determine body type from stats
fn determine_body_type(rng: &mut ChaCha8Rng, kaiju: &Kaiju) -> BodyType {
    let stats = &kaiju.stats;

    // High speed = serpentine, high defense = quadruped, high attack = bipedal
    let speed_ratio = stats.speed as f32 / 100.0;
    let def_ratio = stats.defense as f32 / 100.0;
    let atk_ratio = stats.attack as f32 / 100.0;

    let weights = [
        def_ratio * 1.2,                 // Quadruped
        atk_ratio * 1.0,                 // Bipedal
        speed_ratio * 1.5,               // Serpentine
        (speed_ratio + atk_ratio) * 0.5, // Winged
        (def_ratio + speed_ratio) * 0.3, // Aquatic
    ];

    let total: f32 = weights.iter().sum();
    let roll = rng.gen::<f32>() * total;

    let mut cumulative = 0.0;
    for (i, weight) in weights.iter().enumerate() {
        cumulative += weight;
        if roll <= cumulative {
            return BodyType::all()[i];
        }
    }

    BodyType::Bipedal
}

/// Determine element from traits
fn determine_element(traits: &[Trait]) -> Element {
    for t in traits {
        if t.category == TraitCategory::Element {
            let name_lower = t.name.to_lowercase();
            if name_lower.contains("fire") || name_lower.contains("flame") {
                return Element::Fire;
            }
            if name_lower.contains("ice") || name_lower.contains("frost") {
                return Element::Ice;
            }
            if name_lower.contains("electric") || name_lower.contains("lightning") {
                return Element::Electric;
            }
            if name_lower.contains("aqua") || name_lower.contains("water") {
                return Element::Aquatic;
            }
        }
    }
    Element::None
}

/// Determine size from HP
fn determine_size(hp: i32) -> SizeCategory {
    match hp {
        0..=150 => SizeCategory::Small,
        151..=250 => SizeCategory::Medium,
        251..=400 => SizeCategory::Large,
        _ => SizeCategory::Massive,
    }
}

/// Generate color scheme
fn generate_color_scheme(rng: &mut ChaCha8Rng, element: Element) -> ColorScheme {
    let (primary, secondary, glow) = match element {
        Element::Fire => (
            (200 + rng.gen_range(0..55), 80 + rng.gen_range(0..40), 30),
            (255, 150 + rng.gen_range(0..50), 50),
            Some((255, 100, 0)),
        ),
        Element::Ice => (
            (180 + rng.gen_range(0..50), 220 + rng.gen_range(0..35), 255),
            (150, 200, 255),
            Some((100, 200, 255)),
        ),
        Element::Electric => (
            (50, 100 + rng.gen_range(0..50), 200 + rng.gen_range(0..55)),
            (255, 255, 100),
            Some((100, 150, 255)),
        ),
        Element::Aquatic => (
            (50, 150 + rng.gen_range(0..50), 200 + rng.gen_range(0..55)),
            (100, 200, 220),
            Some((50, 150, 200)),
        ),
        Element::None => (
            (
                100 + rng.gen_range(0..100),
                100 + rng.gen_range(0..100),
                100 + rng.gen_range(0..100),
            ),
            (
                150 + rng.gen_range(0..50),
                150 + rng.gen_range(0..50),
                150 + rng.gen_range(0..50),
            ),
            None,
        ),
    };

    ColorScheme {
        primary,
        secondary,
        accent: (255, 220, 100), // Eyes/claws
        glow,
    }
}

#[cfg(test)]
mod tests;
