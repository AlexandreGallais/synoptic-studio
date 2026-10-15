---
id: CHK-015
epic: E15
feature: F14
title: Checkpoint in the middle of F14
status: draft
points: 1
---

# E15 · F14 · CHK-015 — Checkpoint in the middle of F14

Pause between the model stories and the playground stories of F14: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given EN-033 to EN-036 merged, when the checkpoint runs, then `docs/domain/shapes.md` (Text), `DERIV-text-box`, the references, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for VAL-015.

## Tasks

One task = one commit, referenced as `CHK-015.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (30 min)
