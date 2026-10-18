---
id: EN-129
epic: E17
feature: F29
title: Timing tests that hold under load
status: ready
points: 2
---

# E17 · F29 · EN-129 — Timing tests that hold under load

`src/model/text-wrap/wrapped-paragraph.test.ts` "wraps 16 000 words in well under a second" took 1.14 s against its 1 s limit on 2026-10-18, and fails about one push in three; `[F18.AC19]` once took 12.3 s against 5 s at a load average near 60 (card of 2026-10-18; C8, I51). The rule is a limit at least four times the test's duration alone, a loaded `check:all` running tests up to 2.5 times slower (`.claude/rules/tooling.md`, EN-036, EN-054); the limits of `scan.test.ts` (1 s), `without-trailing-spaces.test.ts` (500 ms), `wrapped-paragraph.test.ts` (1 s) and `parse-font-face.test.ts` (400 ms) were never written with their measure.

## Acceptance criteria

- Given a test of `src/` or `tests/` with a `timeout`, when `[F29.AC1]` reads it, then a comment beside the limit gives the test's duration alone, measured as the median of 5 runs of its file alone (`alone: 210 ms`), and the limit is at least four times it; a limit without its measure, or below four times it, fails naming the file and the line.
- Given every such test, when its limit is set from the measure, then `check:all` passes 5 times in a row with the tests of `src/` run beside the other lanes, the load average read and written in the story's report.
- Given a test whose work, at four times its duration alone, would exceed the 5 s Vitest default, when its limit is set, then the test keeps its size and its limit says so in its comment, never a smaller input chosen to fit.

## Tasks

- [ ] T1 — `[F29.AC1]` in `tests/fitness.test.ts`, failing on the limits written without their measure (1 h)
- [ ] T2 — Each timing test measured alone, its limit and comment set, `check:all` run 5 times (2 h)

One task = one commit, referenced as `EN-129.T1` in the commit footer.
