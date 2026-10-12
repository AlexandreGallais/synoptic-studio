---
id: CHK-008
epic: E01
feature: F10
title: Checkpoint in the middle of F10
status: ready
points: 1
---

# E01 · F10 · CHK-008 — Checkpoint in the middle of F10

Pause between the model of the round shape and its playground: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-008.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)
