---
id: SP-013
epic: E14
feature: F10
title: "Research for F10: an ellipse from four cubic sides"
status: ready
points: 1
---

# E14 · F10 · SP-013 — Research for F10: an ellipse from four cubic sides

Find, read and record the sources of F10's stories before their implementation (ADR-0020, research protocol §8): how a circle and an ellipse are drawn with cubic sides, how close they are, and why not elliptical arcs.

## Acceptance criteria

- Given the round shape, when the spike ends, then the handle length of a quarter circle, its largest radial gap and the ellipse as the circle stretched have a verified `REF-*` and a derivation with hand-computed checks.
- Given the choice between cubic sides and SVG elliptical arcs, when the spike ends, then an ADR records it with its reasons (ADR-0046 point 3 left it to F10).
- Given the findings, when the spike ends, then the stories' criteria cite them and a research note records them.

## Questions

- The handle length of a quarter circle as a cubic, and its error.
- The ellipse as an affine image of the circle: are the handles stretched with it?
- What Figma, Inkscape and SVG do with an ellipse; why not `A` commands.

## Tasks

One task = one commit, referenced as `SP-013.Tn`.

- [ ] T1 — Read the sources; research note, derivation `DERIV-round-shape-curve` and ADR-0048 (2 h)
