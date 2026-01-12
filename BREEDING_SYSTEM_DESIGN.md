# BREEDING SYSTEM DESIGN
**Version**: 2.1 (Implementation Complete)
**References**: `BREEDING_ALGORITHM_SPEC.md`, `GENOME_ENCODING_SPEC.md`, `TRAIT_SYSTEM_DESIGN.md`

## 1. Design Philosophy: "Genetic Depth"
The breeding system is not just a random generator; it is a **simulation of distinct biological lineages**. Each breed decision must weigh **Cost** vs. **Time** vs. **Risk**. The system heavily rewards mastery of hidden mechanics (Recessive/Polygenic traits) while providing casual players with cool results via Dominant traits.

---

## 2. Core Breeding Mechanics

### 2.1 Inheritance Probabilities
Based on User Requirements (overriding previous default specs):

| Trait Type | Inheritance Rate | Logic |
|------------|------------------|-------|
| **Dominant** | **75%** | Highly predictable. Drives the "Base Archetype" of the offspring. |
| **Recessive** | **25%** | Low chance. Requires consistent lineage or luck to preserve. |
| **Mutation** | **2% - 5%** | Ultra-rare. The "Jackpot" mechanic. |

**Implementation**: See `kaiju_server/src/breeding/inheritance.rs`

### 2.2 Hybridization & Stats
Offspring stats are not just averages; they are **weighted resultants** of parents, influenced by Element Types.

**Example Scenario**:
- **Parent A (Fire)**: High Attack, Low Defense.
- **Parent B (Water)**: High Defense, Balanced stats.
- **Offspring (Fire-Water Hybrid)**:
    - **HP**: `avg(A, B) * variance` -> 58
    - **Attack**: `weighted_avg(A_high, B_mid)` -> 51
    - **Defense**: `weighted_avg(A_low, B_high)` -> 79
    - **Speed**: `variance_bloom` -> 109 (Speed breakout due to Steam/Gas element interaction?)

**Hybrid Elements** (Implemented in `element_system.rs`):
- **Fire + Water** -> **Steam / Misty** (High Speed / Evasion)
- **Earth + Wind** -> **Sandstorm** (Erosion damage)
- **Dark + Light** -> **Eclipse** (Phase shifting)
- **Electric + Water** -> **Conductivity** (Amplified damage)
- **Fire + Earth** -> **Volcanic** (High defense + damage)
- **Ice + Wind** -> **Blizzard** (High speed + slow)
- **Poison + Dark** -> **Necrotic** (DoT + debuff)
- **Nature + Water** -> **Swamp** (Regen + poison)
- **Electric + Wind** -> **Storm** (Chain attacks)
- **Ice + Water** -> **Arctic** (Freeze + defense)
- **Psychic + Dark** -> **Void** (Reality warp)
- **Light + Nature** -> **Radiant** (Healing + buff)

---

## 3. Economy & Time Constraints

Breeding is expensive and slow to prevent market flooding and increase the value of each decision.

### 3.1 Base Breeding Costs (Credits)
| Parent 1 Rarity | Parent 2 Rarity | Cost |
|-----------------|-----------------|------|
| Common | Common | **1,000** |
| Common | Rare | **5,000** |
| Rare | Rare | **15,000** |
| Legendary | Any | **50,000** |

*Note: Rarity is determined by the sum of Trait Power + Base Stat Total.*

### 3.2 Time Requirements
| Phase | Duration | Description |
|-------|----------|-------------|
| **Gestation** | **6 - 48 hours** | Time until egg hatches. Scales with Rarity/Generation. |
| **Maturation** | **24 - 120 hours** | Time until Kaiju can battle/breed. |
| **Cooldown** | **12 hours** | Minimum rest between breeding cycles per parent. |

**Implementation**: See `kaiju_server/src/breeding/stat_calculator.rs`

---

## 4. Special Materials (Item System)
Players can influence the RNG using consumables.

| Item Name | Effect | Rarity | Cost |
|-----------|--------|--------|------|
| **Elemental Essence** | **Guarantees** primary element inheritance (e.g., Fire Essence guarantees Fire type). | Uncommon | 2,000 |
| **Mutation Catalyst** | Increases Mutation Chance by **+5%** (Flat). Max 1 per breed. | Rare | 5,000 |
| **Genetic Stabilizer** | Prevents inheritance of **Negative Traits** and **Stat Down-scaling**. | Rare | 3,000 |
| **Fertility Idol** | Reduces Gestation Time by **50%**. | Uncommon | 1,500 |

**API Usage**:
```json
POST /breeding/breed
{
  "parent_a_id": "uuid-a",
  "parent_b_id": "uuid-b",
  "client_seed": 12345,
  "user_id": "uuid-user",
  "materials": [
    { "type": "mutation_catalyst" },
    { "type": "elemental_essence", "element": "fire" }
  ]
}
```

---

## 5. AI Prompt Engineering (The "Titanclaw" Standard)

To match the "Depth" requirement, the AI prompt generation must be highly structured. We do not use simple tags; we build a **Visual Narrative**.

### Prompt Construction Template
```text
[Subject]: [Name], [Title], [Body Type], [Scale]
[Features]: [Skin/Texture], [Appendages], [Head/Face], [Tail]
[Atmosphere]: [VFX/Energy], [Damage/Scars], [Color Palette]
[Composition]: [Pose], [Setting], [Camera/Lighting]
[Style]: kaiju monster, creature design, highly detailed, concept art, dramatic lighting, photorealistic, no weapons, no armor, natural creature
```

### Example: "Titanclaw the Volcanic Behemoth"
- **Subject**: Titanclaw, the Volcanic Behemoth, massive quadruped muscular thick limbs, massive building-sized enormous scale
- **Features**: Fractured rocky plates with glowing cracks, Massive clawed fists and obsidian spikes, Dinosaur-like with reinforced skull, molten eyes, Thick, segmented with spiked club end
- **Atmosphere**: Glowing magma veins with volcanic emissions, Asymmetrical damage, shattered spikes, scarred eye, Charcoal black, volcanic orange, deep reds
- **Composition**: Mid-roar, ground fracturing under weight, Destroyed cityscape with smoke and fire, Wide angle full body shot, dramatic perspective, Full body centered, entire creature visible
- **Style**: kaiju monster, creature design...

**Implementation Note**: The Breeding Service will construct these prompts dynamically by pulling `visual_keywords` from inherited Traits and combining them.

---

## 6. Admin Visibility & Debugging (The "Glass Box")

For admins to verify the "Depth", the logging system must expose the internal RNG rolls.

### Debug Log Format (Structured JSON)
Every breeding event emits a `BreedingLog`:

```json
{
  "event_id": "breed_x892",
  "parents": { "A": "Kaiju_123 (Fire)", "B": "Kaiju_456 (Water)" },
  "rolls": {
    "stat_inheritance": {
      "hp": { "roll": 0.98, "formula": "(300+350)/2 * 1.05 * 0.98", "result": 334 },
      "attack": { "roll": 1.04, "formula": "...", "result": 62 }
    },
    "trait_inheritance": {
      "Fire Breath (Dominant)": { "chance": 0.75, "roll": 0.42, "result": "INHERITED" },
      "Toxic Cloud (Recessive)": { "chance": 0.25, "roll": 0.88, "result": "LOST" },
      "Mutation Check": { "chance": 0.05, "roll": 0.03, "result": "SUCCESS -> 'Neon Spines'" }
    }
  },
  "modifiers": {
    "items_used": ["Mutation Catalyst"],
    "facility_bonus": 0.0
  },
  "outcome": {
    "id": "Kaiju_789",
    "genome_hash": "0xABC...",
    "rarity_calc": "Common + Rare = Rare"
  }
}
```

**Implementation**: See `kaiju_server/src/breeding/breeding_log.rs`

**Admin Dashboard**:
Admins can view a "Tree Diagram" of any Kaiju, clicking on connections to see *exactly* why a trait was passed or lost (the specific RNG roll vs Threshold).

---

## 7. Implementation Summary

### Modules Created
| Module | Purpose |
|--------|---------|
| `breeding/mod.rs` | Module exports |
| `breeding/breeding_config.rs` | All breeding parameters and rates |
| `breeding/breeding_log.rs` | Full admin visibility logging |
| `breeding/element_system.rs` | Hybrid elements and visual keywords |
| `breeding/inheritance.rs` | Dominant/Recessive/Polygenic trait logic |
| `breeding/mutation.rs` | Mutation triggering and effects |
| `breeding/stat_calculator.rs` | Stat inheritance with constraints |
| `breeding/trait_registry.rs` | Default trait and synergy definitions |
| `breeding/service.rs` | Main AdvancedBreedingService |

### API Endpoints Added
| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/breeding/breed` | POST | Start breeding with optional materials |
| `/breeding/status/:job_id` | GET | Check breeding job status |
| `/breeding/locked` | GET | Get list of kaiju currently in breeding |
| `/breeding/cost` | POST | Calculate breeding costs before starting |

### Database Migrations
- `20240101000013_create_breeding_depth.sql`: Breeding logs, cooldowns, and materials inventory tables

### Key Features
1. ✅ **Inheritance by Type**: Dominant (75%), Recessive (25%), Polygenic (accumulation), Conditional
2. ✅ **Hybrid Elements**: 12 unique hybrid combinations with stat bonuses and visual keywords
3. ✅ **Mutation System**: 4 mutation types (Stat Boost, New Trait, Trait Power, Hidden Unlock)
4. ✅ **Stat Constraints**: Floors, soft ceilings, hard caps, generation scaling with diminishing returns
5. ✅ **Materials Support**: 4 special breeding materials that modify outcomes
6. ✅ **Full Logging**: Glass Box admin visibility with structured JSON logs
7. ✅ **Cost Calculation**: Economy system based on parent rarity
8. ✅ **Time Requirements**: Gestation and maturation scaling with generation/power
