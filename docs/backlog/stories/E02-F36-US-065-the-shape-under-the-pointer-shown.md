---
id: US-065
epic: E02
feature: F36
title: The shape under the pointer shown
status: done
points: 3
---

# E02 · F36 · US-065 — The shape under the pointer shown

As a symbol designer, I want to see which shape is under the pointer, so that I know which one a click will pick.

The Product Owner asked for two layers (2026-10-22): the drawing's SVG holds only the shapes and receives the clicks; a transparent SVG above it, which the pointer passes through (`pointer-events: none`), draws the hover outline, the selection and every other help, so that the drawing is never polluted. Which shape is under the pointer is EN-143's `shapeAtPoint`, the painted area (`interaction.md` §2).

## Acceptance criteria

- Given three shapes, when the pointer passes over one, then that shape is outlined in the layer above, and the outline goes when the pointer leaves it.
- Given two shapes overlapping, when the pointer is over both, then the top one is outlined.
- Given a rounded rectangle, when the pointer is in its cut corner, then it is not outlined.
- Given the hover shown, when the SVG is exported, then it holds no outline.

## Product Owner test

`npm run dev`, `http://localhost:5173/`.

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the page, then Reset | the rectangle 120 × 80 the page opens with, alone; the Selector button pressed |
| 2 | Pass the pointer over the rectangle, then off it | a blue line 2 pixels wide along its edge, its rounded corners included, while the pointer is over it; nothing once it is off |
| 3 | Click Rectangle and click the drawing so that the new rectangle overlaps the first, then Round shape and click elsewhere | three shapes; the Selector pressed again |
| 4 | Pass the pointer over each shape, then over the part where the two rectangles overlap | each shape outlined in turn, the round shape along its circle; where they overlap, the rectangle placed last |
| 5 | Put the pointer in a rounded corner of the first rectangle, just outside the curve, then inside it | no outline outside the curve, the outline inside it |
| 6 | Put the pointer inside the round shape's box but outside its circle | no outline |
| 7 | Click Text, click the drawing, then pass the pointer over the text, between its letters too | the text outlined along its block, wherever the pointer is in it |
| 8 | Click Rectangle and drag straight down, then pass the pointer slowly across the line it leaves | the rectangle of width 0 outlined when the pointer is within about 4 pixels of it |
| 9 | Click Rectangle, then pass the pointer over a shape | no outline while a tool is armed |
| 10 | Click an empty place of the drawing, Rectangle still armed; click in Width, put the pointer back over the new rectangle and press the Up arrow key a few times | the outline widens with the rectangle at each key, the pointer still |
| 11 | With a shape outlined, click Export SVG and open the file | every shape, and no outline |

## Tasks

- [x] T1 — The layer above the drawing and the hover: interface and failing tests (1 h)
- [x] T2 — The layer and the outline of the shape under the pointer, the tests of T1 passing; the Product Owner test card (2 h)

One task = one commit, referenced as `US-065.Tn` in the commit footer.

## Question for the Product Owner

Asked on 2026-10-22, before T1: the story held four pieces of work for three tasks — the shape under a point computed in the model (ADR-0005), its contours made of segments, arcs and cubic sides; the drawing split around the edited shape so that a shape picked from the middle keeps its place; every field of the panel written back from a picked shape; the layer above the drawing. Three rules were missing: what counts as under the pointer (the painted area or the box), what the panel shows for a picked rectangle (stored as a free shape, ADR-0032), and what a picked text in another family does to the font session.

**Answer** (Product Owner, 2026-10-22): the painted area, only what is visible, so that shapes can be picked precisely; a picked rectangle shows as a Rectangle with its radius; a picked text loads its whole style — its family, weight, italic, strokes and every option — since typing next is likely meant in that style. The split agreed: EN-143 the shape at a point in the model, US-065 the hover, US-067 a click selects; Delete (US-066) moved to the next feature, which brings the selection of several shapes, so that F36 keeps four stories.
