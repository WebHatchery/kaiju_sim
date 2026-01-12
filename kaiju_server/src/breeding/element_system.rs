//! Element System
//! Handles hybrid element creation and elemental interactions.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::ElementType;

/// Hybrid element result from combining two elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridElement {
    pub name: String,
    pub display_name: String,
    /// Primary stat bonuses
    pub primary_stat_bonus: StatBonus,
    /// Visual keywords for image generation
    pub visual_keywords: Vec<String>,
    /// Combat advantages
    pub advantages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatBonus {
    Speed(i32),
    Attack(i32),
    Defense(i32),
    Hp(i32),
    Evasion(i32),
    Multiple(Vec<(String, i32)>),
}

/// Element interaction matrix for hybrid creation
pub static ELEMENT_HYBRIDS: Lazy<HashMap<(ElementType, ElementType), HybridElement>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Fire + Water -> Steam/Misty (High Speed/Evasion)
    map.insert(
        (ElementType::Fire, ElementType::Water),
        HybridElement {
            name: "steam".to_string(),
            display_name: "Steam".to_string(),
            primary_stat_bonus: StatBonus::Multiple(vec![
                ("speed".to_string(), 15),
                ("evasion".to_string(), 10),
            ]),
            visual_keywords: vec![
                "steam vents".to_string(),
                "misty aura".to_string(),
                "vapor trails".to_string(),
                "obscuring fog".to_string(),
            ],
            advantages: vec!["evasion".to_string(), "speed".to_string()],
        },
    );
    
    // Earth + Wind -> Sandstorm (Erosion damage)
    map.insert(
        (ElementType::Earth, ElementType::Wind),
        HybridElement {
            name: "sandstorm".to_string(),
            display_name: "Sandstorm".to_string(),
            primary_stat_bonus: StatBonus::Attack(12),
            visual_keywords: vec![
                "sand particles".to_string(),
                "erosive winds".to_string(),
                "desert storm".to_string(),
                "gritty texture".to_string(),
            ],
            advantages: vec!["erosion".to_string(), "blind".to_string()],
        },
    );
    
    // Dark + Light -> Eclipse (Phase shifting)
    map.insert(
        (ElementType::Dark, ElementType::Light),
        HybridElement {
            name: "eclipse".to_string(),
            display_name: "Eclipse".to_string(),
            primary_stat_bonus: StatBonus::Multiple(vec![
                ("attack".to_string(), 10),
                ("defense".to_string(), 10),
            ]),
            visual_keywords: vec![
                "phase shifting".to_string(),
                "corona glow".to_string(),
                "shadow and light contrast".to_string(),
                "dual nature".to_string(),
            ],
            advantages: vec!["phase_shift".to_string(), "dual_affinity".to_string()],
        },
    );
    
    // Electric + Water -> Conductivity (Amplified damage)
    map.insert(
        (ElementType::Electric, ElementType::Water),
        HybridElement {
            name: "conductivity".to_string(),
            display_name: "Conductivity".to_string(),
            primary_stat_bonus: StatBonus::Attack(18),
            visual_keywords: vec![
                "electric currents in water".to_string(),
                "crackling waves".to_string(),
                "plasma bubbles".to_string(),
            ],
            advantages: vec!["amplified_damage".to_string(), "chain_lightning".to_string()],
        },
    );
    
    // Fire + Earth -> Volcanic (High defense + damage)
    map.insert(
        (ElementType::Fire, ElementType::Earth),
        HybridElement {
            name: "volcanic".to_string(),
            display_name: "Volcanic".to_string(),
            primary_stat_bonus: StatBonus::Multiple(vec![
                ("attack".to_string(), 12),
                ("defense".to_string(), 15),
            ]),
            visual_keywords: vec![
                "molten rock armor".to_string(),
                "lava veins".to_string(),
                "obsidian plating".to_string(),
                "magma core".to_string(),
            ],
            advantages: vec!["armor".to_string(), "burn".to_string()],
        },
    );
    
    // Ice + Wind -> Blizzard (High speed + slow)
    map.insert(
        (ElementType::Ice, ElementType::Wind),
        HybridElement {
            name: "blizzard".to_string(),
            display_name: "Blizzard".to_string(),
            primary_stat_bonus: StatBonus::Speed(20),
            visual_keywords: vec![
                "swirling snow".to_string(),
                "frozen winds".to_string(),
                "ice crystal tempest".to_string(),
            ],
            advantages: vec!["slow".to_string(), "frostbite".to_string()],
        },
    );
    
    // Poison + Dark -> Necrotic (DoT + debuff)
    map.insert(
        (ElementType::Poison, ElementType::Dark),
        HybridElement {
            name: "necrotic".to_string(),
            display_name: "Necrotic".to_string(),
            primary_stat_bonus: StatBonus::Attack(15),
            visual_keywords: vec![
                "decaying aura".to_string(),
                "toxic shadows".to_string(),
                "withering touch".to_string(),
            ],
            advantages: vec!["dot".to_string(), "debuff".to_string(), "drain".to_string()],
        },
    );
    
    // Nature + Water -> Swamp (Regen + poison)
    map.insert(
        (ElementType::Nature, ElementType::Water),
        HybridElement {
            name: "swamp".to_string(),
            display_name: "Swamp".to_string(),
            primary_stat_bonus: StatBonus::Hp(50),
            visual_keywords: vec![
                "murky algae".to_string(),
                "bog creature".to_string(),
                "marsh dweller".to_string(),
            ],
            advantages: vec!["regen".to_string(), "toxic".to_string()],
        },
    );
    
    // Electric + Wind -> Storm (Chain attacks)
    map.insert(
        (ElementType::Electric, ElementType::Wind),
        HybridElement {
            name: "storm".to_string(),
            display_name: "Storm".to_string(),
            primary_stat_bonus: StatBonus::Multiple(vec![
                ("speed".to_string(), 12),
                ("attack".to_string(), 12),
            ]),
            visual_keywords: vec![
                "thunderstorm aura".to_string(),
                "lightning wings".to_string(),
                "tempest form".to_string(),
            ],
            advantages: vec!["chain_attack".to_string(), "stun".to_string()],
        },
    );
    
    // Ice + Water -> Arctic (Freeze + defense)
    map.insert(
        (ElementType::Ice, ElementType::Water),
        HybridElement {
            name: "arctic".to_string(),
            display_name: "Arctic".to_string(),
            primary_stat_bonus: StatBonus::Defense(18),
            visual_keywords: vec![
                "frozen scales".to_string(),
                "icy depths".to_string(),
                "glacial form".to_string(),
            ],
            advantages: vec!["freeze".to_string(), "armor".to_string()],
        },
    );
    
    // Psychic + Dark -> Void (Reality warp)
    map.insert(
        (ElementType::Psychic, ElementType::Dark),
        HybridElement {
            name: "void".to_string(),
            display_name: "Void".to_string(),
            primary_stat_bonus: StatBonus::Multiple(vec![
                ("attack".to_string(), 15),
                ("speed".to_string(), 10),
            ]),
            visual_keywords: vec![
                "reality distortion".to_string(),
                "void tendrils".to_string(),
                "dimensional rift".to_string(),
            ],
            advantages: vec!["reality_warp".to_string(), "teleport".to_string()],
        },
    );
    
    // Light + Nature -> Radiant (Healing + buff)
    map.insert(
        (ElementType::Light, ElementType::Nature),
        HybridElement {
            name: "radiant".to_string(),
            display_name: "Radiant".to_string(),
            primary_stat_bonus: StatBonus::Hp(40),
            visual_keywords: vec![
                "golden flora".to_string(),
                "blessed growth".to_string(),
                "solar vines".to_string(),
            ],
            advantages: vec!["heal".to_string(), "buff".to_string()],
        },
    );
    
    map
});

/// Get hybrid element from two parent elements
pub fn get_hybrid_element(elem_a: ElementType, elem_b: ElementType) -> Option<HybridElement> {
    // Check both orderings
    ELEMENT_HYBRIDS.get(&(elem_a, elem_b))
        .or_else(|| ELEMENT_HYBRIDS.get(&(elem_b, elem_a)))
        .cloned()
}

/// Determine element inheritance based on parents
pub fn determine_offspring_element(
    parent_a_element: ElementType,
    parent_b_element: ElementType,
    roll: f32,
    forced_element: Option<ElementType>,
) -> (ElementType, Option<HybridElement>) {
    // If element is forced by item, use that
    if let Some(forced) = forced_element {
        return (forced, None);
    }
    
    // Same element -> guaranteed inheritance
    if parent_a_element == parent_b_element {
        return (parent_a_element, None);
    }
    
    // Different elements -> check for hybrid
    if let Some(hybrid) = get_hybrid_element(parent_a_element, parent_b_element) {
        // 30% chance for true hybrid, otherwise inherit one parent's element
        if roll < 0.30 {
            // Hybrid keeps primary element of parent A but gains hybrid properties
            return (parent_a_element, Some(hybrid));
        }
    }
    
    // 50/50 between parent elements
    if roll < 0.50 {
        (parent_a_element, None)
    } else {
        (parent_b_element, None)
    }
}

/// Generate visual keywords for an element
pub fn get_element_visual_keywords(element: ElementType) -> Vec<String> {
    match element {
        ElementType::Fire => vec![
            "flames".to_string(),
            "molten glow".to_string(),
            "ember particles".to_string(),
            "volcanic energy".to_string(),
        ],
        ElementType::Water => vec![
            "water shimmer".to_string(),
            "aquatic scales".to_string(),
            "flowing essence".to_string(),
            "ocean depths".to_string(),
        ],
        ElementType::Ice => vec![
            "ice crystals".to_string(),
            "frozen patterns".to_string(),
            "frost aura".to_string(),
            "glacial blue".to_string(),
        ],
        ElementType::Electric => vec![
            "lightning patterns".to_string(),
            "crackling energy".to_string(),
            "electric aura".to_string(),
            "voltage sparks".to_string(),
        ],
        ElementType::Earth => vec![
            "rocky plates".to_string(),
            "stone texture".to_string(),
            "earthen armor".to_string(),
            "mineral veins".to_string(),
        ],
        ElementType::Wind => vec![
            "air currents".to_string(),
            "streamlined form".to_string(),
            "wind swept".to_string(),
            "aerodynamic".to_string(),
        ],
        ElementType::Dark => vec![
            "shadow wisps".to_string(),
            "dark aura".to_string(),
            "void essence".to_string(),
            "midnight black".to_string(),
        ],
        ElementType::Light => vec![
            "radiant glow".to_string(),
            "golden light".to_string(),
            "luminescent".to_string(),
            "holy aura".to_string(),
        ],
        ElementType::Poison => vec![
            "toxic drip".to_string(),
            "venomous markings".to_string(),
            "acid green".to_string(),
            "poison sacs".to_string(),
        ],
        ElementType::Nature => vec![
            "plant growth".to_string(),
            "vine patterns".to_string(),
            "organic texture".to_string(),
            "forest green".to_string(),
        ],
        ElementType::Psychic => vec![
            "mental energy".to_string(),
            "third eye".to_string(),
            "psionic aura".to_string(),
            "ethereal glow".to_string(),
        ],
        ElementType::Neutral => vec![
            "balanced form".to_string(),
            "natural colors".to_string(),
            "primal essence".to_string(),
        ],
    }
}
