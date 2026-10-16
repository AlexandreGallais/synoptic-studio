---
id: EN-061
epic: E17
feature: F18
title: Modules per folder counted, no spread in a fold
status: ready
points: 1
---

# E17 · F18 · EN-061 — Modules per folder counted, no spread in a fold

Turn ADR-0043's signal and the spread found in F14 into checks (row 32 of RET-003; I26, I27; F18.AC18).

SP-020 ([research note 0022](../../research/notes/0022-f18-parallel-run.md)): a spread on a fold's accumulator costs `O(n^2)` (`REF-OXLINT-ACCUMULATING-SPREAD`), reported with ESLint's `no-restricted-syntax` and a selector (`REF-ESLINT-RESTRICTED-SYNTAX`); the fix Oxlint shows mutates the accumulator, which ADR-0014 forbids: `map`, `flatMap` or a loop over a local array instead.

## Acceptance criteria

- Given a folder of `src/` or `playground/` past 15 modules, when the tests run, then a test fails unless the folder is listed in `docs/conventions/files.md` with its reason.
- Given a `reduce` whose accumulator is spread at each step (`[...acc, x]`), when the lint runs, then it is reported.

## Tasks

One task = one commit, referenced as `EN-061.Tn`.

- [ ] T1 — The folder count test and the lint rule, mutation-checked (2 h)
