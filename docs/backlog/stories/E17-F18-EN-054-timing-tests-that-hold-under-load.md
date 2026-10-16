---
id: EN-054
epic: E17
feature: F18
title: Timing tests that hold under load
status: done
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

- [x] T1 — F15.AC4 reworded in F15's file and its test; the font-list test made faster (1 h)

## Measures

Each test with a duration limit, run alone on 2026-10-16 (`npx vitest run <file> --reporter=verbose`), against its limit; the default limits are Vitest's, 5 s for a test and 10 s for a hook.

| Test                                                         | Alone  | Limit            | Ratio |
| ------------------------------------------------------------ | ------ | ---------------- | ----- |
| `[F15.AC4]` 1 000 texts measured (`parse-font-face.test.ts`) | 7 ms   | 400 ms (was 100) | 57    |
| `font-list-written.test.ts`, each test with its hook         | 0.27 s | 5 s and 10 s     | 18    |
| the same, before this story (590 faces)                      | 3.4 s  | 5 s and 30 s     | 1.5   |
| `check-targets.test.ts`, ESLint over every tracked file      | 4.3 s  | 30 s (was 20)    | 7     |
| `whole-values-to-hex.test.ts`, one color in 31 (hsb)         | 3.4 s  | 30 s             | 9     |
| `hsl-to-rgb.test.ts`, one color in 31                        | 1.3 s  | 20 s             | 15    |
| `hsb-to-rgb.test.ts`, one color in 31                        | 1.1 s  | 20 s             | 18    |
| `scan.test.ts`, 200 000 items                                | 74 ms  | 1 s              | 13    |

The font-list page test now lists 60 faces of 20 families: the 590 faces of 186 families stay counted once each by `distinct-family-names.test.ts`, a model test without a page. Both changed tests were broken once on purpose (a 1 ms limit, 20 options instead of 21) and failed.

## For the main thread

- Improvement log: I47, I74 and I75 handled by EN-054 (limits at least four times their measure alone, table above).
