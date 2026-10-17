---
id: CHK-009
epic: E01
feature: F06
title: Checkpoint in the middle of F06
status: done
points: 1
---

# E01 · F06 · CHK-009 — Checkpoint in the middle of F06

Pause between the model and the playground: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-009.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (0.5 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (0.5 h)

## Light evolvability check (2026-10-12)

| Question | Answer |
| --- | --- |
| Size | 116 modules in `src/` (model 49, geometry 40, math 17, io 6, render 4); 662 tests; `geometry` grew by 10 small files for one rule, each one export as ADR-0019 wants |
| Tools | mutations by the auditor caught one real gap (the pentagon's highest scale untested) and one unobservable clamp, both fixed; StrykerJS still blocked |
| Fitness functions | the bisections rest on two monotonic properties checked by sweeps of 6 001 values per number of corners, not proved: they run in every test run, about 0.3 s; proving them is in the improvement log |
| Patterns | a rule found by bisection on a sound interval: `bisection` is generic and reusable (a rule of E13's layout or E14's curves may need it) |
| Agent configuration | `CLAUDE.md` state line brought up to date (126 lines); three pitfalls added to `.claude/rules/library.md` (a required model field, bisection precision, an unobservable clamp) |
| Process | the Product Owner chose the rule from a research answer with three options and numbers: the questionnaire with numeric examples worked; a research request cost one round trip |

## Test cards so far

None yet: US-021 comes after this checkpoint.
