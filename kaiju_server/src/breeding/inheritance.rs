//! Trait Inheritance System
//! Implements Dominant, Recessive, Polygenic, and Conditional inheritance.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::{BreedingConfig, TraitInheritResult, TraitRoll};

/// Full trait definition with inheritance mechanics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TraitCategory,
    pub inheritance_type: InheritanceType,
    pub power: i32,
    pub is_hidden: bool,
    /// Visual keywords for image generation
    pub visual_keywords: Vec<String>,
    /// Combat condition (if any)
    pub condition: Option<TraitCondition>,
    /// Polygenic accumulation points
    pub polygenic_points: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitCategory {
    Element,
    Modifier,
    Mutation,
    Synergy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InheritanceType {
    /// 75% inheritance, 100% if both parents
    Dominant,
    /// 25% inheritance, 70% if both parents
    Recessive,
    /// Requires accumulation from ancestors
    Polygenic { threshold: i32 },
    /// Custom condition-based inheritance
    Conditional {
        condition: ConditionalRequirement,
        chance: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionalRequirement {
    /// Requires combined parent generation >= threshold
    MinCombinedGeneration(u32),
    /// Requires both parents to have stat >= threshold
    BothParentsStat { stat: String, min_value: i32 },
    /// Requires parent to have specific traits
    RequiredTraits(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitCondition {
    Always,
    HpBelow(i32),
    HpAbove(i32),
    TurnMin(u32),
    Environment(String),
    EnemyHpAbove(i32),
    AfterKill,
    FirstTurn,
    Darkness,
    Daylight,
}

/// Synergy definitions - emergent traits from combinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyDefinition {
    pub id: String,
    pub name: String,
    /// Required trait IDs
    pub required_traits: Vec<String>,
    /// Power of the synergy
    pub power: i32,
    /// Activation chance when requirements met
    pub activation_chance: f32,
    /// Combat effect description
    pub effect: String,
    /// Visual keywords
    pub visual_keywords: Vec<String>,
}

/// Incompatible trait pairs
pub const INCOMPATIBLE_TRAITS: &[(&str, &str)] = &[
    ("berserker_rage", "defensive_stance"),
    ("camouflage", "bioluminescence"),
    ("speed_boost", "hardened_carapace"),
];

/// Trait inheritance processor
pub struct TraitInheritanceProcessor {
    config: BreedingConfig,
}

impl TraitInheritanceProcessor {
    pub fn new(config: BreedingConfig) -> Self {
        Self { config }
    }

    /// Process trait inheritance for offspring
    #[allow(clippy::too_many_arguments)] // mirrors the independent per-parent inputs needed to roll inheritance; a param struct would just move the same fields
    pub fn process_inheritance<R: Rng>(
        &self,
        parent_a_traits: &[TraitDefinition],
        parent_b_traits: &[TraitDefinition],
        parent_a_gen: u32,
        parent_b_gen: u32,
        parent_a_stats: &HashMap<String, i32>,
        parent_b_stats: &HashMap<String, i32>,
        rng: &mut R,
    ) -> (Vec<TraitDefinition>, Vec<TraitDefinition>, Vec<TraitRoll>) {
        let mut visible_traits: Vec<TraitDefinition> = Vec::new();
        let mut hidden_traits: Vec<TraitDefinition> = Vec::new();
        let mut trait_rolls: Vec<TraitRoll> = Vec::new();

        // Create lookup sets
        let parent_a_trait_ids: HashSet<_> = parent_a_traits.iter().map(|t| t.id.clone()).collect();
        let parent_b_trait_ids: HashSet<_> = parent_b_traits.iter().map(|t| t.id.clone()).collect();

        // Combine all unique traits from both parents
        let mut all_traits: HashMap<String, TraitDefinition> = HashMap::new();
        for t in parent_a_traits.iter().chain(parent_b_traits.iter()) {
            all_traits.entry(t.id.clone()).or_insert_with(|| t.clone());
        }

        // Process each trait
        for (trait_id, trait_def) in all_traits.iter() {
            let in_parent_a = parent_a_trait_ids.contains(trait_id);
            let in_parent_b = parent_b_trait_ids.contains(trait_id);
            let both_parents = in_parent_a && in_parent_b;

            let (chance, inheritance_desc) = self.calculate_inheritance_chance(
                &trait_def.inheritance_type,
                both_parents,
                parent_a_gen,
                parent_b_gen,
                parent_a_stats,
                parent_b_stats,
                &parent_a_trait_ids,
                &parent_b_trait_ids,
            );

            let roll: f32 = rng.gen();
            let result = if roll < chance {
                // Inherited - determine if visible or hidden
                if trait_def.is_hidden {
                    hidden_traits.push(trait_def.clone());
                    TraitInheritResult::InheritedHidden
                } else {
                    visible_traits.push(trait_def.clone());
                    TraitInheritResult::Inherited
                }
            } else {
                TraitInheritResult::Lost
            };

            trait_rolls.push(TraitRoll {
                trait_name: trait_def.name.clone(),
                trait_id: trait_def.id.clone(),
                inheritance_type: inheritance_desc,
                in_parent_a,
                in_parent_b,
                chance,
                roll,
                result,
            });
        }

        // Check for incompatibilities and remove conflicts
        let (visible_traits, hidden_traits, blocked_rolls) =
            self.resolve_incompatibilities(visible_traits, hidden_traits, rng);

        // Add blocked trait rolls
        for blocked in blocked_rolls {
            trait_rolls.push(TraitRoll {
                trait_name: blocked.clone(),
                trait_id: blocked,
                inheritance_type: "Blocked".to_string(),
                in_parent_a: false,
                in_parent_b: false,
                chance: 0.0,
                roll: 0.0,
                result: TraitInheritResult::Blocked,
            });
        }

        // Enforce trait caps
        let visible_traits =
            self.enforce_trait_cap(visible_traits, self.config.balance.max_visible_traits);
        let hidden_traits =
            self.enforce_trait_cap(hidden_traits, self.config.balance.max_hidden_traits);

        (visible_traits, hidden_traits, trait_rolls)
    }

    #[allow(clippy::too_many_arguments)] // mirrors the independent per-parent inputs needed to roll inheritance; a param struct would just move the same fields
    fn calculate_inheritance_chance(
        &self,
        inheritance_type: &InheritanceType,
        both_parents_have: bool,
        parent_a_gen: u32,
        parent_b_gen: u32,
        parent_a_stats: &HashMap<String, i32>,
        parent_b_stats: &HashMap<String, i32>,
        parent_a_traits: &HashSet<String>,
        parent_b_traits: &HashSet<String>,
    ) -> (f32, String) {
        match inheritance_type {
            InheritanceType::Dominant => {
                if both_parents_have {
                    (
                        self.config.inheritance.dominant_both_parents_rate,
                        "Dominant (both parents)".to_string(),
                    )
                } else {
                    (
                        self.config.inheritance.dominant_rate,
                        "Dominant".to_string(),
                    )
                }
            }
            InheritanceType::Recessive => {
                if both_parents_have {
                    (
                        self.config.inheritance.recessive_both_parents_rate,
                        "Recessive (both parents)".to_string(),
                    )
                } else {
                    (
                        self.config.inheritance.recessive_rate,
                        "Recessive".to_string(),
                    )
                }
            }
            InheritanceType::Polygenic { threshold } => {
                // Calculate polygenic points
                let mut points = 0;
                if both_parents_have {
                    points = 4; // Both visible = 4 points
                } else {
                    points = 2; // One parent = 2 points
                }

                if points >= *threshold {
                    (
                        1.0,
                        format!("Polygenic (points={} >= {})", points, threshold),
                    )
                } else {
                    // May still inherit as hidden
                    (
                        0.5,
                        format!(
                            "Polygenic (points={} < {}, hidden possible)",
                            points, threshold
                        ),
                    )
                }
            }
            InheritanceType::Conditional { condition, chance } => {
                let condition_met = self.evaluate_condition(
                    condition,
                    parent_a_gen,
                    parent_b_gen,
                    parent_a_stats,
                    parent_b_stats,
                    parent_a_traits,
                    parent_b_traits,
                );

                if condition_met {
                    (*chance, format!("Conditional (met: {:?})", condition))
                } else {
                    (0.0, format!("Conditional (not met: {:?})", condition))
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)] // mirrors the independent per-parent inputs needed to evaluate a condition; a param struct would just move the same fields
    fn evaluate_condition(
        &self,
        condition: &ConditionalRequirement,
        parent_a_gen: u32,
        parent_b_gen: u32,
        parent_a_stats: &HashMap<String, i32>,
        parent_b_stats: &HashMap<String, i32>,
        parent_a_traits: &HashSet<String>,
        parent_b_traits: &HashSet<String>,
    ) -> bool {
        match condition {
            ConditionalRequirement::MinCombinedGeneration(min_gen) => {
                (parent_a_gen + parent_b_gen) >= *min_gen
            }
            ConditionalRequirement::BothParentsStat { stat, min_value } => {
                let a_val = parent_a_stats.get(stat).copied().unwrap_or(0);
                let b_val = parent_b_stats.get(stat).copied().unwrap_or(0);
                a_val >= *min_value && b_val >= *min_value
            }
            ConditionalRequirement::RequiredTraits(required) => required
                .iter()
                .all(|t| parent_a_traits.contains(t) || parent_b_traits.contains(t)),
        }
    }

    fn resolve_incompatibilities<R: Rng>(
        &self,
        mut visible: Vec<TraitDefinition>,
        mut hidden: Vec<TraitDefinition>,
        rng: &mut R,
    ) -> (Vec<TraitDefinition>, Vec<TraitDefinition>, Vec<String>) {
        let mut blocked = Vec::new();

        for (trait_a, trait_b) in INCOMPATIBLE_TRAITS {
            let has_a_visible = visible.iter().any(|t| t.id == *trait_a);
            let has_b_visible = visible.iter().any(|t| t.id == *trait_b);
            let has_a_hidden = hidden.iter().any(|t| t.id == *trait_a);
            let has_b_hidden = hidden.iter().any(|t| t.id == *trait_b);

            // If both are present, remove one randomly
            if (has_a_visible || has_a_hidden) && (has_b_visible || has_b_hidden) {
                let remove_a: bool = rng.gen();
                let to_remove = if remove_a { *trait_a } else { *trait_b };

                visible.retain(|t| t.id != to_remove);
                hidden.retain(|t| t.id != to_remove);
                blocked.push(to_remove.to_string());
            }
        }

        (visible, hidden, blocked)
    }

    fn enforce_trait_cap(
        &self,
        mut traits: Vec<TraitDefinition>,
        cap: usize,
    ) -> Vec<TraitDefinition> {
        if traits.len() <= cap {
            return traits;
        }

        // Sort by power (keep highest power traits)
        traits.sort_by_key(|t| std::cmp::Reverse(t.power));
        traits.truncate(cap);
        traits
    }

    /// Check for synergies based on trait combinations
    pub fn check_synergies(
        &self,
        visible_traits: &[TraitDefinition],
        hidden_traits: &[TraitDefinition],
        synergy_defs: &[SynergyDefinition],
    ) -> Vec<SynergyDefinition> {
        let all_trait_ids: HashSet<_> = visible_traits
            .iter()
            .chain(hidden_traits.iter())
            .map(|t| t.id.clone())
            .collect();

        synergy_defs
            .iter()
            .filter(|syn| {
                syn.required_traits
                    .iter()
                    .all(|req| all_trait_ids.contains(req))
            })
            .cloned()
            .collect()
    }
}

/// Convert old Trait format to new TraitDefinition
pub fn convert_legacy_trait(old: &crate::breeding_service::Trait) -> TraitDefinition {
    let inheritance_type = match old.inheritance {
        crate::breeding_service::TraitInheritance::Dominant => InheritanceType::Dominant,
        crate::breeding_service::TraitInheritance::Recessive => InheritanceType::Recessive,
        crate::breeding_service::TraitInheritance::Polygenic => {
            InheritanceType::Polygenic { threshold: 4 }
        }
        crate::breeding_service::TraitInheritance::Conditional => InheritanceType::Conditional {
            condition: ConditionalRequirement::MinCombinedGeneration(5),
            chance: 0.4,
        },
    };

    TraitDefinition {
        id: old.id.clone(),
        name: old.name.clone(),
        description: old.description.clone(),
        category: TraitCategory::Element, // Default
        inheritance_type,
        power: old.power,
        is_hidden: old.is_hidden,
        visual_keywords: Vec::new(),
        condition: None,
        polygenic_points: 0,
    }
}

/// Convert new TraitDefinition back to legacy format
pub fn convert_to_legacy_trait(new: &TraitDefinition) -> crate::breeding_service::Trait {
    let inheritance = match &new.inheritance_type {
        InheritanceType::Dominant => crate::breeding_service::TraitInheritance::Dominant,
        InheritanceType::Recessive => crate::breeding_service::TraitInheritance::Recessive,
        InheritanceType::Polygenic { .. } => crate::breeding_service::TraitInheritance::Polygenic,
        InheritanceType::Conditional { .. } => {
            crate::breeding_service::TraitInheritance::Conditional
        }
    };

    crate::breeding_service::Trait {
        id: new.id.clone(),
        name: new.name.clone(),
        description: new.description.clone(),
        inheritance,
        is_hidden: new.is_hidden,
        power: new.power,
    }
}
