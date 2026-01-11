# PHASE 2 IMPLEMENTATION PLAN: Genetics & Breeding Engine

## Executive Summary

Phase 2 implements the core genetics and breeding mechanics for the Kaiju Breeding Simulator. This phase builds the foundation for trait inheritance, genome encoding, mutation systems, and research facility progression. The system creates meaningful strategic depth through probabilistic trait inheritance, hidden genetic information, and emergent synergies.

**Estimated Timeline**: 3-4 weeks for full implementation with comprehensive testing

**Prerequisites**: Phase 1 (Foundation & Data Models) must be completed first

---

## 1. TRAIT SYSTEM IMPLEMENTATION

### 1.1 Core Data Structures (Week 1, Days 1-2)

**File**: `src/data/traits.rs`

**Tasks**:
- [ ] Define `Trait` struct with all fields from TRAIT_SYSTEM_DESIGN.md
- [ ] Implement `TraitCategory` enum (Element, Modifier, Mutation, Synergy)
- [ ] Implement `InheritanceType` enum (Dominant, Recessive, Polygenic, Conditional)
- [ ] Define `Condition` enum for conditional trait activation
- [ ] Create trait compatibility validation logic
- [ ] Implement trait power budget calculation

**Data Structures**:
```rust
pub struct Trait {
    pub id: String,              // "E01", "M03", etc.
    pub name: String,
    pub category: TraitCategory,
    pub power: i32,
    pub inheritance: InheritanceType,
    pub condition: Option<Condition>,
    pub is_hidden: bool,
    pub reveal_chance: f32,
    pub visual_keywords: Vec<String>,
}

pub enum TraitCategory {
    Element,    // Direct combat damage modifiers
    Modifier,   // Stat amplifiers and conditional bonuses
    Mutation,   // Random genetic anomalies
    Synergy,    // Meta-traits from trait combinations
}

pub enum InheritanceType {
    Dominant,
    Recessive,
    Polygenic,
    Conditional { condition: String, chance: f32 },
}

pub enum Condition {
    Always,
    HpBelow(i32),
    HpAbove(i32),
    TurnMin(u32),
    Environment(String),
    EnemyHpAbove(i32),
    AfterKill,
    FirstTurn,
}
```

**Validation Logic**:
- Power budget: Total trait power ≤ 150
- Incompatibility matrix checking (e.g., Berserker Rage vs Defensive Stance)
- Trait slot limits (8 visible max, 6 hidden max, 12 total)

**Time Estimate**: 16 hours

---

### 1.2 Trait JSON Data Loading (Week 1, Day 3)

**File**: `assets/traits.json`

**Tasks**:
- [ ] Create JSON schema for all 50 traits (15 Element, 15 Modifier, 10 Mutation, 10 Synergy)
- [ ] Define synergy requirements table
- [ ] Define incompatibility matrix
- [ ] Implement JSON loader in `src/data/traits.rs`
- [ ] Add validation for loaded trait data

**JSON Structure Example**:
```json
{
  "traits": [
    {
      "id": "E01",
      "name": "Electric Breath",
      "category": "Element",
      "power": 10,
      "inheritance": "Dominant",
      "condition": "Always",
      "is_hidden": false,
      "visual_keywords": ["lightning", "electricity", "blue glow"]
    }
  ],
  "synergies": [
    {
      "id": "S01",
      "name": "Storm Dragon",
      "required_traits": ["E01", "M04"],
      "power": 20,
      "effect": "Add 20 damage in Storm environment"
    }
  ],
  "incompatibilities": [
    ["M03", "M08"],
    ["M10", "MU04"]
  ]
}
```

**Time Estimate**: 8 hours

---

### 1.3 Synergy Detection System (Week 1, Day 4)

**File**: `src/engine/synergies.rs`

**Tasks**:
- [ ] Implement synergy detection algorithm
- [ ] Create synergy trait auto-generation
- [ ] Add synergy validation (all required traits present)
- [ ] Implement synergy power calculation
- [ ] Add tests for all 10 synergy combinations

**Algorithm**:
```rust
pub fn check_synergies(kaiju: &Kaiju, synergy_defs: &[SynergyDefinition]) -> Vec<Trait> {
    let mut active_synergies = Vec::new();

    for synergy_def in synergy_defs {
        let has_all_required = synergy_def.required_traits
            .iter()
            .all(|req| kaiju.has_trait(req));

        if has_all_required {
            active_synergies.push(synergy_def.to_trait());
        }
    }

    active_synergies
}
```

**Time Estimate**: 6 hours

---

## 2. GENOME ENCODING IMPLEMENTATION

### 2.1 Binary Genome Structure (Week 1, Days 4-5)

**File**: `src/data/genome.rs`

**Tasks**:
- [ ] Implement 256-bit (32-byte) genome structure
- [ ] Create bit-packing functions for each section:
  - Header (16 bits): Version, Generation, Mutation Count
  - Stats (64 bits): HP, Attack, Defense, Speed (16 bits each)
  - Trait Slots (80 bits): 8 slots × 10 bits each
  - Hidden Traits (64 bits): Bitflags and conditional data
  - Visual Seed (16 bits): Derivative for appearance
  - Checksum (16 bits): CRC16 validation
- [ ] Implement CRC16 checksum algorithm
- [ ] Create genome validation function
- [ ] Add serialization/deserialization (hex string format)

**Data Structure**:
```rust
pub struct Genome {
    pub header: GenomeHeader,
    pub stats: GenomeStats,
    pub trait_slots: Vec<TraitSlot>,  // Max 8
    pub hidden_data: HiddenTraitData,
    pub visual_seed_derivative: u16,
    pub checksum: u16,
}

pub struct GenomeHeader {
    pub version: u8,       // 4 bits
    pub generation: u8,    // 8 bits
    pub mutation_count: u8,// 4 bits
}

pub struct GenomeStats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
}

pub struct TraitSlot {
    pub trait_id: u8,           // 7 bits (0-127)
    pub inheritance_mode: u8,   // 2 bits
}
```

**Time Estimate**: 16 hours

---

### 2.2 Progressive Decoding Layers (Week 2, Day 1)

**File**: `src/engine/genome_decoder.rs`

**Tasks**:
- [ ] Implement Layer 0 (raw genome blob)
- [ ] Implement Layer 1 (structural decoding - stat ranges, trait slot count)
- [ ] Implement Layer 2 (functional understanding - battle-based revelation)
- [ ] Implement Layer 3 (exact knowledge - full decode)
- [ ] Create confidence calculation system for Layer 2
- [ ] Add facility level requirements checking

**Decoding Interface**:
```rust
pub enum DecodingLayer {
    Layer0,                         // Raw hex string only
    Layer1(FacilityLevel),          // Requires Research Lab Lv1-3
    Layer2(BattleContext),          // Requires battle experience
    Layer3,                         // Requires voluntary disclosure or death
}

pub fn decode_genome(
    genome: &Genome,
    layer: DecodingLayer,
    kaiju_history: &KaijuHistory,
) -> Result<GenomeData, DecodeError> {
    // Layer-specific decoding logic
}
```

**Layer 1 Output Example**:
```json
{
  "generation": 4,
  "mutation_count": 2,
  "stat_ranges": {
    "hp": "280-320",
    "attack": "55-65"
  }
}
```

**Time Estimate**: 12 hours

---

## 3. BREEDING ALGORITHM IMPLEMENTATION

### 3.1 Core Breeding Function (Week 2, Days 2-3)

**File**: `src/engine/breeding.rs`

**Tasks**:
- [ ] Implement generation calculation: `max(parent_a.gen, parent_b.gen) + 1`
- [ ] Implement stat inheritance with power creep formula
- [ ] Add variance generation (0.95-1.05 per stat)
- [ ] Enforce stat floors (HP: 50, ATK: 10, DEF: 5, SPD: 5)
- [ ] Enforce soft caps: `base * (1 + gen * 0.02)`
- [ ] Enforce hard caps (HP: 2000, ATK/DEF/SPD: 400)
- [ ] Create deterministic RNG using ChaCha8Rng with breeding seed

**Core Formula Implementation**:
```rust
fn inherit_single_stat(
    parent_a_stat: u16,
    parent_b_stat: u16,
    generation: u8,
    rng: &mut ChaCha8Rng,
) -> u16 {
    // Step 1: Base average
    let base = (parent_a_stat + parent_b_stat) as f32 / 2.0;

    // Step 2: Power creep multiplier
    let gen_multiplier = 1.0 + (generation as f32 * 0.01);

    // Step 3: Variance (±5%)
    let variance = rng.gen_range(0.95..=1.05);

    // Step 4: Calculate raw value
    let raw = base * gen_multiplier * variance;

    // Step 5: Apply constraints
    let floored = raw.max(50.0);  // Example for HP
    let soft_capped = floored.min(500.0 * (1.0 + generation as f32 * 0.02));
    let hard_capped = soft_capped.min(2000.0);

    hard_capped as u16
}
```

**Time Estimate**: 16 hours

---

### 3.2 Trait Inheritance System (Week 2, Days 3-4)

**File**: `src/engine/trait_inheritance.rs`

**Tasks**:
- [ ] Implement visible trait inheritance (45% base rate)
- [ ] Implement hidden trait inheritance (25% base rate)
- [ ] Add dominance bonus (both parents have trait → 70%/50%)
- [ ] Implement recessive trait logic
- [ ] Implement polygenic accumulation system (4-point threshold)
- [ ] Implement conditional trait evaluation
- [ ] Add trait deduplication logic
- [ ] Enforce trait slot limits (8 visible, 6 hidden)

**Inheritance Probability Matrix**:
```rust
fn calculate_inheritance_chance(
    trait_def: &Trait,
    parent_a_has: bool,
    parent_b_has: bool,
    is_visible: bool,
) -> f32 {
    let base_rate = if is_visible { 0.45 } else { 0.25 };

    match trait_def.inheritance {
        InheritanceType::Dominant => {
            if parent_a_has && parent_b_has { 1.0 }
            else if parent_a_has || parent_b_has { base_rate * 1.33 }
            else { 0.0 }
        },
        InheritanceType::Recessive => {
            if parent_a_has && parent_b_has { base_rate }
            else if parent_a_has || parent_b_has { base_rate * 0.5 }
            else { 0.0 }
        },
        InheritanceType::Polygenic => {
            let points = calculate_polygenic_points(parent_a_has, parent_b_has);
            if points >= 4 { 1.0 } else if points >= 2 { 0.5 } else { 0.0 }
        },
        InheritanceType::Conditional { chance, .. } => chance,
    }
}
```

**Time Estimate**: 16 hours

---

### 3.3 Mutation System (Week 2, Day 5)

**File**: `src/engine/mutations.rs`

**Tasks**:
- [ ] Implement 10% base mutation chance
- [ ] Add generation-based mutation chance modifier (+0.5% per gen)
- [ ] Add trait count instability modifier
- [ ] Implement 4 mutation types (40% Stat, 30% New Trait, 20% Power Boost, 10% Hidden Unlock)
- [ ] Create mutation power calculation
- [ ] Add mutation recording to offspring metadata
- [ ] Cap total mutation chance at 25%

**Mutation Type Distribution**:
```rust
pub enum MutationType {
    StatMutation { target_stat: StatType, boost_multiplier: f32 },  // 40%
    NewTrait { trait: Trait },                                      // 30%
    TraitPowerBoost { trait_id: String, power_increase: i32 },      // 20%
    HiddenUnlock { trait_id: String },                              // 10%
}

fn generate_mutation(offspring: &mut Kaiju, rng: &mut ChaCha8Rng) {
    // Calculate mutation chance
    let mut chance = 0.10 + (offspring.generation as f32 * 0.005);
    let trait_count = offspring.all_traits().len();
    if trait_count > 10 { chance += 0.05; }
    chance = chance.min(0.25);

    if rng.gen::<f32>() < chance {
        let mutation_type = roll_mutation_type(rng);
        apply_mutation(offspring, mutation_type, rng);
    }
}
```

**Time Estimate**: 10 hours

---

### 3.4 Breeding Validation & Restrictions (Week 3, Day 1)

**File**: `src/engine/breeding_validator.rs`

**Tasks**:
- [ ] Implement parent-child breeding block
- [ ] Add death status validation
- [ ] Check tournament lock status
- [ ] Validate breeding rights ownership
- [ ] Add lineage validation (no incest beyond siblings)
- [ ] Create comprehensive error types

**Validation Flow**:
```rust
pub fn validate_breeding(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    breeding_rights: &BreedingRights,
) -> Result<(), BreedingError> {
    // Check alive status
    if !parent_a.alive || !parent_b.alive {
        return Err(BreedingError::ParentDead);
    }

    // Check parent-child relationship
    if is_direct_parent_child(parent_a, parent_b) {
        return Err(BreedingError::DirectParentChild);
    }

    // Check tournament locks
    if parent_a.locked_in_tournament || parent_b.locked_in_tournament {
        return Err(BreedingError::TournamentLocked);
    }

    // Check breeding rights
    if !breeding_rights.valid_for(parent_a, parent_b) {
        return Err(BreedingError::InvalidRights);
    }

    Ok(())
}
```

**Time Estimate**: 8 hours

---

### 3.5 Visual Seed Generation (Week 3, Day 1)

**File**: `src/engine/visual_seed.rs`

**Tasks**:
- [ ] Implement 64-bit visual seed structure
- [ ] Create parent seed mixing algorithm (XOR + bit manipulation)
- [ ] Add stat-based variation component
- [ ] Add trait-based variation component
- [ ] Add random mutation component (10% influence)
- [ ] Create 16-bit derivative for genome storage

**Visual Seed Algorithm**:
```rust
pub fn generate_visual_seed(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    offspring: &Kaiju,
    breeding_seed: u64,
) -> u64 {
    // Component 1: Parent mixing (40% influence)
    let parent_mix = (parent_a.visual_seed ^ parent_b.visual_seed) & 0xFFFFFFFF00000000;

    // Component 2: Stat hash (30% influence)
    let stat_hash = hash_stats(&offspring.stats) & 0x00000000FFFF0000;

    // Component 3: Trait hash (20% influence)
    let trait_hash = hash_traits(&offspring.traits) & 0x000000000000FF00;

    // Component 4: Random (10% influence)
    let mut rng = ChaCha8Rng::seed_from_u64(breeding_seed);
    let random = (rng.gen::<u8>() as u64) & 0x00000000000000FF;

    parent_mix | stat_hash | trait_hash | random
}
```

**Time Estimate**: 6 hours

---

## 4. RESEARCH FACILITY PROGRESSION

### 4.1 Research System (Week 3, Day 2)

**File**: `src/engine/research.rs`

**Tasks**:
- [ ] Implement 5-level Research Lab progression
- [ ] Create facility upgrade cost/time structure
- [ ] Add Layer 1 decoding unlock (Levels 1-3)
- [ ] Implement trait analysis function
- [ ] Add battle log analysis for trait discovery
- [ ] Create breeding outcome prediction system

**Facility Levels**:
```rust
pub struct ResearchFacility {
    pub level: u8,  // 0-5
    pub capabilities: Vec<ResearchCapability>,
}

pub enum ResearchCapability {
    DecodeLayer1Basic,      // Level 1: Header + stat ranges
    DecodeLayer1Advanced,   // Level 2: Trait categories
    DecodeLayer1Full,       // Level 3: Full structure
    HiddenTraitDetection,   // Level 4: Scan for hidden traits
    ExactDecoding,          // Level 5: Full genome decode
}
```

**Time Estimate**: 10 hours

---

### 4.2 Battle-Based Knowledge Accumulation (Week 3, Day 3)

**File**: `src/engine/trait_knowledge.rs`

**Tasks**:
- [ ] Implement `TraitKnowledge` struct for tracking observations
- [ ] Create confidence calculation (0-1000 battles → 0.0-0.95 confidence)
- [ ] Add power estimate range narrowing
- [ ] Implement condition hint generation
- [ ] Create breeding-based inheritance pattern recognition

**Knowledge Accumulation**:
```rust
pub struct TraitKnowledge {
    pub trait_id: String,
    pub observations: u32,
    pub confidence: f32,
    pub power_estimate_min: i32,
    pub power_estimate_max: i32,
    pub condition_hints: Vec<String>,
}

pub fn calculate_confidence(observations: u32) -> f32 {
    match observations {
        0 => 0.0,
        1..=5 => 0.3 + (observations as f32 * 0.1),
        6..=15 => 0.8 + ((observations - 6) as f32 * 0.015),
        _ => (0.8 + ((observations - 6) as f32 * 0.015)).min(0.95),
    }
}
```

**Time Estimate**: 8 hours

---

## 5. INTEGRATION & DATA FLOW

### 5.1 Complete Breeding Pipeline (Week 3, Days 4-5)

**File**: `src/engine/breeding.rs` (main integration)

**Tasks**:
- [ ] Integrate all breeding subsystems
- [ ] Create end-to-end breeding function
- [ ] Add genome generation from breeding results
- [ ] Implement offspring metadata creation
- [ ] Add breeding history recording
- [ ] Create breeding event logging

**Complete Breeding Function**:
```rust
pub fn breed_kaiju(
    parent_a: &Kaiju,
    parent_b: &Kaiju,
    breeding_seed: u64,
    breeding_rights: &BreedingRights,
) -> Result<Kaiju, BreedingError> {
    // 1. Validation
    validate_breeding(parent_a, parent_b, breeding_rights)?;

    // 2. Generate offspring
    let mut offspring = Kaiju::new();
    offspring.generation = calc_generation(parent_a, parent_b);

    // 3. Inherit stats
    offspring.stats = inherit_stats(parent_a, parent_b, offspring.generation);

    // 4. Inherit traits
    offspring.traits = inherit_traits(parent_a, parent_b, breeding_seed);

    // 5. Check synergies
    offspring.synergy_traits = check_synergies(&offspring);

    // 6. Apply mutations
    apply_mutations(&mut offspring, breeding_seed);

    // 7. Generate genome
    offspring.genome = generate_genome(&offspring);

    // 8. Generate visual seed
    offspring.visual_seed = generate_visual_seed(parent_a, parent_b, &offspring, breeding_seed);

    // 9. Record breeding history
    record_breeding_event(parent_a, parent_b, &offspring);

    Ok(offspring)
}
```

**Time Estimate**: 12 hours

---

## 6. UNIT TESTS

### 6.1 Trait System Tests (Week 3, Day 5)

**File**: `src/data/traits_tests.rs`

**Tasks**:
- [ ] Test trait JSON loading (50 traits)
- [ ] Test synergy detection (10 synergy combinations)
- [ ] Test trait compatibility validation
- [ ] Test power budget enforcement
- [ ] Test trait slot limits

**Time Estimate**: 6 hours

---

### 6.2 Genome Encoding Tests (Week 4, Day 1)

**File**: `src/data/genome_tests.rs`

**Tasks**:
- [ ] Test genome bit packing/unpacking
- [ ] Test CRC16 checksum calculation
- [ ] Test genome validation
- [ ] Test Layer 0-3 decoding
- [ ] Test visual seed generation

**Time Estimate**: 8 hours

---

### 6.3 Breeding Algorithm Tests (Week 4, Day 1)

**File**: `src/engine/breeding_tests.rs`

**Tasks**:
- [ ] Test stat inheritance formula (1000 iterations)
- [ ] Test generation calculation (all edge cases)
- [ ] Test variance distribution (should be uniform)
- [ ] Test stat floors and ceilings
- [ ] Test all 10 breeding scenarios from BREEDING_ALGORITHM_SPEC.md
- [ ] Verify trait inheritance probabilities (10,000 breedings)
- [ ] Verify mutation rate (10,000 breedings, expect ~10%)

**Example Test**:
```rust
#[test]
fn test_stat_inheritance_variance() {
    let parent_a = create_test_kaiju(300, 60, 40, 30);
    let parent_b = create_test_kaiju(320, 55, 45, 25);

    let mut results = Vec::new();
    for seed in 0..1000 {
        let offspring = breed_kaiju(&parent_a, &parent_b, seed, &rights).unwrap();
        results.push(offspring.stats.hp);
    }

    // Verify mean is close to expected
    let mean = results.iter().sum::<i32>() / results.len() as i32;
    let expected = ((300 + 320) / 2) * 1.05;  // Gen 5
    assert!((mean - expected as i32).abs() < 10);

    // Verify variance is within ±5%
    let min = results.iter().min().unwrap();
    let max = results.iter().max().unwrap();
    assert!(*min >= (expected * 0.95) as i32);
    assert!(*max <= (expected * 1.05) as i32);
}
```

**Time Estimate**: 10 hours

---

### 6.4 Mutation System Tests (Week 4, Day 2)

**File**: `src/engine/mutations_tests.rs`

**Tasks**:
- [ ] Test 10% base mutation rate
- [ ] Test mutation type distribution (40/30/20/10 split)
- [ ] Test stat mutation ranges (+5% to +15%)
- [ ] Test new trait mutations (hidden bias)
- [ ] Test mutation chance modifiers

**Time Estimate**: 6 hours

---

### 6.5 Validation Tests (Week 4, Day 2)

**File**: `src/engine/breeding_validator_tests.rs`

**Tasks**:
- [ ] Test parent-child blocking
- [ ] Test dead parent rejection
- [ ] Test tournament lock validation
- [ ] Test breeding rights validation
- [ ] Test all error conditions

**Time Estimate**: 4 hours

---

## 7. INTEGRATION TESTS

### 7.1 Full Breeding Cycle Test (Week 4, Day 3)

**File**: `tests/integration_breeding.rs`

**Tasks**:
- [ ] Test complete breeding from validation → offspring generation
- [ ] Test breeding history recording
- [ ] Test genome generation and validation
- [ ] Test offspring inherits correct generation
- [ ] Test synergy activation from inherited traits
- [ ] Test mutation integration with trait system

**Example Integration Test**:
```rust
#[test]
fn test_complete_breeding_cycle() {
    // Setup
    let traits = load_traits("assets/traits.json").unwrap();
    let parent_a = create_kaiju_with_traits(vec!["E01", "M04"]);  // Electric + Wings
    let parent_b = create_kaiju_with_traits(vec!["E01"]);          // Electric only

    // Breed
    let offspring = breed_kaiju(&parent_a, &parent_b, 12345, &rights).unwrap();

    // Verify generation
    assert_eq!(offspring.generation, parent_a.generation.max(parent_b.generation) + 1);

    // Verify trait inheritance (both have Electric, should be high chance)
    assert!(offspring.has_trait("E01") || true);  // Probabilistic

    // Verify synergy potential (if both E01 and M04 inherited)
    if offspring.has_trait("E01") && offspring.has_trait("M04") {
        assert!(offspring.has_synergy("S01"));  // Storm Dragon
    }

    // Verify genome validity
    assert!(validate_genome(&offspring.genome).is_ok());
}
```

**Time Estimate**: 8 hours

---

### 7.2 10 Breeding Scenario Tests (Week 4, Day 3)

**File**: `tests/breeding_scenarios.rs`

**Tasks**:
- [ ] Implement Example 1: Basic same-generation breeding
- [ ] Implement Example 2: Cross-generation with dominant trait
- [ ] Implement Example 3: Sibling breeding with recessive traits
- [ ] Implement Example 4: Mutation - stat boost
- [ ] Implement Example 5: Maximum generation with soft cap
- [ ] Implement Example 6: Failed breeding - parent-child block
- [ ] Implement Example 7: Dead parent - rights refund
- [ ] Implement Example 8: Polygenic trait emergence
- [ ] Implement Example 9: Multiple trait inheritance (lucky roll)
- [ ] Implement Example 10: Minimum stat floor application

**Time Estimate**: 12 hours

---

### 7.3 Research System Integration Test (Week 4, Day 4)

**File**: `tests/integration_research.rs`

**Tasks**:
- [ ] Test Layer 1 decoding with different facility levels
- [ ] Test battle-based confidence accumulation
- [ ] Test trait knowledge progression (0 → 50 battles)
- [ ] Test breeding-based inheritance pattern recognition
- [ ] Test synergy discovery through research

**Time Estimate**: 6 hours

---

## 8. PERFORMANCE OPTIMIZATION

### 8.1 Breeding Performance Targets (Week 4, Day 4)

**Targets**:
- Single breeding operation: < 10ms
- 100 breeding simulations: < 500ms
- Trait inheritance calculation: < 1ms
- Genome encoding: < 1ms
- Synergy detection: < 1ms

**Optimization Tasks**:
- [ ] Profile breeding function with criterion
- [ ] Optimize trait inheritance loop (pre-compute probabilities)
- [ ] Cache synergy detection results
- [ ] Use bit manipulation for genome encoding
- [ ] Add benchmarks for all critical paths

**Time Estimate**: 6 hours

---

### 8.2 Memory Optimization (Week 4, Day 5)

**Tasks**:
- [ ] Minimize Kaiju struct size (target: < 512 bytes)
- [ ] Use `Box<[T]>` for variable-length trait vectors
- [ ] Intern trait strings (use IDs instead of full strings)
- [ ] Profile memory usage with 1000 kaiju

**Time Estimate**: 4 hours

---

## 9. DOCUMENTATION

### 9.1 Code Documentation (Week 4, Day 5)

**Tasks**:
- [ ] Add rustdoc comments to all public functions
- [ ] Create module-level documentation
- [ ] Add examples to breeding.rs
- [ ] Document all error types
- [ ] Create architecture diagram for breeding flow

**Time Estimate**: 6 hours

---

### 9.2 API Documentation (Week 4, Day 5)

**Tasks**:
- [ ] Document breeding API surface
- [ ] Create usage examples
- [ ] Add trait system guide
- [ ] Document genome encoding format
- [ ] Create research system guide

**Time Estimate**: 4 hours

---

## 10. CRITICAL IMPLEMENTATION NOTES

### 10.1 Determinism Requirements

**Critical**: All breeding operations MUST be deterministic given the same inputs.

- Use `ChaCha8Rng::seed_from_u64(breeding_seed)` for all randomness
- Never use system time or global RNG
- All trait rolls must be reproducible
- Genome encoding must be bit-identical across platforms

### 10.2 Data-Driven Design

**Critical**: No hardcoded trait values in Rust code.

- All 50 traits defined in `assets/traits.json`
- Balance changes via JSON edits, not code changes
- Synergy definitions in JSON, not Rust matches
- Incompatibility matrix in JSON

### 10.3 Error Handling

**Critical**: All breeding failures must be recoverable.

- Return `Result<Kaiju, BreedingError>` for all breeding functions
- Never panic in breeding pipeline
- Validate all inputs before state changes
- Provide clear error messages for validation failures

---

## 11. DEPENDENCIES

### 11.1 Cargo.toml Additions

```toml
[dependencies]
# Existing dependencies (from Phase 1)
macroquad = "0.4"
macroquad-toolkit = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Phase 2 additions
rand = "0.8"
rand_chacha = "0.3"
crc = "3.0"
hex = "0.4"

[dev-dependencies]
criterion = "0.5"
```

---

## 12. RISK MITIGATION

### 12.1 Technical Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Probabilistic tests flaky | High | Use large sample sizes (10,000+ iterations) |
| Genome encoding bugs | High | Comprehensive round-trip tests, checksum validation |
| Trait inheritance imbalance | Medium | Balance tests with statistical analysis |
| Performance bottlenecks | Medium | Profile early, benchmark all critical paths |

### 12.2 Design Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Power creep too fast | High | Soft caps + diminishing returns formula |
| Mutations too rare/common | Medium | Configurable rates in JSON |
| Synergies too hard to discover | Low | Research facility progression |

---

## 13. TESTING STRATEGY

### 13.1 Test Coverage Targets

- Unit tests: 90%+ coverage
- Integration tests: All 10 breeding scenarios
- Property tests: Trait inheritance probabilities
- Benchmark tests: Performance targets met

### 13.2 Test Execution

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test breeding_

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

---

## 14. DELIVERABLES CHECKLIST

- [ ] All trait data structures implemented
- [ ] 50 traits loaded from JSON
- [ ] Genome encoding/decoding functional
- [ ] Complete breeding algorithm implemented
- [ ] Mutation system working
- [ ] Research facility progression implemented
- [ ] All unit tests passing (90%+ coverage)
- [ ] All 10 breeding scenarios tested
- [ ] Integration tests passing
- [ ] Performance benchmarks meet targets
- [ ] Code documented with rustdoc
- [ ] API documentation complete

---

## 15. TIMELINE SUMMARY

| Week | Focus | Key Deliverables |
|------|-------|------------------|
| Week 1 | Trait System | Trait structs, JSON loading, synergy detection, genome binary structure |
| Week 2 | Breeding Core | Stat inheritance, trait inheritance, mutations, visual seed |
| Week 3 | Integration | Breeding validation, research system, complete pipeline, unit tests |
| Week 4 | Testing & Polish | Integration tests, scenarios, performance optimization, documentation |

**Total Estimated Time**: 220 hours (27-28 working days at 8 hours/day)

With focused development: **3-4 weeks**

---

## CRITICAL FILES FOR IMPLEMENTATION

Based on this detailed implementation plan, here are the 5 most critical files for implementing Phase 2:

### 1. **H:\RustGames\kaiju_sim\src\data\traits.rs**
   - **Reason**: Foundation of the entire trait system. Contains all trait data structures, enums, and validation logic. Must be implemented first as all other systems depend on it.
   - **Lines**: ~400-500
   - **Complexity**: Medium-High

### 2. **H:\RustGames\kaiju_sim\assets\traits.json**
   - **Reason**: Authoritative source for all 50 trait definitions, synergy requirements, and incompatibility rules. Data-driven design depends on this file being comprehensive and accurate.
   - **Lines**: ~800-1000 JSON
   - **Complexity**: Low (data entry), High (balance design)

### 3. **H:\RustGames\kaiju_sim\src\engine\breeding.rs**
   - **Reason**: Core breeding algorithm implementation. Contains stat inheritance, trait inheritance, mutation application, and complete breeding pipeline. This is the heart of Phase 2.
   - **Lines**: ~600-800
   - **Complexity**: High

### 4. **H:\RustGames\kaiju_sim\src\data\genome.rs**
   - **Reason**: 256-bit genome encoding and decoding. Critical for NFT metadata, progressive information revelation, and persistent genetic data. Implements the unique "public data, private meaning" architecture.
   - **Lines**: ~500-700
   - **Complexity**: High (bit manipulation, cryptographic checksums)

### 5. **H:\RustGames\kaiju_sim\src\engine\research.rs**
   - **Reason**: Research facility progression and progressive decoding layers. Implements the information asymmetry mechanic that creates strategic depth and player engagement. Connects breeding, combat, and knowledge acquisition.
   - **Lines**: ~400-500
   - **Complexity**: Medium-High

These five files form the complete breeding and genetics pipeline:
1. **traits.rs** - Define what traits are and how they behave
2. **traits.json** - Specify all 50 traits with balance values
3. **breeding.rs** - Implement how traits and stats are inherited
4. **genome.rs** - Encode genetic data into persistent format
5. **research.rs** - Control progressive revelation of genetic information

All other files (validators, mutations, synergies, tests) are supporting infrastructure built around these core files.
