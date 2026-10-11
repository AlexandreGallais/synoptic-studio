---
id: SP-005
epic: E01
feature: F08
title: Research for F08
status: done
points: 1
---

# E01 · F08 · SP-005 — Research for F08

Find, read and record the sources of every story of F08 before its implementation (ADR-0020, research protocol §8).

## Acceptance criteria

- Given each story of F08, when the spike ends, then every behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and a research note records what was found and what remains open.

## Questions

- A free shape keeps the fractional vertices of a converted polygon, while every other input is an integer (ADR-0003): an ADR for this exception (moved vertices land on integers).
- The model of a free shape: vertices, radii, contour order (Q11) after a conversion, an added or a deleted vertex; its bounding box as width and height.
- Converting a shape to a free path in design tools (Inkscape "Object to Path", Figma "Flatten" or vector editing), as functional references.
- Inserting a node in the middle of a segment (Inkscape "Insert new nodes") and deleting nodes; what happens to the neighbours.
- Concave contours: the known limit of `DERIV-local-radius-clamp` (an arc touching a non-adjacent edge) becomes reachable — detect, not correct.

## Tasks

One task = one commit, referenced as `SP-005.Tn`.

- [x] T1 — Find, read and record the sources; ADR for fractional vertices (2 h)
- [x] T2 — Write the research note and update the stories (1 h)
