---
id: US-069
epic: E02
feature: F37
title: A selection rectangle selects shapes
status: draft
points: 3
---

# E02 · F37 · US-069 — A selection rectangle selects shapes

As a symbol designer, I want to select the shapes of an area by dragging a rectangle, so that many shapes are selected in one gesture.

Written at SP-027 (note 0028, §3). Figma, Sketch and Inkscape start the rectangle from an empty place of the drawing, a press on a shape being kept for moving it, which E02's next feature brings (`REF-FIGMA-SELECT-LAYERS`, `REF-SKETCH-SELECTING`, `REF-INKSCAPE-KEYS`). Draft until SP-027's questions 1 (what it takes) and 2 (by the painted area or the box) are answered: with the painted area and touching, an EN before it computes whether a contour touches a rectangle (question 8).

## Acceptance criteria

The examples draw, in that order, a rectangle A from (0, 0) to (100, 100), a rectangle B from (150, 0) to (250, 100) and a round shape C, a circle of diameter 100 in the box from (300, 0) to (400, 100), with the Selector armed.

- Given the pointer pressed on an empty place of the drawing, when it is dragged, then a rectangle is drawn in the layer above the drawing from the point pressed to the pointer, in any direction, never in the export; it goes at the release.
- Given a rectangle dragged from (−10, −10) to (200, 110), when the pointer is released, then the shapes selected are those SP-027's question 1 decides: A and B if it takes what it touches, A alone if it takes what it encloses; dragged from (200, 110) to (−10, −10), the same.
- Given a rectangle dragged from (390, 90) to (420, 120), when the pointer is released, then C is selected if question 2 tests the boxes, its box touched; not if it tests the painted area, the circle's nearest point to (390, 90) lying 56.6 from its center (350, 50), beyond its radius of 50 (touching only; enclosing gives the same answer both ways).
- Given A selected, when a rectangle taking B alone is dragged with Shift held, then A and B are selected; without Shift, B alone, and the panel edits it as a click does.
- Given a rectangle taking no shape, when the pointer is released, then none is selected; with Shift held, the selection does not change.
- Given the pointer pressed on A, when it is dragged, then no rectangle is drawn and the selection does not change (a press on a shape is left for moving, E02's next feature); released on the press's integer point it is a click, which selects as before.
- Given a tool armed, when the drawing is dragged, then the tool draws its shape (`interaction.md` §1), never a selection rectangle.
- Given the rectangle shown, it is a band 1 screen pixel wide in the outline's color about a light tint of it, never a native stroke (agent's choice, to confirm at VAL-038).

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset; add two rectangles side by side and a round shape to their right | four shapes |
| 2 | Press an empty place above and left of the first rectangle and drag across half of the second | a rectangle following the pointer; at the release, the shapes question 1 decides selected |
| 3 | Drag a rectangle over the round shape's box corner, outside the circle | the circle selected or not, as question 2 decides |
| 4 | Hold Shift and drag a rectangle over the round shape | it is added to the selection |
| 5 | Press on a rectangle and drag | no rectangle drawn |
| 6 | Click Rectangle, then drag on the drawing | a new rectangle drawn, no selection rectangle |

## Tasks

- [ ] T1 — The shapes a rectangle takes, by the answer to question 2: interface and failing tests (1 h)
- [ ] T2 — The drag from an empty place with the Selector, the rectangle in the layer above, the selection at the release, the tests of T1 passing (2 h)
- [ ] T3 — Shift + drag adds; the rules in `interaction.md` §2; the Product Owner test card (1 h)

Files: `src/model/drawing/` (the shapes in a rectangle, from their straight boxes, `contourBoundingBox` and `textStraightBox`, or from their painted area by the EN of question 8), `playground/selection/` (the drag, beside `listenToPicking`), `playground/hover/show-hover.ts` (the rectangle drawn in the layer above), `playground/placing/read-drawing-point.ts`. `@see` `docs/domain/interaction.md#_2-selector`, `REF-SKETCH-SELECTING`, `REF-INKSCAPE-SELECTOR`.

One task = one commit, referenced as `US-069.Tn` in the commit footer.
