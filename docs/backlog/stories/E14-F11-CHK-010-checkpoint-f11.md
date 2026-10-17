---
id: CHK-010
epic: E14
feature: F11
title: Checkpoint in the middle of F11
status: done
points: 1
---

# E14 · F11 · CHK-010 — Checkpoint in the middle of F11

Pause before the folders move: bring documentation and methods up to date (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/conventions/`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer.

## Tasks

One task = one commit, referenced as `CHK-010.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration (1 h)
- [x] T2 — Light evolvability check; improvement log (0.5 h)

## Light evolvability check (2026-10-13)

| Question | Answer |
| --- | --- |
| Size | 117 modules in `src/`, barrels and tests not counted (model 49, geometry 42, math 17, io 5, render 4), 77 in the playground plus 20 test actions; 830 tests; `check:all` at about 1 min 20 s; EN-022 splits model, geometry and playground by theme (ADR-0042) |
| Tools | StrykerJS still blocked (issue 6210 open on 2026-10-13); the verifier's mutations by hand found two survivors in EN-020 and three in EN-021, all killed now |
| Fitness functions | every math and geometry function has a property in its own test file, checked so that an empty list fails (EN-020); the folder limit of ADR-0042 becomes a test in EN-022 |
| Patterns | test helpers and fixtures shared (EN-021); planning EN-022 showed value imports between playground panels in both directions (shape ↔ points, shape ↔ position, shape ↔ guided test): the selection and position readers move with EN-022 so that panels import one way |
| Agent configuration | `AGENTS.md` and `CLAUDE.md` at 143 lines; a worktree linked to the main `node_modules` ran no Git hook (now in `docs/tooling/agent.md`); the unused-imports rule reads JSDoc words as uses (improvement log, a question for the Product Owner) |
| Process | the run went SP-010, EN-020, EN-021 with an auditor and a verifier on each; their reviews changed ADR-0042 before its merge (question to the Product Owner) and added the tests that killed surviving mutations |

No user story in F11: no test card for the next stop.
