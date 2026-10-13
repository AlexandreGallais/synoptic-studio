---
id: SP-011
epic: E14
feature: F12
title: "Research for F12: curves in symbols, ways to type a handle, strokes"
status: ready
points: 3
---

# E14 · F12 · SP-011 — Research for F12: curves in symbols, ways to type a handle, strokes

Find, read and record what tools and sources do for points on a curve with handles, before any code (ADR-0020, research protocol §8); write the ADR that lets curves into symbols, and the stories of F12.

## Acceptance criteria

- Given the ways to type a handle (x and y of its end, an angle and a length relative to its point, pasted Bézier data, others), when the spike ends, then each is described with the tools that use it, its sources, a numeric example, and what it needs from the library (Q32).
- Given Figma's three handle modes and the corner radius kept under the handles (`docs/domain/shapes.md`), when the spike ends, then the model of a point is proposed: its stored integers, how a mode constrains the other handle, how the radius comes back.
- Given ADR-0001 (segments and circular arcs only), when the spike ends, then a new ADR proposes cubic Bézier pieces in symbols, with what it means for strokes (offset), the bounding box and the path writer, each with its source or a derivation to write.
- Given the findings, when the spike ends, then the stories of F12 are written in delivery order, with acceptance criteria, and every handle-typing method found has a story so that each can be tried in the playground.

## Questions

- Which input methods for handles do Figma, Inkscape, Illustrator, Affinity, Penpot and CAD tools offer in their numeric panels, and which do users ask for?
- How does Figma store a vector network's handles (absolute, or relative to the vertex), and how does it keep a corner radius on a curved vertex?
- What is the exact bounding box of a cubic Bézier (extremes from the derivative), and how is a cubic offset for a stroke (approximation and tolerance)?
- Is a quarter of a circle or an ellipse written as one cubic with the 0.5523 constant, and with which error?

## Tasks

One task = one commit, referenced as `SP-011.Tn`.

- [ ] T1 — Read the sources (library first, then online), record them (3 h)
- [ ] T2 — Research note: ways to type a handle, the point's model, curves and strokes (2 h)
- [ ] T3 — ADR proposal superseding ADR-0001 for symbols; the stories of F12 written (2 h)
