---
id: CHK-017
epic: E15
feature: F15
title: Checkpoint in the middle of F15
status: ready
points: 1
---

# E15 · F15 · CHK-017 — Checkpoint in the middle of F15

Pause between the font stories and the HTML layer of F15: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given EN-039 and EN-040 merged, when the checkpoint runs, then `docs/domain/shapes.md` (Fonts from files, Embedded fonts), ADR-0053, `DERIV-font-embedding`, the references, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for VAL-017.

## Tasks

One task = one commit, referenced as `CHK-017.Tn`.

- [ ] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [ ] T2 — Light evolvability check; improvement log; test cards listed (30 min)
