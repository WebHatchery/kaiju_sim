# AGENTS.md

This file provides guidance to Codex when working in this repository.

This project uses the shared RustGames agent instructions in [`../AGENTS.md`](../AGENTS.md). Codex should read and apply that file when working here.

## Project Overview

**Kaiju Breeding Simulator** is a Rust/Macroquad strategic management and auto-battle game. Players breed, train, and fight persistent kaiju whose stats, traits, lineage, and documented history define their legacy.

The current client has a playable local MVP:
- start a new game and choose a starter
- train kaiju through targeted stat programs
- fight seeded AI arena battles
- breed two living kaiju into a new offspring
- save/load local progress
- view roster, kaiju details, battle results, event history, and local leaderboard

## Technology

- Engine: Macroquad plus `macroquad-toolkit`
- Language: Rust 2021
- Targets: native Windows and `wasm32-unknown-unknown`
- Data: JSON files under `assets/`
- Future services: `kaiju_server/` and `contracts/` are not required for the V1 local MVP.

## Core Documents

Keep documentation lean. The retained docs are:
- `kaiju_sim.md`: core game design document
- `IMPLEMENTATION_GUIDE.md`: high-level implementation roadmap
- `GAMEPLAY_WALKTHROUGH.md`: short player-facing flow
- `KAIJU_HISTORY_DESIGN.md`: V1 kaiju history/event-record design
- `CODE_STANDARDS.md`: Rust/Macroquad coding standards
- `MACROQUAD_TOOLKIT.md`: local UI toolkit reference
- `AGENTS.md`: working instructions for agents

Do not recreate deleted phase plans, external ownership specs, or deep subsystem specs unless the user explicitly asks for a new focused document.

## Commands

Native build:
```bash
cargo build --release
```

Native run:
```bash
cargo run
```

Tests:
```bash
cargo test
```

WebGL/WASM check or build:
```bash
cargo check --target wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Publish:
```powershell
.\publish.ps1
.\publish.ps1 -Production
.\publish.ps1 -WebGLOnly
.\publish.ps1 -DeployOnly
```

## Architecture

- `src/data/`: type definitions, serializable structures, and JSON loading
- `src/engine/`: stateless game logic such as breeding, combat, training helpers, research, and tournament simulation
- `src/state/`: mutable game state, save/load, player data, battle result state, and phase management
- `src/ui/`: shared UI colors, typography, spacing, assets, actions, and components
- `src/screens/`: screen-specific rendering and UI intent generation
- `assets/`: data and sprites

State mutation should happen through state methods or top-level action handling, not inside reusable UI components.

## Design Rules

- Keep gameplay constants and balance values in JSON under `assets/`.
- Keep the local MVP playable without the server.
- Use deterministic seeds for battles and breeding where practical.
- Do not hardcode new balance numbers in Rust if they belong in `assets/balance.json`.
- UI should return `Option<UiAction>` for user intent.
- Avoid ECS or broad architecture rewrites unless the project clearly needs them.
- Remove unused code instead of suppressing warnings.
- Preserve existing user changes in the worktree.

## Gameplay Notes

Breeding:
- generation is `max(parent_a.generation, parent_b.generation) + 1`
- stats blend from parents with generation power creep and narrow variance
- visible and hidden traits inherit probabilistically
- direct self-breeding and direct parent-child breeding are blocked

Combat:
- speed determines turn order
- damage uses attack, defense scaling, trait bonuses, environment multipliers, and narrow variance
- battle logs and seeds should allow replay/audit behavior

Training:
- training consumes gold, grants XP, and improves a targeted stat
- tuning belongs in `assets/balance.json`

History:
- each kaiju stores a chronological event log
- events should record creation, roster joins, training, battles, breeding, offspring, transfers, research, and death where applicable
- local V1 history is the source of truth for kaiju legacy
- future multiplayer can sync or compare these records, but should not be required for V1

## Testing Focus

Prioritize tests for:
- breeding validation and stat inheritance
- combat determinism and damage formulas
- save/load round trips including `Kaiju.history`
- local MVP action handling
- WebGL compile compatibility

Before finishing gameplay changes, run:
```bash
cargo fmt
cargo check
cargo test
```

For client changes intended for web, also run:
```bash
cargo check --target wasm32-unknown-unknown
```
