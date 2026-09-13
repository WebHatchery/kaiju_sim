# TODO — Kaiju Breeding Simulator

## Standards and architecture

- Migrate `src/**/tests.rs` and `kaiju_server/src/**/tests.rs` into each crate's `tests/` directory; remove source-side test modules. Expose game logic through `src/lib.rs` and consume it from `main.rs`; migrate legacy `mod.rs` roots to named files when restructuring (§2.3, §11.4).
- Review migrated suites by feature: consolidate related cases toward five without losing regressions, or document why distinct coverage needs more (combat currently has seven). Correct the obsolete “non-test lines” comment in `tests/code_standards.rs`; retain both source gates with empty exceptions (§2.2, §11.3).
- Remove blanket `allow(unused)` from `src/main.rs` and `kaiju_server/src/lib.rs`; delete unused code and resolve warnings, documenting any narrowly justified Clippy allowances (§1.4, §10.2).
- Split the oversized `main` and `breed_selected_kaiju` functions in `src/main.rs` into cohesive coordination/action helpers of at most 100 lines; keep every Rust file within 800 total lines (§4.1).
- Move breeding parent selection and projection rules out of `src/screens/breeding.rs`: return `UiAction` intents, let the dispatcher own selection, and compute projections in the engine (§5.1, §7.1).
- Move hardcoded starter definitions, opponent tuning, server breeding defaults, and player-facing Rust strings into typed JSON under `assets/`, loaded through toolkit APIs (§5.3).
- Extend `src/data/loader.rs` semantic validation to cover tournament IDs/types/references, required stat keys and cap ordering, probability bounds, training ranges, and nonnegative costs/rewards; add public-API tests for valid data and representative invalid configurations (§5.3, §11).
- After test migration, add isolated save/load round-trip coverage for training, bred rosters, battle rewards, and event history, plus corrupt-save handling; the current persistence suite only checks the save path (§6, §11).

## Player controls and reliability

- Add visible paging or touch scrolling to roster, breeding candidates, and arena selection so every eligible kaiju remains reachable; replace fixed-width/minimum-size layouts that overflow smaller browser windows (§7.5).
- Replace ordinary press-triggered buttons/cards in `src/ui/shell.rs` and active screens with shared toolkit release interactions, preserving custom appearance and allowing cancelled taps (§7.4).
- Remove the production F5 save-deletion shortcut or provide a clearly labeled, confirmed reset control. Correct `game_page.json`'s false “Debug save snapshot” description and name visible touch equivalents for shortcuts (§7.5).
- Show recoverable startup/load/save errors in the UI: replace the startup early return and console-only Continue failure, and handle discarded `force_save` results in `src/main.rs` with visible feedback and retry controls (§6, §7.5).
- Explain the awarded companion breeder before starter confirmation, and add first-session prompts naming the visible controls for training, fighting, breeding, and saving (§7.5).

## Gameplay and records

- Add JSON-configured training fatigue or stamina and enforce stat caps so repeated training cannot grow stats without bound.
- Replace breeding preview averages and heuristic mutation labels with engine-derived offspring stat ranges, mutation probabilities, and the actual incubation cost before confirmation.
- Record which traits came from each parent versus mutation at hatching, and display that provenance in the offspring record.
- Add persistent rival progression and seasonal arena rules/rewards to replace the static event presentation.
- Connect the recorded combat turns to animated kaiju playback with visible pause, resume, and skip controls.

## Publishing and server

- Update `README.md` validation instructions to require parameterless `.\publish.ps1` after meaningful game changes; consolidate `screenshots/latest/` captures into `docs/verification/`, replace duplicate states, and update image links (§8.3, §12).
- Investigate the custom storage bridge using `STORAGE_BRIDGE_TODO.md`; verify current WASM loading, save/load, audio, and networking before removing the workaround and adopting the shared bridge.
- Implement mutation generation in `kaiju_server/src/breeding_service.rs` and persist/retrieve the breeding log event ID currently returned as `None` in `kaiju_server/src/api/breeding.rs`.
