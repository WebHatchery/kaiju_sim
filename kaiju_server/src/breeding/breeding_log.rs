//! Breeding Debug Log System
//! Provides full admin visibility into breeding RNG rolls.
//! Based on the "Glass Box" design from BREEDING_SYSTEM_DESIGN.md

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Complete breeding event log for admin debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingLog {
    /// Unique event identifier
    pub event_id: String,
    /// Parent information
    pub parents: ParentInfo,
    /// All RNG rolls made during breeding
    pub rolls: BreedingRolls,
    /// Applied modifiers
    pub modifiers: BreedingModifiers,
    /// Final outcome
    pub outcome: BreedingOutcome,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentInfo {
    pub parent_a: ParentRecord,
    pub parent_b: ParentRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentRecord {
    pub id: Uuid,
    pub name: String,
    pub generation: u32,
    pub element: String,
    pub trait_count: usize,
    pub base_stat_total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingRolls {
    /// Stat inheritance rolls
    pub stat_inheritance: HashMap<String, StatRoll>,
    /// Trait inheritance rolls
    pub trait_inheritance: Vec<TraitRoll>,
    /// Polygenic synthesis checks
    pub polygenic_checks: Vec<PolygenicRoll>,
    /// Mutation roll
    pub mutation_roll: MutationRoll,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatRoll {
    /// The RNG roll value (0.0-1.0 normalized to variance range)
    pub roll: f32,
    /// The formula used
    pub formula: String,
    /// Base value before modifiers
    pub base_value: f32,
    /// Generation multiplier applied
    pub gen_multiplier: f32,
    /// Variance multiplier applied
    pub variance: f32,
    /// Final result after constraints
    pub result: i32,
    /// Any caps that were applied
    pub caps_applied: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitRoll {
    /// Trait name
    pub trait_name: String,
    /// Trait ID
    pub trait_id: String,
    /// Inheritance type (Dominant, Recessive, etc)
    pub inheritance_type: String,
    /// Was trait present in parent A
    pub in_parent_a: bool,
    /// Was trait present in parent B
    pub in_parent_b: bool,
    /// Calculated inheritance chance
    pub chance: f32,
    /// The actual RNG roll
    pub roll: f32,
    /// Result of inheritance
    pub result: TraitInheritResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitInheritResult {
    Inherited,
    Lost,
    InheritedHidden,
    Blocked, // Incompatibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolygenicRoll {
    /// The emergent trait that could be synthesized
    pub trait_name: String,
    /// Required component traits
    pub required_traits: Vec<String>,
    /// Whether all required traits are present
    pub components_present: bool,
    /// Activation chance
    pub activation_chance: f32,
    /// The actual RNG roll
    pub roll: f32,
    /// Whether synthesis succeeded
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRoll {
    /// Base mutation chance
    pub base_chance: f32,
    /// Modifiers applied (generation, trait count, items)
    pub modifiers: Vec<MutationModifier>,
    /// Final mutation chance
    pub final_chance: f32,
    /// The actual RNG roll
    pub roll: f32,
    /// Whether mutation occurred
    pub triggered: bool,
    /// If triggered, what type of mutation
    pub mutation_type: Option<MutationType>,
    /// If triggered, what was affected
    pub mutation_result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationModifier {
    pub source: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationType {
    StatBoost,
    NewTrait,
    TraitPowerBoost,
    HiddenUnlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingModifiers {
    /// Items used in breeding
    pub items_used: Vec<String>,
    /// Facility bonus (if any)
    pub facility_bonus: f32,
    /// Breeder experience bonus
    pub breeder_bonus: f32,
    /// Elemental essence applied
    pub element_forced: Option<String>,
    /// Genetic stabilizer active
    pub stabilizer_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreedingOutcome {
    /// Offspring ID
    pub id: Uuid,
    /// Offspring name
    pub name: String,
    /// Genome hash
    pub genome_hash: String,
    /// Generation
    pub generation: u32,
    /// Calculated rarity
    pub rarity: String,
    /// Rarity calculation explanation
    pub rarity_calc: String,
    /// Final visible traits
    pub visible_traits: Vec<String>,
    /// Final hidden traits  
    pub hidden_traits: Vec<String>,
    /// Final stats
    pub stats: HashMap<String, i32>,
    /// Primary element
    pub element: String,
    /// Hybrid element (if applicable)
    pub hybrid_element: Option<String>,
    /// Gestation time calculated (hours)
    pub gestation_hours: f32,
    /// Maturation time calculated (hours)
    pub maturation_hours: f32,
}

impl BreedingLog {
    pub fn new(parent_a_id: Uuid, parent_b_id: Uuid) -> Self {
        Self {
            event_id: format!(
                "breed_{}",
                Uuid::new_v4().to_string().split('-').next().unwrap()
            ),
            parents: ParentInfo {
                parent_a: ParentRecord {
                    id: parent_a_id,
                    name: String::new(),
                    generation: 0,
                    element: String::new(),
                    trait_count: 0,
                    base_stat_total: 0,
                },
                parent_b: ParentRecord {
                    id: parent_b_id,
                    name: String::new(),
                    generation: 0,
                    element: String::new(),
                    trait_count: 0,
                    base_stat_total: 0,
                },
            },
            rolls: BreedingRolls {
                stat_inheritance: HashMap::new(),
                trait_inheritance: Vec::new(),
                polygenic_checks: Vec::new(),
                mutation_roll: MutationRoll {
                    base_chance: 0.0,
                    modifiers: Vec::new(),
                    final_chance: 0.0,
                    roll: 0.0,
                    triggered: false,
                    mutation_type: None,
                    mutation_result: None,
                },
            },
            modifiers: BreedingModifiers {
                items_used: Vec::new(),
                facility_bonus: 0.0,
                breeder_bonus: 0.0,
                element_forced: None,
                stabilizer_active: false,
            },
            outcome: BreedingOutcome {
                id: Uuid::nil(),
                name: String::new(),
                genome_hash: String::new(),
                generation: 0,
                rarity: String::new(),
                rarity_calc: String::new(),
                visible_traits: Vec::new(),
                hidden_traits: Vec::new(),
                stats: HashMap::new(),
                element: String::new(),
                hybrid_element: None,
                gestation_hours: 0.0,
                maturation_hours: 0.0,
            },
            timestamp: chrono::Utc::now(),
        }
    }

    /// Convert to formatted JSON for logging
    pub fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| format!("{:?}", self))
    }

    /// Log to tracing at debug level
    pub fn log_debug(&self) {
        tracing::debug!("BreedingLog: {}", self.to_json_string());
    }

    /// Log to tracing at info level (summarized)
    pub fn log_info(&self) {
        tracing::info!(
            "Breeding Complete: {} x {} -> {} (Gen {}, {})",
            self.parents.parent_a.name,
            self.parents.parent_b.name,
            self.outcome.name,
            self.outcome.generation,
            self.outcome.rarity
        );

        if self.rolls.mutation_roll.triggered {
            tracing::info!(
                "  MUTATION: {:?} -> {}",
                self.rolls.mutation_roll.mutation_type,
                self.rolls
                    .mutation_roll
                    .mutation_result
                    .as_deref()
                    .unwrap_or("unknown")
            );
        }

        let inherited_count = self
            .rolls
            .trait_inheritance
            .iter()
            .filter(|t| {
                matches!(
                    t.result,
                    TraitInheritResult::Inherited | TraitInheritResult::InheritedHidden
                )
            })
            .count();
        let lost_count = self
            .rolls
            .trait_inheritance
            .iter()
            .filter(|t| matches!(t.result, TraitInheritResult::Lost))
            .count();

        tracing::info!(
            "  Traits: {} inherited, {} lost",
            inherited_count,
            lost_count
        );
    }
}
