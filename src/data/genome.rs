//! 256-bit genome encoding for kaiju genetic data.
//!
//! The genome is a compact binary representation of a kaiju's genetic makeup,
//! enabling deterministic breeding records and progressive information
//! revelation.

use crc::{Crc, CRC_16_IBM_SDLC};
use serde::{Deserialize, Serialize};

/// CRC-16 algorithm for genome checksum
const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

/// 256-bit genome structure (32 bytes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    /// Header: version, generation, mutation count (16 bits)
    pub header: GenomeHeader,
    /// Stats: HP, Attack, Defense, Speed (64 bits)
    pub stats: GenomeStats,
    /// Trait slots: up to 8 traits (80 bits)
    pub trait_slots: Vec<TraitSlot>,
    /// Hidden trait data (64 bits)
    pub hidden_data: HiddenTraitData,
    /// Visual seed derivative (16 bits)
    pub visual_seed_derivative: u16,
    /// CRC16 checksum (16 bits)
    pub checksum: u16,
}

/// Genome header (16 bits total)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct GenomeHeader {
    /// Genome format version (4 bits, 0-15)
    pub version: u8,
    /// Generation number (8 bits, 0-255)
    pub generation: u8,
    /// Mutation count (4 bits, 0-15)
    pub mutation_count: u8,
}

/// Genome stats section (64 bits total, 16 bits each)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct GenomeStats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
}

/// Single trait slot (10 bits)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TraitSlot {
    /// Trait ID (7 bits, 0-127)
    pub trait_id: u8,
    /// Inheritance mode (2 bits, 0-3)
    pub inheritance_mode: u8,
    /// Is visible (1 bit)
    pub is_visible: bool,
}

/// Hidden trait data (64 bits)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HiddenTraitData {
    /// Bitflags for hidden trait presence (32 bits)
    pub presence_flags: u32,
    /// Conditional data (32 bits)
    pub conditional_data: u32,
}

impl Genome {
    /// Create a new genome from kaiju data
    pub fn new(
        generation: u8,
        mutation_count: u8,
        stats: GenomeStats,
        trait_slots: Vec<TraitSlot>,
        hidden_data: HiddenTraitData,
        visual_seed: u64,
    ) -> Self {
        let header = GenomeHeader {
            version: 1,
            generation,
            mutation_count,
        };

        let visual_seed_derivative = (visual_seed & 0xFFFF) as u16;

        let mut genome = Self {
            header,
            stats,
            trait_slots,
            hidden_data,
            visual_seed_derivative,
            checksum: 0,
        };

        // Calculate and set checksum
        genome.checksum = genome.calculate_checksum();
        genome
    }

    /// Encode genome to 32-byte array
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];

        // Header (bytes 0-1)
        bytes[0] = (self.header.version & 0x0F) | ((self.header.mutation_count & 0x0F) << 4);
        bytes[1] = self.header.generation;

        // Stats (bytes 2-9)
        bytes[2..4].copy_from_slice(&self.stats.hp.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.stats.attack.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.stats.defense.to_le_bytes());
        bytes[8..10].copy_from_slice(&self.stats.speed.to_le_bytes());

        // Trait slots (bytes 10-19, up to 8 slots, 10 bits each = 80 bits)
        let mut trait_bits: u128 = 0;
        for (i, slot) in self.trait_slots.iter().take(8).enumerate() {
            let slot_value: u128 = ((slot.trait_id as u128) & 0x7F)
                | (((slot.inheritance_mode as u128) & 0x03) << 7)
                | ((slot.is_visible as u128) << 9);
            trait_bits |= slot_value << (i * 10);
        }
        let trait_bytes = trait_bits.to_le_bytes();
        bytes[10..20].copy_from_slice(&trait_bytes[0..10]);

        // Hidden data (bytes 20-27)
        bytes[20..24].copy_from_slice(&self.hidden_data.presence_flags.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.hidden_data.conditional_data.to_le_bytes());

        // Visual seed derivative (bytes 28-29)
        bytes[28..30].copy_from_slice(&self.visual_seed_derivative.to_le_bytes());

        // Checksum (bytes 30-31)
        bytes[30..32].copy_from_slice(&self.checksum.to_le_bytes());

        bytes
    }

    /// Decode genome from 32-byte array
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, GenomeError> {
        // Header
        let version = bytes[0] & 0x0F;
        let mutation_count = (bytes[0] >> 4) & 0x0F;
        let generation = bytes[1];

        let header = GenomeHeader {
            version,
            generation,
            mutation_count,
        };

        // Stats
        let hp = u16::from_le_bytes([bytes[2], bytes[3]]);
        let attack = u16::from_le_bytes([bytes[4], bytes[5]]);
        let defense = u16::from_le_bytes([bytes[6], bytes[7]]);
        let speed = u16::from_le_bytes([bytes[8], bytes[9]]);

        let stats = GenomeStats {
            hp,
            attack,
            defense,
            speed,
        };

        // Trait slots
        let mut trait_bytes = [0u8; 16];
        trait_bytes[0..10].copy_from_slice(&bytes[10..20]);
        let trait_bits = u128::from_le_bytes(trait_bytes);

        let mut trait_slots = Vec::new();
        for i in 0..8 {
            let slot_value = (trait_bits >> (i * 10)) & 0x3FF;
            let trait_id = (slot_value & 0x7F) as u8;
            if trait_id != 0 {
                trait_slots.push(TraitSlot {
                    trait_id,
                    inheritance_mode: ((slot_value >> 7) & 0x03) as u8,
                    is_visible: ((slot_value >> 9) & 0x01) != 0,
                });
            }
        }

        // Hidden data
        let presence_flags = u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        let conditional_data = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);

        let hidden_data = HiddenTraitData {
            presence_flags,
            conditional_data,
        };

        // Visual seed derivative
        let visual_seed_derivative = u16::from_le_bytes([bytes[28], bytes[29]]);

        // Checksum
        let stored_checksum = u16::from_le_bytes([bytes[30], bytes[31]]);

        let genome = Self {
            header,
            stats,
            trait_slots,
            hidden_data,
            visual_seed_derivative,
            checksum: stored_checksum,
        };

        // Validate checksum
        let calculated = genome.calculate_checksum();
        if calculated != stored_checksum {
            return Err(GenomeError::InvalidChecksum {
                expected: stored_checksum,
                calculated,
            });
        }

        Ok(genome)
    }

    /// Encode genome to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Decode genome from hex string
    pub fn from_hex(hex_str: &str) -> Result<Self, GenomeError> {
        let bytes = hex::decode(hex_str).map_err(|_| GenomeError::InvalidHex)?;
        if bytes.len() != 32 {
            return Err(GenomeError::InvalidLength(bytes.len()));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Self::from_bytes(&arr)
    }

    /// Calculate CRC16 checksum (excluding the checksum bytes)
    fn calculate_checksum(&self) -> u16 {
        let bytes = self.to_bytes();
        let data = &bytes[0..30]; // Exclude checksum bytes
        CRC16.checksum(data)
    }

    /// Validate genome integrity
    pub fn validate(&self) -> Result<(), GenomeError> {
        let calculated = self.calculate_checksum();
        if calculated != self.checksum {
            return Err(GenomeError::InvalidChecksum {
                expected: self.checksum,
                calculated,
            });
        }

        // Validate version
        if self.header.version == 0 || self.header.version > 15 {
            return Err(GenomeError::InvalidVersion(self.header.version));
        }

        // Validate trait slots
        if self.trait_slots.len() > 8 {
            return Err(GenomeError::TooManyTraits(self.trait_slots.len()));
        }

        Ok(())
    }
}

/// Genome decoding/encoding errors
#[derive(Debug, Clone)]
pub enum GenomeError {
    InvalidChecksum { expected: u16, calculated: u16 },
    InvalidHex,
    InvalidLength(usize),
    InvalidVersion(u8),
    TooManyTraits(usize),
}

impl std::fmt::Display for GenomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChecksum {
                expected,
                calculated,
            } => {
                write!(
                    f,
                    "Invalid checksum: expected {:04x}, got {:04x}",
                    expected, calculated
                )
            }
            Self::InvalidHex => write!(f, "Invalid hex string"),
            Self::InvalidLength(len) => write!(f, "Invalid genome length: {} bytes", len),
            Self::InvalidVersion(v) => write!(f, "Invalid genome version: {}", v),
            Self::TooManyTraits(n) => write!(f, "Too many traits: {} (max 8)", n),
        }
    }
}

impl std::error::Error for GenomeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_roundtrip() {
        let stats = GenomeStats {
            hp: 300,
            attack: 60,
            defense: 40,
            speed: 30,
        };

        let traits = vec![
            TraitSlot {
                trait_id: 1,
                inheritance_mode: 0,
                is_visible: true,
            },
            TraitSlot {
                trait_id: 5,
                inheritance_mode: 1,
                is_visible: false,
            },
        ];

        let genome = Genome::new(5, 2, stats, traits, HiddenTraitData::default(), 12345);

        // Test byte roundtrip
        let bytes = genome.to_bytes();
        let decoded = Genome::from_bytes(&bytes).unwrap();

        assert_eq!(decoded.header.generation, 5);
        assert_eq!(decoded.header.mutation_count, 2);
        assert_eq!(decoded.stats.hp, 300);
        assert_eq!(decoded.stats.attack, 60);
        assert_eq!(decoded.trait_slots.len(), 2);
    }

    #[test]
    fn test_genome_hex_roundtrip() {
        let genome = Genome::new(
            3,
            1,
            GenomeStats {
                hp: 200,
                attack: 50,
                defense: 35,
                speed: 25,
            },
            vec![],
            HiddenTraitData::default(),
            99999,
        );

        let hex = genome.to_hex();
        let decoded = Genome::from_hex(&hex).unwrap();

        assert_eq!(decoded.header.generation, 3);
        assert_eq!(decoded.stats.hp, 200);
    }

    #[test]
    fn test_checksum_validation() {
        let genome = Genome::new(
            1,
            0,
            GenomeStats::default(),
            vec![],
            HiddenTraitData::default(),
            0,
        );

        let mut bytes = genome.to_bytes();
        // Corrupt a byte
        bytes[5] ^= 0xFF;

        let result = Genome::from_bytes(&bytes);
        assert!(result.is_err());
    }
}
