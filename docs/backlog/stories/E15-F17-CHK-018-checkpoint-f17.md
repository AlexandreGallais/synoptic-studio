---
id: CHK-018
epic: E15
feature: F17
title: Checkpoint in the middle of F17
status: ready
points: 1
---

# E15 · F17 · CHK-018 — Checkpoint in the middle of F17

Pause between the removals and the fixes of F17: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is, with no balanced wrap, no word broken at a character and no HTML layer left.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-018.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (1 h)
