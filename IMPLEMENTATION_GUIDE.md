# Implementation Guide

This guide is the high-level roadmap for Kaiju Breeding Simulator. It intentionally replaces the old phase-by-phase spec stack; keep detailed design in code, tests, JSON data, or a new focused document only when it is actively useful.

## Current State

The Rust/Macroquad client has a playable local MVP:
- starter selection
- local roster and kaiju details
- targeted training
- seeded AI arena fights
- battle result logs
- local breeding
- local leaderboard
- save/load

The server and NFT layers still exist in the repository, but the MVP gameplay loop must remain playable without running `kaiju_server`.

## Build And Verify

Native:
```bash
cargo run
cargo test
```

Web target:
```bash
cargo check --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Formatting:
```bash
cargo fmt
```

## Active Architecture

- `src/main.rs`: Macroquad entry point, phase routing, and top-level UI action handling
- `src/data/`: serializable types, static game data loaders, environments, traits, kaiju models
- `src/engine/`: stateless mechanics such as combat, breeding, research, tournaments, and local MVP helpers
- `src/state/`: mutable game state, persistence, player data, game phases, battle result state
- `src/ui/`: shared rendering helpers, colors, layout constants, assets, and action definitions
- `src/screens/`: screen-specific UI rendering
- `assets/`: balance JSON, trait JSON, tournament JSON, and sprites

## MVP Gameplay Loop

1. Player starts a local game and chooses a starter.
2. Player trains kaiju by spending gold for stat gains and XP.
3. Player enters an arena fight against a generated AI opponent.
4. Battle result pays gold/XP and records a replayable seeded log.
5. Player breeds two living kaiju to create offspring with inherited stats and traits.
6. Player repeats the loop to build stronger bloodlines.

## Data-Driven Rules

Put balance values in `assets/balance.json`.

Current MVP tuning includes:
- training cost, XP, and stat gain range
- arena entry fee and battle rewards
- breeding cost
- breeding inheritance and mutation rates
- combat damage variance, defense scaling, and max turns

Do not introduce Rust-side magic numbers for gameplay tuning when a JSON field is appropriate.

## Next Implementation Priorities

1. Improve local MVP UX
   - clearer selected parent state
   - better roster filtering
   - visible training affordability states
   - battle result navigation polish

2. Add stronger persistence tests
   - save/load round trip for trained kaiju
   - save/load round trip after breeding
   - save/load round trip after battle rewards

3. Expand battle presentation
   - animated turn playback
   - clearer trait/environment callouts
   - combat log export or copy support if needed

4. Expand breeding presentation
   - predicted stat ranges before breeding
   - inherited trait summary after hatching
   - lineage display from existing parent IDs

5. Reintroduce server features selectively
   - server auth/sync should be optional
   - server outages must not block local play
   - server APIs should mirror local engine behavior where possible

6. Future NFT integration
   - follow `nft_design.md`
   - keep gameplay off-chain
   - NFT state should represent ownership/provenance, not combat logic

## Testing Focus

Unit tests:
- breeding validation and inheritance
- combat determinism
- damage formula boundaries
- training reward application
- phase/action handling where practical

Integration tests:
- create game, train, fight, breed
- save/load after each major action
- WebAssembly compile check

Manual checks:
- native launch
- starter selection flow
- training screen usability
- arena fight from each starter
- breeding two starters into offspring
- leaderboard update after wins

## Retained References

- `kaiju_sim.md`: game design
- `GAMEPLAY_WALKTHROUGH.md`: player-facing flow
- `CODE_STANDARDS.md`: Rust/Macroquad standards
- `MACROQUAD_TOOLKIT.md`: UI toolkit reference
- `nft_design.md`: future ownership/provenance layer
