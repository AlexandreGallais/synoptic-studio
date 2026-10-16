---
id: EN-060
epic: E17
feature: F18
title: A documentation pass on E15's long pages
status: ready
points: 2
---

# E17 · F18 · EN-060 — A documentation pass on E15's long pages

Shorten the long pages of E15 with the `doc-editor` agent, after checking what ADR-0060 already removed (row 31 of RET-003; I24, I31, I39, I45; F18.AC17).

## Acceptance criteria

- Given `shapes.md` (Box, Text), Q37, Q38, `interaction.md` (Colors), `DERIV-text-box`, `DERIV-color-spaces` and notes 0017–0018, when the pass ends, then no bullet is longer than 400 characters and rules with several fields are tables.
- Given `cssFirstBaselineOffset`, when its TSDoc is read, then it uses one convention for the descender.

## Tasks

One task = one commit, referenced as `EN-060.Tn`.

- [ ] T1 — The doc-editor pass and its fixes (3 h)
