---
id: CHK-016
epic: E15
feature: F16
title: Checkpoint in the middle of F16
status: ready
points: 1
---

# E15 · F16 · CHK-016 — Checkpoint in the middle of F16

Pause between the model stories and the playground story of F16: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given EN-037 and EN-038 merged, when the checkpoint runs, then `docs/domain/interaction.md` (Colors), `shapes.md` (Text), ADR-0056, `DERIV-color-spaces`, the references, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for VAL-016.

## Tasks

One task = one commit, referenced as `CHK-016.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (30 min)
