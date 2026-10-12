---
id: US-002
epic: E01
feature: F01
title: Draw a sharp rectangle in the playground
status: done
points: 2
---

# E01 · F01 · US-002 — Draw a sharp rectangle in the playground

As a symbol designer, I want to type a width and a height and see the rectangle so that I can check the result immediately.

## Acceptance criteria

- Given the playground open, when I type width 120 and height 80, then a 120 × 80 rectangle is drawn as one `<path>`.
- Given the playground open, when I type a value, then the panel shows the model and the `d` attribute of each pipeline stage.
- Given a non-integer input, when I type it, then it is not accepted (integers only).

## Tasks

One task = one commit, referenced as `US-002.Tn`.

- [x] T1 — Add the SVG element creation procedures in `src/render/` (`createSvgElement`, `createPathElement`, `@see REF-MDN-CREATEELEMENTNS`) (2 h)
- [x] T2 — Build the playground page: width and height inputs, canvas, pipeline panel (3 h)
- [x] T3 — Demonstrate the criteria to the Product Owner (0.5 h) — in the pull request: `npm run dev`, then type sizes
