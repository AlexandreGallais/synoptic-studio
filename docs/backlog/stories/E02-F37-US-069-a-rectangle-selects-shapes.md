---
id: US-069
epic: E02
feature: F37
title: A selection rectangle selects shapes
status: done
points: 3
---

# E02 · F37 · US-069 — A selection rectangle selects shapes

As a symbol designer, I want to select the shapes of an area by dragging a rectangle, so that many shapes are selected in one gesture.

Written at SP-027 (note 0028, §3). Figma, Sketch and Inkscape start the rectangle from an empty place of the drawing, a press on a shape being kept for moving it, which E02's next feature brings (`REF-FIGMA-SELECT-LAYERS`, `REF-SKETCH-SELECTING`, `REF-INKSCAPE-KEYS`). Brought in line with SP-027's answers at T1: the rectangle takes the shapes whose painted area it touches, only those it encloses with Alt held (questions 1 and 2, computed by EN-144's `shapesTouchedByBox` and `shapesEnclosedByBox`), Shift adding them to the selection; it is drawn in the layer above the drawing, as the outlines (`interaction.md` §2, Two layers). F37 is no longer validated by a VAL (VAL-038 dropped): the agent's choices go to `interaction.md`, the Q table and F37's guide page, for the review's summary.

## Acceptance criteria

The examples draw, in that order, a rectangle A from (0, 0) to (100, 100), a rectangle B from (150, 0) to (250, 100) and a round shape C, a circle of diameter 100 in the box from (300, 0) to (400, 100), with the Selector armed and Shape mode, C selected as it was drawn last.

- Given the pointer pressed on an empty place of the drawing, when it is dragged, then a rectangle is drawn in the layer above the drawing from the point pressed to the pointer, each rounded to the nearest integer as a tool's drag rounds them (`draggedBox`), in any direction, never in the export; it goes at the release. Pressed at (−10, −10) and moved to (200, 110), it runs from (−10, −10) to (200, 110), as when pressed at (200, 110) and moved to (−10, −10).
- Given a rectangle dragged from (−10, −10) to (200, 110), when the pointer is released, then A and B are selected, the shapes whose painted area it touches; with Alt held at the release, A alone, the one it encloses whole, and the panel edits it as a click does; dragged from (200, 110) to (−10, −10), the same.
- Given a rectangle dragged from (390, 90) to (420, 120), when the pointer is released, then C is not selected, though its box is touched: the circle's nearest point to (390, 90) lies 56.6 from its center (350, 50), beyond its radius of 50; none is selected.
- Given A selected, when a rectangle taking B alone, from (140, −10) to (260, 110), is dragged with Shift held at the release, then A and B are selected; without Shift, B alone, and the panel edits it as a click does.
- Given a rectangle taking no shape, from (110, 10) to (140, 90) between A and B, when the pointer is released, then none is selected; with Shift held, the selection does not change.
- Given the pointer pressed on A, when it is dragged to (200, 50) and released over B, then no rectangle is drawn and the selection does not change, the click the release brings picking nothing (a press on a shape is left for moving, E02's next feature); released on the press's integer point, (50.3, 50.2) after a press at (50, 50), it is a click, which selects A as before.
- Given a tool armed, when the drawing is dragged, then the tool draws its shape (`interaction.md` §1), never a selection rectangle.
- Given Points mode, when the empty drawing is dragged, then no rectangle is drawn and the selection does not change, the mode editing one shape (agent's choice).
- Given the rectangle shown, it is a light tint of the outline's color, its outline a band 1 screen pixel wide centred on its sides, in the outline's color, a fill, never a native stroke; the outline of the shape under the pointer is not drawn meanwhile (agent's choice, for the review's summary).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; add two rectangles side by side and a round shape to their right | four shapes |
| 2 | Press an empty place above and left of the first rectangle and drag across half of the second | a tinted rectangle following the pointer; at the release, every shape it touches selected, each outlined, a dashed box around them |
| 3 | Do it again with Alt held at the release (Option on a Mac) | only the shapes the rectangle holds whole selected |
| 4 | Drag a rectangle over the corner of the round shape's box, outside the circle | nothing selected |
| 5 | Hold Shift and drag a rectangle over the round shape | it is added to the selection |
| 6 | Press on a rectangle and drag | no rectangle drawn, the selection unchanged |
| 7 | Click Rectangle, then drag on the drawing | a new rectangle drawn, no selection rectangle |

## Tasks

- [x] T1 — The criteria in line with SP-027's answers; the shapes selected after a rectangle (`src/model/selection/`), the drag with the Selector and the rectangle in the layer above: interface and failing tests (1 h)
- [x] T2 — The drag from an empty place with the Selector, the rectangle in the layer above, the shapes touched, or enclosed with Alt, selected at the release, a drag never a click; the rules in `interaction.md` §2; the tests of T1 passing but Shift's (2 h)
- [x] T3 — Shift + drag adds; its rule in `interaction.md` §2, the Q table, F37's guide page; the Product Owner test card (1 h)

Files: `src/model/selection/` (the shapes selected after a rectangle, Shift adding them), `playground/selection-drag/` (the drag with the Selector, beside `listenToPicking`, `playground/selection/` holding 15 modules), `playground/hover/` (the drag kept, its rectangle drawn in the layer above by `showHover`), `playground/shape-drawing/listen-to-drawing.ts`. `@see` `docs/domain/interaction.md#_2-selector`, `REF-SKETCH-SELECTING`, `REF-INKSCAPE-SELECTOR`.

One task = one commit, referenced as `US-069.Tn` in the commit footer.
