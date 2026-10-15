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

From the Product Owner's validation of E15 (2026-10-15): holding an arrow key on the size, the opacity or the height lags, while typing feels fluid. Cause measured by SP-019.

## Acceptance criteria

- Given an arrow key held on the size for 1 second (about 30 repeats), when the values change, then the drawing is redrawn at most once per animation frame, the last value drawn, and no redraw waits behind another.
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

- [ ] T1 — Redraws gathered into one per animation frame in the playground (2 h)
- [ ] T2 — Metrics, wrapped lines and block kept by their inputs, with tests counting the computations (3 h)
- [ ] T3 — Measures before and after recorded; test card (1 h)
