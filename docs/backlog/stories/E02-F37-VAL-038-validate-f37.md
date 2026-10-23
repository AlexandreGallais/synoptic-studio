---
id: VAL-038
epic: E02
feature: F37
title: Validate F37
status: dropped
points: 1
---

# E02 · F37 · VAL-038 — Validate F37

The Product Owner validates F37 on the checklist page `?checklist=E02` and its demo page.

## Acceptance criteria

- Given F37's criteria, when the validation ends, then the Product Owner has tried each line of the checklist and each criterion is proven by a test and shown in a demo page.

## Decisions to confirm

- SP-027: the default keys follow Figma's where its help pages name them; where they do not, Delete or Backspace deletes the selection, as Inkscape and Penpot do, and Ctrl + Shift + Z or Ctrl + Y redoes, as Penpot, macOS, Inkscape and Windows do (`interaction.md` §1, The default keys, Q50). Chosen by the agent on 2026-10-22.

## Tasks

- [ ] T1 — The tests per criterion, the checklist lines and the demo page (2 h)

One task = one commit, referenced as `VAL-038.T1`.

## Dropped

Dropped on 2026-10-23: the Product Owner holds no validation per feature any more; a feature ends with its audit, and the validation is held when the Product Owner comes back and asks for one, before the merge into `main`, from the review's summary.
