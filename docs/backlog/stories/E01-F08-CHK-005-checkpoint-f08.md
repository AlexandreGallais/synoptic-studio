---
id: CHK-005
epic: E01
feature: F08
title: Checkpoint in the middle of F08
status: draft
points: 1
---

# E01 · F08 · CHK-005 — Checkpoint in the middle of F08

Pause between moving points and adding or deleting them: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-005.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)
