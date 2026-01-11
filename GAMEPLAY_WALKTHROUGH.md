# Kaiju Simulator: Feature Testing Guide

This guide walks you through the key features of the application to verify functionality across all implemented phases (0-10).

## 1. Setup & initialization
- **Step 1: Launch Infrastructure (Required)**:
  - Run `kaiju_server`: `cargo run --bin kaiju-server` (Port 3000).
  - Run ComfyUI locally on port 8188 (`python main.py --listen`).
- **Step 2: Launch Client**:
  - Run `cargo run --bin kaiju_sim`.
  - *Note: Client will fail to start/function if Server is not available.*

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

## 5. Breeding System (Server-Authoritative)
- **Requirement**: `kaiju_server` must be running locally on port 3000.
- **Selection**:
  - Click on "Parent A" or "Parent B" slots.
  - OR simply click on any available Kaiju card in the grid to select it.
- **Action**: Click "Breed" (Costs 100 Gold).
- **Process**:
    1.  Client sends `BreedRequest` to Server.
    2.  Server executes breeding logic (Gene combination, Mutation).
    3.  Server mints offspring to Database.
    4.  Server responds with Offspring Data & Transaction Hash.
- **Result**:
  - **Success**: Notification "Breeding successful! [Name] created." including Hash.
  - **Roster**: New Kaiju is added to the roster.
  - **Visual Gen**: Watch console/logs for ComfyUI trigger (Server-side).

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
- **Logs**:
  - **Client Console**: Confirm `[CLIENT->SERVER]` request logs (Payload & URL) and `[SERVER->CLIENT]` status.
  - **Server Console**: Confirm `[INFO]` logs showing "Minted Offspring" and `[DEBUG]` logs showing stat inheritance details (e.g. "Stat HP: Base=100...").
