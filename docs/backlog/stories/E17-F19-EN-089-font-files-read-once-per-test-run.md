---
id: EN-089
epic: E17
feature: F19
title: Font files read once per test run
status: done
points: 2
---

# E17 · F19 · EN-089 — Font files read once per test run

Read each test font file (`tests/inter-regular.ttf` and the collections EN-067 builds from it) once, in one test that proves the reading, and give the other tests the bytes already read. Under the load of parallel agents, tests that each read and rebuild Inter went over Vitest's default limit of 5 s per test (`parse-collection-faces` 6.1 s, `parse-font-face` 5.6 s on 2026-10-17), while they take about 1 s alone.

Decided by the Product Owner on 2026-10-17: the reading of a file is tested once; the other tests use what was read, so that no test repeats the work of another. The 5 s limit stays: it catches a test that hangs, it is not a limit on reading. Sharing only what was read keeps each test's own claim: a test of a parser still calls that parser.

## Acceptance criteria

- Given the font fixtures, when the test run starts, then each font file is read from disk once per test worker, and one test per file proves that reading (its length and digest).
- Given a test of a parser or a builder, when it runs, then it calls the function it tests on the shared bytes and never reads the file itself; a fixture never holds a parser's result in the place of the call under test.
- Given the shared bytes, when a test runs, then it cannot change them (a frozen copy, or a fresh view each time), so no test depends on another's order.
- Given the test lane timed before and after on this Mac, when the story ends, then both durations are written in the story.

## Tasks

One task = one commit, referenced as `EN-089.Tn`.

- [ ] T1 — Shared, read-once font fixtures; one reading test per file; tests moved onto them (2 h)
- [ ] T2 — Timings before and after; no test reads a font file outside the fixtures (a test that checks it) (1 h)

## Timings

`npm run test` on this Mac, alone under the heavy lock, 2026-10-17; Vitest's own `Duration`, the lock's wait left out.

| Run | Duration | Imports (summed over workers) | Reads of `inter-regular.ttf` |
| --- | --- | --- | --- |
| before (`ee753b08`) | 101.5 s | 120.3 s | 193, one per test file |
| after T1 (read once, fixtures built lazily) | 99.3 s | 116.5 s | 1 |
| after T2 (large faces compared by digest) | 96.3 s | 115.3 s | 1 |

Alone with its two neighbours, the shared-tables test of `parseCollectionFaces` went from 2.3 s to 75 ms and is back under Vitest's 5 s; the three faces read by `parseFontFace` stay at about 2.6 s alone, the parser measuring Inter three times, and keep their 20 s.

## What EN-090 can reuse

- Vitest runs each test file in its own process (`isolate`, 193 distinct process ids read Inter): a module-level constant is rebuilt once per file, never once per worker. Data read once per run goes through a `globalSetup` and `provide`/`inject`; data built from it is built lazily at a file's first call (`builtOnce`), so the files that never ask for it pay nothing.
- TypeScript 7 cannot augment Vitest's `ProvidedContext` (TS2300, duplicate identifier; TypeScript 6 can): `provide` and `inject` are called through `Reflect.apply`, the value checked by `instanceof` at the reading side, with no `as` and no rule disabled.
- Shared bytes are given as a fresh copy at each call (`new Uint8Array(…)`, a copy of 411 640 bytes costs microseconds); shared number arrays are frozen once (`Object.freeze`), which keeps their identity, so that a collection still writes a shared table once.
- `toEqual` on typed arrays of hundreds of kilobytes compares element by element and was the slow part, not the parser: compare such files by length and SHA-256 (`partsDigest`) — the same claim, 30 times faster.
- The largest shares of the lane are elsewhere: `setup` 36 % (`vitest.setup.ts` imports `@typescript-eslint/rule-tester` into each of the 492 files, though the ESLint tests alone need it) and `import` 28 % (the `tests` barrel loads every fixture module into every file); Vitest itself estimates 22 s saved with `isolate: false`.
