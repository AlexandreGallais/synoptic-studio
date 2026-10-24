---
id: US-090
epic: E02
feature: F46
title: The selection rectangle always answers, the canvas resized without a flicker
status: ready
points: 2
---

# E02 · F46 · US-090 — The selection rectangle always answers, the canvas resized without a flicker

As a symbol designer, I want the selection rectangle to answer every press and drag, and the drawing never to flicker when the panels or the window change size.

Seen by the Product Owner on 2026-10-24 on F45's branch:

- Once, a press on the empty drawing then a drag drew no selection rectangle, the Selector armed, not a shape's tool (no shape drawn either); the steps before it not written down. Candidates to check: a press left open (a `pointerup` lost outside the window, a pointer capture not released, a pan drag or Space held, US-078 and AUD-044's `blur`, a held arrow or a burst open, F43 and AUD-045), a selection drag left in its state.
- The drawing clips, or flickers, when Pipeline's column is shown or hidden or the window resized: US-084 redraws at the next frame after the `ResizeObserver` hears the change, so that for a frame the old `viewBox` is fitted to the new box.

## Acceptance criteria

- Given the cause of the rectangle doing nothing found and reproduced, then a test fails before the fix and passes after, and the cause is written here; given any press left open (a release outside the window, the window losing the focus mid-drag, Space or a pan left held), then the next press on the empty drawing with the Selector draws a selection rectangle.
- Given the canvas resized, then both SVGs take the new `viewBox` in the `ResizeObserver`'s own callback, before the browser paints, not at the next frame (`REF-RESIZE-OBSERVER-1`: its callbacks run before paint).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The rectangle's failure reproduced (each candidate above, in happy-dom and in a headless Chromium from the machine's caches if needed), its cause written; tests of both criteria first, failing (1 h)
- [ ] T2 — The fixes, the tests passing; the Product Owner test card (1 h)

One task = one commit, referenced as `US-090.Tn` in the commit footer.
