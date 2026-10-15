---
id: US-048
epic: E15
feature: F17
title: Values changed fast redrawn without lag
status: done
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

## Measures before and after

Measured on 2026-10-16 in Chromium's headless shell (HeadlessChrome 153), on the playground built before the story (`feature/f17-text-corrections` after US-047) and after it: the Text tool, 590 installed faces of 186 families listed (a fake `queryLocalFonts`, as CoreText's count in SP-019), the size focused at 16, then the Up arrow held — 30 key repeats sent 33 ms apart through the DevTools protocol (`Input.dispatchKeyEvent`, `autoRepeat`) — the size ending at 46. Three runs each; the work is the time spent in the `input` listeners and in the animation frames.

| Measure, three runs                       | Before              | After               |
| ----------------------------------------- | ------------------- | ------------------- |
| redraws for 30 repeats                    | 30, one per `input` | 30, one a frame     |
| work in the `input` listeners, total      | 146.5, 141.8, 142.3 | 0.3, 0.4, 0.6 ms    |
| work in the animation frames, total       | 0                   | 25.9, 23.0, 23.7 ms |
| one redraw, mean                          | 4.7 to 4.9 ms       | 0.8 ms              |
| one redraw, longest                       | 9.3 to 10.2 ms      | 1.5 to 1.8 ms       |
| a key's wait before its listener, longest | 0.5 ms              | 0.4 to 0.8 ms       |
| size drawn at the key's release           | 46                  | 46                  |

On this machine a redraw already fitted in the 33 ms between two repeats before the story, so no repeat waited behind another in the headless browser; the Product Owner's lag of 2026-10-15 came with Embed fonts on, each face written in base 64 at every redraw (21 ms a face, SP-019), which US-045 moved to the export. After the story a redraw costs a sixth of what it did, and repeats faster than the display's frames are drawn once a frame. The repeats were 33 ms apart, longer than a frame: each gave its own frame; the gathering itself is shown by the tests (30 sizes typed before one frame, one redraw).

## Product Owner test

| Step | Do                                                | You should see                                             |
| ---- | ------------------------------------------------- | ---------------------------------------------------------- |
| 1    | A text in a box; hold the Up arrow on the size    | the text grows smoothly, stopping when the key is released |
| 2    | The same on the opacity, then on the box's height | no lag                                                     |

## Tasks

One task = one commit, referenced as `US-048.Tn`.

- [x] T1 — Redraws gathered into one per animation frame in the playground; the font list written only when the session changes, in one pass (2 h)
- [x] T2 — Metrics, wrapped lines and block kept by their inputs, with tests counting the computations (3 h)
- [x] T3 — Measures before and after recorded; test card (1 h)
