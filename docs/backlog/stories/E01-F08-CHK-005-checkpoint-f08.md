---
id: CHK-005
epic: E01
feature: F08
title: Checkpoint in the middle of F08
status: done
points: 1
---

# E01 · F08 · CHK-005 — Checkpoint in the middle of F08

Pause between moving points and adding or deleting them: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-005.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)

## Light evolvability check (2026-10-11)

| Question            | Answer                                                                                                                                                                              |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 86 modules in `src/` (model 38, geometry 27, math 11, io 6, render 4); 45 playground modules; 448 tests; the subfolders of `model` stay for the E01 retrospective (improvement log) |
| Tools               | StrykerJS still blocked; mutations by hand (five on US-014, all caught); `--cooldown` for version checks waits for the review of versions at the end of E01                         |
| Fitness functions   | a stored playground value is checked again on reading (`isValidFreeShape`): a refused value never reaches the drawing                                                               |
| Patterns            | two playground copies merged (`readShapeKind`, `writeInputValue`); the playground state spread over DOM attributes asks for a state module (improvement log)                        |
| Agent configuration | the auditor found two crashes the tests missed: every playground story now tests a refused shape with each new control                                                              |
| Process             | the run goes on as planned: US-015 (midpoint), US-016 (delete), AUD-005, VAL-005                                                                                                    |

## Test cards for the next stop

- [US-014](./E01-F08-US-014-convert-and-move.md#product-owner-test) — convert a polygon, move a point by its x, y.
