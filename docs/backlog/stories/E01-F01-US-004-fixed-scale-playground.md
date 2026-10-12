---
id: US-004
epic: E01
feature: F01
title: Show shapes at a fixed scale in the playground
status: done
points: 1
---

# E01 · F01 · US-004 — Show shapes at a fixed scale in the playground

As a symbol designer, I want the playground to draw shapes at a fixed scale so that I see the shape grow or shrink when I change its size.

Asked by the Product Owner after US-002 (2026-10-08): the automatic framing made every rectangle fill the canvas, whatever its size.

## Acceptance criteria

- Given a 120 × 80 rectangle, when I type 240 × 160, then the drawn rectangle is twice as large on screen: 1 user unit = 1 CSS pixel.
- Given the playground open, when the window is resized, then the scale stays 1 unit = 1 pixel and the canvas fills the new space.
- Given a rectangle larger than the canvas, when it is drawn, then it is cut by the canvas edges, not shrunk.

## Tasks

One task = one commit, referenced as `US-004.Tn`.

- [x] T1 — View box equal to the canvas size in pixels, origin a margin away from the top-left corner (1 h)
- [x] T2 — Redraw when the window is resized (0.5 h)
- [x] T3 — Demonstrate the criteria to the Product Owner (0.5 h) — in the pull request: `npm run dev`
