---
id: EN-145
epic: E02
feature: F39
title: The playground fast on a large rounded shape
status: ready
points: 1
---

# E02 · F39 · EN-145 — The playground fast on a large rounded shape

The test `[F02.AC5] keeps 100 000 and smaller values exactly` of `playground/mount/mount-playground.test.ts` passes in 1 063 ms alone and timed out at Vitest's 5 s under the pre-push's load, 6.5 s three times on 2026-10-23. SP-028 found why ([note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md), §6): the rectangle, left by the test before it with a radius of 100 000, is 99 999 × 100 000, its corners quarter circles of radius 49 999.5, and the band outlining it, the shape being selected, follows each by chords of at most 2 screen pixels (`contourBand`, Q48), about 39 270 a corner. With the radius typed 0 first the test takes 11 ms. First in F39, so that the pre-push stops failing on it.

The band itself made fast went to E02's zoom and pan feature (Product Owner, 2026-10-23, SP-028's answer 8): the helps of the layer above the drawing are drawn only within the area shown and its padding, which that feature brings.

## Acceptance criteria

- Given the playground's test `[F02.AC5] keeps 100 000 and smaller values exactly`, when it runs alone, then it takes under 100 ms, the rectangle's radius typed 0 before its sizes, a comment giving the reason and the timings (1 063 ms with the radius of 100 000 the test before it leaves, 11 ms with 0); it still proves that 100 000 and 99 999 are kept exactly.

## Tasks

- [ ] T1 — The test of F02's fifth criterion made fast: the radius typed 0 before the sizes, the reason and the timings in a comment; timed alone before and after (1 h)

Files: `playground/mount/mount-playground.test.ts`.

One task = one commit, referenced as `EN-145.Tn` in the commit footer.
