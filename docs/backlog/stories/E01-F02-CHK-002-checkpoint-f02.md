---
id: CHK-002
epic: E01
feature: F02
title: Checkpoint in the middle of F02
status: done
points: 1
---

# E01 · F02 · CHK-002 — Checkpoint in the middle of F02

Pause between the polygon contour and the rounding and playground stories of F02: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-002.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)

## Light evolvability check (2026-10-09)

| Question            | Answer                                                                                                                                                                                                                                                                                                                         |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Size                | 52 modules in `src/` (geometry 21, math 11, model 10, io 6, render 4); 287 tests; `check:all` 28 s with `deps:tools` (23 s at CHK-001); no layer close to a package split                                                                                                                                                      |
| Tools               | StrykerJS still blocked (stryker-js issue 6210 open); the auditor keeps the mutation role and every story added mutation checks. `knip` still waiting for the retrospective. New on `main`: `npm run deps:tools` (GitHub Actions, Node.js LTS), asked by the Product Owner; the feature branch was rebased on `main` to get it |
| Fitness functions   | nothing new; "every exported `math` / `geometry` function has a property test" holds for the three new geometry functions (and for `regularPolygonContour`) — still a candidate check                                                                                                                                          |
| Patterns            | test helpers repeat: `expectPoints` in two test files, the shoelace signed area in two; the size rule was duplicated and is now `isModelSize` (US-005 review) — improvement log                                                                                                                                                |
| Agent configuration | `CLAUDE.md` + rules 184 lines; state line and tooling rule updated; guards never hit in this run                                                                                                                                                                                                                               |
| Process             | the auditor now reviews before the `done` commit (EN-008 had to reorder its commits; `/run` steps 2 and 5 now say so); every review changed the story before merge (3 for 3)                                                                                                                                                   |

## Product Owner test cards so far

None yet in F02: US-005 has no playground of its own; US-007 will carry the card that also shows US-005 and US-006.
