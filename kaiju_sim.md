# Kaiju Breeding Simulator

Game Design Document, V1 local MVP.

## High Concept

Kaiju Breeding Simulator is a strategic management and auto-battle game where players act as private kaiju breeders. Players train, fight, and breed persistent kaiju to build stronger bloodlines and lasting records.

Every kaiju is unique, persistent, and documented. Its value comes from stats, traits, lineage, battle record, training record, breeding record, and the story those events create over time.

## Player Fantasy

The player is:
- a breeder and genetic planner
- a trainer preparing kaiju for combat
- a caretaker of long-running bloodlines
- a competitor trying to climb local and future global rankings

The core fantasy is legacy. A kaiju may win, lose, breed, or die, but its history remains visible.

## V1 Scope

V1 is a local, offline-first game loop:
- choose a starter
- train kaiju
- fight seeded AI arena battles
- breed two eligible living kaiju
- save/load local progress
- inspect each kaiju's documented history
- compare roster performance on a local leaderboard

V1 does not require accounts, server sync, real-player matchmaking, real-player breeding, or external ownership systems.

## Core Gameplay Loop

1. Choose or load a roster.
2. Train kaiju to improve targeted stats.
3. Fight AI arena battles for gold, XP, and records.
4. Breed two eligible kaiju to produce offspring.
5. Review the resulting kaiju history and lineage.
6. Repeat to refine stronger bloodlines.

## Kaiju Entity

Each kaiju has:
- unique local ID
- legacy ID
- name
- generation
- parent IDs
- original breeder
- current owner or keeper
- current stats
- visible traits
- hidden traits
- visual seed or image URI
- experience
- win counts
- alive/dead status
- chronological event history

## Event History

History is a first-class V1 system. Each kaiju stores a chronological event log that can be saved, loaded, and displayed in the kaiju record screen.

History events should cover:
- creation
- joining the roster
- training
- battles
- breeding as a parent
- hatching as offspring
- research discoveries
- transfer, if future systems add it
- death, if lethal systems add it

Events should include enough detail to be useful later: title, detail text, event kind, timestamp, related kaiju IDs, and battle seed when applicable.

## Genetics

Traits are inherited through breeding and divided into:
- visible traits, such as elemental affinities and physical features
- hidden traits, such as conditional bonuses and long-term growth modifiers

Breeding rules:
- generation is `max(parent_a.generation, parent_b.generation) + 1`
- stats blend from parents with narrow variance
- later generations may have gentle power growth
- traits inherit probabilistically
- mutations may occur
- direct self-breeding and direct parent-child breeding are blocked

## Training

Training consumes gold and grants XP plus a targeted stat improvement.

Training focus areas:
- Endurance: HP
- Power: attack
- Guard: defense
- Reflex: speed

Training results are recorded in the kaiju history.

## Combat

Combat is simulated automatically.

Battle rules:
- speed determines turn order
- damage uses attack, defense scaling, traits, environment, and narrow variance
- battle seeds should allow audit/replay behavior
- arena battles produce rewards and event records

V1 combat is against generated AI opponents. Future multiplayer can compare player-owned rosters without changing the local combat core.

## Breeding

Breeding is local in V1. The player selects two eligible living kaiju, pays the breeding cost, and receives an offspring.

The offspring records its creation. Both parents record their participation. This makes bloodline history readable without relying on any external service.

## Death And Legacy

V1 can support alive/dead status, but lethal content should be introduced carefully. If a kaiju dies:
- it cannot train, fight, or breed
- its record remains visible
- lineage links remain valid
- the death event remains in history

## Leaderboards

The local leaderboard ranks roster kaiju by performance indicators such as power, wins, generation, and documented activity. It is a local prestige screen for V1 and a bridge to future global ranking.

## Future Multiplayer V2

V2 can add optional networked systems:
- player-vs-player arena comparisons
- real-player tournaments
- opt-in breeding with other players' kaiju
- shared leaderboards
- account sync

V2 must not make local V1 play depend on a server. The local kaiju history remains the foundation for syncing, comparing, and explaining player-owned rosters.

## Design Pillars

- Documented history creates drama.
- Breeding decisions should matter over several generations.
- Combat should be readable and mostly predictable.
- Power should grow slowly enough to keep old records meaningful.
- The local game should remain playable without network services.
