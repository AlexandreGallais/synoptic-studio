---
id: CHK-000
epic: E00
feature: F00
title: Checkpoint in the middle of F00
status: draft
points: 1
---

# E00 · F00 · CHK-000 — Checkpoint in the middle of F00

Pause between the geometry and the rendering stories of F00: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-000.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)
