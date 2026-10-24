---
id: US-089
epic: E02
feature: F46
title: A turned shape moved by the arrows onto integers
status: ready
points: 2
---

# E02 · F46 · US-089 — A turned shape moved by the arrows onto integers

As a symbol designer, I want a shape moved by the arrows to land on whole numbers, turned or not, so that its position stays exact as after a drag.

Asked by the Product Owner on 2026-10-24: a shape turned has an X with decimals; moved by an arrow it keeps them, where a drag's release lands its straight box's top-left on integers (F39, `interaction.md` §2, How shapes move). This replaces F39's rule that an arrow moves by exactly 1 or 10, the shape's fractions kept, for the position: an arrow now moves by 1 or 10 and lands the straight box's top-left on integers, on the drawing and in X and Y; the shape's own fractions inside its box are kept, as at a drag's release.

## Acceptance criteria

- Given a sharp 100 × 50 turned by 30° about its middle, its straight box's top-left at (6.65064, −2.5), when Right is pressed on the drawing, then the top-left is at (8, −2): moved by 1 and landed on integers, the nearest integer on each axis, a tie up, as a drag's release (6.65064 + 1 = 7.65064 → 8 for the axis moved, −2.5 + 0 → −2 for the other, a tie); Shift + Right lands at 17 from 6.65064 (16.65064 → 17). Rounded so, the axis moved always goes toward the arrow, 1 or 10 from an anchor on integers and less from a fraction, never 0 (numbers checked by hand on 2026-10-24; the wording "the nearest integer in the arrow's direction" corrected, the rounding being a drag's, `landedShift`).
- Given the same shape, when Up is pressed in its X (6.65064), then X is 8 and Y −2, the shape moved as by Right; Up in Y, likewise, as by Down: Y −2.5 + 1 = −1.5 → −1, a tie up, and X 6.65064 → 7.
- Given a shape on integers, then an arrow moves it by exactly 1 or 10, as today.
- Given several shapes, then their frame's top-left lands on integers, as at a drag's release, each keeping its place in the frame.
- Given an arrow held, then the move is still one step of the history.

## Product Owner test

`npm run dev`, `http://localhost:5173/`. The page opens with a rectangle 120 × 80 at (0, 0), selected. X and Y in the panel are the top-left of the upright box around the shape as drawn; on the page, y grows downwards.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Type `0` in Radius and press Enter, then `30` in Angle and Enter | the rectangle turned, X −11.96152 and Y −24.64102 |
| 2 | Click the rectangle on the drawing, then press Right | X −11 and Y −25: moved right and landed on whole numbers, as a drag lands it |
| 3 | Press Right again, then Shift + Down | X −10, Y −25; then Y −15: by exactly 1, then 10, once on whole numbers |
| 4 | Hold Right for about a second, release it, then press Ctrl + Z (Cmd + Z on a Mac) | X going up by 1 at each repeat, always a whole number; then X −10 and Y −15 again at once, the hold one step |
| 5 | Click in X and press Up | X −9, Y −15, as Right on the drawing |
| 6 | Type `45` in Angle and press Enter | X −7.74915, Y −21.06966, the box turned further about its middle |
| 7 | Type `+1` in X and press Enter | X −6.74915, Y −21.06966: a value typed keeps the fraction |
| 8 | Press Up in Y | X −7, Y −20, as Down on the drawing: both landed on whole numbers |
| 9 | Press Shift + Up in X | X 3, Y −20 |

## Tasks

- [x] T1 — The rule in `interaction.md` §2 and its Q row; tests of the criteria first, failing (30 min)
- [x] T2 — The code (the nudge on the drawing and the arrow in X and Y landing the straight box's top-left as a drag's release does, `src/model/moving/`), the tests passing; the Product Owner test card (1 h)

One task = one commit, referenced as `US-089.Tn` in the commit footer.
