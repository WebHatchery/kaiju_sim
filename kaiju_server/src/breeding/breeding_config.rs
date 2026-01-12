//! Breeding Configuration
//! Centralized config for all breeding parameters based on BREEDING_SYSTEM_DESIGN.md

use serde::{Deserialize, Serialize};

/// Master breeding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingConfig {
    /// Inheritance rates
    pub inheritance: InheritanceRates,
    /// Mutation parameters
    pub mutation: MutationConfig,
    /// Stat calculation parameters
    pub stats: StatConfig,
    /// Economy parameters
    pub economy: EconomyConfig,
    /// Time requirements
    pub timing: TimingConfig,
    /// Balance constraints
    pub balance: BalanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceRates {
    /// Dominant trait inheritance (from design: 75%)
    pub dominant_rate: f32,
    /// Recessive trait inheritance (from design: 25%)
    pub recessive_rate: f32,
    /// Boost when both parents have dominant trait
    pub dominant_both_parents_rate: f32,
    /// Boost when both parents have recessive trait
    pub recessive_both_parents_rate: f32,
    /// Base visible trait inheritance
    pub visible_base_rate: f32,
    /// Base hidden trait inheritance
    pub hidden_base_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationConfig {
    /// Base mutation chance (from design: 2-5%)
    pub base_chance: f32,
    /// Maximum mutation chance (capped)
    pub max_chance: f32,
    /// Generation modifier per generation
    pub generation_modifier: f32,
    /// High trait count threshold
    pub high_trait_threshold: usize,
    /// Bonus chance for high trait count
    pub high_trait_bonus: f32,
    /// Mutation stat boost min (5%)
    pub stat_boost_min: f32,
    /// Mutation stat boost max (15%)
    pub stat_boost_max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatConfig {
    /// Generation power creep (1% per gen)
    pub generation_power_creep: f32,
    /// Stat variance minimum (95%)
    pub variance_min: f32,
    /// Stat variance maximum (105%)
    pub variance_max: f32,
    /// Stat floors
    pub floors: StatFloors,
    /// Base caps (Gen 1)
    pub base_caps: StatCaps,
    /// Hard ceilings (absolute max)
    pub hard_ceilings: StatCaps,
    /// Soft ceiling generation scaling
    pub soft_ceiling_scaling: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatFloors {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub energy: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatCaps {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub energy: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyConfig {
    /// Cost matrix based on rarity
    pub breeding_costs: BreedingCostMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingCostMatrix {
    pub common_common: i64,
    pub common_rare: i64,
    pub rare_rare: i64,
    pub legendary_any: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingConfig {
    /// Gestation time range in hours
    pub gestation_min_hours: f32,
    pub gestation_max_hours: f32,
    /// Maturation time range in hours
    pub maturation_min_hours: f32,
    pub maturation_max_hours: f32,
    /// Cooldown between breeding cycles
    pub cooldown_hours: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    /// Maximum power budget
    pub max_power_budget: i32,
    /// Maximum visible traits
    pub max_visible_traits: usize,
    /// Maximum hidden traits
    pub max_hidden_traits: usize,
    /// Total trait cap
    pub total_trait_cap: usize,
}

impl Default for BreedingConfig {
    fn default() -> Self {
        Self {
            inheritance: InheritanceRates {
                dominant_rate: 0.75,
                recessive_rate: 0.25,
                dominant_both_parents_rate: 1.0, // Guaranteed if both have it
                recessive_both_parents_rate: 0.70,
                visible_base_rate: 0.45,
                hidden_base_rate: 0.25,
            },
            mutation: MutationConfig {
                base_chance: 0.03, // 3% base (middle of 2-5% range)
                max_chance: 0.25,
                generation_modifier: 0.005,
                high_trait_threshold: 10,
                high_trait_bonus: 0.05,
                stat_boost_min: 1.05,
                stat_boost_max: 1.15,
            },
            stats: StatConfig {
                generation_power_creep: 0.01,
                variance_min: 0.95,
                variance_max: 1.05,
                floors: StatFloors {
                    hp: 50,
                    attack: 10,
                    defense: 5,
                    speed: 5,
                    energy: 10,
                },
                base_caps: StatCaps {
                    hp: 500,
                    attack: 100,
                    defense: 100,
                    speed: 100,
                    energy: 100,
                },
                hard_ceilings: StatCaps {
                    hp: 2000,
                    attack: 400,
                    defense: 400,
                    speed: 400,
                    energy: 400,
                },
                soft_ceiling_scaling: 0.02,
            },
            economy: EconomyConfig {
                breeding_costs: BreedingCostMatrix {
                    common_common: 1000,
                    common_rare: 5000,
                    rare_rare: 15000,
                    legendary_any: 50000,
                },
            },
            timing: TimingConfig {
                gestation_min_hours: 6.0,
                gestation_max_hours: 48.0,
                maturation_min_hours: 24.0,
                maturation_max_hours: 120.0,
                cooldown_hours: 12.0,
            },
            balance: BalanceConfig {
                max_power_budget: 150,
                max_visible_traits: 8,
                max_hidden_traits: 6,
                total_trait_cap: 12,
            },
        }
    }
}

/// Special breeding materials that can be used
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BreedingMaterial {
    /// Guarantees primary element inheritance
    ElementalEssence(ElementType),
    /// Increases mutation chance by +5%
    MutationCatalyst,
    /// Prevents negative traits and stat down-scaling
    GeneticStabilizer,
    /// Reduces gestation time by 50%
    FertilityIdol,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ElementType {
    Fire,
    Water,
    Ice,
    Electric,
    Earth,
    Wind,
    Dark,
    Light,
    Poison,
    Nature,
    Psychic,
    Neutral,
}

impl ElementType {
    pub fn all() -> Vec<ElementType> {
        vec![
            Self::Fire,
            Self::Water,
            Self::Ice,
            Self::Electric,
            Self::Earth,
            Self::Wind,
            Self::Dark,
            Self::Light,
            Self::Poison,
            Self::Nature,
            Self::Psychic,
            Self::Neutral,
        ]
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fire" | "flame" | "volcanic" | "magma" => Some(Self::Fire),
            "water" | "aqua" | "ocean" | "aquatic" => Some(Self::Water),
            "ice" | "frost" | "cold" | "arctic" => Some(Self::Ice),
            "electric" | "lightning" | "thunder" | "volt" => Some(Self::Electric),
            "earth" | "rock" | "ground" | "stone" => Some(Self::Earth),
            "wind" | "air" | "storm" | "gust" => Some(Self::Wind),
            "dark" | "shadow" | "void" | "darkness" => Some(Self::Dark),
            "light" | "solar" | "lunar" | "radiant" => Some(Self::Light),
            "poison" | "toxic" | "venom" | "acid" => Some(Self::Poison),
            "nature" | "plant" | "forest" | "bio" => Some(Self::Nature),
            "psychic" | "mental" | "psi" | "mind" => Some(Self::Psychic),
            _ => Some(Self::Neutral),
        }
    }
}

/// Rarity classification for breeding costs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KaijuRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl KaijuRarity {
    /// Calculate rarity from trait power + stat total
    pub fn calculate(total_trait_power: i32, base_stat_total: i32) -> Self {
        let score = total_trait_power + (base_stat_total / 10);
        match score {
            0..=30 => Self::Common,
            31..=60 => Self::Uncommon,
            61..=100 => Self::Rare,
            101..=150 => Self::Epic,
            _ => Self::Legendary,
        }
    }
}
