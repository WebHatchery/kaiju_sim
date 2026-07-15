//! Advanced Breeding Service
//! Main entry point for the enhanced breeding system.

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    convert_legacy_trait, convert_to_legacy_trait, determine_offspring_element,
    get_element_visual_keywords, BreedingConfig, BreedingLog, BreedingMaterial, BreedingModifiers,
    BreedingOutcome, BreedingRolls, ElementType, KaijuRarity, KaijuStats as NewKaijuStats,
    MutationProcessor, ParentRecord, PolygenicRoll, StatCalculator, TraitDefinition,
    TraitInheritanceProcessor, TraitRegistry,
};

use crate::breeding_service::{BreedingResult, KaijuData, KaijuStats, Trait};
use crate::name_generator::generate_kaiju_name;

/// Enhanced breeding service with full genetic simulation
pub struct AdvancedBreedingService {
    config: BreedingConfig,
    trait_registry: TraitRegistry,
}

impl AdvancedBreedingService {
    pub fn new() -> Self {
        Self {
            config: BreedingConfig::default(),
            trait_registry: TraitRegistry::new(),
        }
    }

    pub fn with_config(config: BreedingConfig) -> Self {
        Self {
            config,
            trait_registry: TraitRegistry::new(),
        }
    }

    /// Main breeding function with full logging
    pub fn breed(
        &self,
        parent_a: &KaijuData,
        parent_b: &KaijuData,
        seed: u64,
        materials: Vec<BreedingMaterial>,
    ) -> Result<(BreedingResult, BreedingLog), String> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut log = BreedingLog::new(parent_a.id, parent_b.id);

        // Parse materials
        let mutation_catalyst = materials
            .iter()
            .any(|m| matches!(m, BreedingMaterial::MutationCatalyst));
        let genetic_stabilizer = materials
            .iter()
            .any(|m| matches!(m, BreedingMaterial::GeneticStabilizer));
        let fertility_idol = materials
            .iter()
            .any(|m| matches!(m, BreedingMaterial::FertilityIdol));
        let forced_element = materials.iter().find_map(|m| {
            if let BreedingMaterial::ElementalEssence(e) = m {
                Some(*e)
            } else {
                None
            }
        });

        // Update log modifiers
        log.modifiers = BreedingModifiers {
            items_used: materials.iter().map(|m| format!("{:?}", m)).collect(),
            facility_bonus: 0.0,
            breeder_bonus: 0.0,
            element_forced: forced_element.map(|e| format!("{:?}", e)),
            stabilizer_active: genetic_stabilizer,
        };

        // Record parent info
        log.parents.parent_a = self.make_parent_record(parent_a);
        log.parents.parent_b = self.make_parent_record(parent_b);

        // Calculate generation
        let generation = parent_a.generation.max(parent_b.generation) + 1;

        // Convert legacy traits to new format
        let parent_a_traits: Vec<TraitDefinition> =
            parent_a.traits.iter().map(convert_legacy_trait).collect();
        let parent_b_traits: Vec<TraitDefinition> =
            parent_b.traits.iter().map(convert_legacy_trait).collect();

        // Trait inheritance
        let trait_processor = TraitInheritanceProcessor::new(self.config.clone());
        let parent_a_stats_map = self.stats_to_map(&parent_a.stats);
        let parent_b_stats_map = self.stats_to_map(&parent_b.stats);

        let (mut visible_traits, mut hidden_traits, trait_rolls) = trait_processor
            .process_inheritance(
                &parent_a_traits,
                &parent_b_traits,
                parent_a.generation,
                parent_b.generation,
                &parent_a_stats_map,
                &parent_b_stats_map,
                &mut rng,
            );

        log.rolls.trait_inheritance = trait_rolls;

        // Check synergies
        let synergies = trait_processor.check_synergies(
            &visible_traits,
            &hidden_traits,
            self.trait_registry.get_synergies(),
        );
        for syn in &synergies {
            let roll: f32 = rng.gen();
            let success = roll < syn.activation_chance;
            log.rolls.polygenic_checks.push(PolygenicRoll {
                trait_name: syn.name.clone(),
                required_traits: syn.required_traits.clone(),
                components_present: true,
                activation_chance: syn.activation_chance,
                roll,
                success,
            });
        }

        // Stat calculation
        let stat_calc = StatCalculator::new(self.config.clone());
        let parent_a_new_stats = NewKaijuStats::new(
            parent_a.stats.hp,
            parent_a.stats.attack,
            parent_a.stats.defense,
            parent_a.stats.speed,
            parent_a.stats.energy,
        );
        let parent_b_new_stats = NewKaijuStats::new(
            parent_b.stats.hp,
            parent_b.stats.attack,
            parent_b.stats.defense,
            parent_b.stats.speed,
            parent_b.stats.energy,
        );

        // Element determination
        let parent_a_elem = self.detect_element(&parent_a.traits);
        let parent_b_elem = self.detect_element(&parent_b.traits);
        let elem_roll: f32 = rng.gen();
        let (primary_element, hybrid_element) =
            determine_offspring_element(parent_a_elem, parent_b_elem, elem_roll, forced_element);

        let (mut stats, stat_rolls) = stat_calc.calculate_offspring_stats(
            &parent_a_new_stats,
            &parent_b_new_stats,
            generation,
            hybrid_element.as_ref(),
            genetic_stabilizer,
            &mut rng,
        );
        log.rolls.stat_inheritance = stat_rolls;

        // Mutation processing
        let mutation_proc = MutationProcessor::new(self.config.clone());
        let total_parent_traits = parent_a.traits.len() + parent_b.traits.len();
        let mut stats_map = stats.to_map();
        let (mutation_result, mutation_roll) = mutation_proc.process_mutation(
            generation,
            total_parent_traits,
            mutation_catalyst,
            &mut visible_traits,
            &mut hidden_traits,
            &mut stats_map,
            &mut rng,
        );
        stats = NewKaijuStats::from_map(&stats_map);
        log.rolls.mutation_roll = mutation_roll;

        // Build genome hash
        let visual_seed = rng.gen::<u64>();
        let mut hasher = Sha256::new();
        hasher.update(stats.hp.to_be_bytes());
        hasher.update(stats.attack.to_be_bytes());
        hasher.update(stats.defense.to_be_bytes());
        hasher.update(stats.speed.to_be_bytes());
        hasher.update(visual_seed.to_be_bytes());
        let genome_hash = hex::encode(hasher.finalize());

        // Calculate rarity
        let total_trait_power: i32 = visible_traits.iter().map(|t| t.power).sum::<i32>()
            + hidden_traits.iter().map(|t| t.power).sum::<i32>();
        let rarity = KaijuRarity::calculate(total_trait_power, stats.total());

        // Calculate timing
        let gestation = stat_calc.calculate_gestation_hours(generation, total_trait_power);
        let maturation = stat_calc.calculate_maturation_hours(generation);
        let gestation = if fertility_idol {
            gestation * 0.5
        } else {
            gestation
        };

        // Convert traits back to legacy format
        let final_traits: Vec<Trait> = visible_traits
            .iter()
            .chain(hidden_traits.iter())
            .map(convert_to_legacy_trait)
            .collect();

        // Build offspring
        let offspring = KaijuData {
            id: Uuid::new_v4(),
            name: generate_kaiju_name(),
            generation,
            parent_ids: Some((parent_a.id, parent_b.id)),
            visual_seed,
            genome_hash: genome_hash.clone(),
            stats: KaijuStats {
                hp: stats.hp,
                attack: stats.attack,
                defense: stats.defense,
                speed: stats.speed,
                energy: stats.energy,
            },
            traits: final_traits,
            owner_id: parent_a.owner_id,
            image_url: String::new(),
            tournaments_won: 0,
        };

        // Complete log outcome
        log.outcome = BreedingOutcome {
            id: offspring.id,
            name: offspring.name.clone(),
            genome_hash,
            generation,
            rarity: format!("{:?}", rarity),
            rarity_calc: format!(
                "trait_power({}) + stat_total({}) / 10 = {}",
                total_trait_power,
                stats.total(),
                total_trait_power + stats.total() / 10
            ),
            visible_traits: visible_traits.iter().map(|t| t.name.clone()).collect(),
            hidden_traits: hidden_traits.iter().map(|t| t.name.clone()).collect(),
            stats: stats.to_map(),
            element: format!("{:?}", primary_element),
            hybrid_element: hybrid_element.map(|h| h.name),
            gestation_hours: gestation,
            maturation_hours: maturation,
        };

        // Log summary
        log.log_info();
        tracing::debug!("{}", log.to_json_string());

        let mutations = if mutation_result.occurred {
            vec![mutation_result.affected_target.unwrap_or_default()]
        } else {
            Vec::new()
        };

        Ok((
            BreedingResult {
                offspring,
                mutations,
            },
            log,
        ))
    }

    fn make_parent_record(&self, k: &KaijuData) -> ParentRecord {
        ParentRecord {
            id: k.id,
            name: k.name.clone(),
            generation: k.generation,
            element: format!("{:?}", self.detect_element(&k.traits)),
            trait_count: k.traits.len(),
            base_stat_total: k.stats.hp
                + k.stats.attack
                + k.stats.defense
                + k.stats.speed
                + k.stats.energy,
        }
    }

    fn stats_to_map(&self, s: &KaijuStats) -> HashMap<String, i32> {
        let mut m = HashMap::new();
        m.insert("hp".into(), s.hp);
        m.insert("attack".into(), s.attack);
        m.insert("defense".into(), s.defense);
        m.insert("speed".into(), s.speed);
        m.insert("energy".into(), s.energy);
        m
    }

    fn detect_element(&self, traits: &[Trait]) -> ElementType {
        for t in traits {
            if let Some(e) = ElementType::from_name(&t.name) {
                if e != ElementType::Neutral {
                    return e;
                }
            }
        }
        ElementType::Neutral
    }
}

impl Default for AdvancedBreedingService {
    fn default() -> Self {
        Self::new()
    }
}
