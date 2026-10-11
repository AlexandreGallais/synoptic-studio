---
id: CHK-006
epic: E01
feature: F09
title: Checkpoint in the middle of F09
status: ready
points: 1
---

# E01 · F09 · CHK-006 — Checkpoint in the middle of F09

Pause between the free shape in the model and the rectangle in the playground: bring documentation and methods up to date, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.

## Tasks

One task = one commit, referenced as `CHK-006.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [ ] T2 — Light evolvability check; improvement log (0.5 h)
