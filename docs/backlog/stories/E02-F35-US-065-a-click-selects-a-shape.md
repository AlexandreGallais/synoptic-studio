---
id: US-065
epic: E02
feature: F35
title: The shape under the pointer shown, a click selects it
status: ready
points: 3
---

# E02 · F35 · US-065 — The shape under the pointer shown, a click selects it

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
