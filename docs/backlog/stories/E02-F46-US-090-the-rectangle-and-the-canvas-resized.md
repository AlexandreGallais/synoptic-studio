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

## The cause

Each candidate was tried on F46's branch, in happy-dom (`press-left-open.test.ts`) and in a headless Chromium driven by the npx cache's Playwright, 2 560 × 1 000, a press on the empty drawing and a drag of 80 pixels after each: a plain drag, a drag released, a pan by Space then Space released, Space held then the window blurred, the middle button's pan, a press dragged out of the window and the window blurred, a drag cancelled, a right click, a rectangle drawn with its tool, the column hidden, an arrow held, a field typed in, a double click and Ctrl + A all drew the rectangle.

One state alone kept it from being drawn: **Space held for the page after it was released**. The page lets Space go at its `keyup` and when the window loses the focus (AUD-044); on macOS no `keyup` is sent for a key released while Cmd is held (`REF-WEBKIT-CMD-KEYUP`, `REF-MOZILLA-CMD-KEYUP`), so Space pressed, then Cmd, then Space released before Cmd, a shortcut with Cmd typed while Space is down, leaves Space held, the window keeping the focus. The next press with the main button is then a pan (`startPanDrag`), stopped from the Selector's listeners: no rectangle, no shape; the view follows the pointer, which on an empty part of the drawing is easily missed. Reproduced in Chromium by Space's `keydown` without its `keyup`, and in happy-dom by Space's `keydown`, then Cmd's `keydown` and `keyup`: the drag pans, no rectangle drawn. The arrows held were made safe from the same macOS behaviour by AUD-045 (Cmd's `keyup` releases them); Space was not.

A second state left open is safe for the rectangle but not for the drawing: a tool's drag whose release is never heard keeps its drawing (`SHAPE_DRAWINGS`), and with the Selector armed again by its button the next press does not drop it, so that its moves resize the shape drawn before while the rectangle is drawn. The other states are each dropped at the next press already: the Selector's drag (`startSelectionDrag`), a pan (`startPanDrag`); a pointer capture the browser releases at the release or with the pointer cancelled. No other candidate, `lostpointercapture` included, left a state the next press keeps.

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
