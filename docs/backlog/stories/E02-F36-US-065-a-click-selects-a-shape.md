---
id: US-065
epic: E02
feature: F36
title: The shape under the pointer shown, a click selects it
status: parked
points: 3
---

# E02 · F36 · US-065 — The shape under the pointer shown, a click selects it

As a symbol designer, I want to see which shape is under the pointer and to pick it by a click, so that I edit the shape I mean.

The Product Owner asked for two layers (2026-10-22): the drawing's SVG holds only the shapes and receives the clicks; a transparent SVG above it, which the pointer passes through (`pointer-events: none`), draws the hover outline, the selection and every other help, so that the drawing is never polluted.

## Acceptance criteria

- Given three shapes, when the pointer passes over one, then that shape is outlined in the layer above, and the outline goes when the pointer leaves it.
- Given two shapes overlapping, when the pointer is over both, then the top one is outlined and taken by a click.
- Given a shape clicked, then it alone is selected, shown in the layer above, and the panel shows and edits its values; a click on the empty drawing selects none, the panel saying so.
- Given the hover and the selection shown, when the SVG is exported, then it holds no outline nor selection.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |

## Tasks

- [ ] T1 — The shape at a point and the selection of one: interface and failing tests (1 h)
- [ ] T2 — The layer above the drawing, the hover and the click, the tests of T1 passing (2 h)
- [ ] T3 — The panel follows the selection; the Product Owner test card (2 h)

One task = one commit, referenced as `US-065.Tn` in the commit footer.

## Question for the Product Owner

Asked on 2026-10-22, before T1: the story holds four pieces of work for three tasks — the shape under a point computed in the model (ADR-0005), its contours made of segments, arcs and cubic sides; the drawing split around the edited shape so that a shape picked from the middle keeps its place; every field of the panel written back from a picked shape; the layer above the drawing. Proposed split, F36 then holding four stories: US-065 the shape under the pointer shown (the model's `shapeAtPoint`, the layer, the hover outline), a new story a click selects it (the drawing split around the edited shape, the panel loaded per kind). Three rules are missing:

- **Q-a, under the pointer**: (1) the painted area, as Figma does — a click in the cut corner of a rounded rectangle or outside a circle but inside its box picks nothing, a derivation needed for arcs and cubic sides; (2) the shape's box, turned with it. And a text by its block or its frame; a shape of width or height 0 picked within a tolerance in screen pixels, or not.
- **Q-b, a picked rectangle**: the model stores it as a free shape (ADR-0032), so (1) the panel shows it as a free shape, or (2) the tool's kind is kept beside it, the panel showing Rectangle with its radius.
- **Q-c, a picked text in another family**: (1) the font session switches to its family, or (2) only the text's own fields are written.
