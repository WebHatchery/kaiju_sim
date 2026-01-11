# BREEDING ALGORITHM SPECIFICATION

**Kaiju Breeding Simulator - Complete Breeding Algorithm Specification**

**Version**: 1.0
**Date**: 2026-01-09
**Status**: Reference Implementation

---

## 1. Stat Inheritance Formula

### 1.1 Core Formula

For each stat (HP, Attack, Defense, Speed):

```
offspring_stat = floor(base_average * generation_multiplier * variance)
```

Where:
- `base_average = (parent_a.stat + parent_b.stat) / 2`
- `generation_multiplier = 1 + (offspring_generation * 0.01)`
- `variance = random_uniform(0.95, 1.05)`

### 1.2 Generation Calculation

```
offspring_generation = max(parent_a.generation, parent_b.generation) + 1
```

**Example**:
- Gen 3 + Gen 5 = Gen 6 offspring
- Gen 4 + Gen 4 = Gen 5 offspring

### 1.3 Variance Bounds

Each stat receives an independent variance roll within the range `[0.95, 1.05]`:
- **Minimum variance**: 0.95 (-5%)
- **Maximum variance**: 1.05 (+5%)
- **Distribution**: Uniform (all values equally likely)

This creates narrow, predictable ranges while preserving individuality.

### 1.4 Stat Floor and Ceiling Constraints

To prevent degenerate or overpowered kaiju:

**Absolute Floors** (per stat):
- HP: 50 minimum
- Attack: 10 minimum
- Defense: 5 minimum
- Speed: 5 minimum

**Soft Ceilings** (per generation):
```
max_stat = base_cap * (1 + generation * 0.02)
```

**Base Caps** (Generation 1):
- HP: 500
- Attack: 100
- Defense: 100
- Speed: 100

**Hard Ceilings** (absolute, any generation):
- HP: 2000
- Attack: 400
- Defense: 400
- Speed: 400

**Application Order**:
1. Calculate raw stat value
2. Apply floor constraint: `max(raw_value, floor)`
3. Apply soft ceiling: `min(value, soft_ceiling)`
4. Apply hard ceiling: `min(value, hard_ceiling)`

### 1.5 Complete Stat Calculation Example

```python
def calculate_offspring_stat(parent_a_stat, parent_b_stat, generation):
    # Step 1: Base average
    base = (parent_a_stat + parent_b_stat) / 2

    # Step 2: Generation multiplier
    gen_multiplier = 1 + (generation * 0.01)

    # Step 3: Random variance
    variance = random.uniform(0.95, 1.05)

    # Step 4: Raw calculation
    raw_value = base * gen_multiplier * variance

    # Step 5: Apply constraints (example for HP)
    floored = max(raw_value, 50)
    soft_capped = min(floored, 500 * (1 + generation * 0.02))
    final = min(soft_capped, 2000)

    return int(final)
```

---

## 2. Generation Calculation

### 2.1 Basic Rule

```
offspring_generation = max(parent_a.generation, parent_b.generation) + 1
```

### 2.2 Rationale

- Ensures generational progression
- Prevents "breeding down" to reset power creep
- Creates clear lineage stratification
- Simplifies tournament generation restrictions

### 2.3 Edge Cases

**Same Generation Parents**:
- Gen 4 + Gen 4 → Gen 5 (standard progression)

**Large Generation Gap**:
- Gen 2 + Gen 8 → Gen 9 (inherits from highest parent)

**Generation 0 (Wild/Starter Kaiju)**:
- Gen 0 + Gen 0 → Gen 1
- Gen 0 + Gen 3 → Gen 4

---

## 3. Trait Inheritance Mechanics

### 3.1 Trait Collection Phase

**Input**: Combined trait pool from both parents
```
visible_pool = parent_a.visible_traits + parent_b.visible_traits
hidden_pool = parent_a.hidden_traits + parent_b.hidden_traits
```

**Deduplication**: If both parents have the same trait, it appears once in the pool but with increased inheritance probability (see 3.3).

### 3.2 Inheritance Probabilities

For each trait in the pool, roll independently:

**Visible Traits**:
- Base probability: **45%** per trait
- If both parents have the trait: **70%** (dominance bonus)

**Hidden Traits**:
- Base probability: **25%** per trait
- If both parents have the trait: **50%** (dominance bonus)

### 3.3 Dominant/Recessive Trait Interactions

Each trait has an inheritance type:

**Dominant (D)**:
- Inherits with base probability if present in one parent
- Guaranteed inheritance if present in both parents (100%)

**Recessive (R)**:
- Inherits at 50% of base probability if present in one parent
- Inherits at full base probability if present in both parents

**Codominant (C)**:
- Both traits express when paired with different trait
- Creates new combined trait (e.g., Red + Blue = Purple)

**Example Calculations**:

| Trait Type | One Parent | Both Parents |
|------------|-----------|--------------|
| Dominant Visible | 45% | 100% |
| Recessive Visible | 22.5% | 45% |
| Dominant Hidden | 25% | 100% |
| Recessive Hidden | 12.5% | 25% |

### 3.4 Polygenic Trait Combinations

Some traits require **multiple genes** to express:

**Example**: "Storm Mastery" (hidden trait)
- Requires: "Electric Affinity" + "Wind Affinity"
- Activation: If offspring inherits both, 30% chance to unlock Storm Mastery
- Polygenic traits do not appear in parent trait pools directly
- They are **emergent** based on trait combinations

**Polygenic Synthesis Table** (examples):

| Required Traits | Emergent Trait | Activation % |
|----------------|----------------|--------------|
| Electric + Water | Conductivity Weakness | 40% |
| Fire + Earth | Volcanic Armor | 35% |
| Wind + Water | Storm Mastery | 30% |
| Dark + Poison | Necrotic Touch | 25% |

**Processing**:
1. Inherit all traits normally
2. Check offspring traits against polygenic table
3. For each matching combination, roll for activation
4. Add emergent traits to hidden traits list

### 3.5 Conditional Traits

Traits with conditions only activate under specific circumstances:

**Structure**:
```rust
struct ConditionalTrait {
    name: String,
    condition: TraitCondition,
    effect: TraitEffect,
}

enum TraitCondition {
    EnvironmentType(EnvironmentType),  // e.g., "Storm", "Underwater"
    OpponentTrait(TraitId),             // e.g., "vs Fire types"
    HealthThreshold(f32),               // e.g., "below 30% HP"
    TurnNumber(u32),                    // e.g., "after turn 3"
}
```

**Inheritance**: Conditional traits inherit using standard probabilities, but their conditions are evaluated during battle, not breeding.

---

## 4. Mutation System

### 4.1 Mutation Probability

After normal trait inheritance:
- **10% chance** for a mutation to occur
- Only **one mutation** per breeding (cannot stack multiple mutations)

### 4.2 Mutation Types

When a mutation triggers, roll for type:

| Mutation Type | Probability | Effect |
|--------------|-------------|---------|
| Stat Mutation | 40% | Randomly boost one stat |
| New Trait | 30% | Add random new trait |
| Trait Power Boost | 20% | Increase existing trait power |
| Hidden Unlock | 10% | Reveal/enhance hidden trait |

### 4.3 Mutation Power Ranges

**Stat Mutation**:
```python
mutated_stat = base_stat * random.uniform(1.05, 1.15)
target_stat = random.choice(['hp', 'attack', 'defense', 'speed'])
```
- Affects one random stat
- Boost range: **+5% to +15%**
- Applied after normal inheritance calculation

**New Trait Mutation**:
- Selects from mutation trait pool (separate from normal traits)
- Power range: **1 to 5**
- 70% hidden, 30% visible

**Trait Power Boost**:
```python
target_trait = random.choice(offspring.traits + offspring.hidden_traits)
target_trait.power += random.randint(1, 3)
```

**Hidden Unlock**:
- Converts one hidden trait to visible
- Or: Increases hidden trait power by 50%

### 4.4 Mutation Flags

Mutated offspring receive a permanent flag:
```rust
struct MutationRecord {
    mutation_type: MutationType,
    generation_occurred: u32,
    affected_stat_or_trait: String,
}
```

This enables:
- Lineage tracking of mutation lines
- Research facility analysis
- Market value assessment

### 4.5 Multiple Mutation Handling

**Current Rule**: Only one mutation per breeding.

**Rationale**:
- Prevents runaway power creep
- Makes mutations special and memorable
- Simplifies probability calculations

**Future Extension Hook**:
- "Unstable Genome" trait could allow multiple mutations
- Higher generation breeding could increase mutation chance
- Special breeding facilities could modify mutation rates

---

## 5. Breeding Restrictions

### 5.1 Parent-Child Breeding Block

**Rule**: Direct parent-child breeding is **prohibited**.

**Implementation**:
```rust
fn can_breed(kaiju_a: &Kaiju, kaiju_b: &Kaiju) -> Result<(), BreedingError> {
    // Check if A is parent of B
    if let Some((parent_a, parent_b)) = &kaiju_b.parent_ids {
        if kaiju_a.token_id == *parent_a || kaiju_a.token_id == *parent_b {
            return Err(BreedingError::DirectParentChild);
        }
    }

    // Check if B is parent of A
    if let Some((parent_a, parent_b)) = &kaiju_a.parent_ids {
        if kaiju_b.token_id == *parent_a || kaiju_b.token_id == *parent_b {
            return Err(BreedingError::DirectParentChild);
        }
    }

    Ok(())
}
```

### 5.2 Sibling Breeding

**Allowed**: Siblings (same parents) CAN breed with each other.

**Rationale**:
- Enables bloodline concentration
- Creates strategic breeding decisions
- Realistic genetic simulation (with consequences)

### 5.3 Generation Gap Limits

**Current System**: No generation gap restrictions.

**Future Consideration**:
- Optional tournament rule: "Same generation only"
- Facility upgrades could enable cross-generation breeding bonuses
- Meta-game balance may require generation gap caps

### 5.4 Death Status Check

**Rule**: Dead kaiju **cannot breed**.

```rust
fn validate_alive(kaiju: &Kaiju) -> Result<(), BreedingError> {
    if !kaiju.alive {
        return Err(BreedingError::ParentDead(kaiju.token_id));
    }
    Ok(())
}
```

### 5.5 Minimum Requirements

Both parents must meet:
- **Alive**: `kaiju.alive == true`
- **Not in active tournament**: `kaiju.locked_in_tournament == false`
- **Breeding rights available**: Valid breeding rights token exists

---

## 6. Breeding Rights System

### 6.1 Rights Token Structure

Breeding rights are **separate NFTs** (ERC-721 or ERC-1155):

```solidity
struct BreedingRight {
    uint256 rightId;
    uint256 parentKaijuId;  // Which kaiju this right is for
    address holder;          // Current owner of the right
    uint8 usesRemaining;     // Typically 1
    uint256 expiryBlock;     // Optional: expires after X blocks
    bool consumed;
}
```

### 6.2 Rights Consumption Flow

**Before Breeding**:
1. Verify holder owns breeding rights for both parents
2. Check both rights are not consumed
3. Check parent kaiju are not dead

**During Breeding**:
1. Generate offspring
2. Mark both breeding rights as consumed
3. Burn or decrement breeding rights NFTs

**After Breeding**:
1. Mint offspring NFT
2. Record breeding event in parent histories
3. Update lineage database

### 6.3 Cooldown Mechanics

**Current System**: No cooldowns implemented.

**Design Space** (for future):
- **Per-Kaiju Cooldown**: 7 days between uses of same kaiju
- **Facility-Based**: Lab upgrades reduce cooldown
- **Generation-Based**: Higher gen kaiju have longer cooldowns

**Rationale for No Cooldown (v1)**:
- Breeding rights already limit usage (one-time tokens)
- Economic scarcity through rights marketplace
- Simpler implementation

### 6.4 Death Impact on Rights

**Automatic Refund Rule**:

```rust
fn handle_kaiju_death(kaiju_id: TokenId, blockchain: &mut Blockchain) {
    // Find all active breeding rights for this kaiju
    let active_rights = blockchain.get_breeding_rights_for_kaiju(kaiju_id);

    for right in active_rights {
        if !right.consumed {
            // Refund the holder
            blockchain.refund_breeding_right(right.holder, right.purchase_price);

            // Burn the rights NFT
            blockchain.burn_nft(right.right_id);

            // Log refund event
            blockchain.emit_event(BreedingRightRefund {
                right_id: right.right_id,
                parent_kaiju: kaiju_id,
                holder: right.holder,
                refund_amount: right.purchase_price,
            });
        }
    }
}
```

**Key Points**:
- Refunds only for **unused** rights
- Consumed rights are not refunded (breeding already occurred)
- Refund amount based on original purchase price
- Prevents value loss from sudden kaiju death

---

## 7. Offspring Naming

### 7.1 Naming Authority

**Player Choice**: The breeding initiator (rights holder) names the offspring.

**Constraints**:
- Length: 3-20 characters
- Allowed: Letters, numbers, spaces, hyphens, apostrophes
- No profanity filter in base system (optional moderation layer)

### 7.2 Name Uniqueness

**Global Uniqueness**: NOT required.
- Multiple kaiju can have the same name
- Token ID is the unique identifier
- Enables thematic breeding (e.g., all "Storm" lineage)

**Display Format**:
```
Name (#TokenID)
Example: "Stormling (#4521)"
```

### 7.3 Name Generation (Optional Helper)

For players who want suggestions:

```python
def generate_offspring_name(parent_a, parent_b):
    # Strategy 1: Portmanteau
    if random.random() < 0.4:
        return portmanteau(parent_a.name, parent_b.name)

    # Strategy 2: Trait-based
    if random.random() < 0.3:
        dominant_trait = select_strongest_trait(offspring)
        return f"{trait_adjective(dominant_trait)} {random_suffix()}"

    # Strategy 3: Generation-based
    if random.random() < 0.2:
        return f"{parent_a.name} Jr." or f"Neo-{parent_b.name}"

    # Strategy 4: Random from name pool
    return random.choice(KAIJU_NAME_POOL)
```

**Example Outputs**:
- Flossy + Reefmaw → "Flomaw" (portmanteau)
- Electric trait dominant → "Voltspine" (trait-based)
- Gen 5 offspring → "Reefmaw II" (generation marker)

### 7.4 Naming Edge Cases

**Empty Name**:
- Default to: `"Kaiju #[TokenID]"`

**Special Characters**:
- Strip emojis and control characters
- Preserve Unicode for international players (optional)

**Reserved Names**:
- System names (e.g., "NULL", "ADMIN") rejected
- Historical legends (optional prestige system)

---

## 8. Visual Seed Generation

### 8.1 Seed Structure

Each kaiju has a **64-bit visual seed** (u64):

```rust
struct VisualSeed(u64);
```

This seed deterministically generates the kaiju's appearance.

### 8.2 Seed Derivation from Parents

```python
def generate_visual_seed(parent_a, parent_b, offspring_stats, offspring_traits):
    # Component 1: Parent seed mixing (40% influence)
    parent_mix = (parent_a.visual_seed ^ parent_b.visual_seed) & 0xFFFFFFFF00000000

    # Component 2: Stat-based variation (30% influence)
    stat_hash = hash(
        offspring.stats.hp,
        offspring.stats.attack,
        offspring.stats.defense,
        offspring.stats.speed
    ) & 0x00000000FFFF0000

    # Component 3: Trait-based variation (20% influence)
    trait_hash = hash(
        ','.join(sorted([t.name for t in offspring.traits]))
    ) & 0x000000000000FF00

    # Component 4: Random mutation (10% influence)
    random_component = random.randint(0, 255) & 0x00000000000000FF

    # Combine all components
    visual_seed = parent_mix | stat_hash | trait_hash | random_component

    return VisualSeed(visual_seed)
```

### 8.3 Seed Interpretation

The visual seed maps to appearance features:

**Bit Allocation** (example):
```
Bits 63-56: Body type (256 variations)
Bits 55-48: Color palette (256 variations)
Bits 47-40: Scale/texture pattern (256 variations)
Bits 39-32: Size modifier (256 variations)
Bits 31-24: Head shape (256 variations)
Bits 23-16: Limb configuration (256 variations)
Bits 15-8:  Extra features (wings, tails, horns)
Bits 7-0:   Fine details (scars, markings)
```

### 8.4 Trait-Mandated Features

Certain traits **override** seed randomness:

**Example**:
- "Wings" trait → Forces wing rendering (seed determines wing style)
- "Electric Affinity" → Forces electrical visual effects
- "Aquatic" → Forces fins/gills/scales

**Implementation**:
```python
def apply_trait_visual_overrides(base_appearance, traits):
    for trait in traits:
        if trait.name == "Wings":
            base_appearance.wings_enabled = True
            base_appearance.wing_style = (trait.power % 8)  # 8 wing styles

        if trait.name == "Electric Affinity":
            base_appearance.aura_color = ELECTRIC_BLUE
            base_appearance.particle_effect = LIGHTNING

        # ... more trait mappings

    return base_appearance
```

### 8.5 Visual Consistency

**Same Seed = Same Appearance**:
- Deterministic rendering ensures kaiju always look the same
- Critical for NFT provenance
- Enables offline verification

**Ancestral Resemblance**:
- Offspring visually resemble parents (due to seed mixing)
- Siblings have similar but distinct appearances
- Lineage creates visual "families"

---

## 9. Example Breeding Scenarios

### Example 1: Basic Same-Generation Breeding

**Parents**:
- **Flossy** (Gen 4)
  - HP: 300, Attack: 60, Defense: 40, Speed: 30
  - Visible Traits: Electric Breath (power 8)
  - Hidden Traits: None
  - Visual Seed: 0xA1B2C3D4E5F60718

- **Reefmaw** (Gen 4)
  - HP: 320, Attack: 55, Defense: 45, Speed: 25
  - Visible Traits: Aqua Hide (power 6)
  - Hidden Traits: None
  - Visual Seed: 0x9876543210ABCDEF

**Calculation**:

1. **Generation**: `max(4, 4) + 1 = 5`

2. **HP**:
   - Base: `(300 + 320) / 2 = 310`
   - Gen multiplier: `1 + (5 * 0.01) = 1.05`
   - Variance roll: `1.02` (example)
   - Raw: `310 * 1.05 * 1.02 = 331.71`
   - Final: `331` (floored)

3. **Attack**:
   - Base: `(60 + 55) / 2 = 57.5`
   - Gen multiplier: `1.05`
   - Variance roll: `0.97`
   - Raw: `57.5 * 1.05 * 0.97 = 58.56`
   - Final: `58`

4. **Defense**:
   - Base: `(40 + 45) / 2 = 42.5`
   - Gen multiplier: `1.05`
   - Variance roll: `1.04`
   - Raw: `42.5 * 1.05 * 1.04 = 46.41`
   - Final: `46`

5. **Speed**:
   - Base: `(30 + 25) / 2 = 27.5`
   - Gen multiplier: `1.05`
   - Variance roll: `0.99`
   - Raw: `27.5 * 1.05 * 0.99 = 28.58`
   - Final: `28`

6. **Traits**:
   - Electric Breath (visible, one parent): Roll 0.38 < 0.45 → **Inherited**
   - Aqua Hide (visible, one parent): Roll 0.52 > 0.45 → **Not inherited**

7. **Mutation**: Roll 0.73 > 0.10 → **No mutation**

8. **Visual Seed**:
   ```
   parent_mix = 0xA1B2C3D4E5F60718 ^ 0x9876543210ABCDEF
              = 0x39C49707F55DCEF7 (upper 32 bits)

   stat_hash = hash(331, 58, 46, 28) & 0x00000000FFFF0000
             = 0x0000000072A40000

   trait_hash = hash("Electric Breath") & 0x000000000000FF00
              = 0x00000000000008B00

   random = 0x42

   visual_seed = 0x39C49707F55D0000 | 0x72A40000 | 0x8B00 | 0x42
               = 0x39C4970772AC8B42
   ```

**Offspring: "Stormling" (#4523)**
- Gen 5
- HP: 331, Attack: 58, Defense: 46, Speed: 28
- Visible Traits: Electric Breath (power 8)
- Hidden Traits: None
- Visual Seed: 0x39C4970772AC8B42

---

### Example 2: Cross-Generation with Dominant Trait

**Parents**:
- **Voltaire** (Gen 3)
  - HP: 280, Attack: 70, Defense: 35, Speed: 40
  - Visible Traits: Lightning Strike (power 10, Dominant)
  - Hidden Traits: Storm Resistance (power 5, Dominant)

- **Tidecaller** (Gen 7)
  - HP: 400, Attack: 65, Defense: 60, Speed: 35
  - Visible Traits: Tidal Wave (power 9)
  - Hidden Traits: None

**Calculation**:

1. **Generation**: `max(3, 7) + 1 = 8`

2. **Stats** (abbreviated):
   - HP: `(280 + 400) / 2 * 1.08 * 1.01 = 371`
   - Attack: `(70 + 65) / 2 * 1.08 * 0.96 = 70`
   - Defense: `(35 + 60) / 2 * 1.08 * 1.03 = 53`
   - Speed: `(40 + 35) / 2 * 1.08 * 0.98 = 40`

3. **Traits**:
   - Lightning Strike (Dominant, one parent): **100% → Inherited**
   - Tidal Wave (visible, one parent): Roll 0.29 < 0.45 → **Inherited**
   - Storm Resistance (Dominant hidden, one parent): **100% → Inherited**

4. **Polygenic Check**:
   - Has: Lightning Strike + Tidal Wave
   - Check table: Electric + Water → Conductivity Weakness
   - Roll: 0.35 < 0.40 → **Activated**
   - Add "Conductivity Weakness" to hidden traits

5. **Mutation**: Roll 0.08 < 0.10 → **Mutation triggered**
   - Type roll: 0.25 → New Trait (30% range)
   - New hidden trait: "Unstable Voltage" (power 3)

**Offspring: "Tempest" (#8921)**
- Gen 8
- HP: 371, Attack: 70, Defense: 53, Speed: 40
- Visible Traits: Lightning Strike (10), Tidal Wave (9)
- Hidden Traits: Storm Resistance (5), Conductivity Weakness (3), Unstable Voltage (3)
- Mutation: New Trait (Unstable Voltage)

---

### Example 3: Sibling Breeding with Recessive Traits

**Parents** (Both are siblings from same parents):
- **Ember** (Gen 6)
  - Visible Traits: Fire Breath (Recessive)
  - Hidden Traits: Pyroclastic (Recessive)

- **Scorch** (Gen 6)
  - Visible Traits: Fire Breath (Recessive), Hardened Scales
  - Hidden Traits: Pyroclastic (Recessive)

**Calculation**:

1. **Generation**: `max(6, 6) + 1 = 7`

2. **Traits**:
   - Fire Breath (Recessive, **both** parents): 45% (full base) → Roll 0.41 → **Inherited**
   - Hardened Scales (one parent): Roll 0.68 > 0.45 → **Not inherited**
   - Pyroclastic (Recessive hidden, **both** parents): 25% (full base) → Roll 0.19 → **Inherited**

**Result**: Sibling breeding successfully concentrates recessive traits.

**Offspring: "Inferno" (#9234)**
- Gen 7
- Visible Traits: Fire Breath
- Hidden Traits: Pyroclastic
- Note: Pure fire lineage preserved through sibling breeding

---

### Example 4: Mutation - Stat Boost

**Parents**:
- **Granite** (Gen 5): HP 350, Attack 50, Defense 80, Speed 20
- **Ironhide** (Gen 5): HP 340, Attack 48, Defense 85, Speed 18

**Calculation**:

1. **Stats** (before mutation):
   - HP: 351, Attack: 50, Defense: 88, Speed: 20

2. **Mutation**: Roll 0.03 < 0.10 → **Triggered**
   - Type roll: 0.12 → Stat Mutation (40% range)
   - Target stat: Defense
   - Boost: 1.12 (between 1.05-1.15)
   - Mutated Defense: `88 * 1.12 = 98.56 → 98`

**Offspring: "Fortress" (#5672)**
- Gen 6
- HP: 351, Attack: 50, Defense: **98** (mutated), Speed: 20
- Mutation Record: Stat boost (Defense +11%)

---

### Example 5: Maximum Generation with Soft Cap

**Parents**:
- **Apex** (Gen 15): HP 520, Attack 110, Defense 105, Speed 85
- **Omega** (Gen 14): HP 510, Attack 115, Defense 100, Speed 90

**Calculation**:

1. **Generation**: `max(15, 14) + 1 = 16`

2. **HP** (with soft cap):
   - Base: `(520 + 510) / 2 = 515`
   - Gen multiplier: `1 + (16 * 0.01) = 1.16`
   - Variance: `1.03`
   - Raw: `515 * 1.16 * 1.03 = 615.14`
   - Soft cap: `500 * (1 + 16 * 0.02) = 660`
   - Hard cap: `2000`
   - Final: `615` (within soft cap)

3. **Attack** (hitting soft cap):
   - Raw: `112.5 * 1.16 * 1.02 = 133.11`
   - Soft cap: `100 * (1 + 16 * 0.02) = 132`
   - Final: `132` (capped)

**Offspring: "Zenith" (#12945)**
- Gen 16
- HP: 615, Attack: 132 (soft capped), Defense: 125, Speed: 105
- Note: Starting to hit soft caps, demonstrating diminishing returns

---

### Example 6: Failed Breeding - Parent-Child Block

**Attempted Parents**:
- **Stormling** (#4523, Gen 5) - parent of Cascade
- **Cascade** (#5891, Gen 6) - child of Stormling

**Validation**:
```rust
can_breed(Stormling, Cascade)?
→ Error: BreedingError::DirectParentChild
```

**Result**: Breeding blocked. Player must select different parents.

---

### Example 7: Dead Parent - Rights Refund

**Setup**:
- **Blaze** (#3421) - Player A owns kaiju
- Player B purchased breeding rights for Blaze (cost: 100 tokens)
- Player C purchased breeding rights for Blaze (cost: 100 tokens)
- Blaze enters lethal tournament and dies

**Refund Process**:
1. Blaze marked as dead
2. System finds 2 active breeding rights
3. Player B refunded 100 tokens, rights NFT burned
4. Player C refunded 100 tokens, rights NFT burned
5. Total refunded: 200 tokens

---

### Example 8: Polygenic Trait Emergence

**Parents**:
- **Zephyr** (Gen 4): Wind Affinity (power 7), Speed Boost (power 4)
- **Aquarius** (Gen 5): Water Affinity (power 8), Regeneration (power 3)

**Trait Inheritance**:
- Wind Affinity: Inherited
- Water Affinity: Inherited
- Speed Boost: Not inherited
- Regeneration: Inherited

**Polygenic Check**:
- Combination: Wind + Water
- Table match: Storm Mastery (30% chance)
- Roll: 0.22 < 0.30 → **Activated**

**Offspring: "Monsoon" (#6781)**
- Gen 6
- Visible Traits: Wind Affinity (7), Water Affinity (8)
- Hidden Traits: Regeneration (3), **Storm Mastery (emergent, power 5)**

---

### Example 9: Multiple Trait Inheritance (Lucky Roll)

**Parents**:
- **Apex Predator** (Gen 8)
  - Visible: Razor Claws (9), Night Vision (6), Stealth (7)
  - Hidden: Pack Tactics (4), Fear Aura (5)

- **Silent Death** (Gen 8)
  - Visible: Poison Fangs (8), Stealth (7)
  - Hidden: Ambush Instinct (6)

**Trait Pool**:
- Visible: Razor Claws, Night Vision, Stealth (x2), Poison Fangs
- Hidden: Pack Tactics, Fear Aura, Ambush Instinct

**Inheritance Rolls** (lucky scenario):
- Razor Claws (45%): Roll 0.32 → **Yes**
- Night Vision (45%): Roll 0.41 → **Yes**
- Stealth (70%, both parents): Roll 0.55 → **Yes**
- Poison Fangs (45%): Roll 0.39 → **Yes**
- Pack Tactics (25%): Roll 0.18 → **Yes**
- Fear Aura (25%): Roll 0.29 → No
- Ambush Instinct (25%): Roll 0.12 → **Yes**

**Offspring: "Shadow Reaper" (#9512)**
- Gen 9
- Visible Traits: Razor Claws (9), Night Vision (6), Stealth (7), Poison Fangs (8)
- Hidden Traits: Pack Tactics (4), Ambush Instinct (6)
- Note: Inherited 6 out of 7 traits (exceptionally high)

---

### Example 10: Minimum Stat Floor Application

**Parents** (deliberately weak):
- **Runt** (Gen 2): HP 80, Attack 15, Defense 10, Speed 12
- **Weakling** (Gen 3): HP 90, Attack 18, Defense 12, Speed 10

**Raw Calculation**:
- HP: `(80 + 90) / 2 * 1.04 * 0.96 = 82.62`
- Attack: `(15 + 18) / 2 * 1.04 * 0.95 = 16.15`
- Defense: `(10 + 12) / 2 * 1.04 * 0.97 = 11.11`
- Speed: `(12 + 10) / 2 * 1.04 * 0.96 = 10.99`

**Floor Application**:
- HP: `max(82.62, 50) = 82` ✓ (above floor)
- Attack: `max(16.15, 10) = 16` ✓ (above floor)
- Defense: `max(11.11, 5) = 11` ✓ (above floor)
- Speed: `max(10.99, 5) = 10` ✓ (above floor)

**Offspring: "Survivor" (#1234)**
- Gen 4
- HP: 82, Attack: 16, Defense: 11, Speed: 10
- Note: All stats above floors, but notably weak compared to typical Gen 4

---

## 10. Edge Cases and Special Scenarios

### 10.1 Both Parents Same Stats

**Scenario**: Cloned or identical parents
- **Result**: Base average equals individual stat
- Variance still applies (0.95-1.05 range)
- Generation multiplier still applies
- Offspring will be **slightly** stronger due to generation increment

### 10.2 Maximum Generation Breeding

**Current System**: No hard generation cap
- Soft caps naturally limit power growth
- Gen multiplier continues indefinitely: `1 + (gen * 0.01)`
- At Gen 100: multiplier = 2.0 (double base stats)

**Recommendation**: Set hard generation cap at **Gen 50** for v1
- Gen 50 multiplier: 1.5x (reasonable ceiling)
- Prevents extreme late-game scenarios
- Can be extended in future versions

### 10.3 One Parent Gen 0 (Wild Kaiju)

**Setup**: Gen 0 + Gen 5 breeding
- **Result**: Gen 6 offspring (inherits highest + 1)
- Wild genes may introduce unique traits
- Useful for "refreshing" bloodlines

### 10.4 All Traits Fail to Inherit

**Probability**: Rare but possible
- No visible traits inherited
- No hidden traits inherited
- No mutation triggered

**Result**: "Blank slate" kaiju
- Only has base stats
- May be valuable for specific breeding strategies
- Research facilities could reveal latent potential

### 10.5 Mutation on Already-Mutated Parent

**Scenario**: Offspring of mutated parent also mutates
- **Result**: Each mutation is independent
- Mutations do not stack directly
- Creates "mutation lineages" trackable via flags

### 10.6 Breeding Rights Expiry During Tournament

**Scenario**: Kaiju locked in lethal tournament, rights expire before resolution
- **Solution**: Rights frozen during tournament lock
- Expiry timer pauses
- After tournament: rights reactivate or refund if kaiju died

### 10.7 Identical Visual Seeds (Collision)

**Probability**: 1 in 2^64 (astronomically low)
- **Handling**: Seed generation includes timestamp + nonce
- Collision detection appends collision counter
- Functionally impossible in practice

### 10.8 Stat Underflow Prevention

**Scenario**: Very weak parents with unlucky variance rolls
- **Protection**: Floors prevent stats dropping below minimums
- Ensures all kaiju remain viable
- May result in identical low-stat offspring (acceptable)

### 10.9 Trait Power Overflow

**Scenario**: Mutation boosts trait power beyond reasonable limits
- **Cap**: Individual trait power capped at **20**
- Prevents single-trait dominance
- Encourages trait diversity over single-trait stacking

### 10.10 Simultaneous Breeding of Same Kaiju

**Scenario**: Two players use breeding rights for same kaiju at exact same time
- **Solution**: Blockchain transaction ordering resolves conflict
- First transaction succeeds, second is rejected (rights already consumed)
- Off-chain queueing prevents partial state

---

## 11. Implementation Checklist

### Core Breeding Function
- [ ] Implement generation calculation
- [ ] Implement stat inheritance with variance
- [ ] Apply generation multiplier
- [ ] Enforce stat floors and ceilings
- [ ] Implement visible trait inheritance (45%)
- [ ] Implement hidden trait inheritance (25%)
- [ ] Implement dominant/recessive logic
- [ ] Implement polygenic trait synthesis
- [ ] Implement mutation system (10%)
- [ ] Generate visual seed from parents
- [ ] Create breeding record in parent histories

### Validation Layer
- [ ] Check both parents alive
- [ ] Check parent-child relationship block
- [ ] Check breeding rights ownership
- [ ] Check breeding rights not consumed
- [ ] Check kaiju not locked in tournament
- [ ] Validate offspring name

### Breeding Rights Management
- [ ] Breeding rights NFT minting
- [ ] Rights consumption on breeding
- [ ] Death-triggered refund system
- [ ] Expiry timer management
- [ ] Rights marketplace integration

### Data Recording
- [ ] Store breeding event in database
- [ ] Update parent breeding histories
- [ ] Record mutation events
- [ ] Update lineage tree
- [ ] Emit blockchain events

### Testing Scenarios
- [ ] Test all 10 example scenarios
- [ ] Test edge cases (same stats, weak parents, etc.)
- [ ] Test validation failures (dead parent, parent-child, etc.)
- [ ] Test mutation probability (run 10,000 breedings, verify ~10%)
- [ ] Test trait inheritance probabilities
- [ ] Test visual seed uniqueness (collision detection)
- [ ] Test stat cap enforcement

---

## 12. Future Extensions

### 12.1 Advanced Genetics
- **Genetic Exhaustion**: Bloodlines lose vitality after many generations
- **Lethal Combinations**: Some trait combos cause stillbirth (rare)
- **Triple Breeding**: Combine 3 parents (requires special facility)
- **Genetic Engineering**: Lab-based trait insertion (expensive, risky)

### 12.2 Environmental Breeding
- **Location-Based Bonuses**: Breeding in specific environments affects traits
- **Seasonal Effects**: Time of year influences mutation rates
- **Cosmic Events**: Rare events trigger unique mutations

### 12.3 Social Breeding
- **Breeding Guilds**: Pooled knowledge improves outcomes
- **Rival Sabotage**: Espionage can introduce negative traits
- **Prestige Breeding**: Champion bloodlines attract better partners

### 12.4 Economy Enhancements
- **Breeding Insurance**: Pay premium to guarantee no bad mutations
- **Trait Marketplace**: Buy/sell specific traits independently
- **Stud Fees**: Automated breeding rights rental system
- **Genetic Patents**: First to create trait combo gets royalties

---

## Appendix A: Trait Inheritance Probability Table

| Scenario | Visible Trait | Hidden Trait |
|----------|--------------|--------------|
| One parent, dominant | 45% | 25% |
| One parent, recessive | 22.5% | 12.5% |
| Both parents, dominant | 100% | 100% |
| Both parents, recessive | 45% | 25% |
| Codominant pair | Special (creates new trait) | N/A |

---

## Appendix B: Stat Constraints Reference

| Stat | Floor | Gen 1 Soft Cap | Hard Cap | Soft Cap Formula |
|------|-------|----------------|----------|------------------|
| HP | 50 | 500 | 2000 | 500 * (1 + gen * 0.02) |
| Attack | 10 | 100 | 400 | 100 * (1 + gen * 0.02) |
| Defense | 5 | 100 | 400 | 100 * (1 + gen * 0.02) |
| Speed | 5 | 100 | 400 | 100 * (1 + gen * 0.02) |

---

## Appendix C: Mutation Type Distribution

| Mutation Type | Weight | Effect |
|--------------|--------|--------|
| Stat Mutation | 40% | +5% to +15% on one random stat |
| New Trait | 30% | Add new random trait (70% hidden) |
| Trait Power Boost | 20% | +1 to +3 power on existing trait |
| Hidden Unlock | 10% | Convert hidden→visible or +50% power |

---

## Appendix D: Polygenic Trait Synthesis Table

| Trait A | Trait B | Emergent Trait | Activation % | Type |
|---------|---------|----------------|--------------|------|
| Electric | Water | Conductivity Weakness | 40% | Negative |
| Fire | Earth | Volcanic Armor | 35% | Positive |
| Wind | Water | Storm Mastery | 30% | Positive |
| Dark | Poison | Necrotic Touch | 25% | Offensive |
| Light | Fire | Solar Flare | 30% | Offensive |
| Ice | Water | Permafrost | 35% | Defensive |
| Electric | Wind | Lightning Speed | 28% | Mobility |
| Earth | Poison | Toxic Spores | 32% | AOE |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-09 | Initial comprehensive specification |

---

## Critical Files for Implementation

Based on this specification, the following files are most critical for implementing the breeding system:

### Critical Files for Implementation

1. **H:\RustGames\kaiju_sim\IMPLEMENTATION_GUIDE.md** - Core architecture reference
   - Defines the overall system structure
   - Contains trait system enum definitions
   - Provides breeding service pseudocode foundation

2. **H:\RustGames\kaiju_sim\kaiju_sim.md** - Original breeding prototype code
   - Contains the reference Python breeding implementation (lines 477-510)
   - Defines the core mathematical formulas currently in use
   - Provides battle simulation context for trait effects

3. **H:\RustGames\kaiju_sim\nft_design.md** - Breeding rights and death mechanics
   - Details the breeding rights NFT system (section 7)
   - Specifies death-triggered refund logic
   - Defines on-chain vs off-chain data separation

4. **Future: `engine/breeding.rs`** - Primary implementation file (to be created)
   - Will contain the complete breed() function
   - Trait inheritance logic
   - Mutation system
   - Validation layer

5. **Future: `data/traits.rs`** - Trait definition and inheritance types (to be created)
   - TraitInheritance enum (Dominant/Recessive/Polygenic/Conditional)
   - Polygenic synthesis lookup table
   - Trait metadata and conditions

---

**END OF SPECIFICATION**
