# GENOME ENCODING SPECIFICATION

**Kaiju Breeding Simulator - Technical Specification v1.0**

**Date**: 2026-01-09
**Status**: Design Complete - Ready for Implementation

---

## Key Design Insights from Documents

From **nft_design.md**, I understand:
- Layered information architecture (4 layers: raw, structural, functional, exact)
- Public data with private meaning through interpretation asymmetry
- Chunk-based progressive decoding tied to facility upgrades
- Deterministic obfuscation (not security-based encryption)

From **kaiju_sim.md**, I see:
- Stats: HP, Attack, Defense, Speed (4 core stats)
- Traits: Visible and hidden, with categories (element, modifier, mutation, synergy)
- Inheritance modes: Dominant, Recessive, Polygenic, Conditional
- Mutations occur with ~10% probability
- Power creep formula: `base * (1 + generation * 0.01) * variance`

From **IMPLEMENTATION_GUIDE.md**, I note:
- Visual seed needs to be stored (u64)
- Genome hash is immutable fingerprint
- Research facilities unlock genome chunks
- Battle experience reveals trait information

---

## Technical Specification Overview

The genome encoding system uses:

1. **Total Size**: 256 bits (32 bytes) - compact for on-chain storage
2. **Encoding Format**: Hexadecimal string (e.g., `0x9FA3_12C8_A04E_77D1_...`)
3. **Deterministic Generation**: From parent genomes + breeding seed
4. **Progressive Decoding**: 4 layers aligned with facility/experience progression
5. **Checksum**: Built-in validation using CRC16

---

## Binary Structure Breakdown

### Genome Layout (256 bits total)

```
Bits 0-15    (16 bits): Header & Metadata
Bits 16-79   (64 bits): Core Stats Encoding
Bits 80-159  (80 bits): Trait Slots (8 slots × 10 bits each)
Bits 160-223 (64 bits): Hidden Trait Data
Bits 224-239 (16 bits): Visual Seed Derivative
Bits 240-255 (16 bits): CRC16 Checksum
```

### Detailed Bit Allocation

#### Header (16 bits)
- Bits 0-3: Version (4 bits) - Schema version for future compatibility
- Bits 4-11: Generation (8 bits) - Supports up to 256 generations
- Bits 12-15: Mutation Count (4 bits) - Number of mutations (0-15)

#### Core Stats (64 bits)
Each stat uses 16 bits (range 0-65535):
- Bits 16-31: HP Base
- Bits 32-47: Attack Base
- Bits 48-63: Defense Base
- Bits 64-79: Speed Base

#### Trait Slots (80 bits, 8 slots)
Each slot: 10 bits
- Bits 0-6 (7 bits): Trait ID (128 possible traits)
- Bit 7: Dominant/Recessive flag
- Bits 8-9: Inheritance mode (00=dominant, 01=recessive, 10=polygenic, 11=conditional)

#### Hidden Trait Data (64 bits)
- Bits 160-175 (16 bits): Hidden trait bitflags (16 possible hidden traits)
- Bits 176-191 (16 bits): Conditional trigger data
- Bits 192-207 (16 bits): Synergy masks
- Bits 208-223 (16 bits): Growth potential modifiers

#### Visual Seed Derivative (16 bits)
- Derived from full 64-bit visual seed
- Used for consistent appearance generation

#### CRC16 Checksum (16 bits)
- Validates genome integrity
- Computed over bits 0-239

---

## Layer 0: Raw Genome (Public, On-Chain)

**What Players See:**
```
0x9FA3_12C8_A04E_77D1_3B2F_8E91_C7A4_5D69
```

**Properties:**
- Fully visible on blockchain
- Deterministically generated from breeding
- Meaningless without decoding key knowledge
- Can be copied but not interpreted
- Hash stored on-chain: `keccak256(genome_bytes)`

**Game Display:**
```
Genome: 9FA3-12C8-A04E-77D1-...
Status: ENCRYPTED - Requires Research Lab Level 1
```

---

## Layer 1: Structural Decoding (Facility-Unlocked)

**Unlock Requirements:**
- Research Lab Level 1: Reveals header + stat ranges
- Research Lab Level 2: Reveals trait slot count
- Research Lab Level 3: Reveals inheritance modes

**Level 1 Reveals (Research Lab Lv1):**
```json
{
  "generation": 4,
  "mutation_count": 2,
  "stat_ranges": {
    "hp": "280-320",
    "attack": "55-65",
    "defense": "38-45",
    "speed": "28-35"
  },
  "visible_trait_slots": 3,
  "hidden_trait_slots": "2-4"
}
```

**Level 2 Reveals (Research Lab Lv2):**
```json
{
  "trait_categories": [
    "Element: Electric",
    "Element: Water",
    "Modifier: Unknown"
  ],
  "inheritance_hints": [
    "Strong dominant genes detected",
    "Recessive trait cluster present"
  ]
}
```

**Level 3 Reveals (Research Lab Lv3):**
```json
{
  "trait_slots": [
    {"category": "element", "inheritance": "dominant"},
    {"category": "element", "inheritance": "recessive"},
    {"category": "modifier", "inheritance": "polygenic"}
  ],
  "mutation_markers": [
    {"slot": 6, "type": "unstable"}
  ]
}
```

---

## Layer 2: Functional Understanding (Experience-Based)

**Unlock Mechanisms:**
1. **Battle Participation**: Each battle reveals partial trait effects
2. **Environmental Exposure**: Storm battles reveal electric traits
3. **Breeding Outcomes**: Children reveal parental trait interactions
4. **Death Autopsies**: Fallen kaiju reveal hidden trait probabilities

**Battle-Based Revelation:**
```json
{
  "battles_participated": 10,
  "revealed_effects": [
    {
      "trait": "Electric Breath",
      "observed_power": "7-9 bonus damage",
      "conditions": ["effective in storm environments"],
      "confidence": 0.85
    }
  ],
  "synergy_hints": [
    "Electric + Water traits show interference pattern"
  ]
}
```

**Breeding-Based Revelation:**
```json
{
  "offspring_count": 3,
  "inheritance_observations": [
    {
      "trait": "Electric Breath",
      "inheritance_rate": "2/3 offspring (67%)",
      "mode": "likely dominant"
    }
  ]
}
```

**Progressive Confidence System:**
- 0 battles: "Unknown effect"
- 1-5 battles: "Approximately +5-10 damage" (wide range)
- 6-15 battles: "Estimated +7-9 damage" (narrow range)
- 16+ battles: "Confirmed +8 damage" (exact, Layer 3)

---

## Layer 3: Exact Knowledge (Voluntary Revelation)

**Revelation Triggers:**
1. **Manual Disclosure**: Player chooses to publish genome
2. **Tournament Requirements**: Lethal tournaments auto-reveal
3. **Death Autopsy**: Automatically published on death
4. **Research Publication**: Selling analysis to marketplace

**Full Genome Decode:**
```json
{
  "header": {
    "version": 1,
    "generation": 4,
    "mutation_count": 2
  },
  "stats": {
    "hp": 300,
    "attack": 60,
    "defense": 40,
    "speed": 30
  },
  "traits": [
    {
      "id": 42,
      "name": "Electric Breath",
      "category": "element",
      "power": 8,
      "inheritance": "dominant",
      "condition": null
    },
    {
      "id": 58,
      "name": "Aqua Hide",
      "category": "element",
      "power": 6,
      "inheritance": "recessive",
      "condition": null
    },
    {
      "id": 91,
      "name": "Storm Synergy",
      "category": "synergy",
      "power": 15,
      "inheritance": "conditional",
      "condition": "requires both Electric and storm environment"
    }
  ],
  "hidden_traits": [
    {
      "id": 103,
      "name": "Unstable Mutation",
      "category": "mutation",
      "power": 3,
      "inheritance": "dominant",
      "condition": "random activation chance per battle"
    }
  ],
  "visual_seed_derivative": 0xA4E7,
  "checksum": 0x9F3A,
  "checksum_valid": true
}
```

**Strategic Risk:**
Once revealed, opponents can:
- Build counter-strategies
- Predict battle outcomes accurately
- Reduce breeding value (mystery lost)
- Exploit conditional weaknesses

---

## Facility Requirements

### Research Lab Progression

| Level | Cost | Time | Unlocks |
|-------|------|------|---------|
| 0 | - | - | Raw genome only |
| 1 | 1000 credits | 1 hour | Header + stat ranges |
| 2 | 5000 credits | 6 hours | Trait categories + inheritance hints |
| 3 | 15000 credits | 24 hours | Full structural analysis |
| 4 | 50000 credits | 72 hours | Hidden trait detection |
| 5 | 150000 credits | 7 days | Exact decoding capability |

### Genetic Scanner Upgrades

| Upgrade | Unlocks |
|---------|---------|
| Basic Scanner | Decode Layer 1 for owned kaiju |
| Advanced Scanner | Decode Layer 1 for any kaiju (opponent scouting) |
| Deep Scanner | Decode Layer 2 with 50+ battle sample size |
| Master Scanner | Decode Layer 3 (requires kaiju death or disclosure) |

### Lineage Analyzer

| Level | Capability |
|-------|------------|
| 1 | View 2 generations back |
| 2 | View 4 generations back |
| 3 | View 8 generations + notable ancestors |
| 4 | Predict offspring genome ranges |
| 5 | Optimize breeding pair selection |

---

## Research Progression System

### Battle-Based Learning

**Experience Accumulation:**
```rust
struct TraitKnowledge {
    trait_id: u8,
    observations: u32,  // Number of battles observed
    confidence: f32,    // 0.0-1.0
    power_estimate_min: i32,
    power_estimate_max: i32,
    condition_hints: Vec<String>,
}
```

**Confidence Calculation:**
```rust
fn calculate_confidence(observations: u32) -> f32 {
    match observations {
        0 => 0.0,
        1..=5 => 0.3 + (observations as f32 * 0.1),
        6..=15 => 0.8 + ((observations - 6) as f32 * 0.015),
        _ => 0.95.min(0.8 + ((observations - 6) as f32 * 0.015))
    }
}
```

### Breeding-Based Learning

**Inheritance Pattern Recognition:**
```rust
struct InheritanceData {
    trait_id: u8,
    parent_present: bool,
    offspring_outcomes: Vec<bool>,  // Did offspring inherit?
    inheritance_rate: f32,
    inferred_mode: InheritanceMode,
}
```

**Mode Inference:**
- Dominant: >50% inheritance rate when one parent has trait
- Recessive: Requires both parents, ~25% if both heterozygous
- Polygenic: Variable, influenced by multiple genes
- Conditional: Depends on other traits or environment

### Death Autopsy System

When a kaiju dies in lethal tournament:
```json
{
  "autopsy_report": {
    "kaiju_id": "token_12345",
    "death_timestamp": 1704931200,
    "final_genome_state": "0x9FA3...",
    "revealed_layers": [0, 1, 2, 3],
    "hidden_traits_exposed": [
      {
        "trait": "Unstable Mutation",
        "activation_rate": "observed 3/10 battles"
      }
    ],
    "cause_of_death": "Outmatched in Storm Arena",
    "insights": [
      "Electric weakness exploited",
      "Defense stat lower than estimated"
    ]
  }
}
```

**Public Archive:**
- All death autopsies published to global database
- Searchable by trait, generation, lineage
- Contributes to collective knowledge
- Enhances meta-game understanding

---

## Deterministic Generation

### Breeding Function

```rust
fn generate_offspring_genome(
    parent_a: &Genome,
    parent_b: &Genome,
    breeding_seed: u64,
) -> Genome {
    let mut rng = ChaCha8Rng::seed_from_u64(breeding_seed);

    // Header
    let generation = parent_a.generation.max(parent_b.generation) + 1;
    let mutation_count = calculate_mutations(&mut rng);

    // Stats
    let stats = inherit_stats(parent_a, parent_b, generation, &mut rng);

    // Traits
    let traits = inherit_traits(
        &parent_a.traits,
        &parent_b.traits,
        &mut rng
    );

    // Hidden traits
    let hidden_traits = inherit_hidden_traits(
        &parent_a.hidden_traits,
        &parent_b.hidden_traits,
        mutation_count,
        &mut rng
    );

    // Visual seed (combine parent seeds)
    let visual_derivative = derive_visual_seed(
        parent_a.visual_seed_derivative,
        parent_b.visual_seed_derivative,
        breeding_seed
    );

    let genome = Genome::new(
        generation,
        mutation_count,
        stats,
        traits,
        hidden_traits,
        visual_derivative
    );

    genome.with_checksum()
}
```

### Stat Inheritance

```rust
fn inherit_stats(
    parent_a: &Genome,
    parent_b: &Genome,
    generation: u8,
    rng: &mut ChaCha8Rng,
) -> Stats {
    let power_creep = 1.0 + (generation as f32 * 0.01);

    Stats {
        hp: inherit_single_stat(
            parent_a.stats.hp,
            parent_b.stats.hp,
            power_creep,
            rng
        ),
        attack: inherit_single_stat(
            parent_a.stats.attack,
            parent_b.stats.attack,
            power_creep,
            rng
        ),
        defense: inherit_single_stat(
            parent_a.stats.defense,
            parent_b.stats.defense,
            power_creep,
            rng
        ),
        speed: inherit_single_stat(
            parent_a.stats.speed,
            parent_b.stats.speed,
            power_creep,
            rng
        ),
    }
}

fn inherit_single_stat(
    parent_a_stat: u16,
    parent_b_stat: u16,
    power_creep: f32,
    rng: &mut ChaCha8Rng,
) -> u16 {
    let base = (parent_a_stat + parent_b_stat) as f32 / 2.0;
    let variance = rng.gen_range(0.95..=1.05);
    ((base * power_creep * variance) as u16).min(65535)
}
```

### Trait Inheritance

```rust
fn inherit_traits(
    parent_a_traits: &[TraitSlot],
    parent_b_traits: &[TraitSlot],
    rng: &mut ChaCha8Rng,
) -> Vec<TraitSlot> {
    let mut inherited = Vec::new();

    // Combine parent traits
    let all_traits: Vec<&TraitSlot> = parent_a_traits
        .iter()
        .chain(parent_b_traits.iter())
        .collect();

    for trait_slot in all_traits {
        let inheritance_chance = match trait_slot.inheritance_mode {
            InheritanceMode::Dominant => 0.65,
            InheritanceMode::Recessive => 0.25,
            InheritanceMode::Polygenic => 0.45,
            InheritanceMode::Conditional => 0.35,
        };

        if rng.gen::<f32>() < inheritance_chance {
            inherited.push(*trait_slot);
        }
    }

    // Deduplicate
    inherited.sort_by_key(|t| t.trait_id);
    inherited.dedup_by_key(|t| t.trait_id);

    // Limit to 8 trait slots
    inherited.truncate(8);

    inherited
}
```

### Mutation Generation

```rust
fn calculate_mutations(rng: &mut ChaCha8Rng) -> u8 {
    if rng.gen::<f32>() < 0.10 {
        rng.gen_range(1..=3)
    } else {
        0
    }
}

fn generate_mutation_trait(rng: &mut ChaCha8Rng) -> TraitSlot {
    TraitSlot {
        trait_id: rng.gen_range(100..=127),  // Mutation trait ID range
        dominant: true,
        inheritance_mode: InheritanceMode::Dominant,
    }
}
```

---

## Visual Seed Integration

### Full Visual Seed (64-bit)

The complete visual seed drives AI image generation:
```rust
struct VisualSeed {
    genome_derivative: u16,  // Stored in genome bits 224-239
    parent_a_hash: u16,      // Derived from parent A genome
    parent_b_hash: u16,      // Derived from parent B genome
    breeding_seed_hash: u16, // From breeding transaction
}

fn construct_full_visual_seed(genome: &Genome, parents: &ParentData, breeding_seed: u64) -> u64 {
    let mut seed: u64 = 0;
    seed |= (genome.visual_seed_derivative as u64) << 48;
    seed |= (parents.hash_a as u64) << 32;
    seed |= (parents.hash_b as u64) << 16;
    seed |= (breeding_seed as u64) & 0xFFFF;
    seed
}
```

### Appearance Generation

```rust
fn generate_appearance(
    genome: &Genome,
    visual_seed: u64,
    decoded_traits: &[Trait],
) -> Appearance {
    let mut rng = ChaCha8Rng::seed_from_u64(visual_seed);

    // Base body plan
    let body_type = select_body_type(&mut rng, genome.generation);

    // Mandatory features from traits
    let mut features = Vec::new();
    for trait in decoded_traits {
        if let Some(visual_feature) = trait.required_visual_feature() {
            features.push(visual_feature);
        }
    }

    // Random aesthetic variations
    let color_scheme = generate_color_scheme(&mut rng, decoded_traits);
    let texture_pattern = generate_texture(&mut rng);
    let scale_factor = rng.gen_range(0.9..=1.1);

    Appearance {
        body_type,
        features,
        color_scheme,
        texture_pattern,
        scale_factor,
    }
}
```

---

## Example Genomes

### Example 1: "Flossy" - Generation 4 Electric Specialist

**Raw Genome (Layer 0):**
```
0x1423_012C_003C_0028_001E_2A80_3A40_0000_5F92_A4E7_9F3A_0000
```

**Breakdown:**
- Header: `0x1423` = Version 1, Gen 4, 2 mutations
- Stats: HP=300, ATK=60, DEF=40, SPD=30
- Traits: Electric Breath (dominant), Storm Affinity (polygenic)
- Hidden: Unstable Mutation, Electric Resistance
- Visual: `0xA4E7`
- Checksum: `0x9F3A`

**Layer 1 Decode (Research Lab Lv1):**
```json
{
  "generation": 4,
  "mutation_count": 2,
  "stat_ranges": {
    "hp": "280-320",
    "attack": "55-65",
    "defense": "35-45",
    "speed": "25-35"
  }
}
```

**Layer 2 Decode (10 battles, 2 offspring):**
```json
{
  "observed_traits": [
    {
      "name": "Electric Breath",
      "power_estimate": "7-9",
      "confidence": 0.85,
      "effective_in": ["storm"]
    }
  ],
  "inheritance_patterns": [
    {
      "trait": "Electric Breath",
      "rate": "2/2 offspring (100%)",
      "likely_mode": "dominant"
    }
  ]
}
```

**Layer 3 Decode (Full Revelation):**
```json
{
  "stats": {"hp": 300, "attack": 60, "defense": 40, "speed": 30},
  "traits": [
    {
      "name": "Electric Breath",
      "category": "element",
      "power": 8,
      "inheritance": "dominant"
    },
    {
      "name": "Storm Affinity",
      "category": "modifier",
      "power": 15,
      "inheritance": "polygenic",
      "condition": "storm environment"
    }
  ],
  "hidden_traits": [
    {
      "name": "Unstable Mutation",
      "power": 3,
      "activation_rate": 0.2
    },
    {
      "name": "Electric Resistance",
      "power": -5,
      "condition": "vs electric attacks"
    }
  ]
}
```

---

## Implementation Notes

### Encoding Library (Rust)

```rust
pub struct GenomeEncoder {
    version: u8,
}

impl GenomeEncoder {
    pub fn encode(&self, kaiju: &KaijuGenetics) -> [u8; 32] {
        let mut buffer = [0u8; 32];

        // Write header
        buffer[0] = (self.version << 4) | (kaiju.generation >> 4);
        buffer[1] = ((kaiju.generation & 0x0F) << 4) | kaiju.mutation_count;

        // Write stats (16-bit each)
        buffer[2..4].copy_from_slice(&kaiju.stats.hp.to_be_bytes());
        buffer[4..6].copy_from_slice(&kaiju.stats.attack.to_be_bytes());
        buffer[6..8].copy_from_slice(&kaiju.stats.defense.to_be_bytes());
        buffer[8..10].copy_from_slice(&kaiju.stats.speed.to_be_bytes());

        // Write trait slots
        for (i, trait_slot) in kaiju.traits.iter().enumerate() {
            let offset = 10 + (i * 2);
            let packed = trait_slot.pack();
            buffer[offset..offset+2].copy_from_slice(&packed.to_be_bytes());
        }

        // Write hidden trait data
        // ... (omitted for brevity)

        // Write visual derivative
        buffer[28..30].copy_from_slice(&kaiju.visual_seed_derivative.to_be_bytes());

        // Calculate and write checksum
        let checksum = crc16(&buffer[0..30]);
        buffer[30..32].copy_from_slice(&checksum.to_be_bytes());

        buffer
    }

    pub fn decode(&self, genome_bytes: &[u8; 32], layer: DecodingLayer) -> Result<GenomeData, DecodeError> {
        // Validate checksum
        let checksum = u16::from_be_bytes([genome_bytes[30], genome_bytes[31]]);
        if checksum != crc16(&genome_bytes[0..30]) {
            return Err(DecodeError::InvalidChecksum);
        }

        match layer {
            DecodingLayer::Layer0 => Ok(GenomeData::Raw(genome_bytes.clone())),
            DecodingLayer::Layer1 => self.decode_layer1(genome_bytes),
            DecodingLayer::Layer2(context) => self.decode_layer2(genome_bytes, context),
            DecodingLayer::Layer3 => self.decode_layer3(genome_bytes),
        }
    }
}
```

### Validation

```rust
fn validate_genome(genome: &Genome) -> ValidationResult {
    let mut issues = Vec::new();

    // Check generation range
    if genome.generation == 0 || genome.generation > 255 {
        issues.push("Invalid generation");
    }

    // Check stat bounds
    if genome.stats.hp == 0 || genome.stats.hp > 10000 {
        issues.push("HP out of valid range");
    }

    // Check trait slot count
    if genome.traits.len() > 8 {
        issues.push("Too many trait slots");
    }

    // Check for duplicate traits
    let unique_traits: HashSet<_> = genome.traits.iter().map(|t| t.trait_id).collect();
    if unique_traits.len() != genome.traits.len() {
        issues.push("Duplicate traits detected");
    }

    // Validate checksum
    let computed_checksum = genome.compute_checksum();
    if genome.checksum != computed_checksum {
        issues.push("Checksum mismatch");
    }

    ValidationResult {
        valid: issues.is_empty(),
        issues,
    }
}
```

### Checksum Algorithm

```rust
fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for byte in data {
        crc ^= (*byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }

    crc
}
```

---

## Summary

This genome encoding specification provides:

1. **Compact Storage**: 256 bits (32 bytes) suitable for on-chain storage
2. **Layered Decoding**: 4 progressive layers aligned with game progression
3. **Deterministic Generation**: Reproducible from parent genomes + seed
4. **Visual Integration**: Built-in visual seed derivative for appearance
5. **Validation**: CRC16 checksum for integrity verification
6. **Extensibility**: Version field allows future schema updates

The system creates meaningful information asymmetry: everyone can see the raw genome, but interpretation requires facility investment, battle experience, and strategic disclosure decisions.

---

### Critical Files for Implementation

Based on this specification, the following files are most critical:

- **H:\RustGames\kaiju_sim\src\data\genome.rs** - Core genome data structure and bit packing/unpacking logic
- **H:\RustGames\kaiju_sim\src\engine\genetics.rs** - Breeding function, inheritance calculation, and deterministic generation
- **H:\RustGames\kaiju_sim\src\engine\research.rs** - Layered decoding logic, confidence calculation, and facility unlock system
- **H:\RustGames\kaiju_sim\assets\genome_schema.json** - JSON schema defining trait IDs, categories, and decoding rules
- **H:\RustGames\kaiju_sim\src\engine\visual_gen.rs** - Visual seed integration and appearance generation from genome data
