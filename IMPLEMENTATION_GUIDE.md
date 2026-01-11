# Kaiju Simulator Implementation Guide

**Version**: 2.0
**Engine**: Macroquad + macroquad-toolkit
**Platform**: WebGL (WASM) + Native Windows
**NFT Model**: Server-Custodial with Optional Blockchain Export
**Database**: MySQL 8.0+

This high-level guide breaks down the Kaiju Breeding Simulator into implementable phases. Each phase links to detailed specification documents that provide production-ready implementation details.

> [!IMPORTANT]
> **NFT Philosophy**: Server-custodial ownership with cryptographic proofs. Blockchain export is optional. All gameplay resolves off-chain with zero gas costs.

> [!NOTE]
> **Cost Model**: 99% of gameplay is free (server-side). Players only pay when exporting to blockchain ($5-10 per mint, user pays).

> [!TIP]
> **Documentation Structure**: This guide provides the high-level roadmap. For implementation details, see:
> - [DESIGN_SPECIFICATIONS_INDEX.md](DESIGN_SPECIFICATIONS_INDEX.md) - Master index for all 6 detailed specs (226 KB, 7605 lines)
> - [DATABASE_SCHEMA.md](DATABASE_SCHEMA.md) - Complete MySQL schema with production-ready SQL
> - [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md) - Rust transfer service with full code examples
> - [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md) - Cryptographic proof system with working implementation

---

## Architecture Overview

```mermaid
graph TD
    subgraph Blockchain["Blockchain Layer (Optional)"]
        NFT[Kaiju NFTs - ERC-721]
        BR[Breeding Rights NFTs]
        L2[L2 Contract - Arbitrum/Base]
    end

    subgraph Server["Server Layer (Primary)"]
        DB[(MySQL Database)]
        TRANS[Transfer Service]
        SIG[Signature System]
        API[REST API]
        AUTH[Authentication]
    end

    subgraph Core["Core Game Systems"]
        GS[GameState]
        KS[Kaiju System]
        GEN[Genetics Engine]
        COM[Combat Engine]
        TOUR[Tournament Engine]
        IMG[AI Image Generator]
    end

    subgraph Data["Data Layer"]
        TD[Trait Definitions JSON]
        BAL[Balance Config JSON]
        TRN[Tournament Config JSON]
        META[NFT Metadata]
    end

    subgraph UI["User Interface (Macroquad)"]
        MENU[Main Menu]
        LAB[Laboratory Screen]
        BREED[Breeding Screen]
        BATTLE[Battle Screen]
        RANK[Leaderboard Screen]
        LIN[Lineage Viewer]
    end

    Blockchain -.->|Mint Export| Server
    Server -.->|Deposit Import| Blockchain
    DB --> Server
    Server --> API
    API --> Core
    Data --> Core
    Core --> UI
    IMG --> META
```

### Data Flow

**Server-Custodial Model**:
1. All kaiju owned in MySQL database
2. Transfers are instant SQL updates (free, <50ms)
3. Cryptographic signatures prove authenticity
4. Optional blockchain export when users want external trading

**Cost Breakdown**:
- Breeding: $0 (database transaction)
- Trading: $0 (database transaction)
- Tournaments: $0 (database transaction)
- Blockchain Mint: $5-10 (user pays, optional)
- Blockchain Deposit: $0-2 (user pays gas only)

---

## Phase 0: Server Infrastructure & Database

**Goal**: Set up server-custodial NFT infrastructure before implementing game logic.

> [!IMPORTANT]
> **Implement this phase BEFORE Phase 1** if you want a fully functional ownership system from the start. Alternatively, start with Phase 1 for local-only gameplay and add server infrastructure later.

### 0.1 Database Setup

**Reference**: See [DATABASE_SCHEMA.md](DATABASE_SCHEMA.md) for complete schema.

- [ ] Install MySQL 14+
- [ ] Create `kaiju_game` database
- [ ] Run schema migrations:
  - Core tables (users, kaiju, ownership_history)
  - Breeding tables (breeding_rights, breeding_history)
  - Tournament tables (tournaments, tournament_entries, battle_logs)
  - Security tables (transfer_locks, server_signature_keys)
- [ ] Create indexes for performance
- [ ] Set up triggers (ownership logging, death refunds, state hash validation)
- [ ] Create views (leaderboards, hall of fame, lineage trees)
- [ ] Implement stored procedures (`transfer_kaiju()`, `breed_kaiju()`)

**Database Features**:
- Audit trails for all transfers
- Tamper detection via state hashing
- Replay protection via nonces
- Automatic refunds on kaiju death
- Business rule enforcement at DB level

### 0.2 Transfer System (Rust)

**Reference**: See [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md) for complete implementation.

**Dependencies** (`Cargo.toml`):
```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono", "json"] }
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
```

**Implementation Tasks**:
- [ ] Create `TransferService` struct
- [ ] Implement `execute_transfer()` with validation:
  - Ownership verification
  - Custody state check (server vs blockchain)
  - Death status check
  - Transfer lock check
  - Nonce increment for replay protection
- [ ] Create API handlers:
  - `POST /api/transfer` - Execute transfer
  - `GET /api/kaiju/:id/proof` - Get ownership proof
  - `GET /api/kaiju/:id/history` - Get transfer history
- [ ] Implement transaction management (ACID compliance)
- [ ] Add error handling and logging

**Transfer Features**:
- Instant (<50ms) zero-cost transfers
- Full transaction safety (rollback on error)
- Complete audit trail
- Signature-based authenticity

### 0.3 Cryptographic Signature System

**Reference**: See [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md) for complete implementation.

**Dependencies**:
```toml
[dependencies]
secp256k1 = { version = "0.28", features = ["rand-std", "serde"] }
sha2 = "0.10"
hex = "0.4"
```

**Implementation Tasks**:
- [ ] Generate server key pairs:
  - Ownership key (for transfer signatures)
  - Battle key (for combat results)
  - Mint key (for blockchain exports)
  - Admin key (for admin operations)
- [ ] Implement `SignatureKeyPair` struct
- [ ] Create `Signer` for generating signatures
- [ ] Create `Verifier` for validating signatures
- [ ] Implement state hash computation
- [ ] Create ownership proof generation
- [ ] Build public verification API:
  - `GET /api/verify/keys` - Get public keys
  - `POST /api/verify/ownership` - Verify proof
  - `GET /api/kaiju/:id/certificate` - Export certificate

**Signature Features**:
- ECDSA signatures (same as Ethereum)
- Tamper detection via SHA-256 hashing
- Public verifiability (anyone can check signatures)
- Offline verification tools

### 0.4 Server Architecture

**Project Structure**:
```
kaiju_server/
├── Cargo.toml
├── src/
│   ├── main.rs              # Server entry point
│   ├── types.rs             # Core types (TransferRequest, etc.)
│   ├── error.rs             # Error types
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── keygen.rs        # Key generation
│   │   ├── signer.rs        # Signature creation
│   │   ├── verifier.rs      # Signature verification
│   │   └── state_hash.rs    # State hashing
│   ├── transfer_service.rs  # Transfer logic
│   ├── api/
│   │   ├── mod.rs
│   │   ├── transfer.rs      # Transfer endpoints
│   │   └── verification.rs  # Verification endpoints
│   └── bin/
│       └── verify_proof.rs  # Standalone verification tool
├── migrations/              # SQL migration files
└── .env.example            # Environment variables template
```

**Environment Variables**:
```env
DATABASE_URL=mysqlql://kaiju_user:PASSWORD@localhost:5432/kaiju_game
SERVER_SECRET_KEY=<hex_encoded_secret_key>
SERVER_PORT=3000
RUST_LOG=info
```

### 0.5 Testing Infrastructure

- [ ] Unit tests for transfer validation
- [ ] Integration tests for full transfer flow
- [ ] Database transaction rollback tests
- [ ] Signature verification tests
- [ ] API endpoint tests
- [ ] Concurrency/race condition tests

### 0.6 Deployment Setup

- [ ] Docker containerization (optional)
- [ ] MySQL backup strategy
- [ ] Key rotation policy
- [ ] Rate limiting for API endpoints
- [ ] Monitoring and logging setup
- [ ] SSL/TLS certificates

**Cost Estimate**:
- Development: One-time setup (1-2 weeks)
- Hosting: $50-500/month depending on traffic
- Blockchain: $0 (until users request minting)

---

## Phase 1: Foundation & Data Models

**Goal**: Establish game project structure and core data types.

### 1.1 Project Setup
- [ ] Initialize Cargo project with workspace integration
- [ ] Configure `Cargo.toml` with macroquad + toolkit dependencies
- [ ] Create standard folder structure (`data/`, `engine/`, `state/`, `ui/`, `screens/`)
- [ ] Copy `publish.ps1` and `index.html` templates

### 1.2 Data Structures

#### Kaiju Entity (`data/kaiju.rs`)
```rust
pub struct Kaiju {
    // Immutable (stored on-chain as metadata hash)
    pub id: KaijuId,
    pub token_id: TokenId,           // NFT token ID
    pub name: String,
    pub generation: u32,
    pub created_at: u64,
    pub original_breeder: WalletAddress,
    pub parent_ids: Option<(TokenId, TokenId)>,  // Ancestry
    pub visual_seed: u64,
    pub genome_hash: String,         // Immutable genome fingerprint
    
    // Mutable (stored off-chain)
    pub stats: KaijuStats,
    pub traits: Vec<Trait>,
    pub hidden_traits: Vec<Trait>,
    pub experience: u32,
    pub alive: bool,
    
    // NFT Metadata
    pub image_uri: Option<String>,   // AI-generated portrait
    pub metadata_uri: String,        // IPFS/Arweave pointer
}

pub struct KaijuStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

/// Ancestry tree for lineage visualization
pub struct Lineage {
    pub kaiju_id: TokenId,
    pub parents: Option<Box<(Lineage, Lineage)>>,
    pub generation: u32,
    pub notable_ancestors: Vec<LineageHighlight>,
}

pub struct LineageHighlight {
    pub token_id: TokenId,
    pub name: String,
    pub achievement: String,  // e.g., "Champion Gen 4 Lethal"
}
```

#### Trait System (`data/traits.rs`)
```rust
pub struct Trait {
    pub id: TraitId,
    pub name: String,
    pub category: TraitCategory,
    pub power: i32,
    pub condition: Option<TraitCondition>,
    pub is_hidden: bool,
}

pub enum TraitCategory {
    Element,
    Modifier,
    Mutation,
    Synergy,
}

pub enum TraitInheritance {
    Dominant,
    Recessive,
    Polygenic,
    Conditional,
}
```

### 1.3 JSON Data Files

| File | Purpose |
|------|---------|
| `assets/traits.json` | Base trait definitions |
| `assets/balance.json` | Combat/breeding constants |
| `assets/tournaments.json` | Tournament configurations |
| `assets/genome_schema.json` | Genome encoding structure |
| `assets/image_prompts.json` | AI generation prompt templates |

---

## Phase 2: Genetics & Breeding Engine

**Goal**: Implement core breeding mechanics and trait inheritance.

> [!NOTE]
> **Detailed Specifications**: See the following documents for complete implementation details:
> - [TRAIT_SYSTEM_DESIGN.md](TRAIT_SYSTEM_DESIGN.md) - 50 traits, inheritance rules, mutation system
> - [GENOME_ENCODING_SPEC.md](GENOME_ENCODING_SPEC.md) - 256-bit genome structure, progressive decoding
> - [BREEDING_ALGORITHM_SPEC.md](BREEDING_ALGORITHM_SPEC.md) - Complete breeding logic with 10 scenarios

### 2.1 Trait System (`data/traits.rs`)

- [ ] Load 50 trait definitions from `assets/traits.json`
- [ ] Implement 4 inheritance types: Dominant, Recessive, Polygenic, Conditional
- [ ] Create synergy detection system (10 synergy traits)
- [ ] Enforce power budget constraints (max 150)
- [ ] Build trait-to-combat modifier mappings

**Reference**: [TRAIT_SYSTEM_DESIGN.md](TRAIT_SYSTEM_DESIGN.md)

### 2.2 Genome Encoding (`data/genome.rs`)

- [ ] Implement 256-bit (32 byte) genome structure
- [ ] Bit allocation: Header (16), Stats (64), Traits (80), Hidden (64), Visual (16), Checksum (16)
- [ ] CRC16 checksum validation
- [ ] Deterministic genome generation from parent genomes + seed
- [ ] Progressive decoding layers (Raw → Structural → Functional → Exact)

**Reference**: [GENOME_ENCODING_SPEC.md](GENOME_ENCODING_SPEC.md)

### 2.3 Breeding Service (`engine/breeding.rs`)

- [ ] Generation calculation: `max(parent_a.gen, parent_b.gen) + 1`
- [ ] Stat inheritance with power creep: `base * (1 + gen * 0.01) * variance(0.95-1.05)`
- [ ] Visible trait inheritance (45% base probability)
- [ ] Hidden trait inheritance (25% base probability)
- [ ] Mutation system (10% chance, 4 types: New Trait, Stat Boost, Synergy Unlock, Negative)
- [ ] Block direct parent-child breeding
- [ ] Visual seed generation from parent seeds
- [ ] Stat floors: HP 50, ATK 10, DEF 5, SPD 5
- [ ] Soft caps: scale with generation `base * (1 + gen * 0.02)`
- [ ] Hard caps: HP 2000, ATK/DEF/SPD 400

**Reference**: [BREEDING_ALGORITHM_SPEC.md](BREEDING_ALGORITHM_SPEC.md) - includes 10 complete breeding scenarios

### 2.4 Genetic Research (`engine/research.rs`)

- [ ] Research facility progression (5 levels)
- [ ] Layer 1 decoding: Research Lab Lv1-3 unlocks structural data
- [ ] Layer 2 decoding: Battle experience reveals functional understanding
- [ ] Layer 3 decoding: Voluntary disclosure or death reveals exact values
- [ ] Trait analysis (reveal hidden traits based on data)
- [ ] Battle log analysis for trait discovery
- [ ] Breeding outcome prediction

**Reference**: [GENOME_ENCODING_SPEC.md](GENOME_ENCODING_SPEC.md) Section 6-8

### 2.5 Data Validation
- [ ] Trait compatibility checks (incompatibility matrix)
- [ ] Lineage validation (no incest)
- [ ] Generation constraints for tournaments
- [ ] Power budget enforcement (total trait power ≤ 150)

---

## Phase 3: Combat Engine

**Goal**: Implement auto-battle simulation with predictable-but-probabilistic outcomes.

> [!NOTE]
> **Detailed Specification**: See [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) for complete damage formulas, environmental effects, and example battles with full turn-by-turn breakdowns.

### 3.1 Combat System (`engine/combat.rs`)

#### Damage Calculation (7-Step Formula)
```rust
fn calculate_damage(attacker: &Kaiju, defender: &Kaiju, env: &Environment) -> i32 {
    // Step 1: Base damage
    let base = attacker.stats.attack - (defender.stats.defense * 0.5);

    // Step 2: Apply minimum floor (5 damage minimum)
    let base = base.max(5.0);

    // Step 3: Trait modifiers (sum of active trait powers)
    let trait_bonus = sum_trait_powers(attacker, TraitCategory::Element);

    // Step 4: Environment effects (15-25% for matching traits)
    let env_mult = environment_multiplier(attacker, env);

    // Step 5: Synergy bonuses (if multiple traits interact)
    let synergy_mult = calculate_synergy_multiplier(attacker);

    // Step 6: Narrow randomness (±5% uniform distribution)
    let variance = rand_range(0.95, 1.05);

    // Step 7: Final damage
    ((base + trait_bonus) * env_mult * synergy_mult * variance) as i32
}
```

**Reference**: [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) Section 3-4

### 3.2 Environment System (`data/environments.rs`)

- [ ] Load 9 environment definitions from `assets/environments.json`
- [ ] Implement environmental multipliers (15-25% bonus for matching traits)
- [ ] Environment types: Neutral, Storm, Volcanic, Aquatic, Arctic, Forest, Desert, Urban, Cosmic
- [ ] Trait-to-environment matching logic

**Example Multipliers**:
- Storm: Electric traits +20%, Aquatic traits +15%
- Volcanic: Fire traits +25%, Ice traits -15%
- Aquatic: Water traits +20%, Electric traits -10%

**Reference**: [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) Section 5

### 3.3 Battle Flow

- [ ] Speed-based turn order (highest speed attacks first)
- [ ] Initiative calculation: `speed * variance(0.98-1.02)`
- [ ] Turn-by-turn damage resolution
- [ ] HP tracking and victory conditions (HP reaches 0)
- [ ] Battle log generation (JSON format for replay)
- [ ] Deterministic RNG using seeded ChaCha8Rng
- [ ] Environmental effect application per turn
- [ ] Maximum battle length: 100 turns (draw if exceeded)

**Reference**: [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) Section 6

### 3.4 Battle Reports

- [ ] Outcome summary (winner, loser, HP remaining, turns taken)
- [ ] Turn-by-turn log (attacker, defender, damage dealt, HP after)
- [ ] Partial trait analysis (hints, not full reveals)
- [ ] Suggested improvement areas (e.g., "Defender's speed advantage was decisive")
- [ ] Environmental impact summary (e.g., "Storm boosted Electric traits by 20%")

**Reference**: [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) Section 8

### 3.5 Deterministic Replay

- [ ] Store battle seed with results
- [ ] Implement replay function using stored seed
- [ ] Validate replay matches original outcome (for auditability)
- [ ] Public replay API for third-party verification

**Reference**: [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) Section 9

---

## Phase 4: Tournament System

**Goal**: Implement competitive tournament brackets and ranking.

> [!NOTE]
> **Detailed Specification**: See [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) for complete tournament mechanics, bracket systems, seeding algorithms, death flow, and an 8-kaiju tournament walkthrough.

### 4.1 Tournament Types (`data/tournaments.rs`)

- [ ] **Non-Lethal Ranked Tournaments**: Standard competitive play, XP rewards
- [ ] **Lethal Tournaments**: All losers die permanently, massive rewards
- [ ] **Generation-Restricted Tournaments**: Brackets by generation (Gen 0-2, Gen 3-5, Gen 6+)
- [ ] **Special Event Tournaments**: Limited-time with unique modifiers
- [ ] Load tournament configurations from `assets/tournament_configs.json`

**Tournament Configuration**:
```rust
pub struct TournamentConfig {
    pub name: String,
    pub tournament_type: TournamentType, // NonLethal, Lethal, GenRestricted
    pub bracket_system: BracketSystem,    // SingleElim, DoubleElim, Swiss
    pub min_generation: Option<u32>,
    pub max_generation: Option<u32>,
    pub entry_fee: u64,
    pub reward_structure: RewardStructure,
    pub environment: Environment,
}
```

**Reference**: [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) Section 2

### 4.2 Tournament Engine (`engine/tournament.rs`)

- [ ] Entry validation (eligibility, restrictions, death status)
- [ ] Seeding algorithm: `(rank*0.5) + (winrate*0.3) + (tourney_wins*0.15) + (gen*0.05)`
- [ ] Bracket generation (3 types):
  - Single Elimination: Standard knockout
  - Double Elimination: Losers bracket for second chance
  - Swiss: Round-robin style, everyone plays N rounds
- [ ] Match scheduling and resolution
- [ ] Reward distribution (XP, currency, titles)
- [ ] Death handling (lethal tournaments)
- [ ] Elo-style ranking updates (K-factor: 32)

**Reference**: [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) Section 3-7

### 4.3 Reward System

- [ ] XP rewards: 100 win, 30 loss, +50 upset bonus
- [ ] Currency rewards: Scale with tournament tier
- [ ] Title awards: "Champion of Gen 5", "Lethal Survivor"
- [ ] Hall of Fame entries for tournament winners
- [ ] Leaderboard ranking updates

**Reference**: [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) Section 8

### 4.4 Death System

- [ ] Permanent kaiju death in lethal tournaments (no resurrection)
- [ ] Death confirmation flow:
  1. Tournament resolves → Loser marked for death
  2. Server updates database (instant)
  3. Blockchain death transaction (if minted, async)
  4. Breeding rights auto-refund
  5. Hall of Fame entry created
- [ ] Breeding rights refund calculation
- [ ] Legacy record creation (full battle history preserved)
- [ ] Hall of Fame entry with death context
- [ ] Prevent all actions on dead kaiju (transfer, breeding, tournaments)
- [ ] "Tombstone, not trash" - NFT remains visible but unusable

**Reference**: [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) Section 9-10

### 4.5 Hall of Fame

- [ ] Public memorial for all dead kaiju
- [ ] Display: Name, generation, final stats, death tournament, owner
- [ ] Lineage tree showing descendants
- [ ] Battle history (wins, losses, notable upsets)
- [ ] Search/filter by generation, trait, death date
- [ ] "Most Glorious Deaths" rankings (died in finals, killed champion, etc.)

**Reference**: [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) Section 11

---

## Phase 5: State Management

**Goal**: Implement game state, persistence, and progression tracking.

### 5.1 Game State (`state/game_state.rs`)
```rust
pub struct GameState {
    pub phase: GamePhase,
    pub player: PlayerData,
    pub roster: Vec<Kaiju>,
    pub pending_breeding: Option<BreedingSession>,
    pub active_tournament: Option<TournamentState>,
}

pub enum GamePhase {
    Loading,
    MainMenu,
    Laboratory,
    Breeding,
    TournamentLobby,
    Battle,
    Results,
}
```

### 5.2 Persistence (`state/persistence.rs`)
- [ ] Save/load player progress
- [ ] Kaiju roster serialization
- [ ] Tournament history
- [ ] Breeding history

### 5.3 Leaderboard System
- [ ] Global ranking calculation
- [ ] NPC placeholder population
- [ ] Seasonal tracking (future)

---

## Phase 6: User Interface

**Goal**: Implement all game screens using macroquad-toolkit.

> [!NOTE]
> **Detailed Specification**: See [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) for complete screen designs with ASCII wireframes, component definitions, color palette, typography, keyboard shortcuts, and navigation flows.

### 6.1 Screen Hierarchy

| Screen | Purpose | Key Actions | Spec Section |
|--------|---------|-------------|--------------|
| **MainMenu** | Entry point | New Game, Continue, Options | Section 4.1 |
| **Laboratory** | Research hub | Analyze kaiju, view lineage, access facilities | Section 4.2 |
| **RosterView** | Kaiju management | Select, view details, compare stats | Section 4.3 |
| **KaijuDetailView** | Detailed inspect (Modal) | Full stats, traits, lineage tree, history | Section 4.4 |
| **BreedingScreen** | Breeding UI | Select parents, preview offspring, confirm breed | Section 4.5 |
| **TournamentLobby** | Tournament selection | Browse tournaments, view odds, enter | Section 4.6 |
| **BattleView** | Combat display | Watch auto-battle, view turn logs, replay | Section 4.7 |
| **ResultsScreen** | Post-match | View rewards, XP gained, ranking changes | Section 4.8 |
| **LeaderboardScreen** | Rankings | Global standings, Hall of Fame, history | Section 4.9 |
| **LineageViewer** | Family tree | Interactive lineage exploration, ancestor highlights | Section 4.10 |

**Reference**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 4

### 6.2 UI Components (`ui/components.rs`)

- [ ] **KaijuCard**: Compact kaiju display with thumbnail, name, gen, stats preview
- [ ] **StatBar**: Horizontal progress bar for HP/ATK/DEF/SPD
- [ ] **TraitBadge**: Pill-shaped trait indicator with icon and tooltip
- [ ] **LineageTree**: Recursive tree rendering for ancestry visualization
- [ ] **TournamentBracket**: SVG-style bracket lines with match results
- [ ] **BattleLog**: Scrollable turn-by-turn combat log with color coding
- [ ] **ProgressBar**: Generic progress indicator (XP, research, etc.)

**Component Specifications**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 5

**Styling**:
- Dark theme color palette (20 colors defined)
- Typography hierarchy (12px-60px, 7 levels)
- Consistent spacing scale (4px, 8px, 12px, 16px, 24px, 32px)

**Reference**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 7-8

### 6.3 Navigation Flow

```rust
pub enum GameScreen {
    MainMenu,
    Laboratory,
    RosterView,
    KaijuDetail(KaijuId),
    Breeding,
    TournamentLobby,
    Battle(BattleId),
    Results,
    Leaderboard,
    LineageViewer(KaijuId),
}
```

**State Machine**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 3

### 6.4 Action Pattern

```rust
pub enum UiAction {
    // Navigation
    GoToMenu,
    GoToLaboratory,
    GoToRoster,
    GoToKaijuDetail(KaijuId),
    GoToBreeding,
    GoToTournamentLobby,
    GoToLeaderboard,
    GoToLineageViewer(KaijuId),

    // Breeding
    SelectParent(KaijuId, ParentSlot),
    PreviewOffspring,
    ConfirmBreeding,
    CancelBreeding,

    // Tournament
    SelectTournament(TournamentId),
    EnterTournament(TournamentId),
    ViewTournamentBracket(TournamentId),

    // Battle
    WatchNextRound,
    SkipBattle,
    ReplayBattle(BattleId),

    // Research
    UpgradeResearchFacility,
    AnalyzeGenome(KaijuId),

    // Roster
    SortRosterBy(SortCriteria),
    FilterRosterBy(FilterCriteria),
}
```

**Complete UiAction Enum**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 9

### 6.5 Keyboard Shortcuts

- `Esc` - Back/Cancel
- `Tab` - Cycle focus
- `Space` - Confirm/Select
- `1-9` - Quick select (roster, breeding slots)
- `F1` - Help overlay
- `F5` - Refresh data

**Full Shortcut List**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 10

### 6.6 Accessibility

- [ ] Keyboard navigation for all interactions
- [ ] Screen reader support for critical info
- [ ] High contrast mode option
- [ ] Colorblind-friendly trait badges
- [ ] Scalable UI (support 1280x720 minimum)

**Reference**: [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) Section 11

---

## Phase 7: Visual & Audio Polish

**Goal**: Add visual generation, effects, and audio.

### 7.1 Visual Generation
- [ ] Seed-based kaiju appearance generation
- [ ] Trait-to-visual mapping (wings = wings appearance)
- [ ] Sprite/portrait rendering

### 7.2 Battle Visuals
- [ ] Turn animations
- [ ] Damage numbers
- [ ] Trait activation effects
- [ ] Environment backdrops

### 7.3 Audio (Future)
- [ ] Battle sound effects
- [ ] UI feedback sounds
- [ ] Ambient music

---

## Phase 8: Economy & Multiplayer Hooks

**Goal**: Prepare systems for future expansion.

### 8.1 Breeding Rights Economy
- [ ] Rights purchase/sale interface
- [ ] Rights tracking and validation
- [ ] Refund system on kaiju death

### 8.2 Multiplayer Preparation
- [ ] Kaiju export/import (portable data)
- [ ] Tournament result signatures
- [ ] Lineage verification

### 8.3 Extension Hooks
- [ ] Genetic exhaustion mechanics
- [ ] Lab specialization
- [ ] Wild tournaments
- [ ] Cataclysm events

---

## Phase 9: Optional Blockchain Export

**Goal**: Add optional blockchain minting for external trading and custody.

> [!IMPORTANT]
> **This phase is OPTIONAL**. The game is fully functional without blockchain. Only implement if players demand external trading or you need marketplace integration.

> [!NOTE]
> **Reference**: See [SERVER_CUSTODIAL_NFT_DESIGN.md](SERVER_CUSTODIAL_NFT_DESIGN.md) for complete architecture.

### 9.1 Server-Custodial Model (Phase 0-8)

**Default State** (Already implemented in Phase 0):
- All kaiju owned in MySQL
- Transfers are free and instant
- Cryptographic proofs provide authenticity
- No blockchain costs

**Three Ownership States**:
1. **Server-Owned** (99% of gameplay): Free transfers, breeding, tournaments
2. **Blockchain-Minted** (optional): User pays $5-10 to mint for external trading
3. **Deposited Back** (optional): User returns NFT to server for free gameplay

### 9.2 When to Implement Blockchain Export

Implement this phase if:
- You have 1000+ active players
- Players request external trading (OpenSea, Blur)
- You want marketplace integration
- You need investor/community credibility

**Cost Decision Matrix**:
- <10% of players minting → Stay server-custodial
- 10-50% minting → Hybrid model (current design)
- >50% minting → Consider full on-chain migration

### 9.3 Minimal NFT Contract (L2 Deployment)

**Deploy on**:
- Base (Coinbase L2) - Recommended for low gas
- Arbitrum One
- Polygon zkEVM

**Contract**: `contracts/KaijuNFT.sol`

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract KaijuNFT is ERC721, Ownable {
    struct KaijuMetadata {
        string metadataURI;        // IPFS hash
        string genomeHash;         // SHA-256 of genome
        uint256 parentTokenIdA;    // 0 if gen-0
        uint256 parentTokenIdB;    // 0 if gen-0
        uint256 mintedAt;
    }

    mapping(uint256 => KaijuMetadata) public kaijuData;
    mapping(uint256 => bool) public isDead;
    uint256 private _nextTokenId = 1;

    // Only server wallet can mint
    address public serverWallet;

    constructor() ERC721("Kaiju Breeding Simulator", "KAIJU") {
        serverWallet = msg.sender;
    }

    function mint(
        address to,
        string memory metadataURI,
        string memory genomeHash,
        uint256 parentTokenIdA,
        uint256 parentTokenIdB
    ) external onlyOwner returns (uint256) {
        uint256 tokenId = _nextTokenId++;
        _mint(to, tokenId);

        kaijuData[tokenId] = KaijuMetadata({
            metadataURI: metadataURI,
            genomeHash: genomeHash,
            parentTokenIdA: parentTokenIdA,
            parentTokenIdB: parentTokenIdB,
            mintedAt: block.timestamp
        });

        return tokenId;
    }

    function markDead(uint256 tokenId) external onlyOwner {
        require(_exists(tokenId), "Token does not exist");
        require(!isDead[tokenId], "Already dead");
        isDead[tokenId] = true;
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal override {
        require(!isDead[tokenId], "Cannot transfer dead kaiju");
        super._beforeTokenTransfer(from, to, tokenId);
    }
}
```

**Deployment Cost**: $50-200 (one-time, depending on L2 gas prices)

### 9.4 Minting Flow (User Pays)

**Server Implementation**: `src/blockchain/mint_service.rs`

```rust
pub struct MintService {
    contract: KaijuNFTContract,
    pool: PgPool,
}

impl MintService {
    pub async fn mint_to_blockchain(
        &self,
        kaiju_id: Uuid,
        user_wallet: Address,
    ) -> Result<TxHash, MintError> {
        // 1. Lock kaiju in DB
        self.lock_kaiju_for_mint(kaiju_id).await?;

        // 2. Generate metadata and upload to IPFS
        let metadata = self.generate_metadata(kaiju_id).await?;
        let metadata_uri = self.upload_to_ipfs(metadata).await?;

        // 3. Call contract (server wallet pays gas initially)
        let tx_hash = self.contract.mint(
            user_wallet,
            metadata_uri,
            genome_hash,
            parent_a_token_id,
            parent_b_token_id,
        ).await?;

        // 4. Wait for confirmation
        self.wait_for_confirmation(tx_hash).await?;

        // 5. Update DB: custody = "blockchain"
        self.update_custody_state(kaiju_id, "blockchain", token_id).await?;

        // 6. Charge user (credit card, crypto, in-game currency)
        self.charge_user(user_id, MINT_FEE).await?;

        Ok(tx_hash)
    }
}
```

**Pricing Model**:
- Gas cost: $0.10-1.00 (L2)
- Service fee: $5-10 (covers overhead + IPFS)
- Total: $5-10 per mint (user pays)

### 9.5 Deposit Flow (Return to Server)

**User Flow**:
1. User transfers NFT to server's custodial wallet
2. Server detects `Transfer` event
3. DB updated: custody = "server"
4. Kaiju now tradeable in-game for free

**Implementation**:
```rust
pub async fn deposit_from_blockchain(
    &self,
    token_id: u64,
    user_wallet: Address,
) -> Result<(), DepositError> {
    // 1. Verify on-chain ownership
    let current_owner = self.contract.owner_of(token_id).await?;
    if current_owner != user_wallet {
        return Err(DepositError::NotOwner);
    }

    // 2. User initiates transfer to server wallet
    // (Frontend guides user through this)

    // 3. Listen for Transfer event
    let event = self.wait_for_transfer_to_custodial_wallet(token_id).await?;

    // 4. Update DB
    let kaiju_id = self.get_kaiju_by_token_id(token_id).await?;
    self.update_custody_state(kaiju_id, "server", Some(token_id)).await?;

    Ok(())
}
```

### 9.6 Death Synchronization

**When kaiju dies in server**:
```rust
pub async fn finalize_death(
    &self,
    kaiju_id: Uuid,
    tournament_id: Uuid,
) -> Result<(), DeathError> {
    // 1. Mark dead in DB (instant)
    self.db.mark_kaiju_dead(kaiju_id, tournament_id).await?;

    // 2. If minted to blockchain, update contract (async)
    if let Some(token_id) = self.get_blockchain_token_id(kaiju_id).await? {
        self.contract.mark_dead(token_id).await?;
    }

    // 3. Refund breeding rights
    self.refund_unused_breeding_rights(kaiju_id).await?;

    // 4. Add to Hall of Fame
    self.hall_of_fame.add_fallen_kaiju(kaiju_id, tournament_id).await?;

    Ok(())
}
```

**Key Point**: Server state is always authoritative. Blockchain updates happen asynchronously.

### 9.7 Custodial Wallet Security

**Critical**: Your server wallet holds all deposited NFTs.

**Security Measures**:
- [ ] Multi-sig wallet (2-of-3 or 3-of-5 signatures)
- [ ] Cold storage for 90% of NFTs
- [ ] Hot wallet for active deposits (10%)
- [ ] 24-hour withdrawal delay for large transfers
- [ ] Regular smart contract audits
- [ ] Crypto insurance (Nexus Mutual, etc.)
- [ ] Backup keys in secure vault

**Trust Model**:
- Server is authoritative for gameplay
- Blockchain is authoritative for ownership
- Players can verify via cryptographic signatures (Phase 0)
- Disputes resolved via signed audit trail

### 9.8 Implementation Checklist

**Phase 9A: Contract Deployment**
- [ ] Deploy KaijuNFT contract to L2
- [ ] Verify contract on block explorer
- [ ] Set up multi-sig custodial wallet
- [ ] Test minting on testnet

**Phase 9B: Minting Service**
- [ ] Implement `MintService` in Rust
- [ ] Add IPFS/Arweave upload
- [ ] Create payment processing (Stripe/crypto)
- [ ] Add mint status tracking

**Phase 9C: Deposit Service**
- [ ] Implement blockchain event listener
- [ ] Add deposit confirmation flow
- [ ] Update UI for blockchain status

**Phase 9D: Death Synchronization**
- [ ] Link death system to contract
- [ ] Add async blockchain updates
- [ ] Handle blockchain failures gracefully

**Phase 9E: Marketplace Integration**
- [ ] Add OpenSea metadata API
- [ ] Support collection verification
- [ ] Add rarity/trait filters

### 9.9 Metadata Structure (IPFS)

**Immutable Metadata** (uploaded once at mint):
```json
{
  "name": "Volthor #1234",
  "description": "Generation 3 Electric/Aquatic hybrid kaiju",
  "image": "ipfs://Qm.../volthor.png",
  "external_url": "https://kaiju.game/kaiju/1234",
  "attributes": [
    {"trait_type": "Generation", "value": 3},
    {"trait_type": "Element", "value": "Electric"},
    {"trait_type": "Element", "value": "Aquatic"},
    {"trait_type": "Experience Level", "value": 12},
    {"trait_type": "Battles Won", "value": 45},
    {"trait_type": "Status", "value": "Alive"}
  ],
  "properties": {
    "genome_hash": "0xABCD1234...",
    "parent_a_token": 567,
    "parent_b_token": 890,
    "visual_seed": "0x9FA3_12C8"
  }
}
```

### 9.10 Cost Summary

| Phase | Implementation | Ongoing Cost |
|-------|---------------|-------------|
| **Phase 0-8 (Server-Only)** | 1-2 weeks dev | $50-500/month hosting |
| **Phase 9 (Blockchain)** | 1 week dev | $0 (users pay mint fees) |
| **Total Before Blockchain** | - | $0 blockchain costs |
| **Total With Blockchain** | - | $0 blockchain costs (user-funded) |

**Recommendation**: Launch with Phase 0-8 only. Add Phase 9 if players request it.

---

## Phase 10: AI Image Generation

**Goal**: Generate unique AI portraits for each kaiju based on genetics and ancestry.

### 10.1 Image Generation Pipeline

```mermaid
flowchart LR
    G[Genome Data] --> P[Prompt Builder]
    T[Visible Traits] --> P
    A[Ancestry Highlights] --> P
    P --> AI[AI Generator]
    AI --> IMG[Portrait Image]
    IMG --> IPFS[IPFS Upload]
    IPFS --> META[NFT Metadata]
```

### 10.2 Prompt Construction (`engine/image_gen.rs`)

- [ ] Build prompts from trait keywords
- [ ] Include ancestral visual themes (bloodline colors, forms)
- [ ] Add generation-appropriate style modifiers
- [ ] Support style tokens for consistency

#### Prompt Template
```json
{
  "base_prompt": "A monstrous kaiju creature, detailed portrait",
  "trait_mappings": {
    "Electric Breath": "crackling with blue electricity, lightning patterns",
    "Aqua Hide": "scales with ocean-blue shimmer, water droplets",
    "Wings": "large leathery wings spread behind"
  },
  "ancestry_modifiers": {
    "champion_bloodline": "golden aura, regal bearing",
    "mutation_heavy": "asymmetric features, chaotic patterns"
  },
  "style_suffix": "fantasy creature art, dramatic lighting, 512x512"
}
```

### 10.3 Image Generation Service

- [ ] Queue-based generation (async)
- [ ] Fallback placeholder during generation
- [ ] IPFS/Arweave upload on completion
- [ ] Metadata URI update

### 10.4 Ancestry Visualization

- [ ] Lineage tree renderer (3-4 generations deep)
- [ ] Champion ancestor badges
- [ ] Trait inheritance indicators
- [ ] Death markers for fallen ancestors

#### Lineage Card UI
```
┌─────────────────────────────────────┐
│ 🦖 STORMLING (Gen 5)               │
│ ┌───────────────────────────────┐  │
│ │   [AI Portrait]               │  │
│ │                               │  │
│ │   ⚡ Electric Breath          │  │
│ │   🌊 Aqua Hide                │  │
│ └───────────────────────────────┘  │
│                                     │
│ 📜 LINEAGE                          │
│ ├── Flossy (Gen 4) ⚡              │
│ │   └── Voltaire (Gen 3) 🏆        │
│ └── Reefmaw (Gen 4) 🌊             │
│     └── Tidecaller (Gen 3) 💀     │
└─────────────────────────────────────┘
```

### 10.5 Image Prompt Catalog (`assets/image_prompts.json`)

- [ ] Define all trait-to-visual mappings
- [ ] Dimension requirements (divisible by 16)
- [ ] Style consistency tokens
- [ ] Negative prompt exclusions

---

## Implementation Priority

```mermaid
gantt
    title Kaiju Simulator Development Phases
    dateFormat X
    axisFormat %s

    section Infrastructure
    Phase 0 - Server & DB      :p0, 0, 2

    section Foundation
    Phase 1 - Data Models      :p1, after p0, 1
    Phase 5 - State Management :p5, after p1, 1

    section Core Gameplay
    Phase 2 - Genetics         :p2, after p1, 2
    Phase 3 - Combat           :p3, after p2, 2
    Phase 4 - Tournaments      :p4, after p3, 1

    section Polish
    Phase 6 - UI               :p6, after p5, 3
    Phase 7 - Visuals          :p7, after p6, 2
    Phase 8 - Economy          :p8, after p7, 1

    section Optional
    Phase 9 - Blockchain       :p9, after p8, 1
    Phase 10 - AI Images       :p10, after p9, 2
```

### Recommended Order

#### Path A: Full Server Infrastructure (Recommended for Multiplayer)

1. **Phase 0** → Server Infrastructure & Database (enables online gameplay)
2. **Phase 1** → Foundation (game data models)
3. **Phase 5** → State Management (enables testing)
4. **Phase 2** → Genetics (core mechanic)
5. **Phase 3** → Combat (core mechanic)
6. **Phase 6** → UI (parallel with phases 2-3)
7. **Phase 4** → Tournaments (integrates breeding + combat)
8. **Phase 7** → Polish
9. **Phase 8** → Economy & expansion
10. **Phase 9** → Optional Blockchain Export (only if players demand it)
11. **Phase 10** → AI Image Generation

**Time to Playable**: 3-4 weeks
**Cost**: $50-500/month hosting, $0 blockchain

#### Path B: Local-Only First (Faster MVP)

1. **Phase 1** → Foundation (game data models)
2. **Phase 5** → State Management (local save/load)
3. **Phase 2** → Genetics (core mechanic)
4. **Phase 3** → Combat (core mechanic)
5. **Phase 6** → UI (parallel with phases 2-3)
6. **Phase 4** → Tournaments (integrates breeding + combat)
7. **Phase 0** → Add Server Infrastructure later (when ready for multiplayer)
8. **Phase 7** → Polish
9. **Phase 8** → Economy
10. **Phase 9** → Optional Blockchain Export
11. **Phase 10** → AI Image Generation

**Time to Playable**: 1-2 weeks
**Cost**: $0 (all local)

---

## Testing Strategy

### Unit Tests
- Breeding calculations (stat inheritance, trait probability)
- Combat damage formulas
- Tournament bracket generation
- Lineage validation
- Genome encoding/decoding
- Prompt construction from traits

### Integration Tests
- Full breeding cycle → battle simulation
- Tournament start to finish
- Save/load round-trip
- NFT metadata generation pipeline
- Image generation queue → IPFS upload

### Manual Testing
- UI navigation and responsiveness
- Battle log readability
- Leaderboard updates
- Lineage tree visualization
- AI portrait quality review

---

## Documentation Reference

### Core Design Documents

| Document | Purpose | Phase |
|----------|---------|-------|
| [kaiju_sim.md](kaiju_sim.md) | Complete Game Design Document - gameplay rules, mechanics, combat prototypes | All |
| [CLAUDE.md](CLAUDE.md) | Project overview and development guidelines for AI assistance | All |
| [DESIGN_SPECIFICATIONS_INDEX.md](DESIGN_SPECIFICATIONS_INDEX.md) | Master index for all design specifications with cross-references and implementation roadmap | All |

### Detailed System Specifications

| Document | Size | Purpose | Phase |
|----------|------|---------|-------|
| [TRAIT_SYSTEM_DESIGN.md](TRAIT_SYSTEM_DESIGN.md) | 30 KB | 50 traits, inheritance mechanics, mutation system, synergy detection | 2 |
| [GENOME_ENCODING_SPEC.md](GENOME_ENCODING_SPEC.md) | 23 KB | 256-bit genome structure, progressive decoding layers, research facilities | 2 |
| [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) | 31 KB | Damage formulas, environmental effects, deterministic auto-battle simulation | 3 |
| [BREEDING_ALGORITHM_SPEC.md](BREEDING_ALGORITHM_SPEC.md) | 35 KB | Stat inheritance, trait inheritance, mutation system, 10 detailed scenarios | 2 |
| [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) | 50 KB | Tournament types, bracket systems, death mechanics, Hall of Fame | 4 |
| [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) | 57 KB | 10 screen specifications, ASCII wireframes, component definitions, navigation flow | 6 |

**Total Specifications**: 226 KB, 7605 lines of detailed design documentation

### Server-Custodial NFT System

| Document | Purpose | Phase |
|----------|---------|-------|
| [SERVER_CUSTODIAL_NFT_DESIGN.md](SERVER_CUSTODIAL_NFT_DESIGN.md) | Server-custodial ownership model with optional blockchain export | 0, 9 |
| [DATABASE_SCHEMA.md](DATABASE_SCHEMA.md) | Complete MySQL schema with audit trails, triggers, and stored procedures | 0 |
| [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md) | Rust implementation for instant zero-cost transfers with transaction safety | 0 |
| [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md) | Cryptographic proof system for ownership verification without blockchain | 0 |

### Blockchain Integration (Optional)

| Document | Purpose | Phase |
|----------|---------|-------|
| [nft_design.md](nft_design.md) | Original NFT technical design (full on-chain model - reference only) | 9 |

### Development Standards

| Document | Purpose | Phase |
|----------|---------|-------|
| [CODE_STANDARDS.md](CODE_STANDARDS.md) | Rust coding standards and conventions for Macroquad projects | All |
| [GAME_DEVELOPMENT_GUIDE.md](GAME_DEVELOPMENT_GUIDE.md) | General Rust game development patterns and architecture | All |
| [MACROQUAD_TOOLKIT.md](MACROQUAD_TOOLKIT.md) | UI toolkit API reference for macroquad-toolkit | 6 |

### Quick Navigation

**Starting Development?**
1. Read [CLAUDE.md](CLAUDE.md) for project overview
2. Review [kaiju_sim.md](kaiju_sim.md) for game design
3. Check [DESIGN_SPECIFICATIONS_INDEX.md](DESIGN_SPECIFICATIONS_INDEX.md) for complete specification list
4. Choose implementation path (see "Implementation Priority" above)
5. Follow [CODE_STANDARDS.md](CODE_STANDARDS.md) while coding

**New to the Project?**
Start with these in order:
1. [DESIGN_SPECIFICATIONS_INDEX.md](DESIGN_SPECIFICATIONS_INDEX.md) - Overview of all specs
2. [TRAIT_SYSTEM_DESIGN.md](TRAIT_SYSTEM_DESIGN.md) - Foundation of genetics
3. [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) - See traits in action
4. [BREEDING_ALGORITHM_SPEC.md](BREEDING_ALGORITHM_SPEC.md) - Understand breeding
5. Other specs as needed for your implementation area

**Implementing Core Systems?**
- **Phase 2 (Genetics/Breeding)**:
  1. [TRAIT_SYSTEM_DESIGN.md](TRAIT_SYSTEM_DESIGN.md) - 50 traits, inheritance rules
  2. [GENOME_ENCODING_SPEC.md](GENOME_ENCODING_SPEC.md) - Binary genome structure
  3. [BREEDING_ALGORITHM_SPEC.md](BREEDING_ALGORITHM_SPEC.md) - Complete breeding logic
- **Phase 3 (Combat)**:
  1. [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) - Damage formulas, environments
  2. [TRAIT_SYSTEM_DESIGN.md](TRAIT_SYSTEM_DESIGN.md) - Trait combat interactions
- **Phase 4 (Tournaments)**:
  1. [TOURNAMENT_SYSTEM_DESIGN.md](TOURNAMENT_SYSTEM_DESIGN.md) - Brackets, death mechanics
  2. [COMBAT_SYSTEM_SPEC.md](COMBAT_SYSTEM_SPEC.md) - Battle simulation
- **Phase 6 (UI)**:
  1. [UI_UX_SPECIFICATION.md](UI_UX_SPECIFICATION.md) - All screen designs with wireframes

**Setting Up Server Infrastructure? (Phase 0)**
1. [DATABASE_SCHEMA.md](DATABASE_SCHEMA.md) - Set up MySQL
2. [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md) - Implement transfer service
3. [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md) - Add cryptographic proofs
4. [SERVER_CUSTODIAL_NFT_DESIGN.md](SERVER_CUSTODIAL_NFT_DESIGN.md) - Understand full architecture

**Adding Blockchain? (Phase 9)**
1. [SERVER_CUSTODIAL_NFT_DESIGN.md](SERVER_CUSTODIAL_NFT_DESIGN.md) - Review minting/deposit flows
2. [nft_design.md](nft_design.md) - Reference for metadata structure
3. [GENOME_ENCODING_SPEC.md](GENOME_ENCODING_SPEC.md) - On-chain genome encoding
4. Deploy minimal ERC-721 contract (see Phase 9.3)

---

## Versioning

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-09 | Initial high-level guide |
| 1.1 | 2026-01-09 | Added Phase 9 (NFT Layer) and Phase 10 (AI Image Generation) |
| 2.0 | 2026-01-10 | **Major Update**: Shifted to server-custodial NFT model<br>- Added Phase 0 (Server Infrastructure & Database)<br>- Complete MySQL schema with audit trails<br>- Rust transfer system for zero-cost instant transfers<br>- Cryptographic signature system for ownership proofs<br>- Rewrote Phase 9 (now optional blockchain export)<br>- Added 4 new technical documents (DATABASE_SCHEMA.md, SERVER_TRANSFER_SYSTEM.md, SIGNATURE_SYSTEM.md, SERVER_CUSTODIAL_NFT_DESIGN.md)<br>- Updated architecture diagram for server-first approach<br>- Added cost breakdowns and implementation paths<br>- Blockchain now optional (users pay only if minting) |

---

## Summary: Server-Custodial Approach

**Key Design Decision**:
- Primary: Server-custodial ownership (MySQL database)
- Secondary: Optional blockchain export for external trading
- Cost: $0 blockchain expenses until players request minting

**Benefits**:
- Zero gas costs for 99% of gameplay
- Instant transfers (<50ms)
- No wallet setup friction for new players
- Cryptographic proofs provide auditability
- Can add blockchain later if needed

**Trade-offs**:
- Requires server infrastructure ($50-500/month hosting)
- Custodial risk (server holds deposited NFTs)
- Trust dependency on server signatures
- Centralization (but with cryptographic auditability)

**Recommendation**: Start with Phase 0-8 (server-only). Add Phase 9 (blockchain) only if players demand external trading.

---

> **Note**: Each phase should be expanded into a detailed implementation plan before development begins. This guide provides the roadmap; individual phase documents (especially Phase 0 technical docs) contain step-by-step implementation details with production-ready code.
