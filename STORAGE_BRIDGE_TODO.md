# kaiju_sim keeps a forked `storage.js` — needs investigation

**Status:** deliberately excluded from the shared web shell migration. Do not
"fix" this by deleting the fork until the questions below are answered.

## What happened

Every other game's `index.html` is now generated from `web/index.template.html`
+ `game_page.json`, and they all load the one canonical localStorage bridge at
`shared-assets/runtime/storage.js` (see `RustGames/web/README.md`).

`kaiju_sim` opts out via `"storage_js": "custom"` in its `game_page.json`, so it
keeps loading its own root-level `storage.js`.

## Why it was excluded

`kaiju_sim/storage.js` (98 lines) is not a cosmetic divergence from the canonical
50-line bridge. It differs in two substantive ways:

1. **It bypasses `miniquad_add_plugin`.** Instead of registering a plugin, it
   mutates `importObject.env` directly.
2. **It performs plugin surgery.** It filters the global `plugins` array to strip
   `macroquad_audio` and `quad_net`.

Neither behaviour exists in any other game. The cache-buster on its script tags
(`?v=20260526-plugin-clean`) and the name "plugin-clean" suggest this was a
targeted fix for a real bug — most likely a crash or hang caused by one of those
two plugins registering against this game's import table.

Folding it into the canonical bridge risks regressing kaiju_sim's audio or
networking in a way that would not show up in a quick smoke test, so it was left
alone.

## Questions to answer

- What was the original failure? Check this project's git history around the
  `20260526-plugin-clean` cache-buster for the commit and message.
- Does `kaiju_sim` actually use `macroquad_audio` / `quad_net` at runtime? If it
  does not, stripping them may simply be dead weight that the shared bridge can
  ignore.
- Is the `importObject.env` mutation load-order-sensitive? The canonical bridge
  relies on `miniquad_add_plugin`, which runs at a different point.
- Does the bug still reproduce with the current `mq_js_bundle.js`? It is
  downloaded fresh on every publish, so an upstream fix may have made the
  workaround obsolete.

## If it turns out to be obsolete

Delete `kaiju_sim/storage.js` and remove `"storage_js": "custom"` from
`kaiju_sim/game_page.json`. The shared bridge then applies automatically and this
file can go away.
