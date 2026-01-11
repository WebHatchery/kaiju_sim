//! Trait system for kaiju genetics and abilities.

use serde::{Deserialize, Serialize};

use super::types::TraitId;

/// Trait categories defining behavior and inheritance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitCategory {
    /// Elemental affinities (fire, water, electric, etc.)
    Element,

    /// Stat modifiers and conditional bonuses
    Modifier,

    /// Random mutations from breeding
    Mutation,

    /// Meta-traits activated by trait combinations
    Synergy,
}

/// Inheritance type determines breeding probability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitInheritance {
    /// High inheritance chance (60%)
    Dominant,

    /// Lower inheritance chance (30%)
    Recessive,

    /// Requires both parents (special rules)
    Polygenic,

    /// Activated under specific conditions
    Conditional,
}

/// Condition for trait activation (combat, environment, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TraitCondition {
    /// Simple string conditions (e.g. "Always")
    Simple(String),

    /// Complex condition with type tag
    Complex(TraitConditionVariant),
}

/// Complex condition variants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitConditionVariant {
    /// Active in specific environment
    Environment(String),

    /// Active when HP below threshold
    LowHealth(i32),

    /// Active when HP above threshold
    HighHealth(i32),

    /// Active against specific trait
    CounterTrait(TraitId),

    /// Requires another trait to be present
    RequiresTrait(TraitId),
}

/// Complete trait definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    /// Unique trait identifier
    pub id: TraitId,

    /// Display name
    pub name: String,

    /// Trait category
    pub category: TraitCategory,

    /// Numeric power/influence (-10 to +25)
    pub power: i32,

    /// Inheritance type
    pub inheritance: TraitInheritance,

    /// Activation condition
    pub condition: TraitCondition,

    /// Whether trait is hidden initially
    pub is_hidden: bool,

    /// Flavor text description
    pub description: String,
}

impl Trait {
    /// Check if trait is active under given conditions
    pub fn is_active(&self, current_hp: i32, max_hp: i32, environment: &str) -> bool {
        match &self.condition {
            TraitCondition::Simple(s) => s == "Always",
            TraitCondition::Complex(variant) => match variant {
                TraitConditionVariant::Environment(env) => env == environment,
                TraitConditionVariant::LowHealth(threshold) => {
                    (current_hp as f32 / max_hp as f32) < (*threshold as f32 / 100.0)
                }
                TraitConditionVariant::HighHealth(threshold) => {
                    (current_hp as f32 / max_hp as f32) >= (*threshold as f32 / 100.0)
                }
                _ => true, // Other conditions checked elsewhere
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_condition_always_active() {
        let trait_def = Trait {
            id: "test_trait".to_string(),
            name: "Test".to_string(),
            category: TraitCategory::Element,
            power: 10,
            inheritance: TraitInheritance::Dominant,
            condition: TraitCondition::Simple("Always".to_string()),
            is_hidden: false,
            description: "Test trait".to_string(),
        };

        assert!(trait_def.is_active(100, 100, "any"));
    }

    #[test]
    fn test_trait_condition_low_health() {
        let trait_def = Trait {
            id: "berserker".to_string(),
            name: "Berserker".to_string(),
            category: TraitCategory::Modifier,
            power: 15,
            inheritance: TraitInheritance::Recessive,
            condition: TraitCondition::Complex(TraitConditionVariant::LowHealth(30)),
            is_hidden: false,
            description: "Activates when HP < 30%".to_string(),
        };

        assert!(trait_def.is_active(25, 100, "any"));
        assert!(!trait_def.is_active(50, 100, "any"));
    }

    #[test]
    fn test_trait_condition_environment() {
        let trait_def = Trait {
            id: "electric_breath".to_string(),
            name: "Electric Breath".to_string(),
            category: TraitCategory::Element,
            power: 12,
            inheritance: TraitInheritance::Dominant,
            condition: TraitCondition::Complex(TraitConditionVariant::Environment(
                "storm".to_string(),
            )),
            is_hidden: false,
            description: "Boosted in storm environments".to_string(),
        };

        assert!(trait_def.is_active(100, 100, "storm"));
        assert!(!trait_def.is_active(100, 100, "volcanic"));
    }
}
