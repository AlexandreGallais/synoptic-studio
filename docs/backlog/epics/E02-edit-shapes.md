---
id: E02
title: Edit shapes with precision
status: draft
---

# E02 — Edit shapes with precision

| Field                       | Content                                                                                                            |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| For                         | symbol designers                                                                                                   |
| who                         | need fast, exact placement and styling                                                                             |
| the solution                | selection, transform, color, alignment and history tools                                                           |
| is a                        | the editors' interaction layer                                                                                     |
| that                        | integer placement with Inkscape-like snapping                                                                      |
| unlike                      | approximate mouse placement                                                                                        |
| our solution                | every move lands on an integer; guides, snapping, alignment and constant spacing help                              |
| Business outcomes           | shapes are selected, moved, resized, colored, aligned, evenly spaced, undone and redone; the canvas zooms and pans |
| Leading indicators          | interaction features accepted                                                                                      |
| Non-functional requirements | snapping tolerance in screen pixels (`REF-INKSCAPE-SNAP`)                                                          |
| In scope                    | selector, marquee, snapping, fill and stroke colors, align, distribute, undo / redo (Q7), zoom and pan             |
| Out of scope                | —                                                                                                                  |
| Closure criteria            | the scenarios of `interaction.md` are demonstrated                                                                 |

## Candidate features

| Feature                    | Status    |
| -------------------------- | --------- |
| Selection and transform    | to refine |
| Colors                     | to refine |
| Snapping                   | to refine |
| Align and constant spacing | to refine |
| History and navigation     | to refine |

## Ideas from the Product Owner (2026-10-11, refinement of F08)

- Dragging points on the drawing, several selected points moving together, on integers.
- Stretching a shape by its width and height from a fixed point chosen in the 3 × 3 grid, as Inkscape: « Quand je clique sur l'ancrage de droite et que je l'augmente, le width grandit, mais vers la gauche »; strokes keep their width in pixels.
- When a polygon is converted to a free shape, a message saying what happens, with a « do not show again » box.
