---
id: US-048
epic: E15
feature: F17
title: Values changed fast redrawn without lag
status: ready
points: 3
---

# E15 · F17 · US-048 — Values changed fast redrawn without lag

As a symbol designer, I want the drawing to follow a value held on an arrow key without lag so that I can tune a text by eye.

From the Product Owner's validation of E15 (2026-10-15): holding an arrow key on the size, the opacity or the height lags, while typing feels fluid. Cause found by SP-019 ([research note 0021](../../research/notes/0021-f17-latin-script-and-defects.md)): every `input` event redraws at once and whole, and a held key sends about 30 a second. Measured in Node.js, the text's own computation is small (`textDrawing` 0.052 ms, run twice per redraw), while what is redone with unchanged inputs costs most: the font list rewritten at each event, its names compared two by two (6.1 ms for 590 installed faces, 15 ms for 1000), and, with **Embed fonts** on, each face written in base 64 (21 ms for a 200 500-byte face) before the browser decodes it. Requests are gathered by `requestAnimationFrame` (`REF-MDN-RAF`).

## Acceptance criteria

- Given an arrow key held on the size for 1 second (about 30 repeats), when the values change, then the drawing is redrawn at most once per animation frame, the last value drawn, and no redraw waits behind another.
- Given installed fonts listed (590 faces), when a value other than the font changes, then the font list is not written again, and its names, when written, are deduplicated in one pass (no comparison of every name with every other).
- Given a redraw, when it runs, then `textDrawing` is computed once.
- Given a change of the opacity only, when the drawing is redrawn, then the font metrics, the wrapped lines and the block are not computed again, being kept by their inputs (counted by a test).
- Given a change of the size, when the drawing is redrawn, then the metrics are kept and the lines and block computed again.
- Given SP-019's measure of a redraw before the story, when it ends, then the same measure after it is recorded in the story.

## Product Owner test

| Step | Do                                                | You should see                                             |
| ---- | ------------------------------------------------- | ---------------------------------------------------------- |
| 1    | A text in a box; hold the Up arrow on the size    | the text grows smoothly, stopping when the key is released |
| 2    | The same on the opacity, then on the box's height | no lag                                                     |

## Tasks

One task = one commit, referenced as `US-048.Tn`.

- [ ] T1 — Redraws gathered into one per animation frame in the playground; the font list written only when the session changes, in one pass (2 h)
- [ ] T2 — Metrics, wrapped lines and block kept by their inputs, with tests counting the computations (3 h)
- [ ] T3 — Measures before and after recorded; test card (1 h)
