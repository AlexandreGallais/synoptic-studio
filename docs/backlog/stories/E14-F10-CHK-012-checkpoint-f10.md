---
id: CHK-012
epic: E14
feature: F10
title: Checkpoint in the middle of F10
status: done
points: 1
---

# E14 · F10 · CHK-012 — Checkpoint in the middle of F10

Check that code, docs, `AGENTS.md`, `CLAUDE.md` and rules agree once the round shape is drawn, before its conversions.

## Acceptance criteria

- Given the stories done so far, when the checkpoint runs, then the domain, the ADRs and the code agree, and the light evolvability check is recorded.
- Given the test cards written so far, when the checkpoint runs, then they are listed for VAL-012.

## Tasks

One task = one commit, referenced as `CHK-012.Tn`.

- [x] T1 — Consistency pass and evolvability check (1 h)

## Light evolvability check (2026-10-14)

| Question            | Answer                                                                                                                                                                       |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 197 modules in `src/` (geometry 98, model 68, math 21, io 6, render 4), 111 in the playground; 1 115 tests; every folder at 15 modules or fewer (`model/free-shape` at 15)   |
| Tools               | StrykerJS still waits; the verifier's probe tests found two bugs in US-029 that the story's own tests missed                                                                 |
| Fitness functions   | a property per math and geometry function holds for the new `arcHandleLength` and `ellipseCorners`; the box of the ellipse is its own box, a property                        |
| Patterns            | `"kind" in model` tells the round shape apart in the playground, `isFreeShape` the free shape; `stretchedToEllipse` repeats the per-axis product of `stretchedHandles` (F13) |
| Agent configuration | `AGENTS.md` and `CLAUDE.md` at 143 lines; its State line names F10 then F13                                                                                                  |
| Process             | the math expert reviewed the spike, the auditor EN-027, the verifier US-029; each changed something                                                                          |

## Test cards so far

- US-029 — the Round tool: a circle, an ellipse, turned, its points not edited.

Consistency: the domain ("Round shape (F10)"), ADR-0048, `DERIV-round-shape-curve`, `DERIV-round-shape` and the code agree; Q4 says the ellipse is settled; `docs/conventions/files.md` lists the new folders.
