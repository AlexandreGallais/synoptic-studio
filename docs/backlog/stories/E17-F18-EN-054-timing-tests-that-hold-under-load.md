---
id: EN-054
epic: E17
feature: F18
title: Timing tests that hold under load
status: draft
points: 1
---

# E17 · F18 · EN-054 — Timing tests that hold under load

Give the timing tests a margin that holds when the machine is loaded, and reword F15.AC4 (F15 is done: this story changes its criterion, ADR-0027; rows 6 of RET-003; I47, I74, I75; F18.AC10).

## Acceptance criteria

- Given `[F15.AC4]`, when the criterion is reworded, then it reads: 1 000 texts are measured in under 0.4 s, and its test checks that limit.
- Given `font-list-written.test.ts`, when it runs alone, then it takes well under its limit (a reduced list of families).
- Given every test with a duration limit, when `check:all` runs on a loaded machine, then none fails on time: each limit is at least four times its measure alone.

## Tasks

One task = one commit, referenced as `EN-054.Tn`.

- [ ] T1 — F15.AC4 reworded in F15's file and its test; the font-list test made faster (1 h)
