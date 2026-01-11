# Kaiju Simulator: Feature Testing Guide

This guide walks you through the key features of the application to verify functionality across all implemented phases (0-10).

## 1. Setup & initialization
- **Ensure Local Services**:
  - Run ComfyUI locally on port 8188 for image generation (`python main.py --listen`).
  - (Optional) Run `kaiju_server` if testing multiplayer/economy features.
- **Launch Client**: Run `cargo run --bin kaiju_sim`.

## 2. Main Menu
- **New Game**: Should initialize a new player profile with starting Gold (1000) and specific starter Kaiju.
- **Continue**: Should load the last auto-saved state (if exists).
- **Settings**: Check volume controls and UI theme preferences.

## 3. Laboratory (Hub)
- **Navigation**: Verify clicking tiles navigates to:
  - Roster
  - Breeding
  - Tournament
  - Leaderboard
- **Status Bar**: Check that Gold and Kaiju Count update correctly.

## 4. Kaiju Roster
- **Cards**: View your list of Kaiju.
  - Hover for summary.
  - Click to select.
- **Details View**:
  - **Visuals**: Check if the dynamic anime sprite loads (from ComfyUI or cache).
  - **Stats**: Verify HP, Attack, Defense, Speed bars.
  - **Genetics**: Check Trait Badges (e.g., "Fire Breath", "Thick Scales").

## 5. Breeding System
- **Selection**: Choose two distinct Kaiju parents.
- **Prediction**: See estimated offspring stats and trait inheritance probabilities.
- **Action**: Click "Breed" (Costs Gold).
- **Result**:
  - Verify a new Egg/Kaiju is added to the roster.
  - **Visual Gen**: Watch console/logs for ComfyUI trigger. A new unique sprite should appear shortly.

## 6. Combat & Tournaments
- **Tournament Menu**: Select a tournament tier (Human -> City -> Planetary).
- **Bracket**: View generated bracket structure.
- **Simulation**:
  - Run matches.
  - Watch battle animations (Attack, Hit, Critical, Defeat).
  - Monitor floating damage numbers.
- **Outcome**:
  - Win: Gain Gold + Fame.
  - Loss: Kaiju might be injured/dead (Death Permadeath check).

## 7. Economy (Server Hooks)
- **Currency**: Verify Gold deducts on Breeding and adds on Tournament wins.
- **Marketplace** (If Server Connected):
  - List a breeding right.
  - Purchase a right from another mocked user.

## 8. Blockchain Export (Advanced)
- **Minting**:
  - Select a Kaiju in details view.
  - Click "Export to Chain" (Mock action).
  - Verify metadata generation (checking traits/stats).
  - Check `kaiju_server` logs for IPFS upload simulation.

## 9. Technical Verification
- **Persistence**: Close and reopen game. Verify Roster and Gold are restored.
- **Logs**: Check console for no panic errors or missing asset warnings.
