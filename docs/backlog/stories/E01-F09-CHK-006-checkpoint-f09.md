---
id: CHK-006
epic: E01
feature: F09
title: Checkpoint in the middle of F09
status: ready
points: 1
---

# E01 · F09 · CHK-006 — Checkpoint in the middle of F09

Pause between the free shape in the model and the rectangle in the playground: bring documentation and methods up to date, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.

## Tasks

One task = one commit, referenced as `CHK-006.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [x] T2 — Light evolvability check; improvement log (0.5 h)

## Light evolvability check (2026-10-11)

| Question            | Answer                                                                                                                                                                                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 82 modules in `src/` (model 34, geometry 27, math 11, io 6, render 4); 414 tests; `model` now holds shapes, radii and the selection of points: split it into subfolders when the free shape grows in F08 (improvement log) |
| Tools               | StrykerJS still blocked; mutations by hand; `knip` left to the E01 retrospective                                                                                                                                           |
| Fitness functions   | the rectangle tests now run through the free shape: a regression net for the model change of ADR-0032                                                                                                                      |
| Patterns            | `stretchToSize` and `fitInBox` are two box-to-box maps on purpose (note 0008); a shared helper only if a third one comes                                                                                                   |
| Agent configuration | two library pitfalls added (multiply first for exact edges; decisions on a paused branch)                                                                                                                                  |
| Process             | a feature inserted mid-run (F09 before F08) after the Product Owner's decision; F08 paused with its spike merged on its own branch                                                                                         |
