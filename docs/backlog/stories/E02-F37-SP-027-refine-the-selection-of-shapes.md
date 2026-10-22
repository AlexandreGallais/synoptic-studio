---
id: SP-027
epic: E02
feature: F37
title: Refine the selection of several shapes
status: ready
points: 3
---

# E02 · F37 · SP-027 — Refine the selection of several shapes

The short spike that opens F37 (ADR-0086), longer than F35's and F36's: the Product Owner asked on 2026-10-22 for a feature's research to be done in full before its stories, reading the code each will touch, so that every question is asked at once and the feature's size is known before any code (`docs/process/improvements.md`). It also holds the research on the usual shortcuts of graphic editors that E02's table set for this feature (Product Owner, 2026-10-22).

## Acceptance criteria

- Given Figma, Inkscape, Illustrator, Photoshop, Penpot, Sketch and Affinity Designer, when the spike ends, then the shortcuts for selecting, adding and removing from a selection, the selection rectangle (touching or enclosing), selecting all, deselecting, deleting, nudging by the arrow keys (and with Shift), undo and redo are recorded from read sources, and a table of defaults is written in `interaction.md` for E02, E18 and E19, Figma's as the default where they differ (`interaction.md` §1, Keys editable).
- Given the playground's code after F36 (`playground/selection/`, `playground/kept-shapes/`, the panel), when the spike ends, then F37's stories are written, 2 to 4 of at most 3 tasks each with US-066 among them, their criteria with numeric examples, the size of each checked against the code each will touch; a larger scope is cut into a further feature, proposed with an example.
- Given what the sources and the code leave open — the panel for several shapes (x, y, width, height, Mixed), what the rectangle selects, how the selection shows — when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- Which keys and gestures select, add, remove, select all and deselect in each editor, and which default the editor takes?
- Does a selection rectangle take the shapes it touches or those it encloses, and does a modifier change that?
- What does the panel show and accept for several shapes, and how is Mixed typed over?

## Tasks

One task = one commit, referenced as `SP-027.Tn`.

- [ ] T1 — The sources found, read and recorded, the research note, the table of shortcuts (2 h)
- [ ] T2 — The code read, F37's stories written, the questions for the Product Owner listed (2 h)
