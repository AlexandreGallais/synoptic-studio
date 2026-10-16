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

- [x] T1 — The folder count test and the lint rule, mutation-checked (2 h)

## For the main thread

- Technical choices made by the worker: the folder count was already enforced by `local/folder-layout`; `tests/folders.test.ts` now also walks `src/` and `playground/` on disk and fails on a folder past 15 missing from `docs/conventions/files.md` or listed there without a reason, so the check holds without the lint. The spread in a fold is one more `no-restricted-syntax` selector in `RESTRICTED_SYNTAX` (`eslint/settings/identifiers.ts`), so it applies to every linted file, `src/` included through `LIBRARY_RESTRICTED_SYNTAX`; its tests are in `eslint/settings/identifiers.test.ts`.
- A selector cannot compare the spread name with the accumulator's: any spread into the array or object a `reduce` or `reduceRight` callback returns is reported, the accumulator's or not. A spread passed to a call (`Math.max(...row)`) or in the initial value passes. A callback given by name (`list.reduce(step, [])`) is not followed.
- Code the selector reported, fixed without a disable: `scripts/check-all.ts` (each lane's results, now pushed into a local list while the checks run one after the other) and `tests/font-fixtures.ts` (a collection's table offsets, now shifted in place in a local copy of each face). No `eslint-disable` added.
- Mutation-checked: a reason emptied in `files.md`, and the walk's test inverted, failed the new folder test; the selector renamed to `reduceX` failed its four reporting cases.

### Journal

- EN-061 turned ADR-0043's 15-module signal into a test that counts every folder on disk, and the spread found in F14 into a lint selector; two places in the tooling copied their accumulator at every step and now build into a local list.

### Improvements

- I26 and I27 handled by EN-061 (modules per folder counted by a test; no spread in a fold).
- F19 carries the rule to Oxc with Oxlint's own `oxc/no-accumulating-spread` (`REF-OXLINT-ACCUMULATING-SPREAD`), which resolves the accumulator by name and follows loops too, then drops the selector; its message suggests mutating the accumulator, so the project's message (`map`, `flatMap`, a local list) stays in `docs/conventions/functions.md`. The folder test is plain Vitest and moves unchanged.
