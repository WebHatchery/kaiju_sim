# Gameplay Walkthrough

This is a short V1 player-flow checklist for the local MVP.

## 1. Launch

Run:
```bash
cargo run
```

The game should open at the title screen. The local MVP does not require a server.

## 2. Start Or Load

- **New Game** opens starter selection.
- **Load Game** restores the latest local save if one exists.
- **Exit Game** saves before closing on native builds.

## 3. Choose A Starter

Pick one starter kaiju:
- Fire: stronger attack
- Ice: stronger defense
- Electric: stronger speed

The starter is added to the roster with an initial history record.

## 4. Laboratory Hub

Use the left navigation to move between:
- Laboratory
- Roster
- Training
- Arena
- Breeding
- Leaderboard

The laboratory dashboard summarizes active kaiju, recent activity, facility status, and top performers.

## 5. Train

Open **Training**, select a living kaiju, and run one of the stat programs:
- Endurance: HP
- Power: attack
- Guard: defense
- Reflex: speed

Training spends gold, grants XP, improves the chosen stat, and adds a history event to the kaiju.

## 6. Fight

Open **Arena**, choose a living kaiju, and start a seeded AI battle.

After the fight:
- rewards are paid
- player stats update
- the battle result screen shows the summary and log
- the kaiju records the battle outcome and seed in its history

## 7. Breed

Open **Breeding**, choose two living eligible parents, and confirm breeding.

The offspring inherits blended stats and traits. Both parents record a breeding event, and the child records its creation and roster entry.

## 8. Review Records

Open **Roster** or a kaiju's **Record** view to inspect:
- current stats
- traits
- generation and parent IDs
- training, battle, and breeding history

## 9. Verify Persistence

Close and reopen the game, then load the save. Roster, gold, stats, and kaiju history should be restored.
