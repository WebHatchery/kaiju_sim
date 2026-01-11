# TOURNAMENT SYSTEM DESIGN

**Kaiju Breeding Simulator - Complete Tournament System Design**

**Version**: 1.0
**Date**: 2026-01-09
**Related Documents**: kaiju_sim.md, IMPLEMENTATION_GUIDE.md, nft_design.md

---

## 1. Tournament Types

### 1.1 Non-Lethal Ranked Tournaments

**Purpose**: Competitive progression without permanent risk.

**Characteristics**:
- Kaiju cannot die
- Awards experience points
- Contributes to global ranking
- Primary path for growth
- No entry cost (v1)

**Variants**:
- **Open Ranked**: Any kaiju can enter, matchmaking by skill
- **Generation-Restricted**: Only kaiju of specific generation (e.g., "Gen 5 Only")
- **Weekly Ladder**: Seasonal rankings that reset
- **Practice Matches**: No ranking impact, reduced XP

**Use Cases**:
- Grinding experience
- Testing builds
- Climbing leaderboards safely
- New player onboarding

---

### 1.2 Lethal "Winner Takes All" Tournaments

**Purpose**: High-stakes, legacy-defining competitions.

**Characteristics**:
- All losers die permanently
- Winner gains massive prestige
- Death finalizes on-chain
- Breeding rights refunded on death
- Hall of Fame entry for winner

**Entry Requirements**:
- Explicit confirmation flow: "YOUR KAIJU WILL DIE IF THEY LOSE"
- Minimum ranking threshold (prevents suicide entries)
- 24-hour registration window (prevents impulsive entries)

**Risk Protection**:
- Pre-tournament odds estimation
- Lineage warning ("This kaiju has 3 offspring")
- Opt-in only, never mandatory

**Example Tournament**:
- **"Champion's Crucible"**: Monthly 8-kaiju elimination
- **"Survival Gauntlet"**: Swiss-style, bottom half dies
- **"Ancient Trial"**: Gen 1-3 only, winner becomes legend

---

### 1.3 Generation-Restricted Tournaments

**Purpose**: Ensure fair competition across power levels.

**Restrictions**:
- **Exact Gen**: "Gen 5 Only"
- **Gen Range**: "Gen 1-3"
- **Max Gen**: "Gen 7 or below"
- **Ancient Only**: Gen 1-2 (prestige events)

**Why This Matters**:
- Prevents overwhelming power creep
- Creates niches for older kaiju
- Maintains breeding diversity
- Adds strategic depth (do you specialize for a bracket?)

**Special Cases**:
- **"Founder's Cup"**: Gen 1 only, extreme rarity
- **"New Blood"**: Latest generation only

---

### 1.4 Special Event Tournaments

**Purpose**: Seasonal variety and narrative hooks.

**Examples**:

| Event Name | Rules | Frequency |
|------------|-------|-----------|
| **Storm Supremacy** | Storm environment only, electric advantage | Monthly |
| **Bloodline Wars** | Teams of related kaiju | Quarterly |
| **Cataclysm Trial** | Random environment each round | Special |
| **Underdog Uprising** | Low-ranking kaiju only | Weekly |
| **Breeding Rights Bonanza** | Winner grants free breeding rights | Monthly |

**Event Modifiers**:
- Environment locks
- Stat caps
- Trait bans
- Team-based brackets
- Unusual reward structures

---

## 2. Bracket Systems

### 2.1 Single Elimination

**Structure**: Standard knockout tournament.

**Characteristics**:
- Simple, fast resolution
- High stakes per match
- One loss = elimination
- Predictable duration

**Best For**:
- Lethal tournaments (death on loss)
- Small brackets (4-8 kaiju)
- Quick events

**Bracket Sizes**: 4, 8, 16, 32

**Match Flow**:
```
Round 1: 8 → 4 (4 matches)
Round 2: 4 → 2 (2 matches)
Finals:  2 → 1 (1 match)
```

---

### 2.2 Double Elimination

**Structure**: Losers get a second chance via losers' bracket.

**Characteristics**:
- Two losses required for elimination
- More forgiving
- Longer tournament duration
- Complex bracket management

**Best For**:
- Non-lethal ranked tournaments
- Prestige events where one upset shouldn't end a run
- Larger brackets (16+)

**Why NOT for Lethal Tournaments**:
- Death must be final and meaningful
- "Second life" violates design philosophy

**Bracket Visualization**:
```
Winners' Bracket:  8 → 4 → 2 → 1
Losers' Bracket:   4 → 2 → 1 → Finals (if loser wins)
```

**Determination**:
- Available in v1 for non-lethal only
- May add in v2 if player demand exists

---

### 2.3 Swiss System

**Structure**: Fixed number of rounds, match by record.

**Characteristics**:
- No elimination
- All kaiju play all rounds
- Pairing by win/loss record
- Ranking determines final placement

**Best For**:
- Large participant pools (32+)
- Fair ranking across skill levels
- Non-lethal progression

**Example (8 participants, 3 rounds)**:
```
Round 1: All 0-0, random pairs
Round 2: 1-0 vs 1-0, 0-1 vs 0-1
Round 3: 2-0 vs 2-0, 1-1 vs 1-1, etc.
```

**Ranking Formula**:
```rust
final_rank = (wins * 100) + tiebreaker_score
```

**Tiebreakers**:
1. Head-to-head result
2. Opponent win percentage
3. Total damage dealt across matches
4. Kaiju generation (lower = tiebreaker advantage)

**Why Swiss for Lethal Tournaments?**
- Possible variant: "Survival Swiss"
- Bottom 50% die after final round
- Rewards consistency over luck

**Implementation Priority**: Phase 2 (after single elimination stable)

---

### 2.4 Seeding Algorithms

**Purpose**: Ensure fair initial matchups and prevent early champion clashes.

**Seeding Sources**:
1. **Global Ranking** (primary)
2. **Recent Performance** (last 10 matches)
3. **Generation Power Level** (soft factor)
4. **Tournament History** (championship wins)

**Seeding Formula**:
```rust
seed_score = (global_rank * 0.5)
           + (recent_winrate * 0.3)
           + (tournament_wins * 0.15)
           + (generation_factor * 0.05)
```

**Bracket Distribution**:
```
Single Elimination (8 kaiju):
Seed 1 vs Seed 8
Seed 4 vs Seed 5
Seed 2 vs Seed 7
Seed 3 vs Seed 6
```

**Special Cases**:
- **Unseeded Tournaments**: Pure random (practice only)
- **Underdog Events**: Reverse seeding (weakest vs weakest first)
- **Chaos Mode**: No seeding, full randomization

---

## 3. Entry Requirements

### 3.1 Eligibility Checks

**System Validation**:
```rust
pub struct EntryValidation {
    pub is_alive: bool,
    pub meets_generation_req: bool,
    pub meets_rank_minimum: bool,
    pub not_in_other_tournament: bool,
    pub owner_confirmed: bool,
    pub sufficient_xp: bool,  // Prevents brand-new kaiju in lethal
}
```

**Validation Flow**:
1. Check kaiju alive status (on-chain for lethal)
2. Verify generation restrictions
3. Confirm ranking threshold (if applicable)
4. Ensure not already registered
5. Check ownership (wallet match)
6. Validate minimum experience (lethal only)

**Rejection Reasons**:
- "Kaiju is dead"
- "Generation 7 not allowed in this tournament"
- "Ranking too low (minimum: 1000)"
- "Already registered in another active tournament"
- "Insufficient experience (need 500 XP minimum)"

---

### 3.2 Entry Fees/Costs

**v1 Design**: No entry fees (simplicity first)

**Future Economy (v2+)**:
```rust
pub struct EntryFee {
    pub currency: CurrencyType,  // In-game gold, token, etc.
    pub amount: u64,
    pub refund_on_death: bool,   // Lethal tournaments only
}
```

**Possible Fee Models**:
- **Flat Fee**: Same cost for all
- **Scaled by Generation**: Higher gen = higher cost
- **Ranking-Based**: Top players pay more
- **Prize Pool Contribution**: Entry fees become winner reward

**Death Refund Logic**:
- If kaiju dies in lethal tournament, entry fee refunded
- Prevents "pay to suicide" griefing
- Aligns with breeding rights refund philosophy

---

### 3.3 Generation Restrictions

**Implementation**:
```rust
pub struct GenerationRestriction {
    pub mode: RestrictionMode,
    pub allowed_generations: Vec<u32>,
}

pub enum RestrictionMode {
    Exact(u32),        // Only Gen 5
    Range(u32, u32),   // Gen 3-6
    Maximum(u32),      // Gen 8 or below
    Ancient,           // Gen 1-2 only
    None,              // Open to all
}
```

**Validation**:
```rust
fn meets_generation_requirement(kaiju: &Kaiju, restriction: &GenerationRestriction) -> bool {
    match restriction.mode {
        RestrictionMode::Exact(gen) => kaiju.generation == gen,
        RestrictionMode::Range(min, max) => {
            kaiju.generation >= min && kaiju.generation <= max
        }
        RestrictionMode::Maximum(max) => kaiju.generation <= max,
        RestrictionMode::Ancient => kaiju.generation <= 2,
        RestrictionMode::None => true,
    }
}
```

**UI Display**:
- Badge showing restriction: "Gen 5 Only"
- Warning if user's kaiju doesn't qualify
- Filter roster view to only show eligible kaiju

---

### 3.4 Stat Caps (If Any)

**Design Decision**: No stat caps in v1

**Rationale**:
- Generation restrictions handle power scaling
- Stat caps feel artificial
- Research and breeding investment should matter

**Future Consideration (v2)**:
- "Capped Tournaments" as special events
- Example: "Max 400 HP Challenge"
- Creates meta where defense/speed matter more

**If Implemented**:
```rust
pub struct StatCaps {
    pub max_hp: Option<i32>,
    pub max_attack: Option<i32>,
    pub max_defense: Option<i32>,
    pub max_speed: Option<i32>,
}
```

---

## 4. Matchmaking

### 4.1 How Kaiju Are Paired

**Initial Pairing (Round 1)**:
```rust
fn create_initial_pairings(kaiju_list: Vec<Kaiju>, seeded: bool) -> Vec<Match> {
    if seeded {
        // Seed-based bracket (strong vs weak)
        create_seeded_bracket(kaiju_list)
    } else {
        // Random shuffle
        create_random_bracket(kaiju_list)
    }
}
```

**Subsequent Rounds**:
- **Single Elimination**: Winners advance, bracket predetermined
- **Double Elimination**: Winners' bracket + losers' bracket pairing
- **Swiss**: Pair by record, avoid rematches

**Swiss Pairing Logic**:
```rust
fn swiss_pair(kaiju_list: Vec<(Kaiju, Record)>, round: u32) -> Vec<Match> {
    // 1. Group by win/loss record
    let groups = group_by_record(kaiju_list);

    // 2. Within each group, pair sequentially
    let mut matches = vec![];
    for group in groups {
        // Sort by tiebreaker
        let sorted = sort_by_tiebreaker(group);

        // Pair 1 vs 2, 3 vs 4, etc.
        for chunk in sorted.chunks(2) {
            if chunk.len() == 2 {
                matches.push(create_match(chunk[0], chunk[1]));
            } else {
                // Bye round (odd number)
                award_bye(chunk[0]);
            }
        }
    }
    matches
}
```

---

### 4.2 Fairness Algorithms

**Goals**:
- Prevent constant mismatches (strong vs weak repeatedly)
- Reward consistency
- Minimize randomness impact
- Preserve competitive integrity

**Fairness Metrics**:
```rust
pub struct FairnessScore {
    pub skill_difference: f32,     // Absolute ranking gap
    pub environment_neutrality: f32, // Neither has huge advantage
    pub prior_matchups: u32,       // Avoid rematches
}
```

**Pairing Optimization**:
```rust
fn optimize_pairings(candidates: Vec<(Kaiju, Kaiju)>) -> Vec<Match> {
    candidates
        .iter()
        .map(|(a, b)| {
            let score = calculate_fairness_score(a, b);
            (a, b, score)
        })
        .sorted_by(|a, b| b.2.cmp(&a.2))  // Highest fairness first
        .take(bracket_size / 2)
        .map(|(a, b, _)| create_match(a, b))
        .collect()
}
```

**Special Cases**:
- **Byes**: If odd number, highest seed gets bye
- **Rematches**: Avoid pairing kaiju who already fought (Swiss)
- **Same Owner**: Prevent if possible (anti-collusion)

---

### 4.3 Upset Potential Calculation

**Purpose**: Inform players of risk before entering lethal tournaments.

**Formula**:
```rust
fn calculate_upset_potential(kaiju_a: &Kaiju, kaiju_b: &Kaiju) -> UpsetAnalysis {
    let ranking_gap = (kaiju_a.ranking - kaiju_b.ranking).abs();
    let stat_difference = compare_stats(kaiju_a, kaiju_b);
    let trait_synergy = analyze_trait_matchup(kaiju_a, kaiju_b);
    let experience_gap = (kaiju_a.experience - kaiju_b.experience).abs();

    let upset_chance = if ranking_gap < 100 {
        0.4 + (stat_difference * 0.1) + (trait_synergy * 0.1)
    } else if ranking_gap < 500 {
        0.25 + (trait_synergy * 0.15)
    } else {
        0.1 + (trait_synergy * 0.1)  // Massive underdog
    };

    UpsetAnalysis {
        expected_winner: if kaiju_a.ranking > kaiju_b.ranking { kaiju_a } else { kaiju_b },
        upset_probability: upset_chance.clamp(0.05, 0.45),  // Never below 5%, never above 45%
        confidence: calculate_confidence(ranking_gap, stat_difference),
    }
}
```

**Display to Players**:
```
Expected Winner: Flossy (Rank 142)
Upset Chance: 15%
Confidence: High (Full stat knowledge)

Key Factors:
✓ Flossy has speed advantage
✗ Reefmaw has environmental advantage (Storm Arena)
? Hidden traits may impact outcome
```

**Confidence Levels**:
- **High**: Full trait visibility, predictable outcome
- **Medium**: Some hidden traits, moderate uncertainty
- **Low**: Many unknowns, research needed

---

## 5. Environment Selection

### 5.1 How Battle Environments Are Chosen

**Environment Types**:
```rust
pub enum Environment {
    Neutral,           // No modifiers
    Storm,             // Electric traits boosted
    Ocean,             // Aqua traits boosted
    Volcanic,          // Fire traits boosted
    Frozen,            // Ice traits boosted, speed reduced
    Radiation,         // Mutation traits activated
    Urban,             // Structural interactions
    Wilderness,        // Primal traits enhanced
}
```

**Selection Methods**:

| Tournament Type | Selection Method | Rationale |
|----------------|------------------|-----------|
| Non-Lethal Ranked | Random per match | Variety, adaptability matters |
| Lethal | Pre-announced | Fair warning, no surprise deaths |
| Special Events | Fixed theme | Environment defines event identity |
| Practice | Player choice | Learning tool |

**Implementation**:
```rust
fn select_environment(tournament: &Tournament, round: u32) -> Environment {
    match tournament.env_mode {
        EnvMode::Fixed(env) => env,
        EnvMode::Random => random_environment(),
        EnvMode::Rotating => ENVIRONMENTS[round % ENVIRONMENTS.len()],
        EnvMode::PlayerChoice => tournament.chosen_environment,
    }
}
```

---

### 5.2 Environment Rotation

**Rotation Schedule (Non-Lethal)**:
```
Match 1: Neutral
Match 2: Storm
Match 3: Ocean
Match 4: Volcanic
Match 5: Neutral (reset)
```

**Benefits**:
- Prevents environment gaming
- Rewards versatile builds
- Adds variety to spectating

**Special Event Rotation**:
- **"Gauntlet of Elements"**: Each round different environment
- **"Champion's Proving"**: Neutral only (pure skill)

---

### 5.3 Special Arena Types

**Standard Arenas**: Basic implementation, clear modifiers

**Special Arenas (v2+)**:
```rust
pub struct SpecialArena {
    pub name: String,
    pub environment: Environment,
    pub hazards: Vec<Hazard>,
    pub conditions: Vec<Condition>,
}
```

**Examples**:

| Arena | Environment | Special Effect |
|-------|-------------|----------------|
| **Thunder Dome** | Storm | Random lightning strikes (5-10 damage) |
| **Collapsing Ruins** | Urban | Debris falls each turn |
| **Tidal Basin** | Ocean | Water level fluctuates (speed penalty) |
| **Mutation Chamber** | Radiation | Traits randomly amplified |

**Hazard Mechanics**:
```rust
pub enum Hazard {
    PeriodicDamage(i32),      // X damage per turn
    StatReduction(StatType, f32), // -20% speed
    TraitAmplification(f32),   // +15% trait power
    RandomEvent(Vec<Effect>),  // Unpredictable
}
```

---

## 6. Reward Structure

### 6.1 Experience Gains for Participants

**Base XP Formula**:
```rust
fn calculate_xp(result: MatchResult, tournament_type: TournamentType) -> u32 {
    let base_xp = match result {
        MatchResult::Win => 100,
        MatchResult::Loss => 30,
    };

    let multiplier = match tournament_type {
        TournamentType::NonLethalRanked => 1.0,
        TournamentType::Lethal => 2.0,  // High risk = high reward
        TournamentType::Practice => 0.5,
        TournamentType::Special => 1.5,
    };

    let round_bonus = match result.round {
        1 => 0,
        2 => 20,  // Quarter-finals
        3 => 50,  // Semi-finals
        4 => 100, // Finals
        _ => 0,
    };

    ((base_xp as f32 * multiplier) as u32) + round_bonus
}
```

**Upset Bonus**:
```rust
if result.was_upset {
    xp += 50;  // Bonus for beating higher-ranked opponent
}
```

**Participation XP**:
- All entrants get minimum 10 XP (prevents zero-gain frustration)
- Encourages entry even if likely to lose

---

### 6.2 Ranking Point Distribution

**Elo-Style System**:
```rust
fn update_ranking(winner: &mut Kaiju, loser: &mut Kaiju, upset: bool) {
    let k_factor = 32;  // Volatility constant
    let expected_winner = 1.0 / (1.0 + 10f32.powf((loser.ranking - winner.ranking) as f32 / 400.0));
    let expected_loser = 1.0 - expected_winner;

    let winner_gain = k_factor * (1.0 - expected_winner);
    let loser_loss = k_factor * (0.0 - expected_loser);

    winner.ranking += winner_gain as i32;
    loser.ranking += loser_loss as i32;

    // Floor at 0
    loser.ranking = loser.ranking.max(0);
}
```

**Tournament Completion Bonus**:
```rust
// Winner gets extra ranking boost
if result.tournament_winner {
    kaiju.ranking += 100;  // Championship bonus
}
```

**Decay System (Future)**:
- Inactivity penalty: -5 ranking per week without match
- Prevents rank camping

---

### 6.3 Special Rewards for Winners

**Non-Lethal Tournament Rewards**:
```rust
pub struct TournamentReward {
    pub xp: u32,
    pub ranking_points: i32,
    pub title: Option<String>,        // "Storm Champion"
    pub badge: Option<BadgeId>,
    pub breeding_discount: Option<f32>, // 10% off next breed
}
```

**Lethal Tournament Rewards**:
```rust
pub struct LethalReward {
    pub xp: u32,                      // Massive XP boost
    pub ranking_points: i32,          // +500 minimum
    pub hall_of_fame_entry: bool,
    pub permanent_title: String,      // "Crucible Survivor"
    pub legacy_boost: f32,            // Bloodline prestige +20%
    pub free_breeding_rights: u32,    // Grant X free breeding uses
}
```

**Title System**:
- Titles display on kaiju card
- Inherited by offspring (partial)
- Examples:
  - "Storm Champion"
  - "Crucible Survivor"
  - "Underdog Legend"
  - "Ancient Master"

**Badge System** (Visual Flair):
- Icon displayed on kaiju portrait
- Permanent record of achievement
- Tradeable? (Future consideration)

---

## 7. Death Mechanics (Lethal Tournaments)

### 7.1 Death Confirmation Flow

**Pre-Tournament**:
```
1. Player selects kaiju for lethal tournament
2. Warning displayed: "THIS IS A LETHAL TOURNAMENT"
3. Confirmation dialog:

   ⚠️ WARNING ⚠️

   If [Kaiju Name] loses ANY match, they will die permanently.

   - This kaiju will be removed from your roster
   - All unused breeding rights will be refunded
   - Their legacy will be preserved in the Hall of Fame
   - This action cannot be undone

   Are you absolutely sure?

   [Cancel] [I Understand, Enter Tournament]

4. Secondary confirmation (typo-proof):

   Type your kaiju's name to confirm: _________

   [Cancel] [Confirm Death Risk]
```

**During Tournament**:
- Match result displays winner immediately
- Loser status changed to "Dying" (grace period for on-chain finalization)

**Post-Match Death**:
```
KAIJU FALLEN

[Kaiju Portrait]
[Kaiju Name]
Generation X | Rank XXX
Born: [Date] - Died: [Date]

Final Record: XX Wins, XX Losses
Notable Achievements: [List]

Legacy preserved in Hall of Fame.
All breeding rights refunded.

[View Hall of Fame Entry] [Return to Roster]
```

---

### 7.2 Breeding Rights Refunds

**Refund Logic**:
```rust
fn handle_kaiju_death(kaiju: &Kaiju, owner: &Player) {
    // 1. Identify all unused breeding rights
    let unused_rights = get_breeding_rights_for_kaiju(kaiju.id)
        .filter(|right| !right.used);

    // 2. Refund each holder
    for right in unused_rights {
        let refund_amount = right.purchase_price;
        refund_currency(right.owner, refund_amount);

        // 3. Notify holders
        send_notification(
            right.owner,
            format!("{} has died. Your breeding rights have been refunded.", kaiju.name)
        );

        // 4. Burn NFT (if rights are NFTs)
        burn_breeding_right_nft(right.token_id);
    }

    // 3. Mark kaiju as dead
    kaiju.alive = false;
    kaiju.death_date = current_timestamp();
    kaiju.death_reason = "Lethal Tournament Loss";

    // 4. Finalize on-chain
    finalize_death_on_chain(kaiju.token_id);
}
```

**Refund Timing**:
- Immediate off-chain credit
- On-chain finalization within 24 hours (batched)

**Protection Against Scams**:
- Breeding rights automatically invalidated on death
- Cannot sell rights after kaiju enters lethal tournament (locked)

---

### 7.3 Legacy Record Creation

**Legacy Data Structure**:
```rust
pub struct LegacyRecord {
    pub kaiju_id: TokenId,
    pub name: String,
    pub generation: u32,
    pub final_stats: KaijuStats,
    pub visible_traits: Vec<Trait>,
    pub lifetime_record: MatchRecord,
    pub tournament_victories: Vec<TournamentId>,
    pub offspring_count: u32,
    pub notable_descendants: Vec<TokenId>,
    pub death_date: u64,
    pub death_context: String,
    pub achievements: Vec<Achievement>,
}
```

**What Gets Preserved**:
- Full stat history
- All battle logs
- Breeding lineage (children, grandchildren)
- Tournament placements
- Titles and badges
- Final portrait image
- Owner history

**What Gets Revealed**:
- Hidden traits (post-mortem autopsy)
- Exact stat caps
- Genetic analysis (for research)

**Visibility**:
- Public Hall of Fame entry
- Searchable by name, generation, owner
- Lineage tree shows deceased ancestors (marked with skull icon)

---

### 7.4 Hall of Fame Entry

**Categories**:
```rust
pub enum HallOfFameCategory {
    LethalTournamentWinner,
    HighestRanking,
    MostTournamentWins,
    LongestWinStreak,
    MostOffspring,
    LegendaryBloodline,
    FirstOfGeneration,
}
```

**Entry Display**:
```
╔══════════════════════════════════════════════════════════╗
║              🏆 HALL OF FAME ENTRY 🏆                   ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  [AI-Generated Portrait]                                ║
║                                                          ║
║  VOLTAIRE THE UNBROKEN                                  ║
║  Generation 3 | Born: Day 45 | Died: Day 203           ║
║                                                          ║
║  ⚡ Electric Breath | 🌊 Aqua Hide | 🔥 Primal Rage    ║
║                                                          ║
║  ACHIEVEMENTS:                                          ║
║  ✓ Champion's Crucible Winner (Day 198)                ║
║  ✓ 47-3 Lifetime Record                                 ║
║  ✓ 12 Tournament Victories                              ║
║  ✓ Bloodline: 23 offspring, 102 descendants            ║
║  ✓ Peak Ranking: #3 Global                             ║
║                                                          ║
║  CAUSE OF DEATH:                                        ║
║  Defeated by Stormling (Gen 5) in Survival Gauntlet    ║
║  Final battle was a 3% upset victory for Stormling     ║
║                                                          ║
║  LEGACY:                                                ║
║  Voltaire's bloodline remains dominant in the Gen 4-6  ║
║  meta. Their electric traits are highly sought after   ║
║  for breeding. 15 Hall of Fame descendants.            ║
║                                                          ║
║  [View Full Battle History] [View Lineage Tree]        ║
╚══════════════════════════════════════════════════════════╝
```

**Sorting Options**:
- By ranking
- By generation
- By death date
- By offspring count
- By tournament wins

---

## 8. Spectating System

### 8.1 Can Players Watch Other Battles?

**Yes** - with restrictions.

**Public Information**:
- Tournament brackets (who's fighting whom)
- Match schedules
- Real-time results
- Winner announcements

**Private Information**:
- Exact damage numbers (hidden during live match)
- Hidden trait activations (revealed after)
- Detailed combat log (owner only, or post-match)

---

### 8.2 Spectator Modes

**Live Spectating**:
```rust
pub struct SpectatorView {
    pub can_watch_live: bool,        // Tournament setting
    pub sees_full_stats: bool,       // False = hide specifics
    pub sees_exact_damage: bool,     // False = show ranges
    pub sees_hidden_traits: bool,    // False = only visible
}
```

**Spectator Tiers**:

| Tier | Access | Cost |
|------|--------|------|
| **Public** | Match results, winner names | Free |
| **Standard** | Live battle animations, approximate outcomes | Free |
| **Premium** (v2) | Exact damage numbers, full logs | Token/subscription |
| **Owner** | Full access to their kaiju's matches | Automatic |

**UI Components**:
```
┌─────────────────────────────────────────────────┐
│  🎭 SPECTATE MODE                              │
├─────────────────────────────────────────────────┤
│  Storm Supremacy - Round 2                     │
│  Flossy vs Reefmaw                             │
│                                                 │
│  [Battle Animation Area]                       │
│                                                 │
│  HP: ████████░░ vs ██████████                  │
│                                                 │
│  Turn 5: Flossy attacks!                       │
│  Damage: ~25-30 (exact hidden)                 │
│                                                 │
│  [Skip to Result] [Exit]                       │
└─────────────────────────────────────────────────┘
```

---

### 8.3 What Information Is Revealed?

**During Match (Spectators)**:
- Kaiju names and generation
- Visible traits only
- Approximate HP bars
- Turn-by-turn summary (vague)

**After Match (Public)**:
- Winner/loser
- Final HP remaining
- Match duration (turns)
- Environment used
- Upset status (if applicable)

**After Match (Owner)**:
- Full damage breakdown
- Hidden trait activations
- Exact turn-by-turn log
- Research insights ("Your kaiju's defense underperformed")

**Post-Tournament (All)**:
- Full bracket results
- Champion stats (partial reveal)
- Upset analysis

---

## 9. Tournament Schedule

### 9.1 How Often Tournaments Run

**Tournament Calendar**:

| Type | Frequency | Participants |
|------|-----------|--------------|
| **Practice Matches** | Continuous | Any time, instant match |
| **Non-Lethal Ranked** | Every 4 hours | 8-16 kaiju |
| **Generation-Restricted** | Daily | 8 kaiju |
| **Lethal (Small)** | Weekly | 8 kaiju |
| **Lethal (Large)** | Monthly | 16-32 kaiju |
| **Special Events** | Weekly/Monthly | Varies |

**Example Weekly Schedule**:
```
Monday    12:00 UTC: Non-Lethal Ranked
Monday    16:00 UTC: Gen 5 Only
Tuesday   12:00 UTC: Non-Lethal Ranked
Tuesday   20:00 UTC: Storm Supremacy (Special)
Wednesday 12:00 UTC: Non-Lethal Ranked
Thursday  12:00 UTC: Gen 3-5 Restricted
Friday    12:00 UTC: Non-Lethal Ranked
Friday    20:00 UTC: Lethal Crucible (8 kaiju)
Saturday  12:00 UTC: Swiss Tournament (32 kaiju)
Sunday    12:00 UTC: Non-Lethal Ranked
Sunday    16:00 UTC: Underdog Uprising (Rank <500)
```

---

### 9.2 Registration Windows

**Non-Lethal**:
- Registration opens: 2 hours before start
- Registration closes: 15 minutes before start
- Tournament begins: At scheduled time

**Lethal**:
- Registration opens: 24 hours before start
- Registration closes: 1 hour before start (allows withdrawal grace period)
- Final confirmation: 30 minutes before start
- Tournament begins: At scheduled time (no delays)

**Registration Flow**:
```rust
pub struct RegistrationWindow {
    pub tournament_id: TournamentId,
    pub opens_at: u64,
    pub closes_at: u64,
    pub max_participants: u32,
    pub current_registrations: u32,
    pub waitlist_enabled: bool,
}
```

**UI Display**:
```
┌───────────────────────────────────────┐
│  UPCOMING TOURNAMENTS                │
├───────────────────────────────────────┤
│  ⚔️ Non-Lethal Ranked               │
│  Starts in: 1h 23m                   │
│  Participants: 6/8                   │
│  [Register Kaiju]                    │
├───────────────────────────────────────┤
│  💀 Lethal Crucible                 │
│  Starts in: 23h 45m                  │
│  Participants: 3/8                   │
│  Registration closes in: 22h 45m     │
│  [Register Kaiju]                    │
└───────────────────────────────────────┘
```

---

### 9.3 Batch vs Continuous Tournaments

**Batch (Primary Model)**:
- Fixed start times
- All matches resolve sequentially
- Clear winner announcement
- Better for spectating

**Continuous (Practice Only)**:
- Instant matchmaking when 2+ players queue
- No brackets, just 1v1
- Quick XP grinding
- No ranking impact

**Hybrid Approach**:
- Ranked tournaments are batched
- Practice matches are continuous
- Special events are batched with ceremony

---

## 10. Example Tournament Flow: Complete 8-Kaiju Tournament

### Scenario: "Storm Supremacy" Non-Lethal Ranked Tournament

**Setup**:
- 8 registered kaiju
- Environment: Storm (fixed)
- Bracket: Single Elimination
- Seeded by global ranking

---

### Step 1: Registration (2 Hours Before Start)

**Registered Kaiju**:
1. **Voltaire** (Gen 3, Rank 142, Electric Breath)
2. **Reefmaw** (Gen 4, Rank 289, Aqua Hide)
3. **Flossy** (Gen 4, Rank 201, Electric Breath)
4. **Stormling** (Gen 5, Rank 445, Electric + Aqua)
5. **Tidecaller** (Gen 3, Rank 88, Aqua Hide)
6. **Ashenblade** (Gen 6, Rank 512, Fire traits - disadvantaged)
7. **Granite** (Gen 2, Rank 104, Earth traits - neutral)
8. **Skydancer** (Gen 5, Rank 367, Flying + Electric)

---

### Step 2: Seeding (15 Minutes Before Start)

**Seeded Bracket**:
```
Seed 1: Tidecaller (Rank 88)
Seed 2: Granite (Rank 104)
Seed 3: Voltaire (Rank 142)
Seed 4: Flossy (Rank 201)
Seed 5: Reefmaw (Rank 289)
Seed 6: Skydancer (Rank 367)
Seed 7: Stormling (Rank 445)
Seed 8: Ashenblade (Rank 512)
```

**Round 1 Matchups**:
```
Match A: Tidecaller (1) vs Ashenblade (8)
Match B: Granite (2) vs Stormling (7)
Match C: Voltaire (3) vs Skydancer (6)
Match D: Flossy (4) vs Reefmaw (5)
```

---

### Step 3: Round 1 Execution

**Match A: Tidecaller vs Ashenblade**
- Environment: Storm
- Tidecaller's aqua traits neutral
- Ashenblade's fire traits weakened by storm
- **Result**: Tidecaller wins (expected, 12 turns)
- Upset: No
- XP: Tidecaller +100, Ashenblade +30

**Match B: Granite vs Stormling**
- Granite: Earth traits, neutral in storm
- Stormling: Electric traits boosted in storm
- **Result**: Stormling wins (upset! 15 turns)
- Upset: Yes (Seed 7 beats Seed 2)
- XP: Stormling +100 +50 (upset), Granite +30

**Match C: Voltaire vs Skydancer**
- Both have electric traits (storm boost equal)
- Voltaire higher stats, more experience
- **Result**: Voltaire wins (expected, 9 turns)
- Upset: No
- XP: Voltaire +100, Skydancer +30

**Match D: Flossy vs Reefmaw**
- Flossy: Electric (storm boost)
- Reefmaw: Aqua (neutral)
- Close match, Reefmaw has higher HP
- **Result**: Reefmaw wins (close, 18 turns)
- Upset: No (ranking close enough)
- XP: Reefmaw +100, Flossy +30

---

### Step 4: Round 2 (Semi-Finals)

**Updated Bracket**:
```
Match E: Tidecaller (1) vs Stormling (7)
Match F: Voltaire (3) vs Reefmaw (5)
```

**Match E: Tidecaller vs Stormling**
- Tidecaller: Favorite, higher rank
- Stormling: Electric boost, upset momentum
- Environment advantage to Stormling
- **Result**: Stormling wins (major upset! 16 turns)
- Upset: Yes (Seed 7 beats Seed 1)
- XP: Stormling +120 +50 (upset) +20 (round bonus), Tidecaller +30 +20 (round bonus)

**Match F: Voltaire vs Reefmaw**
- Voltaire: Electric specialist, storm advantage
- Reefmaw: Aqua specialist, neutral
- Voltaire has experience edge
- **Result**: Voltaire wins (expected, 11 turns)
- Upset: No
- XP: Voltaire +100 +20 (round bonus), Reefmaw +30 +20 (round bonus)

---

### Step 5: Finals

**Match G: Stormling (7) vs Voltaire (3)**
- Both electric-focused
- Voltaire higher base stats and experience
- Stormling riding upset streak, Gen 5 advantage
- Environment equally beneficial

**Battle Log (Simulated)**:
```
Turn 1: Voltaire attacks (speed advantage) → 28 damage
Turn 2: Stormling attacks → 24 damage
Turn 3: Voltaire attacks → 31 damage (trait proc)
Turn 4: Stormling attacks → 26 damage
...
Turn 14: Voltaire HP: 45, Stormling HP: 62
Turn 15: Stormling attacks → 29 damage
Turn 16: Voltaire attacks → 27 damage
Turn 17: Stormling attacks → 33 damage (critical!)
```

**Result**: Stormling wins (upset! 17 turns)
- Upset: Yes (Seed 7 beats Seed 3)
- XP: Stormling +100 +50 (upset) +50 (semi) +100 (finals) = 300 total
- XP: Voltaire +30 +50 (finals participation) = 80 total

---

### Step 6: Tournament Conclusion

**Final Standings**:
1. **Stormling** (Champion) - 3 wins
2. **Voltaire** (Runner-up) - 2 wins
3. **Tidecaller** (Semi-finalist) - 1 win
4. **Reefmaw** (Semi-finalist) - 1 win
5-8. Others (eliminated Round 1)

**Rewards Distributed**:

**Stormling**:
- XP: 300
- Ranking: +150 points (major upset run)
- Title: "Storm Supremacy Champion"
- Badge: Storm Crown icon
- Legacy boost: +10% breeding value

**Voltaire**:
- XP: 180
- Ranking: +50 points (runner-up)
- Title: None (already a champion elsewhere)

**All Others**:
- XP: 30-50 (participation + round bonuses)
- Ranking: -5 to +20 (based on expected performance)

---

### Step 7: Post-Tournament Analysis

**UI Display**:
```
╔═══════════════════════════════════════════════════════╗
║  🏆 STORM SUPREMACY - TOURNAMENT COMPLETE            ║
╠═══════════════════════════════════════════════════════╣
║                                                       ║
║  CHAMPION: STORMLING                                 ║
║  [Portrait]                                          ║
║  Generation 5 | Rank 445 → 595                       ║
║                                                       ║
║  BRACKET:                                            ║
║  ┌─ Tidecaller ────────┐                            ║
║  │                      ├─ Stormling ─┐             ║
║  └─ Ashenblade (out) ──┘              │             ║
║  ┌─ Granite (out) ─────┐              ├─ Stormling ║
║  │                      ├─ Stormling ─┘      ↓      ║
║  └─ Stormling ─────────┘                 CHAMPION   ║
║  ┌─ Voltaire ──────────┐                            ║
║  │                      ├─ Voltaire ───┐            ║
║  └─ Skydancer (out) ───┘               │            ║
║  ┌─ Flossy (out) ──────┐               ├─ Voltaire ║
║  │                      ├─ Reefmaw ────┘            ║
║  └─ Reefmaw ───────────┘                            ║
║                                                       ║
║  UPSETS: 3 major upsets!                            ║
║  - Stormling defeated Granite (Seed 2)              ║
║  - Stormling defeated Tidecaller (Seed 1)           ║
║  - Stormling defeated Voltaire (Seed 3)             ║
║                                                       ║
║  ENVIRONMENT: Storm (boosted electric traits)       ║
║                                                       ║
║  [View Full Bracket] [Hall of Fame] [Exit]          ║
╚═══════════════════════════════════════════════════════╝
```

**Spectator Insights**:
- Electric traits dominated (as expected for Storm)
- Stormling's Gen 5 stats overcame experience gap
- Granite's upset loss opened bracket for Stormling's run
- Tidecaller's loss was the biggest shock (Rank 88 lost to Rank 445)

**Research Opportunities**:
- Players analyze Stormling's hidden traits (what enabled upsets?)
- Breeding demand for Stormling rights increases dramatically
- Voltaire's owner researches storm counters

---

## 11. Technical Implementation Notes

### 11.1 Data Structures

```rust
// Core tournament structure
pub struct Tournament {
    pub id: TournamentId,
    pub name: String,
    pub tournament_type: TournamentType,
    pub bracket_type: BracketType,
    pub environment_mode: EnvMode,
    pub registration_window: RegistrationWindow,
    pub max_participants: u32,
    pub participants: Vec<KaijuId>,
    pub bracket: Option<Bracket>,
    pub current_round: u32,
    pub status: TournamentStatus,
    pub rewards: RewardStructure,
}

pub enum TournamentType {
    NonLethalRanked,
    LethalWinnerTakesAll,
    GenerationRestricted(GenerationRestriction),
    SpecialEvent(EventConfig),
}

pub enum BracketType {
    SingleElimination,
    DoubleElimination,
    Swiss { rounds: u32 },
}

pub enum TournamentStatus {
    RegistrationOpen,
    RegistrationClosed,
    InProgress { current_round: u32 },
    Completed,
    Cancelled,
}

// Match representation
pub struct Match {
    pub id: MatchId,
    pub tournament_id: TournamentId,
    pub round: u32,
    pub kaiju_a: KaijuId,
    pub kaiju_b: KaijuId,
    pub environment: Environment,
    pub result: Option<MatchResult>,
    pub battle_log: Option<BattleLog>,
}

pub struct MatchResult {
    pub winner: KaijuId,
    pub loser: KaijuId,
    pub turns: u32,
    pub final_hp: (i32, i32),
    pub was_upset: bool,
    pub upset_probability: f32,
}

// Bracket management
pub struct Bracket {
    pub matches: Vec<Match>,
    pub rounds: Vec<Round>,
    pub advancement_map: HashMap<MatchId, MatchId>,  // Winner of match X goes to match Y
}

pub struct Round {
    pub number: u32,
    pub matches: Vec<MatchId>,
    pub status: RoundStatus,
}

pub enum RoundStatus {
    Pending,
    InProgress,
    Completed,
}
```

---

### 11.2 Tournament Engine Functions

```rust
// Entry validation
pub fn validate_entry(
    kaiju: &Kaiju,
    tournament: &Tournament,
    owner: &Player
) -> Result<(), EntryError> {
    // Check alive status
    if !kaiju.alive {
        return Err(EntryError::KaijuDead);
    }

    // Check generation restrictions
    if let TournamentType::GenerationRestricted(restriction) = &tournament.tournament_type {
        if !meets_generation_requirement(kaiju, restriction) {
            return Err(EntryError::GenerationMismatch);
        }
    }

    // Check ranking minimum (lethal only)
    if tournament.tournament_type == TournamentType::LethalWinnerTakesAll {
        if kaiju.ranking < tournament.min_ranking_threshold {
            return Err(EntryError::RankingTooLow);
        }
    }

    // Check ownership
    if kaiju.owner != owner.wallet_address {
        return Err(EntryError::NotOwner);
    }

    // Check not already registered elsewhere
    if is_registered_in_active_tournament(kaiju.id) {
        return Err(EntryError::AlreadyRegistered);
    }

    Ok(())
}

// Bracket generation
pub fn generate_bracket(participants: Vec<Kaiju>, bracket_type: BracketType) -> Bracket {
    match bracket_type {
        BracketType::SingleElimination => generate_single_elimination(participants),
        BracketType::DoubleElimination => generate_double_elimination(participants),
        BracketType::Swiss { rounds } => generate_swiss(participants, rounds),
    }
}

fn generate_single_elimination(participants: Vec<Kaiju>) -> Bracket {
    let seeded = seed_participants(participants);
    let mut matches = vec![];
    let rounds_count = (seeded.len() as f32).log2().ceil() as u32;

    // Round 1: Pair top with bottom
    for i in 0..(seeded.len() / 2) {
        matches.push(Match {
            id: generate_match_id(),
            tournament_id: /* ... */,
            round: 1,
            kaiju_a: seeded[i].id,
            kaiju_b: seeded[seeded.len() - 1 - i].id,
            environment: select_environment(/* ... */),
            result: None,
            battle_log: None,
        });
    }

    // Create advancement map
    let advancement_map = create_advancement_map(&matches, rounds_count);

    Bracket {
        matches,
        rounds: vec![],
        advancement_map,
    }
}

// Match execution
pub fn execute_match(match_data: &mut Match, kaiju_a: &Kaiju, kaiju_b: &Kaiju) -> MatchResult {
    let battle_result = auto_battle(kaiju_a, kaiju_b, &match_data.environment);

    let result = MatchResult {
        winner: battle_result.winner,
        loser: battle_result.loser,
        turns: battle_result.turns,
        final_hp: (battle_result.hp_a, battle_result.hp_b),
        was_upset: is_upset(kaiju_a, kaiju_b, &battle_result),
        upset_probability: calculate_upset_potential(kaiju_a, kaiju_b).upset_probability,
    };

    match_data.result = Some(result.clone());
    match_data.battle_log = Some(battle_result.log);

    result
}

// Tournament progression
pub fn advance_tournament(tournament: &mut Tournament) {
    let current_round = &mut tournament.bracket.as_mut().unwrap().rounds[tournament.current_round as usize];

    if current_round.status != RoundStatus::Completed {
        return;  // Can't advance until round complete
    }

    // Check if tournament complete
    if tournament.current_round == tournament.bracket.as_ref().unwrap().rounds.len() as u32 - 1 {
        finalize_tournament(tournament);
        return;
    }

    // Advance to next round
    tournament.current_round += 1;

    // Create next round matches based on advancement map
    let next_round_matches = create_next_round_matches(tournament);
    tournament.bracket.as_mut().unwrap().matches.extend(next_round_matches);
}

// Reward distribution
pub fn distribute_rewards(tournament: &Tournament, final_standings: Vec<(KaijuId, u32)>) {
    for (kaiju_id, placement) in final_standings {
        let reward = calculate_reward(tournament, placement);
        apply_reward(kaiju_id, reward);
    }

    // Special handling for lethal tournament deaths
    if tournament.tournament_type == TournamentType::LethalWinnerTakesAll {
        handle_lethal_deaths(tournament);
    }
}
```

---

### 11.3 UI State Management

```rust
pub enum TournamentScreenState {
    Lobby,                    // Browse available tournaments
    Registration(TournamentId), // Registering kaiju
    Confirmation(TournamentId, KaijuId), // Confirm lethal entry
    Waiting(TournamentId),    // Tournament not started yet
    Spectating(TournamentId, MatchId), // Watching match
    Results(TournamentId),    // Tournament complete
}

pub enum TournamentAction {
    BrowseTournaments,
    RegisterKaiju(TournamentId, KaijuId),
    ConfirmLethalEntry(TournamentId, KaijuId),
    WithdrawRegistration(TournamentId, KaijuId),
    SpectateMatch(TournamentId, MatchId),
    ViewBracket(TournamentId),
    ViewResults(TournamentId),
    ReturnToLobby,
}
```

---

## 12. Future Expansion Hooks

### 12.1 Player-Created Tournaments (v2)

**Concept**: Allow players to create custom tournaments with house rules.

**Features**:
- Custom entry fees (creator takes small cut)
- Custom restrictions (trait bans, stat caps)
- Private invitations
- Clan tournaments

**Safety**:
- Lethal tournaments require admin approval
- Anti-scam protections (escrow system)

---

### 12.2 Team Tournaments (v2)

**Concept**: 2v2 or 3v3 kaiju battles.

**Mechanics**:
- Turn order: Interleaved
- Targeting: Strategic choice
- Synergies: Traits that boost allies

**Use Case**: Social play, clan rivalries

---

### 12.3 Tournament Betting/Prediction Markets (v3)

**Concept**: Players predict outcomes, earn rewards for accuracy.

**Mechanics**:
- Bet on winner
- Bet on upset
- Bet on turn count
- In-game currency only (avoid gambling regulations)

**Why Later**: Requires robust anti-manipulation systems

---

### 12.4 Dynamic Environment Events (v2)

**Concept**: Environments change mid-battle.

**Examples**:
- Storm intensifies (electric boost increases)
- Lava erupts (random damage)
- Earthquake (speed debuff)

**Implementation**:
- Event triggers at specific turns or HP thresholds
- Adds unpredictability to high-level play

---

## 13. Testing & Validation Checklist

### 13.1 Unit Tests
- [ ] Seeding algorithm produces correct bracket pairings
- [ ] Upset detection identifies true upsets
- [ ] XP calculation matches formula
- [ ] Ranking updates follow Elo correctly
- [ ] Entry validation catches all error cases
- [ ] Breeding rights refund on death

### 13.2 Integration Tests
- [ ] Full 8-kaiju single elimination completes
- [ ] Lethal tournament death flow (off-chain + on-chain)
- [ ] Swiss system avoids rematches
- [ ] Environment selection rotates correctly
- [ ] Spectator view hides sensitive data

### 13.3 Edge Cases
- [ ] Odd number of participants (bye handling)
- [ ] Tournament with only 2 entrants
- [ ] All upsets scenario (seed 8 wins)
- [ ] Tie in Swiss system (tiebreaker logic)
- [ ] Registration closes exactly at deadline (race condition)
- [ ] Kaiju dies mid-registration (should cancel entry)

---

## 14. Performance Considerations

### 14.1 Battle Simulation
- All battles run server-side (not on-chain)
- Deterministic RNG (replay-able with seed)
- Batch processing: All round 1 matches simultaneously

### 14.2 Database Queries
- Index: `tournaments.status = 'RegistrationOpen'`
- Index: `kaiju.alive = true AND kaiju.owner = ?`
- Cache: Active tournament brackets (TTL: 5 minutes)

### 14.3 On-Chain Optimization
- **No per-match transactions**
- Death finalization batched (every 6 hours)
- Breeding rights invalidation off-chain, burn on-chain later

---

## Critical Files for Implementation

Based on this design, the following files are most critical for implementing the tournament system:

### H:\RustGames\kaiju_sim\src\data\tournament.rs
**Reason**: Core tournament data structures (Tournament, Match, Bracket, MatchResult). This defines the fundamental types for all tournament operations.

### H:\RustGames\kaiju_sim\src\engine\tournament_engine.rs
**Reason**: Tournament logic engine including bracket generation, match execution, seeding algorithms, and progression logic. This is the heart of the tournament system.

### H:\RustGames\kaiju_sim\src\engine\combat.rs
**Reason**: Auto-battle simulation that tournaments depend on. Must implement deterministic combat with environment effects, trait modifiers, and narrow randomness bands.

### H:\RustGames\kaiju_sim\src\state\tournament_state.rs
**Reason**: Tournament state management including registration tracking, active tournament status, and participant roster. Handles save/load persistence for ongoing tournaments.

### H:\RustGames\kaiju_sim\src\screens\tournament_screen.rs
**Reason**: Tournament UI including lobby, registration, bracket viewer, spectator mode, and results screen. Implements the complete player-facing tournament experience.

---

**End of Tournament System Design**

This document provides a complete foundation for implementing tournaments in the Kaiju Breeding Simulator. All systems are designed to align with the game's core philosophy: **risk is optional but rewarded, power is temporary, legacy is permanent**.
