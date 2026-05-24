//! Local MVP helpers for training, AI opponents, and config adaptation.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::data::loader;
use crate::data::{Kaiju, KaijuStats, TrainingFocus, Trait, TraitCategory};
use crate::engine::{breeding, combat};

/// Result of one targeted training session.
#[derive(Debug, Clone)]
pub struct TrainingOutcome {
    pub focus: TrainingFocus,
    pub stat_gain: i32,
    pub xp_gain: u32,
    pub cost: i64,
}

/// Convert JSON combat balance into the simulation engine config.
pub fn combat_config_from_balance(config: &loader::CombatConfig) -> combat::CombatConfig {
    combat::CombatConfig {
        defense_scaling: config.base_damage_formula.defense_coefficient,
        minimum_damage: config.base_damage_formula.minimum_damage,
        variance_min: config.damage_variance_min,
        variance_max: config.damage_variance_max,
        max_turns: config.max_battle_turns,
    }
}

/// Convert JSON breeding balance into the breeding engine config.
pub fn breeding_config_from_balance(config: &loader::BreedingConfig) -> breeding::BreedingConfig {
    breeding::BreedingConfig {
        generation_power_creep: config.generation_power_creep,
        stat_variance_min: config.stat_variance_min,
        stat_variance_max: config.stat_variance_max,
        visible_trait_inheritance_chance: config.visible_trait_inheritance_chance,
        hidden_trait_inheritance_chance: config.hidden_trait_inheritance_chance,
        mutation_chance: config.mutation_chance,
        stat_floors: breeding::StatValues {
            hp: stat_or(&config.stat_floors, "hp", 50),
            attack: stat_or(&config.stat_floors, "attack", 10),
            defense: stat_or(&config.stat_floors, "defense", 5),
            speed: stat_or(&config.stat_floors, "speed", 5),
            energy: stat_or(&config.stat_floors, "energy", 50),
        },
        stat_base_caps: breeding::StatValues {
            hp: stat_or(&config.stat_base_caps, "hp", 500),
            attack: stat_or(&config.stat_base_caps, "attack", 100),
            defense: stat_or(&config.stat_base_caps, "defense", 100),
            speed: stat_or(&config.stat_base_caps, "speed", 100),
            energy: stat_or(&config.stat_base_caps, "energy", 200),
        },
        stat_hard_caps: breeding::StatValues {
            hp: stat_or(&config.stat_hard_caps, "hp", 2000),
            attack: stat_or(&config.stat_hard_caps, "attack", 400),
            defense: stat_or(&config.stat_hard_caps, "defense", 400),
            speed: stat_or(&config.stat_hard_caps, "speed", 400),
            energy: stat_or(&config.stat_hard_caps, "energy", 500),
        },
        soft_cap_multiplier_per_generation: config.soft_cap_multiplier_per_generation,
    }
}

/// Roll a deterministic training result from JSON MVP rules.
pub fn roll_training(
    focus: TrainingFocus,
    seed: u64,
    config: &loader::TrainingMvpConfig,
) -> TrainingOutcome {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let min = config.stat_gain_min.min(config.stat_gain_max);
    let max = config.stat_gain_min.max(config.stat_gain_max);

    TrainingOutcome {
        focus,
        stat_gain: rng.gen_range(min..=max),
        xp_gain: config.xp_gain,
        cost: config.cost,
    }
}

/// Generate a local AI opponent near the selected kaiju's power band.
pub fn generate_opponent(player_kaiju: &Kaiju, seed: u64, traits: &[Trait]) -> Kaiju {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let name = opponent_name(&mut rng);
    let pressure = rng.gen_range(0.88..=1.14);
    let stat_jitter = |value: i32, rng: &mut ChaCha8Rng| -> i32 {
        ((value as f32 * pressure * rng.gen_range(0.92..=1.08)) as i32).max(10)
    };

    let mut selected_traits = Vec::new();
    let visible_traits: Vec<_> = traits
        .iter()
        .filter(|trait_def| !trait_def.is_hidden)
        .filter(|trait_def| {
            matches!(
                trait_def.category,
                TraitCategory::Element | TraitCategory::Modifier
            )
        })
        .cloned()
        .collect();
    if !visible_traits.is_empty() {
        selected_traits.push(visible_traits[rng.gen_range(0..visible_traits.len())].clone());
    }

    let mut opponent = Kaiju::new_wild(
        name,
        KaijuStats::new(
            stat_jitter(player_kaiju.stats.hp, &mut rng),
            stat_jitter(player_kaiju.stats.attack, &mut rng),
            stat_jitter(player_kaiju.stats.defense, &mut rng),
            stat_jitter(player_kaiju.stats.speed, &mut rng),
            stat_jitter(player_kaiju.stats.energy, &mut rng),
        ),
        selected_traits,
    );
    opponent.current_owner = "arena".to_string();
    opponent.image_uri = Some(image_for_kaiju(&opponent));
    opponent
}

/// Choose a local sprite based on the kaiju's strongest visible signal.
pub fn image_for_kaiju(kaiju: &Kaiju) -> String {
    for trait_def in &kaiju.traits {
        if trait_def.name.contains("Fire") {
            return "assets/sprites/kaiju/kaiju_fire_elemental_1768091138860.png".to_string();
        }
        if trait_def.name.contains("Electric") {
            return "assets/sprites/kaiju/kaiju_electric_elemental_1768091175509.png".to_string();
        }
        if trait_def.name.contains("Aqua") || trait_def.name.contains("Water") {
            return "assets/sprites/kaiju/kaiju_serpentine_neutral_1768091108255.png".to_string();
        }
        if trait_def.name.contains("Armor") {
            return "assets/sprites/kaiju/kaiju_quadruped_neutral_1768091073894.png".to_string();
        }
        if trait_def.name.contains("Swift") {
            return "assets/sprites/kaiju/kaiju_bipedal_neutral_1768091093175.png".to_string();
        }
    }

    match kaiju.visual_seed % 6 {
        0 => "assets/sprites/kaiju/kaiju_fire_elemental_1768091138860.png",
        1 => "assets/sprites/kaiju/kaiju_ice_elemental_1768091156648.png",
        2 => "assets/sprites/kaiju/kaiju_electric_elemental_1768091175509.png",
        3 => "assets/sprites/kaiju/kaiju_serpentine_neutral_1768091108255.png",
        4 => "assets/sprites/kaiju/kaiju_quadruped_neutral_1768091073894.png",
        _ => "assets/sprites/kaiju/kaiju_bipedal_neutral_1768091093175.png",
    }
    .to_string()
}

pub fn offspring_name(parent_a: &Kaiju, parent_b: &Kaiju, token_id: u64) -> String {
    let left = parent_a.name.chars().take(3).collect::<String>();
    let right = parent_b.name.chars().rev().take(3).collect::<String>();
    format!(
        "{}{}-{}",
        left,
        right.chars().rev().collect::<String>(),
        token_id
    )
}

fn stat_or(values: &std::collections::HashMap<String, i32>, key: &str, fallback: i32) -> i32 {
    values.get(key).copied().unwrap_or(fallback)
}

fn opponent_name(rng: &mut ChaCha8Rng) -> String {
    let prefixes = ["Razor", "Mire", "Obsidian", "Crater", "Tempest", "Ashen"];
    let suffixes = ["maw", "horn", "spine", "claw", "hide", "crest"];
    format!(
        "{}{}",
        prefixes[rng.gen_range(0..prefixes.len())],
        suffixes[rng.gen_range(0..suffixes.len())]
    )
}
