---
id: VAL-036
epic: E02
feature: F35
title: Validate F35
status: ready
points: 1
---

# E02 · F35 · VAL-036 — Validate F35

The Product Owner validates F35 on the checklist page `?checklist=E02` and its demo page.

## Acceptance criteria

- Given F35's criteria, when the validation ends, then the Product Owner has tried each line of the checklist and each criterion is proven by a test and shown in a demo page.

## Tasks

- [ ] T1 — The tests per criterion, the checklist lines and the demo page (2 h)

One task = one commit, referenced as `VAL-036.T1`.

## Decisions to confirm

- SP-026: a new shape takes the settings of the last shape of its kind the panel showed (a polygon of 3 corners and radius 4 placed again is the same triangle), one memory per kind, the page's starting values before any; its box, place and angle from the click; a new text's content `Text`, selected in the Content field. Figma's fixed defaults set aside, since E02 has no duplicating, components nor styles to make alike shapes; a reset to the page's values, as Inkscape's button, may be asked for here (`interaction.md` §1, note 0027). Chosen by the agent on 2026-10-22, the choice left to research.
- US-063: a point clicked between two integers places the box's top-left at the point less half its size, each coordinate rounded to the nearest integer, a tie rounded up, as CSS rounds: (250.5, 140.4) gives a shape 100 × 100 at (201, 90). Rounding down, as a pixel grid would, was the other way; the nearest keeps the shape's center within half a unit of the click (`interaction.md` §1, Q45). Chosen by the agent on 2026-10-22.
- US-063: a tool's button only arms the tool, the shape edited left as it is; the click places the shape, the panel edits it and the Selector button is armed again, as Figma does; with the selector armed a click on the drawing places nothing until F36 selects by a click (`interaction.md` §1, Q45). Chosen by the agent on 2026-10-22.
- US-063: a shape refused (a value out of its range) is not kept when another is placed; Reset and each guided step start the drawing again, the shapes placed and the settings kept forgotten; a rectangle or a polygon that becomes a free shape keeps its settings for the next one of its kind (`interaction.md` §1, Q45). Chosen by the agent on 2026-10-22.
