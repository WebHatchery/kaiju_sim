use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing;
use uuid::Uuid; // Added for logging

// --- Domain Types (Mirrors Client) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaijuStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub energy: i32,
}

impl KaijuStats {
    pub fn new(hp: i32, attack: i32, defense: i32, speed: i32, energy: i32) -> Self {
        Self {
            hp,
            attack,
            defense,
            speed,
            energy,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub id: String,
    pub name: String,
    pub description: String,
    pub inheritance: TraitInheritance,
    pub is_hidden: bool,
    pub power: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TraitInheritance {
    Dominant,
    Recessive,
    Polygenic,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaijuData {
    pub id: Uuid,
    pub name: String,
    pub generation: u32,
    pub parent_ids: Option<(Uuid, Uuid)>,
    pub visual_seed: u64,
    pub genome_hash: String,
    pub stats: KaijuStats,
    pub traits: Vec<Trait>,
    pub owner_id: Uuid,
    pub image_url: String, // Added image URL
    #[serde(default)]
    pub tournaments_won: i32,
}

#[derive(Debug, Clone)]
pub struct BreedingConfig {
    pub generation_power_creep: f32,
    pub stat_variance_min: f32,
    pub stat_variance_max: f32,
    pub mutation_chance: f32,
    // Add other fields as needed...
}

impl Default for BreedingConfig {
    fn default() -> Self {
        Self {
            generation_power_creep: 0.01,
            stat_variance_min: 0.95,
            stat_variance_max: 1.05,
            mutation_chance: 0.10,
        }
    }
}

#[derive(Debug)]
pub struct BreedingResult {
    pub offspring: KaijuData,
    pub mutations: Vec<String>,
}

pub enum BreedingError {
    ParentDead,
    SameKaiju,
    // ...
}

// --- Service ---

pub struct BreedingService;

impl Default for BreedingService {
    fn default() -> Self {
        Self::new()
    }
}

impl BreedingService {
    pub fn new() -> Self {
        Self
    }

    pub fn breed(
        &self,
        parent_a: &KaijuData,
        parent_b: &KaijuData,
        seed: u64,
    ) -> Result<BreedingResult, String> {
        // Using String error for simplicity in port
        let config = BreedingConfig::default();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Generation
        let generation = parent_a.generation.max(parent_b.generation) + 1;

        // Stats Logic (Simplified Port)
        let hp = self.inherit_stat(
            "HP",
            parent_a.stats.hp,
            parent_b.stats.hp,
            generation,
            &config,
            &mut rng,
        );
        let attack = self.inherit_stat(
            "Attack",
            parent_a.stats.attack,
            parent_b.stats.attack,
            generation,
            &config,
            &mut rng,
        );
        let defense = self.inherit_stat(
            "Defense",
            parent_a.stats.defense,
            parent_b.stats.defense,
            generation,
            &config,
            &mut rng,
        );
        let speed = self.inherit_stat(
            "Speed",
            parent_a.stats.speed,
            parent_b.stats.speed,
            generation,
            &config,
            &mut rng,
        );
        let energy = self.inherit_stat(
            "Energy",
            parent_a.stats.energy,
            parent_b.stats.energy,
            generation,
            &config,
            &mut rng,
        );

        let stats = KaijuStats::new(hp, attack, defense, speed, energy);
        tracing::debug!("Generated Stats: {:?}", stats);

        // Traits Logic (Placeholder for full logic)
        let traits = self.inherit_traits(&parent_a.traits, &parent_b.traits, &mut rng);

        // Mutation (Simplified)
        let mutations = Vec::new(); // TODO: Add mutation logic

        // Visual Seed
        let visual_seed = rng.gen::<u64>();

        // Genome Hash (SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(stats.hp.to_be_bytes());
        hasher.update(stats.attack.to_be_bytes());
        hasher.update(stats.defense.to_be_bytes());
        hasher.update(stats.speed.to_be_bytes());
        hasher.update(visual_seed.to_be_bytes());
        let result_hash = hasher.finalize();
        let genome_hash = hex::encode(result_hash);

        tracing::info!("Minted Offspring: Hash={}", genome_hash);

        // Select random asset for now
        let assets = [
            "kaiju_bipedal_neutral_1768091093175.png",
            "kaiju_electric_elemental_1768091175509.png",
            "kaiju_fire_elemental_1768091138860.png",
            "kaiju_ice_elemental_1768091156648.png",
            "kaiju_quadruped_neutral_1768091073894.png",
            "kaiju_serpentine_neutral_1768091108255.png",
        ];
        let asset_idx = rng.gen_range(0..assets.len());
        let selected_asset = assets[asset_idx];

        let offspring = KaijuData {
            id: Uuid::new_v4(),
            name: format!("Offspring of {} & {}", parent_a.name, parent_b.name),
            generation,
            parent_ids: Some((parent_a.id, parent_b.id)),
            visual_seed,
            genome_hash,
            stats,
            traits,
            owner_id: parent_a.owner_id, // Default to A
            image_url: format!(
                "http://localhost:3000/assets/sprites/kaiju/{}",
                selected_asset
            ),
            tournaments_won: 0,
        };

        Ok(BreedingResult {
            offspring,
            mutations,
        })
    }

    fn inherit_stat(
        &self,
        name: &str,
        a: i32,
        b: i32,
        gen: u32,
        config: &BreedingConfig,
        rng: &mut ChaCha8Rng,
    ) -> i32 {
        let base = (a + b) as f32 / 2.0;
        let creep = 1.0 + (gen as f32 * config.generation_power_creep);
        let variance = rng.gen_range(config.stat_variance_min..=config.stat_variance_max);
        let result = (base * creep * variance) as i32;
        tracing::debug!(
            "Stat {}: Base={} Creep={} Var={} -> {}",
            name,
            base,
            creep,
            variance,
            result
        );
        result
    }

    fn inherit_traits(
        &self,
        traits_a: &[Trait],
        traits_b: &[Trait],
        rng: &mut ChaCha8Rng,
    ) -> Vec<Trait> {
        // Simplified inheritance: random mix
        let mut mixed = Vec::new();
        mixed.extend_from_slice(traits_a);
        // Deduplicate logic would go here
        mixed
    }
}
