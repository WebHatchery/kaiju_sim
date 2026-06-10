//! Core breeding algorithm for kaiju.
//!
//! Implements stat inheritance, trait inheritance, mutations, and
//! deterministic offspring generation.

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::data::genome::{Genome, GenomeStats, HiddenTraitData, TraitSlot};
use crate::data::{
    new_kaiju_id, now_timestamp, Kaiju, KaijuEvent, KaijuEventKind, KaijuStats, Trait,
    TraitInheritance,
};

/// Breeding configuration loaded from balance.json
#[derive(Debug, Clone)]
pub struct BreedingConfig {
    /// Power creep per generation (default: 0.01)
    pub generation_power_creep: f32,
    /// Stat variance minimum (default: 0.95)
    pub stat_variance_min: f32,
    /// Stat variance maximum (default: 1.05)
    pub stat_variance_max: f32,
    /// Visible trait inheritance chance (default: 0.45)
    pub visible_trait_inheritance_chance: f32,
    /// Hidden trait inheritance chance (default: 0.25)
    pub hidden_trait_inheritance_chance: f32,
    /// Base mutation chance (default: 0.10)
    pub mutation_chance: f32,
    /// Stat floors
    pub stat_floors: StatValues,
    /// Soft caps (base)
    pub stat_base_caps: StatValues,
    /// Hard caps (absolute maximum)
    pub stat_hard_caps: StatValues,
    /// Soft cap multiplier per generation
    pub soft_cap_multiplier_per_generation: f32,
}

/// Stat value container
#[derive(Debug, Clone, Copy)]
pub struct StatValues {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub energy: i32,
}

impl Default for BreedingConfig {
    fn default() -> Self {
        Self {
            generation_power_creep: 0.01,
            stat_variance_min: 0.95,
            stat_variance_max: 1.05,
            visible_trait_inheritance_chance: 0.45,
            hidden_trait_inheritance_chance: 0.25,
            mutation_chance: 0.10,
            stat_floors: StatValues {
                hp: 50,
                attack: 10,
                defense: 5,
                speed: 5,
                energy: 50,
            },
            stat_base_caps: StatValues {
                hp: 500,
                attack: 100,
                defense: 100,
                speed: 100,
                energy: 200,
            },
            stat_hard_caps: StatValues {
                hp: 2000,
                attack: 400,
                defense: 400,
                speed: 400,
                energy: 500,
            },
            soft_cap_multiplier_per_generation: 0.02,
        }
    }
}

/// Breeding error types
#[derive(Debug, Clone)]
pub enum BreedingError {
    /// One or both parents are dead
    ParentDead,
    /// Direct parent-child breeding is not allowed
    DirectParentChild,
    /// Kaiju is locked in tournament
    TournamentLocked,
    /// Invalid breeding rights
    InvalidRights,
    /// Same kaiju cannot breed with itself
    SameKaiju,
}

impl std::fmt::Display for BreedingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParentDead => write!(f, "One or both parents are dead"),
            Self::DirectParentChild => write!(f, "Parent-child breeding not allowed"),
            Self::TournamentLocked => write!(f, "Kaiju is locked in tournament"),
            Self::InvalidRights => write!(f, "Invalid breeding rights"),
            Self::SameKaiju => write!(f, "Cannot breed kaiju with itself"),
        }
    }
}

impl std::error::Error for BreedingError {}

/// Breeding result with offspring and inheritance details.
#[derive(Debug)]
pub struct BreedingResult {
    /// The offspring kaiju
    pub offspring: Kaiju,
    /// Mutations that occurred
    pub mutations: Vec<MutationRecord>,
    /// Traits inherited from parent A
    pub traits_from_a: Vec<String>,
    /// Traits inherited from parent B
    pub traits_from_b: Vec<String>,
}

/// Record of a mutation that occurred during breeding
#[derive(Debug, Clone)]
pub struct MutationRecord {
    pub mutation_type: MutationType,
    pub description: String,
}

/// Types of mutations
#[derive(Debug, Clone)]
pub enum MutationType {
    /// Stat boost mutation
    StatMutation { stat: String, boost_percent: f32 },
    /// New random trait
    NewTrait { trait_id: String },
    /// Existing trait power boost
    TraitPowerBoost {
        trait_id: String,
        power_increase: i32,
    },
    /// Hidden trait revealed
    HiddenUnlock { trait_id: String },
}

/// Breed two kaiju to produce offspring
pub fn breed_kaiju(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    breeding_seed: u64,
    config: &BreedingConfig,
) -> Result<BreedingResult, BreedingError> {
    // Validation
    validate_breeding(parent_a, parent_b)?;

    // Create deterministic RNG
    let mut rng = ChaCha8Rng::seed_from_u64(breeding_seed);

    // Calculate generation
    let generation = parent_a.generation.max(parent_b.generation) + 1;

    // Inherit stats
    let stats = inherit_stats(parent_a, parent_b, generation, config, &mut rng);

    // Inherit traits
    let (traits, traits_from_a, traits_from_b) =
        inherit_traits(parent_a, parent_b, config, &mut rng);

    // Apply mutations
    let (final_traits, mutations) = apply_mutations(traits, generation, config, &mut rng);

    // Generate visual seed
    let visual_seed = generate_visual_seed(parent_a, parent_b, &stats, breeding_seed);

    // Generate deterministic genome hash (Simulating SHA-256/On-chain identity)
    let mut hasher = DefaultHasher::new();
    stats.hp.hash(&mut hasher);
    stats.attack.hash(&mut hasher);
    stats.defense.hash(&mut hasher);
    stats.speed.hash(&mut hasher);
    stats.energy.hash(&mut hasher);
    visual_seed.hash(&mut hasher);
    let hash_u64 = hasher.finish();
    let genome_hash = format!("{:016x}", hash_u64);

    // Create offspring
    let offspring = Kaiju {
        id: new_kaiju_id(),
        token_id: 0,
        name: format!("Offspring of {} & {}", parent_a.name, parent_b.name),
        generation,
        created_at: now_timestamp(),
        original_breeder: parent_a.current_owner.clone(),
        parent_ids: Some((parent_a.token_id, parent_b.token_id)),
        visual_seed,
        genome_hash,
        stats,
        traits: final_traits,
        hidden_traits: Vec::new(),
        experience: 0,
        alive: true,
        current_owner: parent_a.current_owner.clone(),
        image_uri: None,
        metadata_uri: String::new(),
        tournaments_won: 0,
        history: vec![KaijuEvent::new(
            KaijuEventKind::Offspring,
            "Breeding result",
            format!("Born from {} and {}.", parent_a.name, parent_b.name),
        )
        .with_related(vec![parent_a.id, parent_b.id])],
    };

    Ok(BreedingResult {
        offspring,
        mutations,
        traits_from_a,
        traits_from_b,
    })
}

/// Validate breeding is allowed
fn validate_breeding(parent_a: &Kaiju, parent_b: &Kaiju) -> Result<(), BreedingError> {
    // Cannot breed with self
    if parent_a.id == parent_b.id {
        return Err(BreedingError::SameKaiju);
    }

    // Both must be alive
    if !parent_a.alive || !parent_b.alive {
        return Err(BreedingError::ParentDead);
    }

    // Check parent-child relationship
    if is_direct_parent_child(parent_a, parent_b) {
        return Err(BreedingError::DirectParentChild);
    }

    Ok(())
}

/// Check if two kaiju have a direct parent-child relationship
fn is_direct_parent_child(a: &Kaiju, b: &Kaiju) -> bool {
    // Check if B is A's parent
    if let Some((parent1, parent2)) = a.parent_ids {
        if parent1 == b.token_id || parent2 == b.token_id {
            return true;
        }
    }

    // Check if A is B's parent
    if let Some((parent1, parent2)) = b.parent_ids {
        if parent1 == a.token_id || parent2 == a.token_id {
            return true;
        }
    }

    false
}

/// Inherit stats from parents
fn inherit_stats(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    generation: u32,
    config: &BreedingConfig,
    rng: &mut ChaCha8Rng,
) -> KaijuStats {
    let hp = inherit_single_stat(
        parent_a.stats.hp,
        parent_b.stats.hp,
        generation,
        config.stat_floors.hp,
        config.stat_base_caps.hp,
        config.stat_hard_caps.hp,
        config,
        rng,
    );

    let attack = inherit_single_stat(
        parent_a.stats.attack,
        parent_b.stats.attack,
        generation,
        config.stat_floors.attack,
        config.stat_base_caps.attack,
        config.stat_hard_caps.attack,
        config,
        rng,
    );

    let defense = inherit_single_stat(
        parent_a.stats.defense,
        parent_b.stats.defense,
        generation,
        config.stat_floors.defense,
        config.stat_base_caps.defense,
        config.stat_hard_caps.defense,
        config,
        rng,
    );

    let speed = inherit_single_stat(
        parent_a.stats.speed,
        parent_b.stats.speed,
        generation,
        config.stat_floors.speed,
        config.stat_base_caps.speed,
        config.stat_hard_caps.speed,
        config,
        rng,
    );

    let energy = inherit_single_stat(
        parent_a.stats.energy,
        parent_b.stats.energy,
        generation,
        config.stat_floors.energy,
        config.stat_base_caps.energy,
        config.stat_hard_caps.energy,
        config,
        rng,
    );

    KaijuStats::new(hp, attack, defense, speed, energy)
}

/// Inherit a single stat with power creep and variance
fn inherit_single_stat(
    parent_a_stat: i32,
    parent_b_stat: i32,
    generation: u32,
    floor: i32,
    base_cap: i32,
    hard_cap: i32,
    config: &BreedingConfig,
    rng: &mut ChaCha8Rng,
) -> i32 {
    // Step 1: Base average
    let base = (parent_a_stat + parent_b_stat) as f32 / 2.0;

    // Step 2: Power creep multiplier
    let gen_multiplier = 1.0 + (generation as f32 * config.generation_power_creep);

    // Step 3: Variance (±5%)
    let variance = rng.gen_range(config.stat_variance_min..=config.stat_variance_max);

    // Step 4: Calculate raw value
    let raw = base * gen_multiplier * variance;

    // Step 5: Apply constraints
    let floored = raw.max(floor as f32);
    let soft_cap =
        base_cap as f32 * (1.0 + generation as f32 * config.soft_cap_multiplier_per_generation);
    let soft_capped = floored.min(soft_cap);
    let hard_capped = soft_capped.min(hard_cap as f32);

    hard_capped as i32
}

/// Inherit traits from parents
fn inherit_traits(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    config: &BreedingConfig,
    rng: &mut ChaCha8Rng,
) -> (Vec<Trait>, Vec<String>, Vec<String>) {
    let mut inherited_traits = Vec::new();
    let mut from_a = Vec::new();
    let mut from_b = Vec::new();

    // Collect all unique traits from both parents
    let mut all_traits: Vec<(&Trait, bool, bool)> = Vec::new();

    for trait_a in &parent_a.traits {
        let in_b = parent_b.traits.iter().any(|t| t.id == trait_a.id);
        all_traits.push((trait_a, true, in_b));
    }

    for trait_b in &parent_b.traits {
        if !parent_a.traits.iter().any(|t| t.id == trait_b.id) {
            all_traits.push((trait_b, false, true));
        }
    }

    // Calculate inheritance for each trait
    for (trait_def, in_a, in_b) in all_traits {
        let chance = calculate_inheritance_chance(trait_def, in_a, in_b, config);

        if rng.gen::<f32>() < chance {
            inherited_traits.push(trait_def.clone());

            if in_a {
                from_a.push(trait_def.id.clone());
            }
            if in_b && !in_a {
                from_b.push(trait_def.id.clone());
            }
        }
    }

    // Limit to 8 visible traits
    if inherited_traits.len() > 8 {
        inherited_traits.truncate(8);
    }

    (inherited_traits, from_a, from_b)
}

/// Calculate inheritance chance for a trait
fn calculate_inheritance_chance(
    trait_def: &Trait,
    parent_a_has: bool,
    parent_b_has: bool,
    config: &BreedingConfig,
) -> f32 {
    let base_rate = if trait_def.is_hidden {
        config.hidden_trait_inheritance_chance
    } else {
        config.visible_trait_inheritance_chance
    };

    match trait_def.inheritance {
        TraitInheritance::Dominant => {
            if parent_a_has && parent_b_has {
                1.0 // 100% if both parents have it
            } else if parent_a_has || parent_b_has {
                base_rate * 1.33 // ~60% for visible
            } else {
                0.0
            }
        }
        TraitInheritance::Recessive => {
            if parent_a_has && parent_b_has {
                base_rate // Standard rate if both have it
            } else if parent_a_has || parent_b_has {
                base_rate * 0.5 // Halved if only one has it
            } else {
                0.0
            }
        }
        TraitInheritance::Polygenic => {
            // Simplified: treat as recessive for now
            if parent_a_has && parent_b_has {
                0.5
            } else {
                0.0
            }
        }
        TraitInheritance::Conditional => {
            // Conditional traits have fixed chance
            0.25
        }
    }
}

/// Apply mutations to inherited traits
fn apply_mutations(
    mut traits: Vec<Trait>,
    generation: u32,
    config: &BreedingConfig,
    rng: &mut ChaCha8Rng,
) -> (Vec<Trait>, Vec<MutationRecord>) {
    let mut mutations = Vec::new();

    // Calculate mutation chance
    let mut chance = config.mutation_chance + (generation as f32 * 0.005);
    if traits.len() > 10 {
        chance += 0.05;
    }
    chance = chance.min(0.25);

    // Check if mutation occurs
    if rng.gen::<f32>() < chance {
        let mutation_roll = rng.gen::<f32>();

        if mutation_roll < 0.4 {
            // 40%: Stat mutation (handled elsewhere, record only)
            mutations.push(MutationRecord {
                mutation_type: MutationType::StatMutation {
                    stat: "hp".to_string(),
                    boost_percent: rng.gen_range(5.0..15.0),
                },
                description: "Stat mutation occurred".to_string(),
            });
        } else if mutation_roll < 0.7 {
            // 30%: New trait (simplified)
            mutations.push(MutationRecord {
                mutation_type: MutationType::NewTrait {
                    trait_id: "mutation_trait".to_string(),
                },
                description: "New trait mutation occurred".to_string(),
            });
        } else if mutation_roll < 0.9 {
            // 20%: Trait power boost
            if let Some(trait_to_boost) = traits.first_mut() {
                let boost = rng.gen_range(1..=5);
                trait_to_boost.power += boost;
                mutations.push(MutationRecord {
                    mutation_type: MutationType::TraitPowerBoost {
                        trait_id: trait_to_boost.id.clone(),
                        power_increase: boost,
                    },
                    description: format!(
                        "Trait {} power increased by {}",
                        trait_to_boost.id, boost
                    ),
                });
            }
        } else {
            // 10%: Hidden unlock
            mutations.push(MutationRecord {
                mutation_type: MutationType::HiddenUnlock {
                    trait_id: "hidden_trait".to_string(),
                },
                description: "Hidden trait unlocked".to_string(),
            });
        }
    }

    (traits, mutations)
}

/// Generate visual seed for offspring
fn generate_visual_seed(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    stats: &KaijuStats,
    breeding_seed: u64,
) -> u64 {
    // Component 1: Parent mixing (40% influence)
    let parent_mix = (parent_a.visual_seed ^ parent_b.visual_seed) & 0xFFFFFFFF00000000;

    // Component 2: Stat hash (30% influence)
    let stat_hash = ((stats.hp as u64) << 48
        | (stats.attack as u64) << 32
        | (stats.defense as u64) << 16
        | stats.speed as u64)
        & 0x00000000FFFF0000;

    // Component 3: Breeding seed (30% influence)
    let seed_component = (breeding_seed & 0xFF00) | ((breeding_seed >> 8) & 0x00FF);

    parent_mix | stat_hash | seed_component
}

/// Generate genome from kaiju
pub fn generate_genome(kaiju: &Kaiju) -> Genome {
    let stats = GenomeStats {
        hp: kaiju.stats.hp as u16,
        attack: kaiju.stats.attack as u16,
        defense: kaiju.stats.defense as u16,
        speed: kaiju.stats.speed as u16,
    };

    let trait_slots: Vec<TraitSlot> = kaiju
        .traits
        .iter()
        .take(8)
        .enumerate()
        .map(|(i, t)| TraitSlot {
            trait_id: (i + 1) as u8, // Use index as ID for now
            inheritance_mode: match t.inheritance {
                TraitInheritance::Dominant => 0,
                TraitInheritance::Recessive => 1,
                TraitInheritance::Polygenic => 2,
                TraitInheritance::Conditional => 3,
            },
            is_visible: !t.is_hidden,
        })
        .collect();

    Genome::new(
        kaiju.generation as u8,
        0,
        stats,
        trait_slots,
        HiddenTraitData::default(),
        kaiju.visual_seed,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::traits::{TraitCategory, TraitCondition};

    fn create_test_kaiju(name: &str, gen: u32, hp: i32, atk: i32) -> Kaiju {
        Kaiju {
            id: new_kaiju_id(),
            token_id: macroquad_toolkit::rng::random_u64(),
            name: name.to_string(),
            generation: gen,
            created_at: now_timestamp(),
            original_breeder: "test".to_string(),
            parent_ids: None,
            visual_seed: macroquad_toolkit::rng::random_u64(),
            genome_hash: format!("{:016x}", macroquad_toolkit::rng::random_u64()),
            stats: KaijuStats::new(hp, atk, 30, 25, 100),
            traits: vec![],
            hidden_traits: vec![],
            experience: 0,
            alive: true,
            current_owner: "test".to_string(),
            image_uri: None,
            metadata_uri: String::new(),
            tournaments_won: 0,
            history: Vec::new(),
        }
    }

    #[test]
    fn test_breed_kaiju_basic() {
        let parent_a = create_test_kaiju("Alpha", 2, 300, 60);
        let parent_b = create_test_kaiju("Beta", 3, 320, 55);
        let config = BreedingConfig::default();

        let result = breed_kaiju(&parent_a, &parent_b, 12345, &config).unwrap();

        // Generation should be max + 1
        assert_eq!(result.offspring.generation, 4);
        assert!(result.offspring.alive);
    }

    #[test]
    fn test_cannot_breed_with_self() {
        let kaiju = create_test_kaiju("Solo", 1, 200, 50);
        let config = BreedingConfig::default();

        let result = breed_kaiju(&kaiju, &kaiju, 12345, &config);
        assert!(matches!(result, Err(BreedingError::SameKaiju)));
    }

    #[test]
    fn test_stat_inheritance_variance() {
        let parent_a = create_test_kaiju("Alpha", 0, 300, 60);
        let parent_b = create_test_kaiju("Beta", 0, 320, 55);
        let config = BreedingConfig::default();

        let mut hp_values = Vec::new();
        for seed in 0..100 {
            let result = breed_kaiju(&parent_a, &parent_b, seed, &config).unwrap();
            hp_values.push(result.offspring.stats.hp);
        }

        // Check variance exists
        let min = *hp_values.iter().min().unwrap();
        let max = *hp_values.iter().max().unwrap();
        assert!(max > min, "Should have variance in HP values");
    }
}
