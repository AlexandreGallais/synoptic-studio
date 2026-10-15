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

- [x] T1 — The turn about the block's middle and the block's frame (2 h)
- [ ] T2 — The Product Owner test card (30 min)
- [ ] T3 — CHK-014's two points: a font without metrics, the fallback's name checked (1 h)

## Decisions taken (2026-10-15)

- **The turn on the group.** The `transform="rotate(α cx cy)"` is written on the `<g>` that holds the `<text>` and its underline and strike-through paths, so that the strokes turn with the letters; the first criterion's "a `transform` on its `<text>`" is met through that group (provisional, to confirm at VAL-014). A text at 0° carries no `transform`.
- **The middle** is `boxCenter` of the measured block at the text's position (`DERIV-turned-shape` step 4), kept in `DrawnText.center`; the transform is written by `formatRotation` (`REF-SVG11-TRANSFORM`, §7.6). The angle is checked by `isShapeAngle`, as a shape's.
- **The frame.** A selected text's block is drawn by the route of a shape's straight box (`createStraightBoxElement`: dashes as thin filled rectangles), in a `<g>` carrying the text's rotation, so that it turns with the text; an empty block draws none.
- **The playground.** In Text mode the angle row (field, −90° and +90°) is shown; the shapes' modes and straight box stay hidden.
