---
id: US-055
epic: E15
feature: F17
title: A text put in a box keeps its fitted size
status: done
points: 2
---

# E15 · F17 · US-055 — A text put in a box keeps its fitted size

As a symbol designer, I want a text that fits its content to keep its size when I put it in a box, so that choosing Box does not change what I see, and I can then resize the box from there.

From the Product Owner's first pass of VAL-018 (2026-10-16), finding C1: choosing Box after Fit content drew the text in the box last typed, 200 × 100 when the page opens, whatever the text's size. The box's width and height are read from their fields, which keep their last value whichever kind the text is (`shapes.md`, Text, Box), and nothing copied the measured block into them.

**Decision** (provisional, taken on 2026-10-16 while the Product Owner was away, to confirm at VAL-018's second pass; `docs/domain/shapes.md`, Text, Box): when a text goes from Fit content to Box, the box's width and height become its measured block **rounded up** to integers, so that nothing is cut and nothing visibly moves: "Pump P-101" in Inter 16, a block 87.953125 × 19.359375, gets a box 88 × 20. A side that is an integer within a rounding error (`EPSILON`) keeps it, since a box holds a line within that error (`DERIV-text-box`, steps 2 and 5): 88.0000000001 gives 88, not 89. The fields then stay editable, as any box's. Each switch from Fit content to Box measures again: Box, Fit content, Box gives the block of the text as it is then. A turned text keeps its turned top-left corner, the one X and Y show (ADR-0057, US-054), as any change of its frame's size does.

**What may still move**, by less than one unit: the lines are aligned across the box's width and the block placed down its height (`DERIV-text-box`, step 8), so a centered line moves right by half the rounding (0.02344 for 88 − 87.953125), and a block in the middle moves down by half of it (0.32031 for 20 − 19.359375). A maximum number of lines set earlier, while the text fitted its content, still applies once it is in a box, as every option read only in a box.

## Acceptance criteria (F17.AC13)

- Given "Pump P-101" in Inter 16 fitting its content, its block 87.953125 × 19.359375, when Box is chosen, then the box's fields show 88 and 20, and the text is drawn on the same line at the same place.
- Given that box, when its width is typed 50, then it is cut as in any box, one line of 19.359375 fitting in 20: "Pum…"; when its height is then typed 40, it shows "Pump" over "P-101".
- Given that box 88 × 20, when Fit content is chosen, a second line "Tank" typed, then Box chosen again, then the box is 88 × 39, the new block 87.953125 × 38.71875 rounded up.
- Given "Pump P-101" fitting its content at (47, 61) turned by 23, X and Y showing 54.27808 and 44.58648, when Box is chosen, then the box is 88 × 20, X and Y still show 54.27808 and 44.58648, and its stored position is (46.87298, 60.98369).
- Given an empty text, its block 0 × 0, when Box is chosen, then the box is 0 × 0.

## Product Owner test

| Step | Do                                                                  | You should see                                                             |
| ---- | ------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 1    | Text tool, Fit content, type "Pump P-101", size 16                  | the text and its dashed frame, about 88 wide and 19.4 tall                 |
| 2    | Choose Box                                                          | Width 88, Height 20; the text stays where it was, its frame now 88 × 20    |
| 3    | Type 50 in Width, then 40 in Height                                 | "Pum…" (one line fits in 20), then "Pump" over "P-101", wrapped in the box |
| 4    | Choose Fit content, add a second line "Tank", then choose Box again | Width 88, Height 39: measured again, not the 50 typed before               |
| 5    | Fit content, angle 23, note X and Y, then choose Box                | X and Y unchanged; the text's top-left corner stays where it was drawn     |
| 6    | Change Width and Height                                             | the box resizes as before, the text wrapped and placed in it               |

## Tasks

One task = one commit, referenced as `US-055.Tn`.

- [x] T1 — The box around a measured block, each side rounded up within a rounding error; `DERIV-text-box` step 11, `REF-ECMA-262-MATH-CEIL` (1 h)
- [x] T2 — Choosing Box after Fit content writes that box in the fields, a turned text keeping its corner; `shapes.md` (1 h)
