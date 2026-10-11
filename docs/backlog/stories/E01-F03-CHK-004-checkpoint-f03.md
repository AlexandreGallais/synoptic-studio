---
id: CHK-004
epic: E01
feature: F03
title: Checkpoint in the middle of F03
status: ready
points: 1
---

# E01 · F03 · CHK-004 — Checkpoint in the middle of F03

Pause between the model and the points mode: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-004.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)

## Light evolvability check (2026-10-11)

| Question            | Answer                                                                                                                                                                                                    |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 65 modules in `src/` (geometry 26, model 18, math 11, io 6, render 4); 373 tests; `check:all` about 33 s; the playground grew to 27 files once its page code was split to stay within the limits (EN-010) |
| Tools               | StrykerJS still blocked (issue 6210); mutations by hand, each review's mutants caught; `knip` left to the E01 retrospective                                                                               |
| Fitness functions   | nothing new; `areValidRadii` and `resizeRadii` have properties; `commonRadius` and `cornersWithRadii` are tested by example                                                                               |
| Patterns            | the playground keeps page state in DOM attributes (`data-step`, `data-radii`): fine for a demo, a state module will be needed by the editor applications (improvement log)                                |
| Agent configuration | `CLAUDE.md` 126 lines; three library pitfalls added (playground limits, `dataset` autofix, `expect.closeTo` in objects)                                                                                   |
| Process             | 3 stories merged by the run (#79, #80, #81); every review changed its story, one real bug caught in US-011 (a removed vertex's radius coming back)                                                        |

## Product Owner test cards so far

- US-011: nothing new to see yet; checks that the radius still rounds every corner alike (`?step=10`). Mixed becomes reachable with US-013.
