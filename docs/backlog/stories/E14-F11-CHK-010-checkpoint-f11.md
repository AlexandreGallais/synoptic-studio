---
id: CHK-010
epic: E14
feature: F11
title: Checkpoint in the middle of F11
status: ready
points: 1
---

# E14 · F11 · CHK-010 — Checkpoint in the middle of F11

Pause before the folders move: bring documentation and methods up to date (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/conventions/`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer.

## Tasks

One task = one commit, referenced as `CHK-010.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration (1 h)
