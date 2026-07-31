# TODO — Kaiju Breeding Simulator

## Gameplay depth

- Training is bounded only by gold; add stamina or fatigue so stats cannot be maxed by grinding.
- Explain the second "companion breeder" kaiju in starter selection — the player picks one starter and silently receives two.
- Grow the mid-game past the vertical slice: rivals, seasonal arena events, arena modifiers.

## Breeding and records

- Show predicted offspring stat ranges and mutation odds before a pairing is confirmed.
- Summarise inherited traits on the hatched offspring's record.

## Presentation

- Make arena fights read as monsters fighting rather than stat rows; the hit-by-hit log is the strongest part and deserves visuals.

## Testing

- Add save/load round-trip tests for trained kaiju, post-breeding rosters, battle rewards, and preserved event history — only one persistence test exists.

## Web build

- Answer the forked `storage.js` questions in `STORAGE_BRIDGE_TODO.md` and rejoin the shared bridge if the plugin workaround is obsolete.

## Server

- `kaiju_server` breeding still stubs mutation generation and does not persist the breeding log event id.
