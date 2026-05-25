# Implementation Guide

This is the high-level roadmap for Kaiju Breeding Simulator. Keep detailed behavior in code, tests, JSON data, or a focused design document only when it is actively useful.

## Current State

The Rust/Macroquad client has a playable local MVP:
- title screen, starter selection, save/load
- local roster and kaiju detail records
- targeted training
- seeded AI arena fights
- battle result logs
- local breeding
- local leaderboard
- persistent kaiju event history

The V1 gameplay loop must remain playable without running `kaiju_server`.

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
4. Battle result pays gold/XP and records a seeded battle event.
5. Player breeds two living kaiju to create offspring with inherited stats and traits.
6. Parent and offspring histories record the breeding event.
7. Player repeats the loop to build stronger documented bloodlines.

## Data-Driven Rules

Put balance values in `assets/balance.json`.

Current MVP tuning includes:
- training cost, XP, and stat gain range
- arena entry fee and battle rewards
- breeding cost
- breeding inheritance and mutation rates
- combat damage variance, defense scaling, and max turns

Do not introduce Rust-side magic numbers for gameplay tuning when a JSON field is appropriate.

## Kaiju History

Each `Kaiju` stores a local `history: Vec<KaijuEvent>`.

Record events for:
- creation and starter selection
- roster joins
- training outcomes
- arena battles and battle seeds
- breeding as parent
- hatching as offspring
- research discoveries
- death, if lethal systems are enabled later

Events should be concise, serializable, and useful in the kaiju detail screen. Treat history as gameplay memory, not as a networking or ownership layer.

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
   - save/load round trip preserving event history

3. Expand battle presentation
   - animated turn playback
   - clearer trait/environment callouts
   - combat log export or copy support if needed

4. Expand breeding presentation
   - predicted stat ranges before breeding
   - inherited trait summary after hatching
   - lineage display from existing parent IDs

5. Future multiplayer V2
   - real-player arena comparisons
   - opt-in breeding with other players' kaiju
   - optional account/sync services
   - no V2 service should block V1 local play

## Testing Focus

Unit tests:
- breeding validation and inheritance
- combat determinism
- damage formula boundaries
- training reward application
- event history creation
- phase/action handling where practical

Integration tests:
- create game, train, fight, breed
- save/load after each major action
- verify kaiju history after each major action
- WebAssembly compile check

Manual checks:
- native launch
- starter selection flow
- training screen usability
- arena fight from each starter
- breeding two starters into offspring
- kaiju detail history display
- leaderboard update after wins

## Retained References

- `kaiju_sim.md`: game design
- `GAMEPLAY_WALKTHROUGH.md`: player-facing flow
- `KAIJU_HISTORY_DESIGN.md`: history/event record design
- `CODE_STANDARDS.md`: Rust/Macroquad standards
- `MACROQUAD_TOOLKIT.md`: UI toolkit reference
