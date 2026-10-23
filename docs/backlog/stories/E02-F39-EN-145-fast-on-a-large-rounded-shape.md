---
id: EN-145
epic: E02
feature: F39
title: The playground fast on a large rounded shape
status: ready
points: 2
---

# E02 · F39 · EN-145 — The playground fast on a large rounded shape

The test `[F02.AC5] keeps 100 000 and smaller values exactly` of `playground/mount/mount-playground.test.ts` passes in 1 063 ms alone and timed out at Vitest's 5 s under the pre-push's load, 6.5 s three times on 2026-10-23. SP-028 found why ([note 0029](../../research/notes/0029-f39-move-shapes-on-integers.md), §6): the rectangle, left by the test before it with a radius of 100 000, is 99 999 × 100 000, its corners quarter circles of radius 49 999.5, and the band outlining it, the shape being selected, follows each by chords of at most 2 screen pixels (`contourBand`, Q48), about 39 270 a corner. With the radius typed 0 first the test takes 11 ms. First in F39, so that the pre-push stops failing on it; its second task waits for SP-028's question 8, a drag redrawing the band at each pointer move.

## Acceptance criteria

- Given the playground's test `[F02.AC5] keeps 100 000 and smaller values exactly`, when it runs alone, then it takes under 100 ms, the rectangle's radius typed 0 before its sizes, a comment giving the reason and the timings (1 063 ms with the radius of 100 000 the test before it leaves, 11 ms with 0); it still proves that 100 000 and 99 999 are kept exactly.
- Given the answer (a) to SP-028's question 8, the band about a rectangle 99 999 × 100 000 of radius 100 000 is drawn by about 393 chords a corner, each straying at most a tenth of a screen pixel from the curve, instead of about 39 270, and a corner of radius 10 by chords of about 2.8; the test `[F02.AC5] caps a width or a radius above 100 000…`, 559 ms alone, then under 100 ms too.

## Tasks

- [ ] T1 — The test of F02's fifth criterion made fast: the radius typed 0 before the sizes, the reason and the timings in a comment; timed alone before and after (1 h)
- [ ] T2 — With SP-028's answer (a) to question 8: the chords of the band chosen by their deviation, a tenth of a screen pixel, derived with the `math-expert` (`DERIV-*`), `contourBand` and its tests, the rule of the outline in `interaction.md` §2 and Q48; dropped with answer (b) or (c) (2 h)

Files: `playground/mount/mount-playground.test.ts`; with T2, `src/geometry/contour-band/` (`piece-samples.ts`), `playground/hover/create-outline-element.ts`, `docs/derivations/`, `docs/domain/interaction.md` §2 (The outline). `@see` `REF-SVG2-PAINT`, the derivation T2 writes.

One task = one commit, referenced as `EN-145.Tn` in the commit footer.
