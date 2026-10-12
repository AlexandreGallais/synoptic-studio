---
id: CHK-008
epic: E01
feature: F10
title: Checkpoint in the middle of F10
status: done
points: 1
---

# E01 · F10 · CHK-008 — Checkpoint in the middle of F10

Pause between the model of the round shape and its playground: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-008.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)

## Light evolvability check (2026-10-12)

| Question            | Answer                                                                                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 109 modules in `src/` (model 54, geometry 30, math 15, io 6, render 4); 642 tests; `model` holds three kinds of shape now: its subfolders are overdue (improvement log) |
| Tools               | StrykerJS still blocked; seven mutations of EN-016 caught by the review                                                                                                 |
| Fitness functions   | `shapeKind` is the one place that tells shapes apart: the playground's `"anchor" in model` tests must go through it in US-020                                           |
| Patterns            | the round shape reuses `rectangleCorners` and `rectangleAsFreeShape`: no new geometry                                                                                   |
| Agent configuration | state line updated; nothing new                                                                                                                                         |
| Process             | F10 started on the validation branch of F04 while its pull request waits for the Product Owner's merge: rebased on `main` afterwards                                    |

## Test cards so far

None yet: US-020 comes next.
