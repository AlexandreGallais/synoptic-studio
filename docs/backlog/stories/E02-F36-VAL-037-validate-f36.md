---
id: VAL-037
epic: E02
feature: F36
title: Validate F36
status: ready
points: 1
---

# E02 · F36 · VAL-037 — Validate F36

The Product Owner validates F36 on the checklist page `?checklist=E02` and its demo page.

## Acceptance criteria

- Given F36's criteria, when the validation ends, then the Product Owner has tried each line of the checklist and each criterion is proven by a test and shown in a demo page.

## Tasks

- [ ] T1 — The tests per criterion, the checklist lines and the demo page (2 h)

One task = one commit, referenced as `VAL-037.T1`.

## Decisions to confirm

- US-064: a drag rounds the press and the pointer to the nearest integer, a tie rounded up, as a click; the box runs from the smaller to the larger of each coordinate, in any direction; with Shift its side is the larger distance, laid from the press toward the pointer, down or right when the pointer is level with the press, a text never squared (`interaction.md` §1, Q46). Chosen by the agent on 2026-10-22.
- US-064: a release on the press's integer point is a click, the shape centred on the point pressed, even when the pointer went away and came back (`interaction.md` §1, Q46). Chosen by the agent on 2026-10-22.
- US-064: a drag leaving a width or a height of 0, straight across or straight down, adds the shape as drawn, 0 × 80 from (10, 10) to (10.3, 90), which draws nothing visible until a side is typed, since 0 is a size the model accepts (Q15); a text gets a box of that size. The other ways were to place it as a click, to add nothing, or to make the side 0 a side of 1 (`interaction.md` §1, Q46). Chosen by the agent on 2026-10-22.
