---
id: EN-089
epic: E17
feature: F19
title: Font files read once per test run
status: ready
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
