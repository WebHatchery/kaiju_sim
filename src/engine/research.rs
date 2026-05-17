//! Research facility and progressive genome decoding.
//!
//! The research system allows players to progressively decode kaiju genomes
//! and discover hidden traits through facility upgrades and battle experience.

use serde::{Deserialize, Serialize};

use crate::data::genome::Genome;

/// Research facility levels and capabilities
#[derive(Debug, Clone)]
pub struct ResearchFacility {
    /// Current facility level (0-5)
    pub level: u8,
}

impl Default for ResearchFacility {
    fn default() -> Self {
        Self { level: 0 }
    }
}

impl ResearchFacility {
    /// Create a new facility at the specified level
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(5),
        }
    }

    /// Get available capabilities at current level
    pub fn capabilities(&self) -> Vec<ResearchCapability> {
        let mut caps = Vec::new();

        if self.level >= 1 {
            caps.push(ResearchCapability::DecodeLayer1Basic);
        }
        if self.level >= 2 {
            caps.push(ResearchCapability::DecodeLayer1Advanced);
        }
        if self.level >= 3 {
            caps.push(ResearchCapability::DecodeLayer1Full);
        }
        if self.level >= 4 {
            caps.push(ResearchCapability::HiddenTraitDetection);
        }
        if self.level >= 5 {
            caps.push(ResearchCapability::ExactDecoding);
        }

        caps
    }

    /// Check if facility can decode at the specified layer
    pub fn can_decode(&self, layer: DecodingLayer) -> bool {
        match layer {
            DecodingLayer::Layer0 => true, // Always available
            DecodingLayer::Layer1Basic => self.level >= 1,
            DecodingLayer::Layer1Advanced => self.level >= 2,
            DecodingLayer::Layer1Full => self.level >= 3,
            DecodingLayer::Layer2 => self.level >= 4,
            DecodingLayer::Layer3 => self.level >= 5,
        }
    }

    /// Upgrade cost for next level
    pub fn upgrade_cost(&self) -> Option<u32> {
        match self.level {
            0 => Some(100),
            1 => Some(250),
            2 => Some(500),
            3 => Some(1000),
            4 => Some(2000),
            _ => None, // Max level
        }
    }
}

/// Research capabilities unlocked by facility levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResearchCapability {
    /// Level 1: Header + stat ranges
    DecodeLayer1Basic,
    /// Level 2: Trait categories
    DecodeLayer1Advanced,
    /// Level 3: Full structure decode
    DecodeLayer1Full,
    /// Level 4: Scan for hidden traits
    HiddenTraitDetection,
    /// Level 5: Full genome decode
    ExactDecoding,
}

/// Decoding layers for progressive revelation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodingLayer {
    /// Raw hex string only
    Layer0,
    /// Basic structure (header, stat ranges)
    Layer1Basic,
    /// Advanced structure (trait categories)
    Layer1Advanced,
    /// Full structure (all Layer 1 info)
    Layer1Full,
    /// Battle-based knowledge
    Layer2,
    /// Complete decode
    Layer3,
}

/// Decoded genome data at various revelation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedGenome {
    /// Raw hex representation
    pub raw_hex: String,
    /// Layer 1: Generation number
    pub generation: Option<u8>,
    /// Layer 1: Mutation count
    pub mutation_count: Option<u8>,
    /// Layer 1: Stat ranges (min, max)
    pub stat_ranges: Option<StatRanges>,
    /// Layer 2: Trait count
    pub trait_count: Option<u8>,
    /// Layer 3: Exact stats
    pub exact_stats: Option<ExactStats>,
}

/// Stat ranges revealed at Layer 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatRanges {
    pub hp: (u16, u16),
    pub attack: (u16, u16),
    pub defense: (u16, u16),
    pub speed: (u16, u16),
}

/// Exact stats revealed at Layer 3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExactStats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
}

/// Decode a genome at the specified layer
pub fn decode_genome(genome: &Genome, layer: DecodingLayer) -> DecodedGenome {
    let raw_hex = genome.to_hex();

    match layer {
        DecodingLayer::Layer0 => DecodedGenome {
            raw_hex,
            generation: None,
            mutation_count: None,
            stat_ranges: None,
            trait_count: None,
            exact_stats: None,
        },

        DecodingLayer::Layer1Basic => DecodedGenome {
            raw_hex,
            generation: Some(genome.header.generation),
            mutation_count: Some(genome.header.mutation_count),
            stat_ranges: Some(calculate_stat_ranges(&genome.stats, 0.15)), // ±15% range
            trait_count: None,
            exact_stats: None,
        },

        DecodingLayer::Layer1Advanced => DecodedGenome {
            raw_hex,
            generation: Some(genome.header.generation),
            mutation_count: Some(genome.header.mutation_count),
            stat_ranges: Some(calculate_stat_ranges(&genome.stats, 0.10)), // ±10% range
            trait_count: Some(genome.trait_slots.len() as u8),
            exact_stats: None,
        },

        DecodingLayer::Layer1Full => DecodedGenome {
            raw_hex,
            generation: Some(genome.header.generation),
            mutation_count: Some(genome.header.mutation_count),
            stat_ranges: Some(calculate_stat_ranges(&genome.stats, 0.05)), // ±5% range
            trait_count: Some(genome.trait_slots.len() as u8),
            exact_stats: None,
        },

        DecodingLayer::Layer2 => DecodedGenome {
            raw_hex,
            generation: Some(genome.header.generation),
            mutation_count: Some(genome.header.mutation_count),
            stat_ranges: Some(calculate_stat_ranges(&genome.stats, 0.02)), // ±2% range
            trait_count: Some(genome.trait_slots.len() as u8),
            exact_stats: None,
        },

        DecodingLayer::Layer3 => DecodedGenome {
            raw_hex,
            generation: Some(genome.header.generation),
            mutation_count: Some(genome.header.mutation_count),
            stat_ranges: None, // Not needed with exact stats
            trait_count: Some(genome.trait_slots.len() as u8),
            exact_stats: Some(ExactStats {
                hp: genome.stats.hp,
                attack: genome.stats.attack,
                defense: genome.stats.defense,
                speed: genome.stats.speed,
            }),
        },
    }
}

/// Calculate stat ranges with given variance
fn calculate_stat_ranges(stats: &crate::data::genome::GenomeStats, variance: f32) -> StatRanges {
    let range = |stat: u16| {
        let min = (stat as f32 * (1.0 - variance)) as u16;
        let max = (stat as f32 * (1.0 + variance)) as u16;
        (min, max)
    };

    StatRanges {
        hp: range(stats.hp),
        attack: range(stats.attack),
        defense: range(stats.defense),
        speed: range(stats.speed),
    }
}

/// Trait knowledge accumulated through battles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitKnowledge {
    /// Trait ID
    pub trait_id: String,
    /// Number of observations
    pub observations: u32,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f32,
    /// Estimated power range
    pub power_estimate: (i32, i32),
    /// Hints about activation conditions
    pub condition_hints: Vec<String>,
}

impl TraitKnowledge {
    /// Create new knowledge with no observations
    pub fn new(trait_id: String) -> Self {
        Self {
            trait_id,
            observations: 0,
            confidence: 0.0,
            power_estimate: (0, 30), // Wide initial range
            condition_hints: Vec::new(),
        }
    }

    /// Add an observation and update knowledge
    pub fn add_observation(&mut self, observed_power: i32, condition_hint: Option<String>) {
        self.observations += 1;
        self.confidence = calculate_confidence(self.observations);

        // Narrow power estimate
        let (min, max) = self.power_estimate;
        self.power_estimate = (min.max(observed_power - 5), max.min(observed_power + 5));

        // Add condition hint if new
        if let Some(hint) = condition_hint {
            if !self.condition_hints.contains(&hint) {
                self.condition_hints.push(hint);
            }
        }
    }
}

/// Calculate confidence based on observation count
fn calculate_confidence(observations: u32) -> f32 {
    match observations {
        0 => 0.0,
        1..=5 => 0.3 + (observations as f32 * 0.1),
        6..=15 => 0.8 + ((observations - 6) as f32 * 0.015),
        _ => (0.8 + ((observations - 6) as f32 * 0.015)).min(0.95),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facility_capabilities() {
        let facility = ResearchFacility::new(3);
        let caps = facility.capabilities();

        assert!(caps.contains(&ResearchCapability::DecodeLayer1Basic));
        assert!(caps.contains(&ResearchCapability::DecodeLayer1Advanced));
        assert!(caps.contains(&ResearchCapability::DecodeLayer1Full));
        assert!(!caps.contains(&ResearchCapability::HiddenTraitDetection));
    }

    #[test]
    fn test_confidence_calculation() {
        assert_eq!(calculate_confidence(0), 0.0);
        assert!(calculate_confidence(5) > 0.5);
        assert!(calculate_confidence(20) > 0.9);
        assert!(calculate_confidence(100) <= 0.95);
    }

    #[test]
    fn test_trait_knowledge_progression() {
        let mut knowledge = TraitKnowledge::new("E01".to_string());

        assert_eq!(knowledge.confidence, 0.0);

        knowledge.add_observation(12, Some("Storm boost".to_string()));
        assert!(knowledge.confidence > 0.0);

        knowledge.add_observation(10, None);
        knowledge.add_observation(11, None);
        knowledge.add_observation(12, None);
        knowledge.add_observation(11, None);

        assert!(knowledge.confidence > 0.5);
        assert!(knowledge.power_estimate.1 < 30); // Should have narrowed
    }
}
