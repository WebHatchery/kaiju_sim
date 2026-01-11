# TRAIT SYSTEM DESIGN DOCUMENT

**Kaiju Breeding Simulator - Trait System v1.0**

**Date**: 2026-01-09
**Status**: Design Complete - Ready for Implementation

---

## 1. DESIGN PHILOSOPHY

The trait system is the genetic heart of the Kaiju Breeding Simulator. It must support:

- **Predictable-but-probabilistic breeding** (45% visible, 25% hidden inheritance)
- **Information asymmetry** (hidden traits create strategic depth)
- **Power creep without runaway scaling** (specialization over raw power)
- **Environmental interactions** (traits perform differently in contexts)
- **Legacy and uniqueness** (every kaiju combination is distinct)

**Core Principle**: Traits are not just stat modifiers—they are conditional, interactive systems that create emergent strategic depth.

---

## 2. TRAIT CATEGORIES

### 2.1 Element Traits

**Purpose**: Elemental affinities that directly modify combat damage and environmental interactions.

**Characteristics**:
- Always **visible** after first combat use
- Power range: 5-15
- Strong environmental dependencies
- Visual implications required (e.g., "Electric Breath" requires lightning visual)

**Combat Behavior**:
- Add flat damage bonus: `damage += trait.power`
- Multiplicative bonuses in favorable environments: `damage *= (1.0 + env_bonus)`
- Elemental counters possible (e.g., Water vs Fire)

---

### 2.2 Modifier Traits

**Purpose**: Stat amplifiers and conditional bonuses that alter kaiju performance.

**Characteristics**:
- Can be **visible or hidden**
- Power range: -10 to +20 (negative traits exist)
- Apply to specific stats or situations
- May have trigger conditions

**Combat Behavior**:
- Percentage-based stat modifications
- Conditional activation (e.g., "when HP < 30%")
- Synergy potential with Element traits

---

### 2.3 Mutation Traits

**Purpose**: Random genetic anomalies from breeding or environmental exposure.

**Characteristics**:
- Always **hidden** initially
- Power range: -5 to +10 (highly variable)
- 10% chance per breeding
- Unpredictable effects
- May become visible after multiple battles

**Combat Behavior**:
- Unstable and context-dependent
- Can provide surprising advantages or weaknesses
- May interact with parent traits unexpectedly

---

### 2.4 Synergy Traits

**Purpose**: Meta-traits that activate when specific trait combinations are present.

**Characteristics**:
- Always **hidden** until conditions met
- Power range: 10-25 (powerful but rare)
- Require 2-3 specific traits to activate
- Extremely difficult to breed intentionally

**Combat Behavior**:
- Multiplicative bonuses when conditions met
- Create "archetype" builds (e.g., Storm Dragon, Volcanic Titan)
- Only revealed through experimentation or research

---

## 3. INHERITANCE TYPES

### 3.1 Dominant (40% of traits)

**Inheritance Probability**:
- If **either parent** has trait: **60%** chance to pass
- If **both parents** have trait: **85%** chance to pass

**Formula**:
```rust
if parent_a.has_trait || parent_b.has_trait {
    let chance = if parent_a.has_trait && parent_b.has_trait { 0.85 } else { 0.60 };
    if rng.gen::<f32>() < chance {
        offspring.add_trait(trait)
    }
}
```

**Examples**: Electric Breath, Armored Hide, Wings

---

### 3.2 Recessive (30% of traits)

**Inheritance Probability**:
- If **one parent** has trait: **15%** chance to pass
- If **both parents** have trait: **70%** chance to pass

**Formula**:
```rust
if parent_a.has_trait && parent_b.has_trait {
    if rng.gen::<f32>() < 0.70 { offspring.add_trait(trait) }
} else if parent_a.has_trait || parent_b.has_trait {
    if rng.gen::<f32>() < 0.15 { offspring.add_trait(trait) }
}
```

**Examples**: Regeneration, Camouflage, Bioluminescence

---

### 3.3 Polygenic (20% of traits)

**Inheritance Probability**:
- Requires **accumulation** from multiple ancestors
- Each parent contributes **inheritance points**
- Threshold required for expression: **4 points**

**Formula**:
```rust
let mut points = 0;
if parent_a.has_trait_visible { points += 2 }
else if parent_a.has_trait_hidden { points += 1 }

if parent_b.has_trait_visible { points += 2 }
else if parent_b.has_trait_hidden { points += 1 }

if points >= 4 {
    offspring.add_trait_visible(trait)
} else if points >= 2 {
    offspring.add_trait_hidden(trait) // latent gene
}
```

**Examples**: Gigantism, Enhanced Intelligence, Rapid Metabolism

---

### 3.4 Conditional (10% of traits)

**Inheritance Probability**:
- Depends on **parent stats, generation, or environmental factors**
- Formula varies per trait

**Example Formulas**:
```rust
// "Ancient Bloodline" - requires high generation
if (parent_a.generation + parent_b.generation) >= 10 {
    if rng.gen::<f32>() < 0.40 { offspring.add_trait(trait) }
}

// "Adaptive Scales" - requires both parents defensive
if parent_a.defense > 40 && parent_b.defense > 40 {
    if rng.gen::<f32>() < 0.50 { offspring.add_trait(trait) }
}
```

**Examples**: Ancient Bloodline, Adaptive Scales, Environmental Resistance

---

## 4. COMPREHENSIVE TRAIT LIST (50 Traits)

### ELEMENT TRAITS (15 traits)

| ID | Name | Category | Power | Inheritance | Combat Effect | Conditions | Visual |
|----|------|----------|-------|-------------|---------------|------------|--------|
| E01 | Electric Breath | Element | 10 | Dominant | +10 damage, +15% in Storm | Always active | Lightning patterns |
| E02 | Flame Core | Element | 12 | Dominant | +12 damage, +20% in Volcanic | Always active | Molten glow |
| E03 | Aqua Hide | Element | 8 | Dominant | +8 damage, +15% in Ocean | Always active | Water shimmer |
| E04 | Frost Aura | Element | 9 | Dominant | +9 damage, slow enemy 5% | Always active | Ice crystals |
| E05 | Toxic Breath | Element | 11 | Recessive | +11 damage, DoT 3/turn | Lasts 3 turns | Poison drip |
| E06 | Radiation Pulse | Element | 13 | Recessive | +13 damage, +25% in Wasteland | Always active | Green glow |
| E07 | Gravity Manipulation | Element | 14 | Conditional (Gen 5+) | +14 damage, reduce enemy speed 10% | Always active | Distortion field |
| E08 | Solar Flare | Element | 10 | Dominant | +10 damage, +20% in Desert | Daylight only | Golden rays |
| E09 | Lunar Pulse | Element | 10 | Dominant | +10 damage, +20% in Night | Night only | Silver shimmer |
| E10 | Earthquake Strike | Element | 15 | Recessive | +15 damage, once per battle | Turn 3+ | Ground cracks |
| E11 | Wind Slash | Element | 7 | Dominant | +7 damage, +10% vs Flying | Always active | Air currents |
| E12 | Shadow Meld | Element | 6 | Recessive | +6 damage, +15% in Darkness | Darkness only | Dark wisps |
| E13 | Crystal Spikes | Element | 11 | Recessive | +11 damage, counter 20% damage | When hit | Crystal protrusions |
| E14 | Sonic Scream | Element | 9 | Dominant | +9 damage, stun 10% chance | Always active | Sound waves |
| E15 | Bio-Plasma | Element | 12 | Polygenic | +12 damage, heal 5 HP on hit | Always active | Purple energy |

---

### MODIFIER TRAITS (15 traits)

| ID | Name | Category | Power | Inheritance | Combat Effect | Conditions | Visual |
|----|------|----------|-------|-------------|---------------|------------|--------|
| M01 | Armored Hide | Modifier | 15 | Dominant | +15 defense | Always active | Thick plates |
| M02 | Regeneration | Modifier | 10 | Recessive | +5 HP per turn | HP < 50% | Healing aura |
| M03 | Berserker Rage | Modifier | 18 | Recessive | +18 attack, -10 defense | HP < 30% | Red glow |
| M04 | Speed Boost | Modifier | 12 | Dominant | +12 speed | Always active | Streamlined form |
| M05 | Energy Drain | Modifier | 8 | Recessive | Steal 5 HP per hit | Always active | Draining tendrils |
| M06 | Endurance | Modifier | 20 | Polygenic | +20% max HP | Always active | Robust build |
| M07 | Precision Strike | Modifier | 10 | Dominant | +10% crit chance (x1.5 damage) | Always active | Focused eyes |
| M08 | Defensive Stance | Modifier | 15 | Recessive | +15 defense, -5 attack | HP > 70% | Defensive posture |
| M09 | Bloodlust | Modifier | 12 | Conditional (Gen 3+) | +12 attack after kill | After enemy defeat | Battle scars |
| M10 | Camouflage | Modifier | 8 | Recessive | Dodge first attack | Turn 1 only | Adaptive skin |
| M11 | Iron Will | Modifier | 10 | Polygenic | Immune to stun/slow | Always active | Determined expression |
| M12 | Volatile Blood | Modifier | -5 | Mutation | Take 5 extra damage | When hit | Unstable appearance |
| M13 | Giant Slayer | Modifier | 15 | Conditional (Def > 40) | +15 damage vs high HP | Enemy HP > 300 | Hunting stance |
| M14 | Last Stand | Modifier | 20 | Recessive | +20 defense at 1-50 HP | Critical HP | Desperate glow |
| M15 | Adaptability | Modifier | 12 | Polygenic | +12% all stats in 3rd battle | Battle count 3+ | Evolving features |

---

### MUTATION TRAITS (10 traits)

| ID | Name | Category | Power | Inheritance | Combat Effect | Conditions | Visual |
|----|------|----------|-------|-------------|---------------|------------|--------|
| MU01 | Unstable Mutation | Mutation | 5 | Random (10%) | +5 random stat | Always active | Chaotic patterns |
| MU02 | Extra Limb | Mutation | 8 | Random (10%) | +8 attack, -3 speed | Always active | Additional appendage |
| MU03 | Hardened Carapace | Mutation | 10 | Random (10%) | +10 defense, -5 speed | Always active | Thick shell |
| MU04 | Bioluminescence | Mutation | 3 | Random (10%) | +3 damage in darkness | Darkness only | Glowing marks |
| MU05 | Overcharged Cells | Mutation | 12 | Random (10%) | +12 attack, -10 HP per turn | Always active | Crackling energy |
| MU06 | Fragile Frame | Mutation | -8 | Random (10%) | -8 defense | Always active | Thin structure |
| MU07 | Rapid Growth | Mutation | 15 | Random (10%) | +15% XP gain | Post-battle | Oversized features |
| MU08 | Parasitic Spores | Mutation | 7 | Random (10%) | Drain 3 HP/turn from enemy | Always active | Fungal growth |
| MU09 | Thermal Vision | Mutation | 6 | Random (10%) | +6 attack in heat | Volcanic/Desert | Heat-sensing organs |
| MU10 | Echo Location | Mutation | 5 | Random (10%) | +5 speed in darkness | Darkness only | Ear/sensor organs |

---

### SYNERGY TRAITS (10 traits)

| ID | Name | Category | Power | Inheritance | Combat Effect | Conditions | Visual |
|----|------|----------|-------|-------------|---------------|------------|--------|
| S01 | Storm Dragon | Synergy | 20 | Hidden | +20 damage in Storm | Electric + Wings | Lightning wings |
| S02 | Volcanic Titan | Synergy | 25 | Hidden | +25% all stats in Volcanic | Flame + Armored | Lava-plated |
| S03 | Deep Sea Horror | Synergy | 18 | Hidden | +18 defense in Ocean | Aqua + Camouflage | Abyssal form |
| S04 | Frozen Colossus | Synergy | 22 | Hidden | +22 attack, slow enemy 15% | Frost + Endurance | Ice giant |
| S05 | Toxic Regenerator | Synergy | 15 | Hidden | Heal 10 HP/turn, poison enemy | Toxic + Regeneration | Oozing heal |
| S06 | Radioactive Berserker | Synergy | 30 | Hidden | +30 attack when HP < 40% | Radiation + Berserker | Glowing rage |
| S07 | Gravity Well | Synergy | 25 | Hidden | Reduce enemy speed 20% | Gravity + Iron Will | Crushing aura |
| S08 | Solar Eclipse | Synergy | 28 | Hidden | +28 damage, alternates day/night bonuses | Solar + Lunar | Celestial form |
| S09 | Earthquake Armor | Synergy | 20 | Hidden | +20 defense, counter 30% damage | Earthquake + Armored | Stone plating |
| S10 | Crystal Wind | Synergy | 23 | Hidden | +23 damage, heal 8 HP on hit | Crystal + Wind | Prismatic currents |

---

## 5. INHERITANCE PROBABILITY FORMULAS

### Base Inheritance Rates

**Visible Traits (from existing visible traits)**:
```rust
const VISIBLE_INHERIT_RATE: f32 = 0.45;

for trait in parent_a.visible_traits.iter().chain(parent_b.visible_traits.iter()) {
    if rng.gen::<f32>() < VISIBLE_INHERIT_RATE {
        offspring.add_visible_trait(trait.clone());
    }
}
```

**Hidden Traits (from existing hidden traits)**:
```rust
const HIDDEN_INHERIT_RATE: f32 = 0.25;

for trait in parent_a.hidden_traits.iter().chain(parent_b.hidden_traits.iter()) {
    if rng.gen::<f32>() < HIDDEN_INHERIT_RATE {
        offspring.add_hidden_trait(trait.clone());
    }
}
```

### Inheritance Type Modifiers

**Dominant Traits**: Apply multiplier
```rust
if trait.inheritance == Inheritance::Dominant {
    inherit_chance *= 1.33; // 45% → 60%
}
```

**Recessive Traits**: Require both parents
```rust
if trait.inheritance == Inheritance::Recessive {
    if both_parents_have_trait {
        inherit_chance = 0.70;
    } else {
        inherit_chance = 0.15;
    }
}
```

**Polygenic Traits**: Accumulation system
```rust
if trait.inheritance == Inheritance::Polygenic {
    let points = calculate_polygenic_points(parent_a, parent_b, trait);
    if points >= 4 { offspring.add_visible_trait(trait); }
    else if points >= 2 { offspring.add_hidden_trait(trait); }
}
```

**Conditional Traits**: Custom logic per trait
```rust
if trait.inheritance == Inheritance::Conditional {
    if evaluate_condition(trait, parent_a, parent_b, offspring) {
        if rng.gen::<f32>() < trait.conditional_chance {
            offspring.add_trait(trait);
        }
    }
}
```

---

## 6. MUTATION SYSTEM

### 6.1 Mutation Trigger Conditions

**Base Mutation Rate**: 10% per breeding

**Mutation Chance Modifiers**:
```rust
let mut mutation_chance = 0.10;

// Generation penalty (later gens more volatile)
mutation_chance += (offspring.generation as f32) * 0.005;

// Parent trait count (more traits = more instability)
let parent_trait_count = parent_a.all_traits().len() + parent_b.all_traits().len();
if parent_trait_count > 10 {
    mutation_chance += 0.05;
}

// Cap at 25%
mutation_chance = mutation_chance.min(0.25);
```

### 6.2 Mutation Effects

**When Mutation Occurs**:
1. Roll for mutation type (weighted):
   - 40%: Beneficial mutation (+stat)
   - 30%: Neutral mutation (visual only or minor effect)
   - 30%: Negative mutation (-stat or drawback)

2. Mutation power determined by generation:
```rust
let power = rng.gen_range(1..=3) + (offspring.generation / 2);
```

3. Always added as **hidden trait** initially

### 6.3 Mutation Revelation

Mutations become visible when:
- Kaiju participates in **3+ battles** (30% reveal chance per battle)
- Research facility reaches **Level 2+** (can scan for mutations)
- Kaiju dies (autopsy reveals all hidden traits)

---

## 7. HIDDEN VS VISIBLE RULES

### 7.1 Starting Visibility

**Always Visible**:
- Element traits (after first use)
- Physical modifier traits (Armored Hide, Wings, Extra Limb)
- Dominant traits that have obvious visual implications

**Always Hidden Initially**:
- Synergy traits (until conditions trigger)
- Mutation traits (until revealed through battles)
- Recessive traits with subtle effects
- Conditional traits (until condition met)

### 7.2 Revelation Mechanics

**Combat Revelation** (automatic):
```rust
if trait.is_hidden && trait.was_used_in_combat() {
    trait.reveal_chance += 0.25; // 25% per combat use
    if rng.gen::<f32>() < trait.reveal_chance {
        trait.is_hidden = false;
    }
}
```

**Research Revelation** (player-initiated):
```rust
fn research_trait(kaiju: &Kaiju, lab_level: u32) -> Option<Trait> {
    let reveal_chance = 0.10 * lab_level as f32;
    for trait in kaiju.hidden_traits.iter_mut() {
        if rng.gen::<f32>() < reveal_chance {
            trait.is_hidden = false;
            return Some(trait.clone());
        }
    }
    None
}
```

**Death Revelation** (guaranteed):
```rust
fn on_kaiju_death(kaiju: &mut Kaiju) {
    for trait in kaiju.hidden_traits.iter_mut() {
        trait.is_hidden = false;
    }
    publish_autopsy_report(kaiju); // Makes info public
}
```

### 7.3 Trait Slot Limits

To prevent trait bloat:
- **Maximum visible traits**: 8
- **Maximum hidden traits**: 6
- **Total trait cap**: 12 (excess traits removed randomly during breeding)

---

## 8. SYNERGY SYSTEM

### 8.1 Synergy Detection

Synergies activate when **all required traits are present**:

```rust
struct SynergyDefinition {
    id: String,
    name: String,
    required_traits: Vec<String>,
    power: i32,
    effect: SynergyEffect,
}

fn check_synergies(kaiju: &Kaiju) -> Vec<Trait> {
    let mut active_synergies = Vec::new();

    for synergy_def in SYNERGY_DEFINITIONS.iter() {
        if synergy_def.required_traits.iter().all(|req|
            kaiju.has_trait(req)
        ) {
            active_synergies.push(synergy_def.to_trait());
        }
    }

    active_synergies
}
```

### 8.2 Synergy Requirements

| Synergy | Requires | Power | Effect |
|---------|----------|-------|--------|
| Storm Dragon | Electric Breath + Wings (any) | 20 | +20 damage in Storm |
| Volcanic Titan | Flame Core + Armored Hide | 25 | +25% all stats in Volcanic |
| Deep Sea Horror | Aqua Hide + Camouflage | 18 | +18 defense in Ocean |
| Frozen Colossus | Frost Aura + Endurance | 22 | +22 attack, slow enemy 15% |
| Toxic Regenerator | Toxic Breath + Regeneration | 15 | Heal 10 HP/turn, poison enemy |
| Radioactive Berserker | Radiation Pulse + Berserker Rage | 30 | +30 attack when HP < 40% |
| Gravity Well | Gravity Manipulation + Iron Will | 25 | Reduce enemy speed 20% |
| Solar Eclipse | Solar Flare + Lunar Pulse | 28 | +28 damage, dual time benefits |
| Earthquake Armor | Earthquake Strike + Armored Hide | 20 | +20 defense, counter 30% |
| Crystal Wind | Crystal Spikes + Wind Slash | 23 | +23 damage, heal 8 HP on hit |

### 8.3 Synergy Inheritance

Synergies **do not inherit directly**—offspring must inherit the component traits:

```rust
// Synergies are recalculated after breeding
fn finalize_offspring(mut offspring: Kaiju) -> Kaiju {
    offspring.synergy_traits = check_synergies(&offspring);
    offspring
}
```

### 8.4 Hidden Synergy Discovery

Synergies remain hidden until:
1. **First activation in combat** (automatic reveal)
2. **Research facility Level 3+** (can detect potential synergies)
3. **Community knowledge** (other players discover and publish)

---

## 9. BALANCE CONSTRAINTS

### 9.1 Power Budget System

Each kaiju has a **power budget** to prevent overpowered combinations:

```rust
const MAX_POWER_BUDGET: i32 = 150;

fn calculate_power_budget(kaiju: &Kaiju) -> i32 {
    let trait_power: i32 = kaiju.all_traits().iter()
        .map(|t| t.power.max(0)) // Negative traits don't reduce budget
        .sum();

    let stat_power: i32 = (kaiju.stats.attack + kaiju.stats.defense + kaiju.stats.speed) / 10;

    trait_power + stat_power
}

fn enforce_power_budget(kaiju: &mut Kaiju) {
    while calculate_power_budget(kaiju) > MAX_POWER_BUDGET {
        // Remove lowest power trait
        if let Some(weakest) = kaiju.all_traits_mut()
            .iter_mut()
            .min_by_key(|t| t.power) {
            kaiju.remove_trait(weakest.id);
        }
    }
}
```

### 9.2 Trait Compatibility Matrix

Some traits are **mutually exclusive**:

```rust
const INCOMPATIBLE_TRAITS: &[(&str, &str)] = &[
    ("Solar Flare", "Lunar Pulse"),      // Can coexist (synergy)
    ("Berserker Rage", "Defensive Stance"), // Mutually exclusive
    ("Camouflage", "Bioluminescence"),   // Mutually exclusive
    ("Speed Boost", "Hardened Carapace"), // Mutually exclusive
];

fn validate_trait_compatibility(kaiju: &Kaiju, new_trait: &Trait) -> bool {
    for (trait_a, trait_b) in INCOMPATIBLE_TRAITS.iter() {
        if kaiju.has_trait(trait_a) && new_trait.name == *trait_b {
            return false;
        }
        if kaiju.has_trait(trait_b) && new_trait.name == *trait_a {
            return false;
        }
    }
    true
}
```

### 9.3 Generation Scaling

Later generations have **diminishing returns** on power creep:

```rust
fn apply_generation_scaling(offspring: &mut Kaiju) {
    let gen = offspring.generation as f32;

    // Power creep formula: base * (1 + gen * 0.01) * diminishing_returns
    let diminishing = 1.0 / (1.0 + (gen / 20.0));
    let creep_multiplier = (1.0 + gen * 0.01) * diminishing;

    offspring.stats.hp = (offspring.stats.hp as f32 * creep_multiplier) as i32;
    offspring.stats.attack = (offspring.stats.attack as f32 * creep_multiplier) as i32;
    offspring.stats.defense = (offspring.stats.defense as f32 * creep_multiplier) as i32;
}
```

At Generation 1: 1.0x stats
At Generation 5: 1.047x stats
At Generation 10: 1.083x stats
At Generation 20: 1.133x stats
At Generation 50: 1.217x stats

### 9.4 Negative Trait Balance

Negative traits provide **compensation**:

```rust
fn apply_negative_trait_compensation(kaiju: &mut Kaiju, negative_trait: &Trait) {
    let compensation = negative_trait.power.abs() * 2;

    // Boost random stat
    match rng.gen_range(0..3) {
        0 => kaiju.stats.attack += compensation,
        1 => kaiju.stats.defense += compensation,
        2 => kaiju.stats.speed += compensation,
        _ => unreachable!(),
    }
}
```

---

## 10. COMBAT INTEGRATION

### 10.1 Trait Application Order

Combat damage calculation with traits:

```rust
fn calculate_damage(attacker: &Kaiju, defender: &Kaiju, env: &Environment, turn: u32) -> i32 {
    // 1. Base damage
    let mut damage = attacker.stats.attack - (defender.stats.defense as f32 * 0.5) as i32;
    damage = damage.max(5);

    // 2. Apply Element traits
    for trait in attacker.visible_traits.iter().filter(|t| t.category == TraitCategory::Element) {
        if trait.condition_met(attacker, defender, env, turn) {
            damage += trait.power;
        }
    }

    // 3. Apply Modifier traits
    let mut damage_multiplier = 1.0;
    for trait in attacker.visible_traits.iter().filter(|t| t.category == TraitCategory::Modifier) {
        if trait.condition_met(attacker, defender, env, turn) {
            damage_multiplier += trait.power as f32 * 0.01;
        }
    }
    damage = (damage as f32 * damage_multiplier) as i32;

    // 4. Apply Synergy traits
    for synergy in attacker.synergy_traits.iter() {
        if synergy.condition_met(attacker, defender, env, turn) {
            damage += synergy.power;
        }
    }

    // 5. Apply environmental multipliers
    let env_mult = calculate_environment_multiplier(attacker, env);
    damage = (damage as f32 * env_mult) as i32;

    // 6. Apply variance (±5%)
    let variance = rng.gen_range(0.95..=1.05);
    damage = (damage as f32 * variance) as i32;

    // 7. Apply defender's defensive traits
    for trait in defender.visible_traits.iter() {
        damage = trait.modify_incoming_damage(damage, attacker, defender, env, turn);
    }

    damage.max(1) // Minimum 1 damage
}
```

### 10.2 Environmental Multipliers

```rust
fn calculate_environment_multiplier(kaiju: &Kaiju, env: &Environment) -> f32 {
    let mut multiplier = 1.0;

    for trait in kaiju.all_traits().iter() {
        match (trait.name.as_str(), env) {
            ("Electric Breath", Environment::Storm) => multiplier += 0.15,
            ("Flame Core", Environment::Volcanic) => multiplier += 0.20,
            ("Aqua Hide", Environment::Ocean) => multiplier += 0.15,
            ("Frost Aura", Environment::Arctic) => multiplier += 0.18,
            ("Radiation Pulse", Environment::Wasteland) => multiplier += 0.25,
            ("Solar Flare", Environment::Desert) => multiplier += 0.20,
            ("Shadow Meld", Environment::Cave) => multiplier += 0.15,
            _ => {}
        }
    }

    multiplier
}
```

### 10.3 Conditional Trait Activation

```rust
impl Trait {
    fn condition_met(&self, attacker: &Kaiju, defender: &Kaiju, env: &Environment, turn: u32) -> bool {
        match self.condition {
            Some(Condition::Always) => true,
            Some(Condition::HpBelow(threshold)) => attacker.current_hp < (attacker.max_hp * threshold / 100),
            Some(Condition::HpAbove(threshold)) => attacker.current_hp > (attacker.max_hp * threshold / 100),
            Some(Condition::TurnMin(min_turn)) => turn >= min_turn,
            Some(Condition::Environment(ref env_type)) => env == env_type,
            Some(Condition::EnemyHpAbove(threshold)) => defender.current_hp > threshold,
            Some(Condition::AfterKill) => attacker.kills_this_battle > 0,
            Some(Condition::FirstTurn) => turn == 1,
            None => true,
        }
    }
}
```

---

## 11. DATA STRUCTURE IMPLEMENTATION

### 11.1 Trait Definition (Rust)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub id: String,
    pub name: String,
    pub category: TraitCategory,
    pub power: i32,
    pub inheritance: InheritanceType,
    pub condition: Option<Condition>,
    pub is_hidden: bool,
    pub reveal_chance: f32,
    pub visual_keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitCategory {
    Element,
    Modifier,
    Mutation,
    Synergy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InheritanceType {
    Dominant,
    Recessive,
    Polygenic,
    Conditional { condition: String, chance: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    Always,
    HpBelow(i32),       // HP below percentage
    HpAbove(i32),       // HP above percentage
    TurnMin(u32),       // Minimum turn number
    Environment(String), // Specific environment
    EnemyHpAbove(i32),  // Enemy HP threshold
    AfterKill,          // After defeating an enemy
    FirstTurn,          // Only on turn 1
}
```

### 11.2 JSON Data File (`assets/traits.json`)

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
      "visual_keywords": ["lightning", "electricity", "blue glow", "crackling energy"]
    },
    {
      "id": "S01",
      "name": "Storm Dragon",
      "category": "Synergy",
      "power": 20,
      "inheritance": { "Conditional": { "condition": "has_traits:E01,M04", "chance": 1.0 } },
      "condition": { "Environment": "Storm" },
      "is_hidden": true,
      "required_traits": ["E01", "M04"],
      "visual_keywords": ["lightning wings", "storm aura", "electric flight"]
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

---

## 12. IMPLEMENTATION PRIORITY

### Phase 1: Core Trait System
1. Implement `Trait` struct and `TraitCategory` enum
2. Load traits from JSON
3. Basic inheritance (45% visible, 25% hidden)
4. Trait application in combat (flat damage bonuses)

### Phase 2: Inheritance Types
1. Dominant/Recessive logic
2. Polygenic accumulation system
3. Conditional trait evaluation

### Phase 3: Mutations
1. 10% mutation chance per breeding
2. Mutation power calculation
3. Hidden → Visible revelation system

### Phase 4: Synergies
1. Synergy detection algorithm
2. Synergy trait auto-generation
3. Synergy-specific combat effects

### Phase 5: Balance & Polish
1. Power budget enforcement
2. Trait compatibility validation
3. Environmental multipliers
4. Negative trait compensation

---

## 13. TESTING STRATEGY

### Unit Tests
- Trait inheritance probability (1000 breed cycles)
- Mutation rate verification
- Synergy detection accuracy
- Power budget enforcement
- Trait compatibility validation

### Integration Tests
- Full breeding cycle with trait inheritance
- Combat damage with multiple traits
- Environmental effect interactions
- Hidden trait revelation over multiple battles

### Balance Tests
- Generation 1 vs Generation 10 power comparison
- Synergy trait power vs individual traits
- Negative trait compensation fairness
- Maximum possible damage output

---

### Critical Files for Implementation

Based on this design, here are the 5 most critical files for implementing the trait system:

1. **/h/RustGames/kaiju_sim/src/data/traits.rs** - Core trait data structures, enums, and trait definitions. This is the foundation of the entire system.

2. **/h/RustGames/kaiju_sim/src/engine/breeding.rs** - Breeding mechanics that handle trait inheritance, mutation generation, and offspring trait assignment using the inheritance formulas defined in this document.

3. **/h/RustGames/kaiju_sim/src/engine/combat.rs** - Combat damage calculation that applies trait bonuses, environmental multipliers, and conditional activations in the correct order.

4. **/h/RustGames/kaiju_sim/assets/traits.json** - JSON data file containing all 50 trait definitions, synergy requirements, and incompatibility rules. This is the authoritative source for all trait data.

5. **/h/RustGames/kaiju_sim/src/engine/research.rs** - Research system that handles hidden trait revelation, synergy discovery, and trait analysis. This implements the progressive knowledge system central to the game's information asymmetry design.

These five files form the complete trait system pipeline: data definition → breeding inheritance → combat application → knowledge discovery.
