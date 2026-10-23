---
id: SP-029
epic: E02
feature: F40
title: Refine undo and redo
status: ready
points: 3
---

# E02 · F40 · SP-029 — Refine undo and redo

The spike that opens F40, its research done in full before its stories (Product Owner, 2026-10-22): the sources, the code each story will touch, every question at once, the feature's size known before any code.

## Acceptance criteria

- Given the two ways a history is built — commands each undone by its inverse, and states of the document kept and restored (snapshots, persistent data sharing what did not change) — when the spike ends, then each is described from read sources (the Command and Memento patterns, Figma's, Inkscape's, Penpot's, a text editor's, immutable data structures), with what it costs in code and in memory for this project, whose model is plain data that no function modifies (ADR-0014, ADR-0025), and the way F40 takes is chosen with its reasons, in an ADR.
- Given the rules of 2026-10-22 (one step per finished action, selection never a step, a memory size set by the user) and the playground's code after F39 (every place that changes the drawing: placing, drawing, moving, nudging, deleting, typing in the panel, editing points, a text), when the spike ends, then how the memory used is measured, what happens at the limit, which actions group into one step (a held arrow key, a text typed), and where undo and redo are offered (keys of `interaction.md` §1, buttons) are written, and F40's stories, 2 to 4 of at most 3 tasks each, criteria with numeric examples; a larger scope proposed as an epic of its own, with an example.
- Given what the sources and the code leave open, when the spike ends, then every question for the Product Owner is listed at once, each with options and a numeric example.

## Questions

- Commands or states: which does a plain-data model, never modified in place, call for?
- How is a history's memory size measured in a browser, and what is dropped when it is reached?
- Which actions make one step: a key held, several values typed, a text's characters?

## Tasks

One task = one commit, referenced as `SP-029.Tn`.

- [ ] T1 — The sources found, read and recorded, the research note, the ADR proposed (2 h)
- [ ] T2 — The code read, F40's stories written, the questions for the Product Owner listed (2 h)
