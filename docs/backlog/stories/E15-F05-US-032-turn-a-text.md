---
id: US-032
epic: E15
feature: F05
title: Turn a text and see its measured block
status: ready
points: 2
---

# E15 · F05 · US-032 — Turn a text and see its measured block

As a designer, I want to turn a text about the middle of its block and see that block, so that a label reads along a tank and I know where it ends.

## Acceptance criteria

- Given a text turned by 90, when drawn, then it turns clockwise about the middle of its measured block by a `transform` on its `<text>`, and its angle is kept from 0 to 359 (ADR-0052, `REF-SVG11-TRANSFORM`, `DERIV-turned-shape`).
- Given a selected text, when shown, then its measured block (`DERIV-text-width`, step 4) is drawn as a dashed frame, as a shape's box.

## Tasks

One task = one commit, referenced as `US-032.Tn`.

- [ ] T1 — The turn about the block's middle and the block's frame (2 h)
- [ ] T2 — The Product Owner test card (30 min)
