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

## Core Docs

- `kaiju_sim.md`
- `IMPLEMENTATION_GUIDE.md`
- `GAMEPLAY_WALKTHROUGH.md`
- `KAIJU_HISTORY_DESIGN.md`
- `CODE_STANDARDS.md`
- `MACROQUAD_TOOLKIT.md`
- `AGENTS.md`
