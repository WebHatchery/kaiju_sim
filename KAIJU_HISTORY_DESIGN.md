# Kaiju History Design

This document replaces the old external ownership concept for V1. Kaiju legacy is represented by a local, strongly documented history of events attached to each kaiju.

## Goals

- Make every kaiju feel persistent and inspectable.
- Preserve the important events that explain a kaiju's value.
- Support save/load without a server.
- Give future multiplayer a clear history format to sync or compare.
- Keep gameplay local and deterministic where practical.

## Core Model

Each `Kaiju` has:
```rust
pub history: Vec<KaijuEvent>
```

Each `KaijuEvent` records:
- timestamp
- event kind
- title
- details
- related kaiju IDs
- optional battle seed

Event kinds include:
- Created
- JoinedRoster
- Training
- Battle
- Breeding
- Offspring
- Research
- Transfer
- Death

## Required V1 Events

Starter creation:
- record when the starter is created
- include the chosen element or origin

Roster join:
- record when any kaiju enters the player's roster

Training:
- record focus area
- record stat and XP gains

Battle:
- record win/loss
- record opponent
- record reward
- record battle seed

Breeding:
- parents record that they produced offspring
- child records parent names and related parent IDs

Death:
- record final cause if lethal systems are introduced

## UI Requirements

The kaiju detail screen should show the history in reverse chronological order, with the newest event first. Each row should make the event type, title, and detail readable without requiring a modal.

History display should prioritize:
- battle outcomes
- breeding participation
- training milestones
- creation and lineage

## Persistence Requirements

History must serialize with the save file. Older saves without history should load successfully by defaulting to an empty event list.

## V2 Multiplayer Use

Future multiplayer can use the same event structure for:
- synced battle records
- real-player tournament logs
- cross-player breeding agreements
- shared leaderboard explanations

V2 features may add account or server metadata, but the V1 history should remain readable without network access.
