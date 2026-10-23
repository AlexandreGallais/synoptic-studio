---
id: SP-030
epic: E02
feature: F42
title: Refine zoom and pan
status: ready
points: 3
---

# E02 · F42 · SP-030 — Refine zoom and pan

The spike that opens F42, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given the two ways the Product Owner allows the view to change (2026-10-23) — the drawing's `viewBox`, and a CSS transform of the whole SVG — and never the shapes' paths or coordinates, when the spike ends, then each is described from read sources (the SVG and CSS specifications, MDN on `viewBox`, `WheelEvent` and its `deltaMode`, `getScreenCTM`, Figma's, Inkscape's and Penpot's zoom and pan), with what it costs for this project: the two SVGs kept aligned, the pointer's point read back in the drawing's units, the export unchanged, the text's rendering and its HTML layer, and the way F42 takes is chosen with its reasons, in an ADR if it is structural.
- Given the playground's code after F41 (`VIEW_SCALE = 1` and each help's screen sizes divided by it, the pointer's points, the layer above the drawing, the box frames and the points' handles still drawn in the drawing's SVG), when the spike ends, then the zoom about the pointer, the pan, fitting the drawing to the window, the keys and the wheel (`interaction.md` §1), the zoom's bounds and steps, and the helps drawn only within the area shown and its padding are written, and F42's stories, 2 to 4 of at most 3 tasks each, criteria with numeric examples; a larger scope proposed as a split, with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- `viewBox` or a CSS transform: which keeps the two layers aligned, the pointer exact and the export unchanged at the least cost?
- Which keys, wheel gestures and bounds do graphic editors use for zoom and pan?
- How are the helps drawn only within the area shown, and how wide is its padding?

## Tasks

- [ ] T1 — Sources read, the code read, the research note, the questions for the Product Owner, F42's stories drafted (3 h)

One task = one commit, referenced as `SP-030.T1`.
