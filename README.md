# Kaiju Breeding Simulator

Kaiju Breeding Simulator is a Rust/Macroquad monster breeding and auto-battler about training kaiju, fighting seeded arena battles, breeding new generations, and preserving each kaiju's event history.

## Current Scope

- local starter selection
- training
- AI arena battles
- breeding
- save/load
- kaiju detail records
- local leaderboard

V1 is offline-first and does not require a server.

## Screenshots

![Fresh UI screenshot contact sheet](screenshots/latest/_contact_sheet.png)

- [Title](screenshots/latest/01_title.png)
- [Starter selection](screenshots/latest/02_starter_selection.png)
- [Laboratory](screenshots/latest/03_laboratory.png)
- [Kaiju record](screenshots/latest/04_kaiju_record.png)
- [Roster](screenshots/latest/05_roster.png)
- [Training](screenshots/latest/06_training.png)
- [Arena](screenshots/latest/07_arena.png)
- [Battle results](screenshots/latest/08_battle_results.png)
- [Breeding](screenshots/latest/09_breeding.png)
- [Records](screenshots/latest/10_records.png)
- [Settings](screenshots/latest/11_settings.png)

## Run

```bash
cargo run
```

## Verify

```bash
cargo fmt
cargo check
cargo test
```

For web compatibility:
```bash
cargo check --target wasm32-unknown-unknown
```

## Architecture

- `src/main.rs` — Macroquad entry point, phase routing, top-level UI action handling
- `src/data/` — serializable types, static data loaders, environments, traits, kaiju models
- `src/engine/` — stateless mechanics: combat, breeding, research, tournaments, animations
- `src/state/` — mutable game state, persistence, player data, phases, battle results
- `src/ui/` — shared rendering helpers, colors, layout constants, assets, action definitions
- `src/screens/` — screen-specific rendering
- `assets/` — balance, trait, and tournament JSON plus sprites

Gameplay tuning belongs in `assets/balance.json`, never as Rust-side magic numbers.

## Art Direction

Dark sci-fi mood, restrained blue/cyan palette, pixel typography, generous spacing
over hard borders. The player fantasy is running a classified kaiju evolution
facility, so effects (ambient grid glow, scanlines, status pulses) stay subtle and
readable rather than neon or free-to-play.

## Core Docs

- `kaiju_sim.md` — game design document
- `KAIJU_HISTORY_DESIGN.md` — kaiju event-history/record design
- `TODO.md` — open work
- `CODE_STANDARDS.md`, `MACROQUAD_TOOLKIT.md`, `AGENTS.md` — shared workspace standards

