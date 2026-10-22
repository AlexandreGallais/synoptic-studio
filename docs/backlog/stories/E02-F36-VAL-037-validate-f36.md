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
- EN-143: a text is under the pointer anywhere in its frame, its block or its box when it sits in one, turned with it, not only on its letters; the other ways were the letters' outlines, which the model does not hold, and the lines' backgrounds (`interaction.md` §2, Q47). Chosen by the agent on 2026-10-22.
- EN-143: a shape or a text whose box has a width or a height of 0 paints nothing, and is under the pointer within 4 screen pixels of the outline its corners join, divided by the view's scale: at scale 1 a shape 0 × 80 at (10, 10) is under (12, 50), not under (15, 50); the other ways were never to pick it, or to widen its box to a side of 1 (`interaction.md` §2, Q47). Chosen by the agent on 2026-10-22.
- US-065: the shape under the pointer is outlined only with the Selector armed, by a band 2 screen pixels wide centred on its contour as drawn, a text's frame, a curve followed by chords of at most 2 screen pixels; nothing with a tool armed, a click then placing a shape; the other ways were the shape's box, which outlines more than is picked, and a tint over its area, which changes its color (`interaction.md` §2, Q48). Chosen by the agent on 2026-10-22.
- US-067: a click picks a shape only when its press was made with the Selector armed, so that the click ending a drag with a tool or placing a shape never picks one too; a click on the shape selected loads it again (`interaction.md` §2, Q49). Chosen by the agent on 2026-10-22.
- US-067: the shape selected is outlined by the hover's band, 2 screen pixels wide about its contour as drawn, in the same color, whichever tool is armed, as Figma keeps a selection shown; the other ways were its box with handles, which come with moving and resizing, and a band of another color (`interaction.md` §2, Q49). Chosen by the agent on 2026-10-22.
- US-067: with none selected, every shape stays drawn, the panel hides its fields, its title says no shape is selected and its status how to select or add one (`interaction.md` §2, Q49). Chosen by the agent on 2026-10-22.
