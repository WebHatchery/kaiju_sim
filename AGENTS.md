# AGENTS.md

This file provides guidance to Codex when working in this repository.

## Project Overview

**Kaiju Breeding Simulator** is a Rust/Macroquad strategic management and auto-battle game. Players breed, train, and fight persistent kaiju whose stats, traits, lineage, and battle history define their legacy.

The current client has a playable local MVP:
- start a new game and choose a starter
- train kaiju through targeted stat programs
- fight seeded AI arena battles
- breed two living kaiju into a new offspring
- save/load local progress
- view roster, kaiju details, battle results, and local leaderboard

## Technology

- Engine: Macroquad plus `macroquad-toolkit`
- Language: Rust 2021
- Targets: native Windows and `wasm32-unknown-unknown`
- Data: JSON files under `assets/`
- Future services: `kaiju_server/` and `contracts/` exist for server/NFT work, but the MVP core loop is local and must not require the server.

## Core Documents

Keep documentation lean. The retained docs are:
- `kaiju_sim.md`: core game design document
- `IMPLEMENTATION_GUIDE.md`: high-level implementation roadmap
- `GAMEPLAY_WALKTHROUGH.md`: short player-facing flow
- `CODE_STANDARDS.md`: Rust/Macroquad coding standards
- `MACROQUAD_TOOLKIT.md`: local UI toolkit reference
- `nft_design.md`: future NFT ownership/provenance design
- `AGENTS.md`: working instructions for agents

Do not recreate deleted phase plans or deep subsystem specs unless the user explicitly asks for a new focused document.

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

NFT philosophy:
- NFT ownership/provenance is future-facing and should not drive local MVP gameplay logic
- gameplay remains off-chain; blockchain state is only for ownership/provenance/death status when that layer is implemented

## Testing Focus

Prioritize tests for:
- breeding validation and stat inheritance
- combat determinism and damage formulas
- save/load round trips
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
