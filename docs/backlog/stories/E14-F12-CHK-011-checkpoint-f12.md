---
id: CHK-011
epic: E14
feature: F12
title: Checkpoint in the middle of F12
status: done
points: 1
---

# E14 · F12 · CHK-011 — Checkpoint in the middle of F12

Pause halfway through F12: bring documentation and methods up to date, check consistency, run the light evolvability check (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/conventions/`, the agents and the rules describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer.
- Given the open item of the improvement log on a cleaner run workflow, when the checkpoint runs, then the agent answers it.

## Tasks

One task = one commit, referenced as `CHK-011.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; the light evolvability check; the improvement log

## Light evolvability check (2026-10-13)

| Question | Answer |
| --- | --- |
| Size | 145 modules in `src/` (geometry 63, model 55, math 17, io 6, render 4), 106 in the playground; 968 tests; every folder under 15 modules but the listed `user-actions` (`local/folder-layout`) |
| Tools | StrykerJS still waits; the verifier's mutations by hand found survivors in each story of F12, all killed; a CodeQL alert on a script was fixed at the release check |
| Fitness functions | a property per math and geometry function (F11), the folder layout in the lint; E01's drawings unchanged with every handle off is a test (`curveContour` against `roundedContour`) |
| Patterns | `curveContour` and `roundedContour` share `filletArcs`; the playground's `readOnlySelected` exists twice (`points/`, `handles/`), three lines, kept for now |
| Agent configuration | `AGENTS.md` and `CLAUDE.md` at 143 lines; the math expert knew only ADR-0001, now ADR-0046 too; waiting loops on subagents are forbidden in the tooling rules |
| Process | `/run` is the go-ahead (ADR-0045); each story had an auditor and a verifier, the math expert on the geometry; their findings changed code in every story |

## Test cards so far

- US-022 — the Handles switch and the handles typed as x, y.
- US-023 — the handles typed as dx, dy.

## The cleaner run workflow (improvement log, 2026-10-13)

Answer: the branches left after a merge come from auto-merged story branches, which GitHub does not delete. GitHub's repository setting "Automatically delete head branches" would delete them at the merge; `main` and `develop` are protected by their rulesets, so it never deletes them. Proposed to the Product Owner, as a setting of the repository; until then `npm run branches:clean` and the API deletion after each merge keep the clone clean.
