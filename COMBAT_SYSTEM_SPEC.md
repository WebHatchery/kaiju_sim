# COMBAT SYSTEM SPECIFICATION

## Complete Combat System Specification for Kaiju Breeding Simulator

**Version**: 1.0
**Date**: 2026-01-09
**Status**: Design Specification

---

## 1. Damage Formula

### 1.1 Base Damage Calculation

```rust
fn calculate_damage(attacker: &Kaiju, defender: &Kaiju, env: &Environment) -> i32 {
    // Step 1: Calculate base damage
    let base = attacker.stats.attack - (defender.stats.defense * 0.5);

    // Step 2: Apply minimum damage floor
    let base = base.max(5.0);

    // Step 3: Calculate trait modifiers
    let trait_bonus = sum_element_trait_powers(attacker);

    // Step 4: Apply environment multiplier
    let env_multiplier = calculate_environment_multiplier(attacker, env);

    // Step 5: Apply randomness (±5%)
    let variance = rand_range(0.95, 1.05);

    // Step 6: Calculate final damage
    let final_damage = (base + trait_bonus) * env_multiplier * variance;

    final_damage as i32
}
```

### 1.2 Formula Components

#### Defense Scaling
- Defense reduces incoming damage at 50% effectiveness
- Formula: `attacker_attack - (defender_defense * 0.5)`
- This ensures defense is valuable but not overwhelming

#### Minimum Damage Floor
- **Minimum damage per hit**: 5 HP
- Purpose: Prevents battles from stalling indefinitely
- Applies after defense calculation but before trait modifiers

#### Trait Bonus
- Element traits add their power value directly to base damage
- Example: "Electric Breath" (power: 8) adds +8 damage
- Multiple element traits stack additively

#### Environment Multiplier
- Applied to `(base + trait_bonus)` product
- Range: 0.85x to 1.25x depending on trait-environment synergy
- See Section 3 for detailed multipliers

#### Randomness Variance
- Range: 0.95 to 1.05 (±5%)
- Applied as final multiplier
- Ensures predictability while allowing minor upsets
- With full knowledge, outcomes are 90%+ predictable

### 1.3 Calculation Order

**Critical**: The order of operations matters:

1. Base attack - (defense × 0.5)
2. Apply minimum floor (5)
3. Add trait power bonuses
4. Multiply by environment modifier
5. Multiply by randomness (0.95-1.05)
6. Round to integer

---

## 2. Trait Modifiers

### 2.1 Trait Categories

```rust
pub enum TraitCategory {
    Element,      // Direct combat power (fire, water, electric, etc.)
    Modifier,     // Conditional effects (speed boosts, defense buffs)
    Mutation,     // Unstable bonuses/penalties
    Synergy,      // Multi-trait interactions
}
```

### 2.2 Element Traits

Element traits provide direct combat bonuses and environmental synergies.

**Standard Element Traits:**

| Trait Name | Power | Description |
|------------|-------|-------------|
| Electric Breath | 8 | Lightning-based attacks |
| Aqua Hide | 6 | Water affinity defense |
| Flame Roar | 9 | Fire-based offense |
| Frost Aura | 7 | Ice-based attacks |
| Terra Slam | 8 | Earth/rock attacks |
| Wind Strike | 7 | Air-based speed attacks |
| Toxic Spines | 8 | Poison damage over time |
| Radiant Beam | 10 | Energy-based attacks |

**Combat Application:**
- Element trait power is added directly to base damage
- Multiple element traits stack additively
- Example: Electric Breath (8) + Frost Aura (7) = +15 damage bonus
- Environment multipliers apply to the total damage (see Section 3)

### 2.3 Modifier Traits

Modifier traits provide conditional combat effects.

**Types of Modifiers:**

#### Speed Modifiers
- **Swift**: +10% to speed stat (affects turn order)
- **Heavy**: -5% to speed, +10% to defense
- **Agile Reflexes**: +15% speed in favorable environments

#### Defense Modifiers
- **Armored Scales**: +8 defense
- **Regeneration**: Heal 3% max HP per turn
- **Damage Reduction**: Reduce incoming damage by 5%

#### Attack Modifiers
- **Berserker**: +15% damage when below 50% HP
- **First Strike**: +20% damage on opening turn
- **Combo Fighter**: +5% damage per consecutive hit (max 3 stacks)

**Implementation Note**:
- Modifiers calculate after base damage but before environment
- Percentage modifiers are multiplicative with final damage
- Flat modifiers apply to stats before combat begins

### 2.4 Mutation Traits

Mutations are unpredictable traits with high risk/reward.

**Characteristics:**
- 10% chance to appear during breeding
- Power range: 1-3 for hidden mutations, up to 5 for visible
- May have positive or negative effects
- May trigger conditionally

**Example Mutations:**

| Mutation | Effect | Power |
|----------|--------|-------|
| Unstable Mutation | ±5% random damage variance (total ±10%) | 2 |
| Volatile Core | +15 damage, -10 defense | 3 |
| Adaptive Scales | Gain +2 defense per turn (max 10 stacks) | 3 |
| Chaotic Energy | Random element bonus each turn (5-12) | 4 |
| Fragile Power | +25% damage, -20% HP | 5 |

### 2.5 Synergy Traits (Future Expansion)

Synergies activate when multiple compatible traits are present.

**Planned Synergies:**
- **Storm Fury**: Electric Breath + Wind Strike = +25% damage in storms
- **Volcanic Might**: Flame Roar + Terra Slam = +20% in volcanic areas
- **Glacial Fortress**: Frost Aura + Armored Scales = +15% defense reduction
- **Toxic Tempest**: Toxic Spines + Wind Strike = Spread poison to environment

**Note**: Synergies are not implemented in v1.0 but the data structure supports them.

---

## 3. Environment System

### 3.1 Environment List

Tournaments and battles take place in one of the following environments:

| Environment | Description | Common Traits Affected |
|-------------|-------------|------------------------|
| **Neutral** | Standard arena | None (baseline) |
| **Storm** | Lightning and heavy rain | Electric, Wind |
| **Volcanic** | Lava flows and heat | Fire, Earth |
| **Aquatic** | Deep water arena | Water, Ice (penalty) |
| **Tundra** | Frozen wasteland | Ice, Fire (penalty) |
| **Desert** | Scorching heat | Fire, Water (penalty) |
| **Forest** | Dense vegetation | Earth, Toxic, Wind |
| **Radiation** | Contaminated zone | Mutation traits, Energy |
| **Void** | Dimensional rift | Energy, unpredictable |

### 3.2 Environment Multipliers

Environment multipliers apply to final damage calculation (after traits are added).

#### Storm Environment
```rust
if environment == "storm" {
    for trait in attacker.traits {
        if trait.name.contains("Electric") || trait.name.contains("Wind") {
            multiplier *= 1.15;  // +15% bonus
        }
        if trait.name.contains("Fire") {
            multiplier *= 0.90;  // -10% penalty
        }
    }
}
```

**Full Storm Multiplier Table:**
- Electric traits: **1.15x** (boosted)
- Wind traits: **1.15x** (boosted)
- Fire traits: **0.90x** (suppressed)
- Water traits: **1.05x** (slight boost)
- All others: **1.0x** (neutral)

#### Volcanic Environment
- Fire traits: **1.25x** (major boost)
- Earth traits: **1.10x** (moderate boost)
- Ice/Frost traits: **0.85x** (suppressed)
- Water traits: **0.90x** (penalty)

#### Aquatic Environment
- Water traits: **1.20x** (major boost)
- Electric traits: **1.10x** (conducts well)
- Ice traits: **0.90x** (water resists freezing)
- Fire traits: **0.80x** (major penalty)

#### Tundra Environment
- Ice traits: **1.20x** (major boost)
- Fire traits: **0.85x** (suppressed)
- Water traits: **1.05x** (slight boost)

#### Desert Environment
- Fire traits: **1.15x** (boosted by heat)
- Earth traits: **1.10x** (sand/rock synergy)
- Water traits: **0.85x** (evaporates)
- Ice traits: **0.80x** (melts)

#### Forest Environment
- Earth traits: **1.15x** (natural terrain)
- Toxic traits: **1.20x** (spreads through vegetation)
- Wind traits: **1.10x** (trees channel wind)
- Fire traits: **0.95x** (risk of backfire)

#### Radiation Environment
- Energy/Radiant traits: **1.25x** (major boost)
- Mutation traits: **1.20x** (unstable energy)
- All others: **1.0x** (neutral)
- Special: 5% chance of temporary mutation per turn

#### Void Environment
- Energy traits: **1.15x** (dimensional power)
- All others: **random(0.85, 1.15)** per turn
- Unpredictable chaos factor

### 3.3 Environment Selection

**Tournament Types:**

1. **Neutral Tournaments**: Always use Neutral environment (fairness)
2. **Specialized Tournaments**: Environment specified (e.g., "Storm Championship")
3. **Wild Tournaments**: Random environment from full list
4. **Lethal Tournaments**: Environment chosen by tournament organizer

**Selection Method:**
```rust
pub fn select_environment(tournament: &Tournament) -> Environment {
    match tournament.tournament_type {
        TournamentType::Neutral => Environment::Neutral,
        TournamentType::Specialized => tournament.environment,
        TournamentType::Wild => {
            let environments = vec![
                Environment::Storm,
                Environment::Volcanic,
                Environment::Aquatic,
                Environment::Tundra,
                Environment::Desert,
                Environment::Forest,
                Environment::Radiation,
            ];
            environments[rand_range(0, environments.len())]
        },
        TournamentType::Lethal => tournament.environment, // Pre-set
    }
}
```

### 3.4 Environment Effects During Battle

Some environments have per-turn effects:

| Environment | Per-Turn Effect |
|-------------|-----------------|
| Volcanic | All kaiju take 2% max HP damage (lava burn) |
| Radiation | 5% chance to gain temporary +5 damage (1 turn) |
| Void | Random stat swap (10% chance): attack ↔ defense |
| Toxic (from traits) | Attacker with Toxic trait: defender loses 2% HP/turn |

**Note**: Per-turn effects apply at end of turn, after damage resolution.

---

## 4. Turn Order (Initiative System)

### 4.1 Speed-Based Initiative

Turn order is determined by the **speed** stat.

```rust
pub fn determine_turn_order(kaiju_a: &Kaiju, kaiju_b: &Kaiju) -> (Kaiju, Kaiju) {
    if kaiju_a.stats.speed >= kaiju_b.stats.speed {
        (kaiju_a, kaiju_b)  // A attacks first
    } else {
        (kaiju_b, kaiju_a)  // B attacks first
    }
}
```

**Tie-Breaking:**
- If speeds are exactly equal, the kaiju with higher attack goes first
- If still tied, random 50/50 determination using battle seed

**Turn Alternation:**
- Turns alternate: A → B → A → B → ...
- No "double turns" or action queuing
- Each turn consists of one attack action

### 4.2 Speed Modifiers

Speed can be modified by traits:

```rust
pub fn calculate_effective_speed(kaiju: &Kaiju) -> i32 {
    let mut speed = kaiju.stats.speed;

    for trait in &kaiju.traits {
        match trait.name.as_str() {
            "Swift" => speed = (speed as f32 * 1.10) as i32,
            "Heavy" => speed = (speed as f32 * 0.95) as i32,
            "Agile Reflexes" => {
                if environment_favorable(kaiju, env) {
                    speed = (speed as f32 * 1.15) as i32;
                }
            },
            _ => {}
        }
    }

    speed
}
```

### 4.3 Initiative Example

**Example Kaiju Stats:**
- Flossy: Speed 30 → Attacks first
- Reefmaw: Speed 25 → Attacks second

**Turn Sequence:**
```
Turn 1: Flossy attacks Reefmaw
Turn 2: Reefmaw attacks Flossy
Turn 3: Flossy attacks Reefmaw
...
```

---

## 5. Battle Flow (Step-by-Step)

### 5.1 Pre-Battle Setup

```rust
pub fn initialize_battle(
    kaiju_a: Kaiju,
    kaiju_b: Kaiju,
    environment: Environment,
    seed: u64  // Deterministic RNG seed
) -> BattleState {

    // Set RNG seed for reproducibility
    set_rng_seed(seed);

    // Calculate effective stats (with trait modifiers)
    let stats_a = apply_pre_battle_modifiers(&kaiju_a);
    let stats_b = apply_pre_battle_modifiers(&kaiju_b);

    // Determine turn order
    let (first, second) = determine_turn_order(&kaiju_a, &kaiju_b);

    // Initialize battle state
    BattleState {
        kaiju_a: kaiju_a.clone(),
        kaiju_b: kaiju_b.clone(),
        hp_a: stats_a.hp,
        hp_b: stats_b.hp,
        environment,
        turn_count: 0,
        first_attacker: first,
        battle_log: Vec::new(),
        seed,
    }
}
```

### 5.2 Turn Resolution Loop

```rust
pub fn execute_battle(mut state: BattleState) -> BattleResult {

    // Determine initial attacker and defender
    let (mut attacker_is_a, mut defender_is_b) =
        (state.first_attacker == state.kaiju_a, true);

    while state.hp_a > 0 && state.hp_b > 0 {
        state.turn_count += 1;

        // Get current attacker and defender
        let (attacker, defender) = if attacker_is_a {
            (&state.kaiju_a, &state.kaiju_b)
        } else {
            (&state.kaiju_b, &state.kaiju_a)
        };

        // Calculate damage
        let damage = calculate_damage(attacker, defender, &state.environment);

        // Apply damage
        if defender_is_b {
            state.hp_b -= damage;
        } else {
            state.hp_a -= damage;
        }

        // Log turn
        state.battle_log.push(BattleLogEntry {
            turn: state.turn_count,
            attacker: attacker.name.clone(),
            defender: defender.name.clone(),
            damage,
            hp_remaining: if defender_is_b { state.hp_b } else { state.hp_a },
        });

        // Apply environment effects
        apply_environment_effects(&mut state);

        // Swap attacker/defender for next turn
        attacker_is_a = !attacker_is_a;
        defender_is_b = !defender_is_b;

        // Safety: max 1000 turns to prevent infinite loops
        if state.turn_count >= 1000 {
            break;
        }
    }

    // Determine winner
    create_battle_result(state)
}
```

### 5.3 Battle Log Entry Structure

```rust
pub struct BattleLogEntry {
    pub turn: u32,
    pub attacker: String,
    pub defender: String,
    pub damage: i32,
    pub hp_remaining: i32,
    pub special_effects: Vec<String>,  // e.g., "Volcanic burn", "Mutation triggered"
}
```

### 5.4 Turn Sequence Detail

**Each Turn:**

1. **Identify Attacker/Defender** (based on speed-determined order)
2. **Calculate Base Damage** (attack - defense*0.5, min 5)
3. **Add Trait Bonuses** (element trait powers)
4. **Apply Environment Multiplier** (based on trait-environment synergy)
5. **Apply Randomness** (±5% variance)
6. **Deal Damage** (reduce defender HP)
7. **Log Turn** (record to battle log)
8. **Apply Environment Effects** (volcanic burn, radiation chance, etc.)
9. **Check Victory Condition** (HP <= 0?)
10. **Swap Attacker/Defender** (prepare next turn)

---

## 6. Critical Hits / Special Events

### 6.1 No Traditional Critical Hits (v1.0)

The system does **not** include random critical hits in v1.0 for design reasons:
- Maintains predictability (core design pillar)
- Prevents excessive variance
- Keeps combat deterministic with seed

### 6.2 Special Events

Special events can occur based on traits or environment:

#### First Strike Bonus
- Trait: "First Strike"
- Effect: +20% damage on turn 1 only
- Applies to opening attacker

#### Berserker Activation
- Trait: "Berserker"
- Trigger: HP drops below 50%
- Effect: +15% damage for remainder of battle

#### Mutation Trigger (Radiation Environment)
- Environment: Radiation
- Chance: 5% per turn
- Effect: Random +5 damage for 1 turn
- Logged as special event

#### Combo Stacking
- Trait: "Combo Fighter"
- Effect: +5% damage per consecutive hit (max 3 stacks)
- Resets if attacker takes damage before next turn

### 6.3 Future Special Event System

**Planned for v2.0:**
- **Synergy Activations**: Multi-trait combinations trigger special moves
- **Environment Reactions**: Specific trait + environment = unique effect
- **Desperation Moves**: Last-stand abilities at critical HP
- **Counter Attacks**: Chance to strike back when hit (speed-based)

---

## 7. Victory Conditions

### 7.1 Primary Victory Condition

**HP Depletion:**
- Battle ends when any kaiju reaches **HP ≤ 0**
- The kaiju with HP > 0 is declared the winner
- If both reach 0 on the same turn (simultaneous), the one with higher remaining HP wins

```rust
pub fn determine_winner(state: &BattleState) -> BattleWinner {
    if state.hp_a > 0 && state.hp_b <= 0 {
        BattleWinner::KaijuA
    } else if state.hp_b > 0 && state.hp_a <= 0 {
        BattleWinner::KaijuB
    } else if state.hp_a > state.hp_b {
        BattleWinner::KaijuA  // Both <= 0, A has more HP
    } else {
        BattleWinner::KaijuB  // Both <= 0, B has more HP
    }
}
```

### 7.2 Timeout Victory

**Maximum Turn Limit:**
- Safety limit: **1000 turns**
- If reached, winner = kaiju with higher HP percentage remaining
- Logged as "Draw by Timeout" in battle records

```rust
if state.turn_count >= 1000 {
    let hp_percent_a = (state.hp_a as f32 / state.kaiju_a.stats.hp as f32) * 100.0;
    let hp_percent_b = (state.hp_b as f32 / state.kaiju_b.stats.hp as f32) * 100.0;

    if hp_percent_a > hp_percent_b {
        return BattleWinner::KaijuA;
    } else {
        return BattleWinner::KaijuB;
    }
}
```

**Note**: Timeout is extremely rare due to minimum damage floor (5) and should only occur with extreme defensive builds.

### 7.3 Special Conditions (Tournament-Specific)

#### Lethal Tournaments
- **Death Rule**: Losing kaiju is permanently killed
- **No Revive**: Death is irreversible
- **Legacy Creation**: Dead kaiju enters Hall of Fame

#### Surrender (Non-Lethal Only)
- Player may forfeit before battle starts
- Counts as loss, but no HP damage applied
- Available only in non-lethal tournaments

### 7.4 Victory Rewards

**Non-Lethal Tournaments:**
- Experience points for both participants (winner gets more)
- Reputation gain
- Tournament rank advancement

**Lethal Tournaments:**
- Winner: Full rewards + loser's breeding rights refunds
- Loser: Permanent death, legacy record created

---

## 8. Randomness Bounds

### 8.1 Variance Range

**Damage Variance:**
- Range: **0.95 to 1.05** (±5%)
- Applied as final multiplier to damage
- Example: 100 base damage → 95-105 actual damage

```rust
let variance = rand_range(0.95, 1.05);  // Uniform distribution
let final_damage = calculated_damage * variance;
```

### 8.2 Deterministic Randomness

**Battle Seed System:**
- Each battle has a unique deterministic seed
- Seed is recorded in battle log
- Anyone can replay the battle with same seed → same outcome
- Ensures auditability and prevents cheating accusations

```rust
pub struct BattleResult {
    pub winner: KaijuId,
    pub loser: KaijuId,
    pub seed: u64,           // Stored for replay
    pub turn_count: u32,
    pub hp_remaining: i32,
    pub battle_log: Vec<BattleLogEntry>,
}
```

### 8.3 RNG Sources

**Variance RNG:**
- Used for ±5% damage variance
- Seeded per-battle
- XORshift or equivalent fast PRNG

**Environment RNG (if applicable):**
- Void environment chaos
- Radiation mutation triggers
- Uses same battle seed for consistency

### 8.4 No Hidden RNG

**Transparency Principle:**
- All randomness uses the battle seed
- No external random factors
- No server-side secret RNG
- Players can verify outcomes locally

### 8.5 Variance Impact Analysis

**With ±5% variance:**
- Close matches (48%-52% win probability): variance matters
- Dominant matches (>70% win probability): variance irrelevant
- Expected upset rate: ~5% in evenly matched fights
- Skill/preparation remains dominant factor

**Example Scenarios:**

| Base Damage | Min Roll (95%) | Max Roll (105%) | Range |
|-------------|----------------|-----------------|-------|
| 50 | 47 | 52 | ±3 |
| 100 | 95 | 105 | ±5 |
| 200 | 190 | 210 | ±10 |

---

## 9. Battle Log Generation

### 9.1 Battle Log Structure

```rust
pub struct BattleLog {
    pub battle_id: String,
    pub timestamp: u64,
    pub seed: u64,
    pub environment: Environment,
    pub participants: (Kaiju, Kaiju),
    pub entries: Vec<BattleLogEntry>,
    pub result: BattleResult,
    pub replay_data: ReplayData,
}

pub struct BattleLogEntry {
    pub turn: u32,
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub hp_remaining: i32,
    pub special_effects: Vec<String>,
}
```

### 9.2 Information Recorded Per Turn

**Mandatory Fields:**
- Turn number
- Attacker name
- Defender name
- Damage dealt
- Defender HP remaining

**Optional Fields:**
- Trait activations (e.g., "Berserker activated")
- Environment effects (e.g., "Volcanic burn: -6 HP")
- Critical moments (e.g., "HP below 50%")

### 9.3 Battle Log Output Format

**Human-Readable (UI Display):**
```
Turn 1: Flossy hits Reefmaw for 28 damage (292 HP remaining)
Turn 2: Reefmaw hits Flossy for 19 damage (281 HP remaining)
Turn 3: Flossy hits Reefmaw for 31 damage (261 HP remaining)
        [Storm bonus: Electric Breath +15%]
Turn 4: Reefmaw hits Flossy for 21 damage (260 HP remaining)
...
Turn 18: Flossy hits Reefmaw for 27 damage (0 HP remaining)
VICTORY: Flossy wins with 260 HP remaining!
```

**Machine-Readable (JSON Export):**
```json
{
  "battle_id": "battle_20260109_001234",
  "timestamp": 1704844800,
  "seed": 9876543210,
  "environment": "Storm",
  "participants": [
    {"name": "Flossy", "token_id": "kaiju_001"},
    {"name": "Reefmaw", "token_id": "kaiju_002"}
  ],
  "turns": [
    {"turn": 1, "attacker": "Flossy", "damage": 28, "hp_remaining": 292},
    {"turn": 2, "attacker": "Reefmaw", "damage": 19, "hp_remaining": 281}
  ],
  "result": {
    "winner": "Flossy",
    "hp_remaining": 260,
    "turns_elapsed": 18
  }
}
```

### 9.4 Partial Trait Revelation

**Public Information (Always Logged):**
- Visible traits that activated
- Environment interactions observed
- Damage ranges (not exact calculations)

**Hidden Information (Not Logged):**
- Exact trait power values
- Hidden trait presence (unless activated)
- Precise calculation breakdowns
- Defense scaling mechanics

**Example Hint:**
```
"Flossy's Electric trait performed exceptionally well in the Storm environment."
```

Not:
```
"Flossy's Electric Breath (power 8) received a 1.15x multiplier from Storm."
```

### 9.5 Battle Report Summary

After battle, players receive a summary report:

```rust
pub struct BattleReport {
    pub outcome: BattleOutcome,
    pub winner: String,
    pub loser: String,
    pub hp_remaining: i32,
    pub turns_elapsed: u32,
    pub insights: Vec<String>,      // Hints about trait performance
    pub suggestions: Vec<String>,   // Improvement recommendations
}
```

**Example Insights:**
- "Electric traits performed above average in this Storm environment."
- "Defender's high defense reduced incoming damage significantly."
- "Fast kaiju controlled tempo by attacking first."

**Example Suggestions:**
- "Consider breeding for higher speed to gain initiative."
- "Defense-focused builds may struggle against high-attack opponents."
- "Storm environment favors Electric and Wind traits."

---

## 10. Example Battle (Complete Walkthrough)

### 10.1 Battle Setup

**Participants:**

**Kaiju A: Flossy**
- Generation: 4
- Stats: HP 300, Attack 60, Defense 40, Speed 30
- Traits: Electric Breath (power 8)
- Hidden Traits: None visible

**Kaiju B: Reefmaw**
- Generation: 4
- Stats: HP 320, Attack 55, Defense 45, Speed 25
- Traits: Aqua Hide (power 6)
- Hidden Traits: None visible

**Battle Parameters:**
- Environment: **Storm**
- Tournament: Non-Lethal Ranked
- Battle Seed: 1234567890

### 10.2 Pre-Battle Calculations

**Turn Order Determination:**
- Flossy speed: 30
- Reefmaw speed: 25
- **Result**: Flossy attacks first

**Environment Analysis:**
- Storm environment
- Flossy (Electric) gets +15% damage boost
- Reefmaw (Aqua) gets +5% damage boost

### 10.3 Turn-by-Turn Breakdown

#### Turn 1: Flossy attacks Reefmaw

**Step 1: Base Damage**
```
base = attack - (defense * 0.5)
base = 60 - (45 * 0.5)
base = 60 - 22.5
base = 37.5
```

**Step 2: Apply Minimum Floor**
```
base = max(37.5, 5) = 37.5  // Above minimum
```

**Step 3: Add Trait Bonus**
```
trait_bonus = Electric Breath power = 8
total = 37.5 + 8 = 45.5
```

**Step 4: Apply Environment Multiplier**
```
Storm + Electric = 1.15x multiplier
total = 45.5 * 1.15 = 52.325
```

**Step 5: Apply Randomness**
```
variance = rand(0.95, 1.05) = 1.02  // Random roll
final = 52.325 * 1.02 = 53.37
final = 53 (rounded)
```

**Result:**
- Damage: 53
- Reefmaw HP: 320 - 53 = **267**

**Log Entry:**
```
Turn 1: Flossy hits Reefmaw for 53 damage
[Storm bonus: Electric trait enhanced]
Reefmaw HP: 267/320
```

---

#### Turn 2: Reefmaw attacks Flossy

**Step 1: Base Damage**
```
base = 55 - (40 * 0.5) = 55 - 20 = 35
```

**Step 2: Minimum Floor**
```
base = 35  // Above minimum
```

**Step 3: Trait Bonus**
```
trait_bonus = Aqua Hide power = 6
total = 35 + 6 = 41
```

**Step 4: Environment Multiplier**
```
Storm + Aqua = 1.05x (slight boost)
total = 41 * 1.05 = 43.05
```

**Step 5: Randomness**
```
variance = rand(0.95, 1.05) = 0.97  // Random roll
final = 43.05 * 0.97 = 41.76
final = 42 (rounded)
```

**Result:**
- Damage: 42
- Flossy HP: 300 - 42 = **258**

**Log Entry:**
```
Turn 2: Reefmaw hits Flossy for 42 damage
Flossy HP: 258/300
```

---

#### Turn 3: Flossy attacks Reefmaw

**Calculation (abbreviated):**
- Base: 37.5
- + Trait: 8 = 45.5
- × Environment: 1.15 = 52.325
- × Variance (0.99): 51.80
- **Final: 52 damage**

**Result:**
- Reefmaw HP: 267 - 52 = **215**

**Log Entry:**
```
Turn 3: Flossy hits Reefmaw for 52 damage
Reefmaw HP: 215/320
```

---

#### Continuing the Battle...

**Turn 5:** Flossy deals 54 damage → Reefmaw HP: 161
**Turn 6:** Reefmaw deals 43 damage → Flossy HP: 170
**Turn 7:** Flossy deals 51 damage → Reefmaw HP: 110
**Turn 8:** Reefmaw deals 44 damage → Flossy HP: 126
**Turn 9:** Flossy deals 53 damage → Reefmaw HP: 57
**Turn 10:** Reefmaw deals 42 damage → Flossy HP: 84
**Turn 11:** Flossy deals 52 damage → Reefmaw HP: 5
**Turn 12:** Reefmaw deals 43 damage → Flossy HP: 41
**Turn 13:** Flossy deals 54 damage → Reefmaw HP: **-49** (DEFEAT)

### 10.4 Battle Result

**Winner:** Flossy
**HP Remaining:** 41/300 (13.7%)
**Turns Elapsed:** 13
**Environment:** Storm
**Seed:** 1234567890

**Victory Breakdown:**
- Flossy benefited significantly from Storm environment (+15% to Electric trait)
- Average damage per turn: Flossy ~52, Reefmaw ~43
- Initiative advantage (speed 30 vs 25) gave Flossy 7 attacks vs Reefmaw's 6
- Close victory (41 HP remaining out of 300)

### 10.5 Battle Report

**Outcome Summary:**
```
VICTORY: Flossy defeats Reefmaw in 13 turns

Final HP: Flossy 41/300 | Reefmaw 0/320

Key Factors:
✓ Storm environment strongly favored Flossy's Electric trait (+15%)
✓ Speed advantage allowed Flossy to attack first and maintain tempo
✓ Trait power differential (Electric 8 vs Aqua 6) provided consistent edge

Performance Insights:
• Flossy's Electric Breath excelled in Storm conditions
• Reefmaw's defense (45) mitigated significant damage
• Battle was decided by environmental advantage and initiative
```

**Recommendations for Reefmaw's Breeder:**
```
Suggested Improvements:
1. Consider Aquatic environment tournaments (Aqua Hide gets +20% vs +5%)
2. Breed for higher speed to contest initiative (target 30+)
3. Add complementary trait to Aqua Hide for better damage output
```

### 10.6 Replay Verification

**Anyone can verify this battle:**

```rust
let result = replay_battle(
    kaiju_a: load_kaiju("Flossy"),
    kaiju_b: load_kaiju("Reefmaw"),
    environment: Environment::Storm,
    seed: 1234567890,
);

assert_eq!(result.winner, "Flossy");
assert_eq!(result.hp_remaining, 41);
assert_eq!(result.turns_elapsed, 13);
```

**Outcome is deterministic** given the same seed and kaiju states.

---

## 11. Implementation Reference

### 11.1 Key Files for Implementation

Based on the IMPLEMENTATION_GUIDE.md structure, combat system will be implemented in:

1. **src/engine/combat.rs** (Primary combat logic)
   - Damage calculation function
   - Battle simulation loop
   - Turn resolution
   - Environment effects

2. **src/data/traits.rs** (Trait definitions)
   - Trait struct and enums
   - Trait power values
   - Trait categories

3. **src/data/kaiju.rs** (Kaiju entity structure)
   - Stats structure
   - Trait references
   - HP tracking

4. **assets/balance.json** (Combat constants)
   - Minimum damage floor
   - Variance range
   - Environment multipliers
   - Defense scaling factor

5. **src/state/battle_state.rs** (Battle state management)
   - BattleState struct
   - Battle log tracking
   - Turn counter

### 11.2 Data-Driven Configuration

**assets/balance.json** should contain:
```json
{
  "combat": {
    "defense_scaling": 0.5,
    "minimum_damage": 5,
    "variance_min": 0.95,
    "variance_max": 1.05,
    "max_turns": 1000
  },
  "environments": {
    "storm": {
      "electric": 1.15,
      "wind": 1.15,
      "fire": 0.90,
      "water": 1.05
    },
    "volcanic": {
      "fire": 1.25,
      "earth": 1.10,
      "ice": 0.85,
      "water": 0.90
    }
  }
}
```

### 11.3 Testing Requirements

**Unit Tests:**
- Damage calculation with known values
- Environment multiplier application
- Turn order determination
- Victory condition detection
- Minimum damage floor enforcement
- Variance bounds (0.95-1.05)

**Integration Tests:**
- Full battle simulation
- Battle log generation
- Deterministic replay verification
- Edge cases (equal speed, equal HP, timeout)

---

## 12. Future Enhancements (v2.0+)

### 12.1 Planned Features

1. **Synergy System**: Multi-trait combo activations
2. **Counter Attacks**: Chance to strike back (speed-based)
3. **Status Effects**: Poison, burn, freeze, stun
4. **Ultimate Abilities**: Once-per-battle special moves
5. **Environmental Hazards**: Damaging terrain, obstacles
6. **Team Battles**: 2v2 or 3v3 formats

### 12.2 Balance Considerations

- Monitor win rate variance (target: 45%-55% in even matches)
- Track environment impact on meta (no single environment should dominate)
- Analyze trait usage (ensure diversity, no "must-have" traits)
- Review battle length (target: 10-30 turns for balanced matches)

---

**Status**: Ready for Implementation
**Next Step**: Create `src/engine/combat.rs` and implement damage calculation function following this specification.
