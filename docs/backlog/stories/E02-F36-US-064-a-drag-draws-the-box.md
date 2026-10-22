---
id: US-064
epic: E02
feature: F36
title: A drag with a tool draws the shape's box
status: done
points: 3
---

# E02 · F36 · US-064 — A drag with a tool draws the shape's box

As a symbol designer, I want to drag with a tool to draw the new shape at the size I want, so that it needs no resizing afterwards.

## Acceptance criteria

- Given the rectangle tool, when the pointer is pressed at (10, 10), dragged and released at (130, 90), then a rectangle 120 × 80 at (10, 10) is added; while dragging, the shape follows the pointer.
- Given a drag up and to the left, from (130, 90) to (10, 10), then the same rectangle is added: the box runs from the smaller to the larger corner.
- Given the pointer released where it was pressed, on the same integer point, then it counts as a click (US-063).
- Given the text tool dragged from (10, 10) to (210, 70), then a text in a box 200 × 60 is added.
- Given the round shape tool and Shift held, when the pointer is dragged from (10, 10) to (130, 90), then a circle in a box 120 × 120 is added: Shift keeps the box square, its side the larger of the two distances dragged, for every tool but text; Shift pressed or released during the drag changes the shape at once (Product Owner, 2026-10-22; Shift as in Figma, the key editable later, E20).
- Given a drag leaving a width or a height of 0, then what is added is the agent's choice, written in `interaction.md` §1.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset | the rectangle 120 × 80 the page opens with, alone; the Selector button pressed |
| 2 | Click Rectangle, press the drawing right of the first rectangle and drag down and to the right, slowly | a rectangle grows from the point pressed, its corner following the pointer; Width and Height change as you drag |
| 3 | Release | the rectangle stays, of the size dragged, above the first; the panel titled Rectangle; the Selector pressed again |
| 4 | Click Rectangle, drag from a point up and to the left | the same kind of box, from the smaller to the larger corner: the rectangle lies between the two points |
| 5 | Click Round shape, hold Shift and drag a box wider than high | a circle, its side the larger distance dragged |
| 6 | Click Polygon, drag without Shift, press Shift while dragging, release Shift, then release the pointer | the box turns square when Shift is pressed and back to the box dragged when it is released, at once; the polygon kept as dragged without Shift |
| 7 | Click Text, drag a box about 200 × 60 | `Text` in a box of that size, Fit set to Box, its Width and Height those dragged, `Text` selected in Content; typing `Pump` replaces it |
| 8 | Click Rectangle, press the drawing and release without moving | a rectangle 100 × 100 centred on the point, as a click (US-063) |
| 9 | Click Rectangle, drag straight down, keeping the pointer on the same column | a rectangle of Width 0 added: nothing visible, the panel shows it; typing a width draws it |
| 10 | Export SVG, open the file | every shape drawn, in the order drawn, each of the size dragged |

## Tasks

- [x] T1 — The box from a press and a release: interface and failing tests (1 h)
- [x] T2 — The box computed, the tests of T1 passing (1 h)
- [x] T3 — The playground draws the shape live while dragging; the Product Owner test card (2 h)

One task = one commit, referenced as `US-064.Tn` in the commit footer.
