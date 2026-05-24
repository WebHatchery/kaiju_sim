# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

## Project Overview

**Kaiju Breeding Simulator** is a strategic management and auto-battle game where players breed, train, and compete with unique kaiju in a competitive global ecosystem. This repository currently contains comprehensive design documents and implementation guides - the actual Rust implementation has not yet been created.

**Core Concept**: Players act as private kaiju breeders, collecting and breeding kaiju to compete in tournaments. Each kaiju is unique and persistent, with a complete history of battles, breeding, and ownership. Power is temporary, but legacy is permanent.

**Technology Stack**:
- **Engine**: Macroquad (rendering) + macroquad-toolkit (UI utilities)
- **Language**: Rust (Edition 2021)
- **Platform**: WebGL (WASM) + Native Windows
- **Blockchain**: ERC-721 (Ethereum L2 / Sidechain) for NFT ownership
- **Data**: JSON files for game configuration and balance

## Key Design Principles

### NFT Philosophy
The NFT represents **identity and provenance, NOT gameplay logic**. All gameplay resolves off-chain; blockchain is authoritative only for ownership and death status. This is an intentional "web2.5 model" to avoid on-chain bloat and high gas costs.

### Architecture Split
- **On-Chain (Immutable)**: Token ID, owner, metadata hash, death flag, parent token IDs
- **Off-Chain (Game Server)**: Full kaiju state, stats, traits, experience, tournament history, battle logs

### Core Gameplay Systems
1. **Genetics & Breeding**: Probabilistic trait inheritance with visible and hidden traits, mutations, and power creep management
2. **Combat**: Fully simulated auto-battles with narrow randomness (±5%), predictable with full knowledge
3. **Death & Legacy**: Permanent death in lethal tournaments, public history that outlives the kaiju
4. **Research & Information**: Progressive decoding of genetic data through facility upgrades and battle experience
5. **Breeding Rights Economy**: Separate NFTs for breeding rights, with refunds on parent death

## Current State

This repository contains **design documents only**. No Rust source code exists yet. The implementation follows a phased approach outlined in `IMPLEMENTATION_GUIDE.md`.

### Key Documents
- **kaiju_sim.md**: Complete game design document (GDD) with rules, mechanics, and combat prototypes
- **IMPLEMENTATION_GUIDE.md**: 10-phase implementation roadmap with technical specifications
- **nft_design.md**: NFT technical implementation with on-chain/off-chain split, genome encoding, and security
- **CODE_STANDARDS.md**: Rust coding standards for Macroquad projects
- **GAME_DEVELOPMENT_GUIDE.md**: General Rust game development patterns and architecture
- **MACROQUAD_TOOLKIT.md**: UI toolkit API reference

## Build Commands (When Implemented)

### Windows Build
```bash
cargo build --release
```

### WebGL/WASM Build
```bash
cargo build --release --target wasm32-unknown-unknown
```

### Deploy to Preview/Production
```powershell
# Deploy to preview server (default)
.\publish.ps1

# Deploy to production
.\publish.ps1 -Production

# Build only WebGL
.\publish.ps1 -WebGLOnly

# Skip build, deploy existing
.\publish.ps1 -DeployOnly
```

The `publish.ps1` script handles both Windows and WebGL builds, packages assets, and deploys to local preview or production directories.

## Architecture Guidelines

### Project Structure (To Be Created)
```
kaiju_sim/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, game loop, phase transitions
│   ├── data/                # Data structures and JSON loading
│   │   ├── kaiju.rs         # Kaiju entity, stats, traits
│   │   ├── traits.rs        # Trait system and inheritance
│   │   └── tournaments.rs   # Tournament configurations
│   ├── engine/              # Stateless game logic services
│   │   ├── breeding.rs      # Genetics and breeding mechanics
│   │   ├── combat.rs        # Auto-battle simulation
│   │   ├── tournament.rs    # Tournament brackets and resolution
│   │   └── research.rs      # Genetic analysis and trait discovery
│   ├── state/               # Game state management
│   │   ├── game_state.rs    # Current game state and phases
│   │   └── persistence.rs   # Save/load system
│   ├── ui/                  # UI components (macroquad-toolkit)
│   │   ├── core.rs          # Base UI utilities and styling
│   │   └── components.rs    # Kaiju cards, stat bars, trait badges
│   └── screens/             # Screen-specific rendering
├── assets/                  # JSON data files
│   ├── traits.json          # Trait definitions
│   ├── balance.json         # Combat/breeding constants
│   └── tournaments.json     # Tournament configurations
└── index.html               # WebGL host page
```

### Module Responsibilities
- **data/**: Type definitions, constants, JSON loading - no game logic
- **engine/**: Stateless services that receive state and return results - never mutate state directly
- **state/**: Owns game state, mutations happen only here via clearly defined actions
- **ui/**: Reads state and returns action intents - contains no game logic
- **screens/**: Phase-specific rendering coordinated by main.rs

### State Machine Pattern
The game uses explicit phase transitions:
```rust
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

## Core Mechanics Implementation Notes

### Breeding System
- Generation = max(parent_a.gen, parent_b.gen) + 1
- Stats blend with power creep: `base * (1 + gen * 0.01) * variance(0.95-1.05)`
- Trait inheritance: 45% for visible traits, 25% for hidden traits
- 10% mutation chance per breeding
- Block direct parent-child breeding

### Combat System
- Speed determines turn order
- Damage formula: `(attack - defense * 0.5) + trait_bonuses * env_multiplier * variance(0.95-1.05)`
- Minimum 5 damage per hit
- Environment effects modify trait effectiveness (e.g., storm boosts electric traits by 15%)
- Narrow randomness ensures predictability with full knowledge

### Genome Encoding Strategy
Public genome data with **layered information architecture**:
- **Layer 0**: Raw genome blob (public, encrypted structure, meaningless alone)
- **Layer 1**: Structural decoding (unlocked by facility upgrades)
- **Layer 2**: Functional understanding (gained through battles and breeding)
- **Layer 3**: Exact knowledge (voluntary revelation, strategically risky)

Key insight: Data is public, but *meaning is contextual and expensive to interpret*.

## Coding Standards

### Data-Driven Design
All game constants and balance values MUST be in JSON files under `assets/`. Never hardcode magic numbers in Rust code.

### No Unused Code
- Remove unused variables, fields, and functions immediately
- Never suppress unused warnings with `_` prefixes on struct fields
- If a field is unused, delete it entirely

### Function Guidelines
- Target: 20-50 lines per function
- Hard limit: 100 lines
- Use ≤3 parameters; pass `&GameState` or config structs for complex data
- Return `Option<T>` for missing values, not panics

### UI Action Pattern
UI components return `Option<UiAction>` to signal user intent:
```rust
pub enum UiAction {
    SelectParent(KaijuId, ParentSlot),
    ConfirmBreeding,
    EnterTournament(TournamentId),
    // ...
}
```

### Macroquad-Toolkit Usage
Import via `use macroquad_toolkit::prelude::*;` for access to:
- `button()` - Standard clickable button (fires on release)
- `button_on_press()` - Instant feedback button (fires on mouse down)
- `panel()` - Panel drawing with optional title
- `progress_bar()` - Progress indicators
- `colors::dark::*` - Consistent dark theme colors

## Implementation Priority

Follow the phased approach in `IMPLEMENTATION_GUIDE.md`:

1. **Phase 1**: Foundation & Data Models (required first)
2. **Phase 2**: Genetics & Breeding Engine
3. **Phase 3**: Combat Engine
4. **Phase 4**: Tournament System
5. **Phase 5**: State Management
6. **Phase 6**: User Interface
7. **Phase 7**: Visual & Audio Polish
8. **Phase 8**: Economy & Multiplayer Hooks
9. **Phase 9**: NFT Layer (blockchain integration)
10. **Phase 10**: AI Image Generation (kaiju portraits)

Each phase should be expanded into a detailed implementation plan before development begins.

## Important Design Constraints

### Avoid Over-Engineering
- No ECS overengineering
- No custom editor tooling initially
- Keep solutions simple and focused
- Only add features directly requested or clearly necessary

### Death Rules
- Death only occurs in lethal tournaments
- Death is permanent and irreversible
- All unused breeding rights are refunded
- Dead kaiju become "tombstones, not trash" - visible in Hall of Fame but unusable

### Anti-Exploit Measures
- All battles use deterministic seeds stored in public logs
- Anyone can replay fights locally for auditability
- Game server is authoritative for simulation, blockchain for ownership
- No upgradeable contracts for core NFT functionality

## Testing Focus

### Unit Tests
- Breeding calculations (stat inheritance, trait probability)
- Combat damage formulas
- Tournament bracket generation
- Lineage validation (no incest)
- Genome encoding/decoding

### Integration Tests
- Full breeding cycle → battle simulation
- Tournament start to finish
- Save/load round-trip
- NFT metadata generation pipeline

### Manual Testing
- UI navigation and responsiveness
- Battle log readability
- Leaderboard updates
- Lineage tree visualization

## When Starting Implementation

1. Initialize Cargo project with workspace integration
2. Set up folder structure following CODE_STANDARDS.md
3. Create JSON data files in `assets/` for traits, balance, and tournaments
4. Implement Phase 1 data structures (Kaiju, Trait, Stats)
5. Build genetics engine before combat system
6. Implement UI in parallel with core mechanics
7. Add NFT layer after core gameplay is stable
8. Integrate AI image generation last

Remember: This is a **systems-driven design**, not spectacle-driven. Public history creates drama, risk is optional but rewarded, and legacy is permanent.
