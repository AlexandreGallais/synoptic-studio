---
id: SP-028
epic: E02
feature: F39
title: Refine the moves of shapes
status: ready
points: 3
---

# E02 · F39 · SP-028 — Refine the moves of shapes

The spike that opens F39, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given Figma, Inkscape, Penpot, Sketch and Affinity Designer, when the spike ends, then how each moves a selection by the mouse (the press on a shape, a shape not selected pressed, a threshold before a drag starts) and by the arrow keys (the step, with Shift; a key held) is recorded from read sources, beside the default keys of `interaction.md` §1.
- Given the rules of 2026-10-22 (a drag moves freely, the anchor lands on integers at the release, the top-left of the straight box by default, the shape's own fractions kept) and the playground's code after F38 (`playground/selection/`, `playground/selection-drag/`, `playground/kept-shapes/`, `src/model/panel-values/`, `straightBoxShift`), when the spike ends, then F39's stories are written, 2 to 4 of at most 3 tasks each, criteria with numeric examples, the size of each checked against the code; a larger scope cut into a further feature, proposed with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- What moves when a shape not selected is pressed and dragged, and when one of several selected is?
- What step do the arrow keys move by, with and without Shift, and how does a turned shape land on integers?

## Tasks

One task = one commit, referenced as `SP-028.Tn`.

- [ ] T1 — The sources found, read and recorded, the research note (2 h)
- [ ] T2 — The code read, F39's stories written, the questions for the Product Owner listed (2 h)
